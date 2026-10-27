//! Sprints, burndown, velocity, standup digest and retrospective notes.

use crate::auth::MaybeAuthUser;
use crate::engineering as rules;
use axum::http::HeaderMap;
use loco_rs::prelude::*;
use sea_orm::{QueryOrder, QuerySelect};
use serde::Deserialize;
use uuid::Uuid;

use super::super::insights::item_ref;
use super::{conditional, refuse, task_view};
use crate::models::_entities::{audit_logs, sprint_notes, sprints, tasks};
use crate::models::audit_logs::Model as AuditModel;

/// `POST /api/plans/{pid}/sprints` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct SprintPayload {
    name: String,
    starts_on: chrono::NaiveDate,
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
    let txn = ctx.db.begin().await.map_err(super::super::db_err)?;
    let row = sprints::ActiveModel {
        pid: sea_orm::ActiveValue::set(Uuid::new_v4()),
        plan_pid: sea_orm::ActiveValue::set(item.pid),
        name: sea_orm::ActiveValue::set(payload.name.clone()),
        starts_on: sea_orm::ActiveValue::set(payload.starts_on),
        ends_on: sea_orm::ActiveValue::set(payload.ends_on),
        deleted_at: sea_orm::ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&txn)
    .await
    .map_err(|e| Error::Model(ModelError::from(e)))?;
    AuditModel::record(&txn, row.pid, "sprint_created", caller.actor(), None)
        .await
        .map_err(super::super::audit_failed)?;
    txn.commit().await.map_err(super::super::db_err)?;
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
    let rows = sprints::Entity::find()
        .filter(sprints::Column::PlanPid.eq(item.pid))
        .filter(sprints::Column::DeletedAt.is_null())
        .order_by_desc(sprints::Column::StartsOn)
        .all(&ctx.db)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?;
    format::json(rows)
}

/// Query for the burndown: the sprint to derive over.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct BurndownQuery {
    sprint: Uuid,
}

/// `GET /api/plans/{pid}/burndown?sprint=` — the honest sprint
/// burndown: remaining task count per day of the sprint window, from
/// real `done_at` stamps only. The response says so; an ideal line is
/// the client's to draw and label.
#[debug_handler]
pub(super) async fn burndown(
    axum::extract::Query(query): axum::extract::Query<BurndownQuery>,
    State(ctx): State<AppContext>,
    Path(pid): Path<String>,
    headers: HeaderMap,
) -> Result<Response> {
    let item = super::super::governance::find_item(&ctx, &pid).await?;
    let sprint = sprints::Entity::find()
        .filter(sprints::Column::Pid.eq(query.sprint))
        .filter(sprints::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?
        .filter(|s| s.plan_pid == item.pid)
        .ok_or(Error::NotFound)?;
    let sprint_tasks = tasks::Entity::find()
        .filter(tasks::Column::SprintPid.eq(sprint.pid))
        .filter(tasks::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?;
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
    let item_tasks = tasks::Entity::find()
        .filter(tasks::Column::PlanPid.eq(item.pid))
        .filter(tasks::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?;
    let task_pids: std::collections::HashSet<Uuid> =
        item_tasks.iter().map(|task| task.pid).collect();
    let titles: std::collections::BTreeMap<Uuid, &str> = item_tasks
        .iter()
        .map(|task| (task.pid, task.title.as_str()))
        .collect();

    let recent = audit_logs::Entity::find()
        .filter(audit_logs::Column::CreatedAt.gte(since))
        .order_by_desc(audit_logs::Column::CreatedAt)
        .limit(500)
        .all(&ctx.db)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?;
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

/// Find one live sprint of one plan, or 404.
pub(super) async fn find_sprint(
    ctx: &AppContext,
    plan_pid: Uuid,
    s_pid: &str,
) -> Result<sprints::Model> {
    let s_pid = super::super::parse_pid(s_pid)?;
    sprints::Entity::find()
        .filter(sprints::Column::Pid.eq(s_pid))
        .filter(sprints::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?
        .filter(|sprint| sprint.plan_pid == plan_pid)
        .ok_or(Error::NotFound)
}

/// `POST .../sprints/{s_pid}/notes` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct NotePayload {
    category: String,
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
    let txn = ctx.db.begin().await.map_err(super::super::db_err)?;
    let row = sprint_notes::ActiveModel {
        pid: sea_orm::ActiveValue::set(Uuid::new_v4()),
        sprint_pid: sea_orm::ActiveValue::set(sprint.pid),
        category: sea_orm::ActiveValue::set(payload.category.clone()),
        body: sea_orm::ActiveValue::set(payload.body.clone()),
        task_pid: sea_orm::ActiveValue::set(None),
        deleted_at: sea_orm::ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&txn)
    .await
    .map_err(|e| Error::Model(ModelError::from(e)))?;
    AuditModel::record(&txn, row.pid, "sprint_note_added", caller.actor(), None)
        .await
        .map_err(super::super::audit_failed)?;
    txn.commit().await.map_err(super::super::db_err)?;
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
    let rows = sprint_notes::Entity::find()
        .filter(sprint_notes::Column::SprintPid.eq(sprint.pid))
        .filter(sprint_notes::Column::DeletedAt.is_null())
        .order_by_asc(sprint_notes::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?;
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
    let note = sprint_notes::Entity::find()
        .filter(sprint_notes::Column::Pid.eq(n_pid))
        .filter(sprint_notes::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?
        .filter(|note| note.sprint_pid == sprint.pid)
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
    let txn = ctx.db.begin().await.map_err(super::super::db_err)?;
    let task = tasks::ActiveModel {
        pid: sea_orm::ActiveValue::set(Uuid::new_v4()),
        plan_pid: sea_orm::ActiveValue::set(item.pid),
        sprint_pid: sea_orm::ActiveValue::set(None),
        title: sea_orm::ActiveValue::set(note.body.clone()),
        description: sea_orm::ActiveValue::set(Some(format!(
            "from {} note in sprint `{}`",
            note.category, sprint.name
        ))),
        status: sea_orm::ActiveValue::set("todo".to_string()),
        assignee_ref: sea_orm::ActiveValue::set(None),
        points: sea_orm::ActiveValue::set(None),
        status_changed_at: sea_orm::ActiveValue::set(now.into()),
        done_at: sea_orm::ActiveValue::set(None),
        deleted_at: sea_orm::ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&txn)
    .await
    .map_err(|e| Error::Model(ModelError::from(e)))?;
    let note_pid = note.pid;
    let mut active: sprint_notes::ActiveModel = note.into();
    active.task_pid = sea_orm::ActiveValue::set(Some(task.pid));
    let updated = active
        .update(&txn)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?;
    AuditModel::record(
        &txn,
        note_pid,
        "sprint_note_converted",
        caller.actor(),
        Some(serde_json::json!({ "task_pid": task.pid })),
    )
    .await
    .map_err(super::super::audit_failed)?;
    txn.commit().await.map_err(super::super::db_err)?;
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
    let sprint_rows = sprints::Entity::find()
        .filter(sprints::Column::PlanPid.eq(item.pid))
        .filter(sprints::Column::DeletedAt.is_null())
        .order_by_asc(sprints::Column::StartsOn)
        .all(&ctx.db)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?;
    let task_rows = tasks::Entity::find()
        .filter(tasks::Column::PlanPid.eq(item.pid))
        .filter(tasks::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?;
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
