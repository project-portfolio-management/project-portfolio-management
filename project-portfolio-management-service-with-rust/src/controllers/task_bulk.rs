//! Bulk task operations (RM-9): edit, copy and move many tasks in one call.
//! Pure rules in [`crate::task_bulk`].
//!
//! **One result per requested task**, so one bad item never aborts the rest
//! and the counts reconcile (`ok + refused == requested`). Every item goes
//! through the same read check as any task access (record-level ABAC and
//! private plans); every change is audited.
//!
//! - **edit** sets `assignee_ref`, `due_date`, `start_date` and/or `component_pid`
//!   (`status` is deliberately *not* bulk-editable: a status change must pass
//!   the workflow, WIP, role and blocker rules, which belong to the board
//!   move);
//! - **copy** makes fresh tasks in a plan (no transition history, assignee
//!   only if asked), records a `copied_from` relation, and carries the custom
//!   field values so a required field is never left empty;
//! - **move** re-homes tasks, re-validating against the **target** workflow:
//!   a status it lacks maps to its initial state **and the result says so**. A
//!   move changes which workflow applies, so it could step round a gate on
//!   either side: where the source or the target restricts moves, it needs an
//!   administrator.

use super::unprocessable;
use std::collections::BTreeMap;

use crate::prelude::*;
use serde::Deserialize;
use serde_json::Value;
use uuid::Uuid;

use super::discussion::resolve_subject;
use super::governance::valid_ref;
use crate::auth::MaybeAuthUser;
use crate::models::_entities::tasks;
use crate::models::audit_logs::Model as AuditModel;
use crate::models::capabilities as cap_models;
use crate::task_bulk::{self as rules, Change, ItemResult};

/// The human text of an error, for a per-item refusal note.
fn error_text(e: &Error) -> String {
    match e {
        Error::CustomError(_, detail) => detail
            .detail
            .clone()
            .or_else(|| detail.code.clone())
            .unwrap_or_else(|| "refused".to_string()),
        other => other.to_string(),
    }
}

/// Build the uniform bulk response: requested, ok and refused counts (which always add up) plus the per-task results.
fn summary(results: &[ItemResult]) -> Result<Response> {
    let ok = results.iter().filter(|r| r.result == "ok").count();
    format::json(serde_json::json!({
        "requested": results.len(),
        "ok": ok,
        "refused": results.len() - ok,
        "results": results,
    }))
}

/// Load a task the caller may read, or the refusal text.
async fn readable_task(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    pid: Uuid,
) -> std::result::Result<tasks::Model, String> {
    resolve_subject(ctx, caller, "task", &pid.to_string())
        .await
        .map_err(|_| "unknown task, or not readable by this caller".to_string())?;
    sqlx::query_as("SELECT * FROM tasks WHERE pid = $1 AND deleted_at IS NULL LIMIT 1")
        .bind(pid)
        .fetch_optional(crate::db::pool(ctx))
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "unknown task".to_string())
}

/// The fields a bulk edit may set.
const ALLOWED: &[&str] = &["assignee_ref", "due_date", "component_pid", "start_date"];

/// `POST /api/tasks/bulk` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct EditBody {
    /// Tasks to edit; duplicates are removed by `dedupe_ids` and an over-long list is refused.
    ids: Vec<Uuid>,
    /// Any of `assignee_ref`, `due_date`, `component_pid`, `start_date`; `null` clears.
    set: BTreeMap<String, Value>,
}

/// `POST /api/tasks/bulk` — set the same fields on many tasks.
#[debug_handler]
async fn bulk_edit(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(b): Json<EditBody>,
) -> Result<Response> {
    let ids = rules::dedupe_ids(&b.ids).map_err(|m| unprocessable(&m))?;
    if b.set.is_empty() {
        return Err(unprocessable("set must name at least one field"));
    }
    if let Some(bad) = b.set.keys().find(|k| !ALLOWED.contains(&k.as_str())) {
        return Err(unprocessable(&format!(
            "{bad:?} is not bulk-editable (allowed: {ALLOWED:?}); status changes go through the board move"
        )));
    }
    // Validate the values once, up front: a malformed value is the caller's
    // mistake, not a per-item outcome.
    let assignee = match b.set.get("assignee_ref") {
        None => Change::Keep,
        Some(Value::Null) => Change::Clear,
        Some(Value::String(s)) if valid_ref(s, &["worker", "person"]) => Change::Set(s.clone()),
        Some(_) => {
            return Err(unprocessable(
                "assignee_ref must be a person:/worker: URN or null",
            ));
        }
    };
    let due = match b.set.get("due_date") {
        None => Change::Keep,
        Some(Value::Null) => Change::Clear,
        Some(Value::String(s)) => Change::Set(
            chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
                .map_err(|_| unprocessable("due_date must be YYYY-MM-DD or null"))?,
        ),
        Some(_) => return Err(unprocessable("due_date must be YYYY-MM-DD or null")),
    };
    let component = match b.set.get("component_pid") {
        None => Change::Keep,
        Some(Value::Null) => Change::Clear,
        Some(Value::String(s)) => Change::Set(
            Uuid::parse_str(s)
                .map_err(|_| unprocessable("component_pid must be a UUID or null"))?,
        ),
        Some(_) => return Err(unprocessable("component_pid must be a UUID or null")),
    };

    let start = match b.set.get("start_date") {
        None => Change::Keep,
        Some(Value::Null) => Change::Clear,
        Some(Value::String(s)) => Change::Set(
            chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
                .map_err(|_| unprocessable("start_date must be YYYY-MM-DD or null"))?,
        ),
        Some(_) => return Err(unprocessable("start_date must be YYYY-MM-DD or null")),
    };
    if let Change::Set(d) = &start {
        let problems = crate::cpm::schedule_problems(Some(*d), None);
        if !problems.is_empty() {
            return Err(unprocessable(&problems.join("; ")));
        }
    }

    let edit = Edit {
        assignee,
        due,
        component,
        start,
    };
    let mut results = Vec::new();
    for pid in ids {
        results.push(edit_one(&ctx, &caller, pid, &edit, "task_bulk_edited", None).await);
    }
    summary(&results)
}

/// What one bulk edit does to each field.
pub(super) struct Edit {
    /// Assignee reference (a person URN); `Clear` unassigns.
    pub(super) assignee: Change<String>,
    /// Due date; `Clear` removes it.
    pub(super) due: Change<chrono::NaiveDate>,
    /// Component the task belongs to; `Clear` detaches it.
    pub(super) component: Change<Uuid>,
    /// Start date; `Clear` removes it.
    pub(super) start: Change<chrono::NaiveDate>,
}

/// Edit one task. `audit_kind` and `audit_detail` name the single audit entry
/// written (the bulk edit's own, or the caller's, e.g. leveling's).
pub(super) async fn edit_one(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    pid: Uuid,
    edit: &Edit,
    audit_kind: &str,
    audit_detail: Option<Value>,
) -> ItemResult {
    let (assignee, due, component) = (&edit.assignee, &edit.due, &edit.component);
    let task = match readable_task(ctx, caller, pid).await {
        Ok(t) => t,
        Err(why) => return ItemResult::refused(pid, why),
    };
    if let Change::Set(c) = component {
        // A lookup that fails counts as no such component, as before.
        let ok = sqlx::query(
            "SELECT 1 FROM components
             WHERE pid = $1 AND plan_pid = $2 AND deleted_at IS NULL
             LIMIT 1",
        )
        .bind(*c)
        .bind(task.plan_pid)
        .fetch_optional(crate::db::pool(ctx))
        .await
        .ok()
        .flatten()
        .is_some();
        if !ok {
            return ItemResult::refused(
                pid,
                "component_pid is not a component of this task's plan",
            );
        }
    }
    if *due != Change::Keep
        && let Err(e) =
            super::task_relations::check_due_change(ctx, pid, due.apply(task.due_date)).await
    {
        return ItemResult::refused(pid, error_text(&e));
    }
    let (new_assignee, new_due, new_component) = (
        assignee.apply(task.assignee_ref.clone()),
        due.apply(task.due_date),
        component.apply(task.component_pid),
    );
    let new_start = edit.start.apply(task.start_date);
    // The change and its audit row are one change: an item whose audit row cannot be written is refused.
    let mut tx = match crate::db::begin(ctx).await {
        Ok(t) => t,
        Err(e) => return ItemResult::refused(pid, e.to_string()),
    };
    // `fetch_one`: an update that finds no row is an error, as before SQ-1.
    if let Err(e) = sqlx::query_scalar::<_, i32>(
        "UPDATE tasks SET assignee_ref = $1, due_date = $2, start_date = $3, component_pid = $4
         WHERE id = $5
         RETURNING id",
    )
    .bind(new_assignee)
    .bind(new_due)
    .bind(new_start)
    .bind(new_component)
    .bind(task.id)
    .fetch_one(tx.conn())
    .await
    {
        return ItemResult::refused(pid, e.to_string());
    }
    if let Err(e) =
        AuditModel::record_on(tx.conn(), pid, audit_kind, caller.actor(), audit_detail).await
    {
        let _ = super::audit_failed(e);
        return ItemResult::refused(
            pid,
            "the change was not made because it could not be recorded",
        );
    }
    if let Err(e) = tx.commit().await {
        return ItemResult::refused(pid, e.to_string());
    }
    if let Change::Set(who) = assignee {
        cap_models::notify_on(
            &mut crate::db::pool(ctx),
            who,
            "task",
            pid,
            "task_assigned",
            "You were assigned a task",
        )
        .await
        .ok();
        if let Some(sub) = crate::notification_prefs::recipient_sub(who) {
            super::watchers::ensure_watching(ctx, "task", pid, &sub).await;
        }
    }
    ItemResult::ok(pid, None)
}

/// `POST /api/tasks/copy` and `/move` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct TransferBody {
    /// Tasks to copy or move; duplicates are removed before processing.
    ids: Vec<Uuid>,
    /// Destination plan; must be visible to the caller, otherwise the whole request is a 422.
    to_plan_pid: Uuid,
    /// Copy only: keep the source's assignee (default false).
    #[serde(default)]
    keep_assignee: bool,
}

/// Check that the caller can see the destination plan and return its id; the error is deliberately vague so an unreadable plan is not confirmed to exist.
async fn target_plan(ctx: &AppContext, caller: &MaybeAuthUser, pid: Uuid) -> Result<Uuid> {
    resolve_subject(ctx, caller, "plan", &pid.to_string())
        .await
        .map_err(|_| unprocessable("to_plan_pid names no plan this caller can see"))?;
    Ok(pid)
}

/// `POST /api/tasks/copy` — copy tasks into a plan.
#[debug_handler]
async fn bulk_copy(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(b): Json<TransferBody>,
) -> Result<Response> {
    let ids = rules::dedupe_ids(&b.ids).map_err(|m| unprocessable(&m))?;
    let target = target_plan(&ctx, &caller, b.to_plan_pid).await?;
    let workflow = super::workflow::in_force(&ctx, target, "task").await?;
    let mut results = Vec::new();
    for pid in ids {
        results.push(
            match copy_one(&ctx, &caller, pid, target, &workflow, b.keep_assignee).await {
                Ok(new) => ItemResult::ok(pid, Some(format!("copied as {new}"))),
                Err(why) => ItemResult::refused(pid, why),
            },
        );
    }
    summary(&results)
}

/// Copy one task into the target plan as a new task in the target workflow's initial state, carrying custom field values and recording a `copied_from` relation. Returns the new task id, or the refusal text for this item.
async fn copy_one(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    pid: Uuid,
    target: Uuid,
    workflow: &crate::workflow::WorkflowDef,
    keep_assignee: bool,
) -> std::result::Result<Uuid, String> {
    let src = readable_task(ctx, caller, pid).await?;
    let initial = workflow
        .states
        .iter()
        .find(|s| s.is_initial)
        .map_or("todo", |s| s.key.as_str())
        .to_string();
    let now = chrono::Utc::now();
    let new_pid = Uuid::new_v4();
    let assignee = keep_assignee.then(|| src.assignee_ref.clone()).flatten();
    let prepared: Vec<(String, Option<Value>)> = sqlx::query_as::<_, (String, Value)>(
        "SELECT field_key, value FROM custom_field_values WHERE subject_kind = 'task' AND subject_pid = $1",
    )
    .bind(pid)
    .fetch_all(crate::db::pool(ctx))
    .await
    .map_err(|e| e.to_string())?
    .into_iter()
    .map(|(key, value)| (key, Some(value)))
    .collect();
    let at = chrono::DateTime::<chrono::FixedOffset>::from(now);
    let mut tx = crate::db::begin(ctx).await.map_err(|e| e.to_string())?;
    let row_pid: Uuid = sqlx::query_scalar(
        "INSERT INTO tasks
             (pid, plan_pid, sprint_pid, title, description, status, assignee_ref, points, due_date,
              component_pid, parent_task_pid, status_changed_at, flow_type, done_at, deleted_at)
         VALUES ($1, $2, NULL, $3, $4, $5, $6, $7, $8, NULL, NULL, $9, $10, $11, NULL)
         RETURNING pid",
    )
    .bind(new_pid)
    .bind(target)
    .bind(&src.title)
    .bind(src.description.as_deref())
    .bind(&initial)
    .bind(assignee.as_deref())
    .bind(src.points)
    .bind(src.due_date)
    .bind(at)
    .bind(src.flow_type.as_deref())
    .bind(crate::workflow::is_done(workflow, &initial).then_some(at))
    .fetch_one(tx.conn())
    .await
    .map_err(|e| e.to_string())?;
    super::custom_fields::store_on(&mut tx, "task", new_pid, &prepared)
        .await
        .map_err(|e| e.to_string())?;
    crate::tba::insert_transition_on(
        &mut tx,
        crate::tba::NewTransition {
            task_pid: new_pid,
            plan_pid: target,
            from_status: None,
            to_status: &initial,
            at: now,
            actor_ref: caller.actor(),
            assignee_ref: assignee.as_deref(),
            blocked_reason: None,
        },
    )
    .await
    .map_err(|e| e.to_string())?;
    sqlx::query(
        "INSERT INTO task_relations (pid, from_task_pid, to_task_pid, kind, deleted_at)
         VALUES ($1, $2, $3, 'copied_from', NULL)",
    )
    .bind(Uuid::new_v4())
    .bind(row_pid)
    .bind(pid)
    .execute(tx.conn())
    .await
    .map_err(|e| e.to_string())?;
    AuditModel::record_on(
        tx.conn(),
        new_pid,
        "task_copied",
        caller.actor(),
        Some(serde_json::json!({ "from": pid })),
    )
    .await
    .map_err(|e| e.to_string())?;
    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(new_pid)
}

/// `POST /api/tasks/move` — re-home tasks into another plan.
#[debug_handler]
async fn bulk_move(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(b): Json<TransferBody>,
) -> Result<Response> {
    let ids = rules::dedupe_ids(&b.ids).map_err(|m| unprocessable(&m))?;
    let target = target_plan(&ctx, &caller, b.to_plan_pid).await?;
    let workflow = super::workflow::in_force(&ctx, target, "task").await?;
    let mut results = Vec::new();
    for pid in ids {
        results.push(
            match move_one(&ctx, &caller, pid, target, &workflow).await {
                Ok(note) => ItemResult::ok(pid, note),
                Err(why) => ItemResult::refused(pid, why),
            },
        );
    }
    summary(&results)
}

/// Move one task to the target plan, refusing if it is already there or has subtasks. A status the target workflow lacks maps to its initial state and the returned note says so; None means no note.
async fn move_one(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    pid: Uuid,
    target: Uuid,
    workflow: &crate::workflow::WorkflowDef,
) -> std::result::Result<Option<String>, String> {
    let task = readable_task(ctx, caller, pid).await?;
    if task.plan_pid == target {
        return Err("the task is already in that plan".to_string());
    }
    let source = super::workflow::in_force(ctx, task.plan_pid, "task")
        .await
        .map_err(|e| e.to_string())?;
    let gated =
        crate::workflow::restricts_saves(&source) || crate::workflow::restricts_saves(workflow);
    if gated && !super::plan_access::is_admin(caller.claims()) {
        return Err(
            "only an administrator can move a task between plans when either workflow restricts moves"
                .to_string(),
        );
    }
    let children: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM tasks WHERE parent_task_pid = $1 AND deleted_at IS NULL",
    )
    .bind(pid)
    .fetch_one(crate::db::pool(ctx))
    .await
    .map_err(|e| e.to_string())?;
    if children > 0 {
        return Err(format!(
            "it has {children} subtask(s); move or detach them first"
        ));
    }
    let (status, mapped) = rules::map_status(workflow, &task.status);
    let (from_status, assignee, old_plan) = (
        task.status.clone(),
        task.assignee_ref.clone(),
        task.plan_pid,
    );
    let now = chrono::Utc::now();
    let at = chrono::DateTime::<chrono::FixedOffset>::from(now);
    let mut tx = crate::db::begin(ctx).await.map_err(|e| e.to_string())?;
    // Plan-specific links do not survive the move; a status the target lacks is mapped and stamped.
    let moved = if mapped {
        sqlx::query_scalar::<_, i32>(
            "UPDATE tasks
             SET plan_pid = $1, component_pid = NULL, sprint_pid = NULL, parent_task_pid = NULL,
                 status = $2, status_changed_at = $3, done_at = $4
             WHERE id = $5
             RETURNING id",
        )
        .bind(target)
        .bind(&status)
        .bind(at)
        .bind(crate::workflow::is_done(workflow, &status).then_some(at))
        .bind(task.id)
    } else {
        sqlx::query_scalar::<_, i32>(
            "UPDATE tasks
             SET plan_pid = $1, component_pid = NULL, sprint_pid = NULL, parent_task_pid = NULL
             WHERE id = $2
             RETURNING id",
        )
        .bind(target)
        .bind(task.id)
    };
    // `fetch_one`: an update that finds no row is an error, as before SQ-1.
    moved
        .fetch_one(tx.conn())
        .await
        .map_err(|e| e.to_string())?;
    if mapped {
        crate::tba::insert_transition_on(
            &mut tx,
            crate::tba::NewTransition {
                task_pid: pid,
                plan_pid: target,
                from_status: Some(&from_status),
                to_status: &status,
                at: now,
                actor_ref: caller.actor(),
                assignee_ref: assignee.as_deref(),
                blocked_reason: None,
            },
        )
        .await
        .map_err(|e| e.to_string())?;
    }
    AuditModel::record_on(
        tx.conn(),
        pid,
        "task_moved_plan",
        caller.actor(),
        Some(serde_json::json!({ "from_plan": old_plan, "to_plan": target })),
    )
    .await
    .map_err(|e| e.to_string())?;
    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(mapped.then(|| {
        format!("status `{from_status}` is not in the target workflow; mapped to `{status}`")
    }))
}

/// The bulk-task routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/tasks/bulk", post(bulk_edit))
        .add("/tasks/copy", post(bulk_copy))
        .add("/tasks/move", post(bulk_move))
}
