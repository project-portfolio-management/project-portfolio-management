//! Sprints, burndown, velocity, standup digest and retrospective notes.

use crate::auth::MaybeAuthUser;
use crate::engineering as rules;
use crate::prelude::*;
use axum::http::HeaderMap;
use serde::Deserialize;
use uuid::Uuid;

use super::super::insights::item_ref;
use super::{conditional, refuse, task_view};
use crate::models::_entities::{audit_logs, sprint_notes, sprints, tasks};
use crate::models::audit_logs::Model as AuditModel;

/// `POST /api/plans/{pid}/sprints` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct SprintPayload {
    /// Sprint name.
    name: String,
    /// First day of the sprint window (inclusive).
    starts_on: chrono::NaiveDate,
    /// Last day of the sprint window; must not precede `starts_on`.
    ends_on: chrono::NaiveDate,
}

/// `POST /api/plans/{pid}/sprints` — create a time-boxed
/// sprint (`ends_on` must not precede `starts_on`).
#[debug_handler]
pub(super) async fn create_sprint(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(payload): Json<SprintPayload>,
) -> Result<Response> {
    if payload.name.trim().is_empty() || payload.name.len() > crate::validation::MAX_TEXT_LEN {
        return Err(refuse("name is required (and capped)"));
    }
    if payload.ends_on < payload.starts_on {
        return Err(refuse("ends_on is before starts_on"));
    }
    let item = super::super::governance::find_item(&ctx, &pid).await?;
    let mut tx = crate::db::begin(&ctx).await?;
    let row: sprints::Model = sqlx::query_as(
        "INSERT INTO sprints (pid, plan_pid, name, starts_on, ends_on, deleted_at)
         VALUES ($1, $2, $3, $4, $5, NULL)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(item.pid)
    .bind(&payload.name)
    .bind(payload.starts_on)
    .bind(payload.ends_on)
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(tx.conn(), row.pid, "sprint_created", caller.actor(), None)
        .await
        .map_err(super::super::audit_failed)?;
    tx.commit().await?;
    format::json(row)
}

/// `GET /api/plans/{pid}/sprints` — the item's sprints, newest
/// first.
#[debug_handler]
pub(super) async fn list_sprints(
    State(ctx): State<AppContext>,
    Path(pid): Path<String>,
) -> Result<Response> {
    let item = super::super::governance::find_item(&ctx, &pid).await?;
    let rows: Vec<sprints::Model> = sqlx::query_as(
        "SELECT * FROM sprints WHERE plan_pid = $1 AND deleted_at IS NULL ORDER BY starts_on DESC",
    )
    .bind(item.pid)
    .fetch_all(crate::db::pool(&ctx))
    .await?;
    format::json(rows)
}

/// Query for the burndown: the sprint to derive over.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct BurndownQuery {
    /// Sprint to derive the burndown for; it must belong to the plan in the URL.
    sprint: Uuid,
}

/// `GET /api/plans/{pid}/burndown?sprint=` — the honest sprint
/// burndown: remaining task count per day of the sprint window, from
/// real `done_at` stamps only. The response says so; an ideal line is
/// the client's to draw and label.
#[debug_handler]
pub(super) async fn burndown(
    axa::extract::Query(query): axa::extract::Query<BurndownQuery>,
    State(ctx): State<AppContext>,
    Path(pid): Path<String>,
    headers: HeaderMap,
) -> Result<Response> {
    let item = super::super::governance::find_item(&ctx, &pid).await?;
    let sprint = live_sprint(&ctx, query.sprint)
        .await?
        .filter(|s| s.plan_pid == item.pid)
        .ok_or(Error::NotFound)?;
    let sprint_tasks: Vec<tasks::Model> =
        sqlx::query_as("SELECT * FROM tasks WHERE sprint_pid = $1 AND deleted_at IS NULL")
            .bind(sprint.pid)
            .fetch_all(crate::db::pool(&ctx))
            .await?;
    let done_dates: Vec<chrono::NaiveDate> = sprint_tasks
        .iter()
        .filter_map(|task| task.done_at.map(|at| at.to_utc().date_naive()))
        .collect();
    let points = rules::burndown(
        sprint_tasks.len(),
        &done_dates,
        sprint.starts_on,
        sprint.ends_on,
    );
    let body = serde_json::json!({
        "as_of": chrono::Utc::now(),
        "sprint": { "pid": sprint.pid, "name": sprint.name,
                     "starts_on": sprint.starts_on, "ends_on": sprint.ends_on },
        "total_tasks": sprint_tasks.len(),
        "derivation": "remaining = assigned tasks minus real done_at stamps on or \
                       before each day; no ideal line, no interpolation",
        "points": points,
    });
    conditional(&headers, &body)
}

/// `GET /api/plans/{pid}/standup` — the last-24h digest,
/// audit-derived: tasks created / moved / completed, current blockers,
/// risks raised. What a standup reads out, from what was recorded.
#[debug_handler]
pub(super) async fn standup(
    State(ctx): State<AppContext>,
    Path(pid): Path<String>,
    headers: HeaderMap,
) -> Result<Response> {
    let item = super::super::governance::find_item(&ctx, &pid).await?;
    let since = chrono::Utc::now() - chrono::Days::new(1);
    let item_tasks = plan_tasks(&ctx, item.pid).await?;
    let task_pids: std::collections::HashSet<Uuid> =
        item_tasks.iter().map(|task| task.pid).collect();
    let titles: std::collections::BTreeMap<Uuid, &str> = item_tasks
        .iter()
        .map(|task| (task.pid, task.title.as_str()))
        .collect();

    let recent: Vec<audit_logs::Model> = sqlx::query_as(
        "SELECT * FROM audit_logs WHERE created_at >= $1 ORDER BY created_at DESC LIMIT 500",
    )
    .bind(chrono::DateTime::<chrono::FixedOffset>::from(since))
    .fetch_all(crate::db::pool(&ctx))
    .await?;
    let mut moves = Vec::new();
    let mut created = Vec::new();
    let mut risks_raised = 0usize;
    for row in &recent {
        match row.action.as_str() {
            "task_moved" if task_pids.contains(&row.entity_pid) => {
                moves.push(serde_json::json!({
                    "at": row.created_at.to_utc(),
                    "task": titles.get(&row.entity_pid),
                    "actor": row.actor,
                    "move": row.snapshot,
                }));
            }
            "task_created" if task_pids.contains(&row.entity_pid) => {
                created.push(serde_json::json!({
                    "at": row.created_at.to_utc(),
                    "task": titles.get(&row.entity_pid),
                    "actor": row.actor,
                }));
            }
            "risk_raised" => {
                // Risk audits key the risk pid, not the item pid; a
                // per-item filter needs the risk rows — count only.
                risks_raised += 1;
            }
            _ => {}
        }
    }
    let blocked: Vec<serde_json::Value> = item_tasks
        .iter()
        .filter(|task| task.status == "blocked")
        .map(task_view)
        .collect();
    let body = serde_json::json!({
        "as_of": chrono::Utc::now(),
        "since": since,
        "item": item_ref(&item),
        "tasks_created": created,
        "tasks_moved": moves,
        "blocked_now": blocked,
        "risks_raised_estate_wide": risks_raised,
    });
    conditional(&headers, &body)
}

/// A plan's live tasks, unordered.
async fn plan_tasks(ctx: &AppContext, plan_pid: Uuid) -> Result<Vec<tasks::Model>> {
    Ok(
        sqlx::query_as("SELECT * FROM tasks WHERE plan_pid = $1 AND deleted_at IS NULL")
            .bind(plan_pid)
            .fetch_all(crate::db::pool(ctx))
            .await?,
    )
}

/// The live sprint with this pid, of any plan.
async fn live_sprint(ctx: &AppContext, s_pid: Uuid) -> Result<Option<sprints::Model>> {
    Ok(
        sqlx::query_as("SELECT * FROM sprints WHERE pid = $1 AND deleted_at IS NULL LIMIT 1")
            .bind(s_pid)
            .fetch_optional(crate::db::pool(ctx))
            .await?,
    )
}

/// Find one live sprint of one plan, or 404.
pub(super) async fn find_sprint(
    ctx: &AppContext,
    plan_pid: Uuid,
    s_pid: &str,
) -> Result<sprints::Model> {
    let s_pid = super::super::parse_pid(s_pid)?;
    live_sprint(ctx, s_pid)
        .await?
        .filter(|sprint| sprint.plan_pid == plan_pid)
        .ok_or(Error::NotFound)
}

/// `POST .../sprints/{s_pid}/notes` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct NotePayload {
    /// Retrospective note category; must be one of the allowed note categories.
    category: String,
    /// Text of the note.
    body: String,
}

/// `POST /api/plans/{pid}/sprints/{s_pid}/notes` — add a retro
/// / feedback note (`went_well` / `improve` / `action` / `feedback`).
#[debug_handler]
pub(super) async fn create_note(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, s_pid)): Path<(String, String)>,
    Json(payload): Json<NotePayload>,
) -> Result<Response> {
    if !rules::NOTE_CATEGORIES.contains(&payload.category.as_str()) {
        return Err(refuse(&format!(
            "category must be one of {:?}",
            rules::NOTE_CATEGORIES
        )));
    }
    if payload.body.trim().is_empty() || payload.body.len() > crate::validation::MAX_TEXT_LEN {
        return Err(refuse("body is required (and capped)"));
    }
    let item = super::super::governance::find_item(&ctx, &pid).await?;
    let sprint = find_sprint(&ctx, item.pid, &s_pid).await?;
    let mut tx = crate::db::begin(&ctx).await?;
    let row: sprint_notes::Model = sqlx::query_as(
        "INSERT INTO sprint_notes (pid, sprint_pid, category, body, task_pid, deleted_at)
         VALUES ($1, $2, $3, $4, NULL, NULL)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(sprint.pid)
    .bind(&payload.category)
    .bind(&payload.body)
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        "sprint_note_added",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::super::audit_failed)?;
    tx.commit().await?;
    format::json(row)
}

/// `GET /api/plans/{pid}/sprints/{s_pid}/notes` — the sprint's
/// notes, grouped-friendly (oldest first).
#[debug_handler]
pub(super) async fn list_notes(
    State(ctx): State<AppContext>,
    Path((pid, s_pid)): Path<(String, String)>,
) -> Result<Response> {
    let item = super::super::governance::find_item(&ctx, &pid).await?;
    let sprint = find_sprint(&ctx, item.pid, &s_pid).await?;
    let rows: Vec<sprint_notes::Model> = sqlx::query_as(
        "SELECT * FROM sprint_notes WHERE sprint_pid = $1 AND deleted_at IS NULL ORDER BY id",
    )
    .bind(sprint.pid)
    .fetch_all(crate::db::pool(&ctx))
    .await?;
    format::json(rows)
}

/// `POST /api/plans/{pid}/sprints/{s_pid}/notes/{n_pid}/convert`
/// — turn an `action` / `feedback` note into a task on the same item
/// (once; the note records the created task). Other categories are
/// observations, not work, and refuse.
#[debug_handler]
pub(super) async fn convert_note(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, s_pid, n_pid)): Path<(String, String, String)>,
) -> Result<Response> {
    let item = super::super::governance::find_item(&ctx, &pid).await?;
    let sprint = find_sprint(&ctx, item.pid, &s_pid).await?;
    let n_pid = super::super::parse_pid(&n_pid)?;
    let note: sprint_notes::Model =
        sqlx::query_as("SELECT * FROM sprint_notes WHERE pid = $1 AND deleted_at IS NULL LIMIT 1")
            .bind(n_pid)
            .fetch_optional(crate::db::pool(&ctx))
            .await?
            .filter(|note: &sprint_notes::Model| note.sprint_pid == sprint.pid)
            .ok_or(Error::NotFound)?;
    if !rules::CONVERTIBLE_NOTE_CATEGORIES.contains(&note.category.as_str()) {
        return Err(refuse(&format!(
            "only {:?} notes convert to tasks",
            rules::CONVERTIBLE_NOTE_CATEGORIES
        )));
    }
    if note.task_pid.is_some() {
        return Err(refuse("note is already converted"));
    }
    let now = chrono::Utc::now();
    let mut tx = crate::db::begin(&ctx).await?;
    let task: tasks::Model = sqlx::query_as(
        "INSERT INTO tasks
             (pid, plan_pid, sprint_pid, title, description, status, assignee_ref, points,
              status_changed_at, done_at, deleted_at)
         VALUES ($1, $2, NULL, $3, $4, 'todo', NULL, NULL, $5, NULL, NULL)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(item.pid)
    .bind(&note.body)
    .bind(format!(
        "from {} note in sprint `{}`",
        note.category, sprint.name
    ))
    .bind(chrono::DateTime::<chrono::FixedOffset>::from(now))
    .fetch_one(tx.conn())
    .await?;
    let note_pid = note.pid;
    // `fetch_one`: an update that finds no row is an error, as SeaORM's was.
    let updated: sprint_notes::Model =
        sqlx::query_as("UPDATE sprint_notes SET task_pid = $1 WHERE id = $2 RETURNING *")
            .bind(task.pid)
            .bind(note.id)
            .fetch_one(tx.conn())
            .await?;
    AuditModel::record_on(
        tx.conn(),
        note_pid,
        "sprint_note_converted",
        caller.actor(),
        Some(serde_json::json!({ "task_pid": task.pid })),
    )
    .await
    .map_err(super::super::audit_failed)?;
    tx.commit().await?;
    format::json(serde_json::json!({ "note": updated, "task": task_view(&task) }))
}

/// `GET /api/plans/{pid}/velocity` — per-sprint completed
/// counts and story-point sums, from real `done_at` stamps only.
/// Points are **team-local**: the served note says never to compare
/// them across teams or items.
#[debug_handler]
pub(super) async fn velocity(
    State(ctx): State<AppContext>,
    Path(pid): Path<String>,
    headers: HeaderMap,
) -> Result<Response> {
    let item = super::super::governance::find_item(&ctx, &pid).await?;
    let sprint_rows: Vec<sprints::Model> = sqlx::query_as(
        "SELECT * FROM sprints WHERE plan_pid = $1 AND deleted_at IS NULL ORDER BY starts_on",
    )
    .bind(item.pid)
    .fetch_all(crate::db::pool(&ctx))
    .await?;
    let task_rows = plan_tasks(&ctx, item.pid).await?;
    let sprints_view: Vec<serde_json::Value> = sprint_rows
        .iter()
        .map(|sprint| {
            let done: Vec<&tasks::Model> = task_rows
                .iter()
                .filter(|task| task.sprint_pid == Some(sprint.pid) && task.done_at.is_some())
                .collect();
            let pointed: i64 = done.iter().filter_map(|t| t.points).map(i64::from).sum();
            let unpointed = done.iter().filter(|t| t.points.is_none()).count();
            serde_json::json!({
                "sprint": { "pid": sprint.pid, "name": sprint.name,
                             "starts_on": sprint.starts_on, "ends_on": sprint.ends_on },
                "tasks_done": done.len(),
                "points_done": pointed,
                "unpointed_done": unpointed,
            })
        })
        .collect();
    let body = serde_json::json!({
        "as_of": chrono::Utc::now(),
        "note": "points are team-local; velocity is for this item's own trend only — \
                 never compare across teams or items",
        "sprints": sprints_view,
    });
    conditional(&headers, &body)
}
