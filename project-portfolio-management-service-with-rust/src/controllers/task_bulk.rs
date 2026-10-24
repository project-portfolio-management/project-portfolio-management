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
//!   a status it lacks maps to its initial state **and the result says so**.

use super::unprocessable;
use std::collections::BTreeMap;

use loco_rs::prelude::*;
use sea_orm::TransactionTrait;
use serde::Deserialize;
use serde_json::Value;
use uuid::Uuid;

use super::discussion::resolve_subject;
use super::governance::valid_ref;
use crate::auth::MaybeAuthUser;
use crate::models::_entities::{components, custom_field_values, task_relations, tasks};
use crate::models::audit_logs::Model as AuditModel;
use crate::models::capabilities as cap_models;
use crate::task_bulk::{self as rules, Change, ItemResult};

/// The human text of an error, for a per-item refusal note.
fn error_text(e: &Error) -> String {
    match e {
        Error::CustomError(_, detail) => detail
            .description
            .clone()
            .or_else(|| detail.error.clone())
            .unwrap_or_else(|| "refused".to_string()),
        other => other.to_string(),
    }
}

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
    tasks::Entity::find()
        .filter(tasks::Column::Pid.eq(pid))
        .filter(tasks::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "unknown task".to_string())
}

/// The fields a bulk edit may set.
const ALLOWED: &[&str] = &["assignee_ref", "due_date", "component_pid", "start_date"];

/// `POST /api/tasks/bulk` body.
#[derive(Debug, Deserialize)]
struct EditBody {
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
    pub(super) assignee: Change<String>,
    pub(super) due: Change<chrono::NaiveDate>,
    pub(super) component: Change<Uuid>,
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
        let ok = components::Entity::find()
            .filter(components::Column::Pid.eq(*c))
            .filter(components::Column::PlanPid.eq(task.plan_pid))
            .filter(components::Column::DeletedAt.is_null())
            .one(&ctx.db)
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
    let mut active: tasks::ActiveModel = task.into();
    active.assignee_ref = ActiveValue::set(new_assignee);
    active.due_date = ActiveValue::set(new_due);
    active.start_date = ActiveValue::set(new_start);
    active.component_pid = ActiveValue::set(new_component);
    // The change and its audit row are one change: an item whose audit row cannot be written is refused.
    let txn = match ctx.db.begin().await {
        Ok(t) => t,
        Err(e) => return ItemResult::refused(pid, e.to_string()),
    };
    if let Err(e) = active.update(&txn).await {
        return ItemResult::refused(pid, e.to_string());
    }
    if let Err(e) = AuditModel::record(&txn, pid, audit_kind, caller.actor(), audit_detail).await {
        let _ = super::audit_failed(e);
        return ItemResult::refused(
            pid,
            "the change was not made because it could not be recorded",
        );
    }
    if let Err(e) = txn.commit().await {
        return ItemResult::refused(pid, e.to_string());
    }
    if let Change::Set(who) = assignee {
        cap_models::notify(
            &ctx.db,
            who,
            "task",
            pid,
            "task_assigned",
            "You were assigned a task",
        )
        .await
        .ok();
        if let Some(sub) = crate::notification_prefs::recipient_sub(who) {
            super::watchers::ensure_watching(&ctx.db, "task", pid, &sub).await;
        }
    }
    ItemResult::ok(pid, None)
}

/// `POST /api/tasks/copy` and `/move` body.
#[derive(Debug, Deserialize)]
struct TransferBody {
    ids: Vec<Uuid>,
    to_plan_pid: Uuid,
    /// Copy only: keep the source's assignee (default false).
    #[serde(default)]
    keep_assignee: bool,
}

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
    let values = custom_field_values::Entity::find()
        .filter(custom_field_values::Column::SubjectKind.eq("task"))
        .filter(custom_field_values::Column::SubjectPid.eq(pid))
        .all(&ctx.db)
        .await
        .map_err(|e| e.to_string())?;
    let prepared: Vec<(String, Option<Value>)> = values
        .into_iter()
        .map(|v| (v.field_key, Some(v.value)))
        .collect();
    let txn = ctx.db.begin().await.map_err(|e| e.to_string())?;
    let row = tasks::ActiveModel {
        pid: ActiveValue::set(new_pid),
        plan_pid: ActiveValue::set(target),
        sprint_pid: ActiveValue::set(None),
        title: ActiveValue::set(src.title.clone()),
        description: ActiveValue::set(src.description.clone()),
        status: ActiveValue::set(initial.clone()),
        assignee_ref: ActiveValue::set(assignee.clone()),
        points: ActiveValue::set(src.points),
        due_date: ActiveValue::set(src.due_date),
        component_pid: ActiveValue::set(None),
        parent_task_pid: ActiveValue::set(None),
        status_changed_at: ActiveValue::set(now.into()),
        flow_type: ActiveValue::set(src.flow_type.clone()),
        done_at: ActiveValue::set(crate::workflow::is_done(workflow, &initial).then(|| now.into())),
        deleted_at: ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&txn)
    .await
    .map_err(|e| e.to_string())?;
    super::custom_fields::store(&txn, "task", new_pid, &prepared)
        .await
        .map_err(|e| e.to_string())?;
    crate::tba::transition_row(
        new_pid,
        target,
        None,
        initial,
        now,
        caller.actor().map(ToString::to_string),
        assignee,
    )
    .insert(&txn)
    .await
    .map_err(|e| e.to_string())?;
    task_relations::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        from_task_pid: ActiveValue::set(row.pid),
        to_task_pid: ActiveValue::set(pid),
        kind: ActiveValue::set("copied_from".to_string()),
        deleted_at: ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&txn)
    .await
    .map_err(|e| e.to_string())?;
    AuditModel::record(
        &txn,
        new_pid,
        "task_copied",
        caller.actor(),
        Some(serde_json::json!({ "from": pid })),
    )
    .await
    .map_err(|e| e.to_string())?;
    txn.commit().await.map_err(|e| e.to_string())?;
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
    let children = tasks::Entity::find()
        .filter(tasks::Column::ParentTaskPid.eq(pid))
        .filter(tasks::Column::DeletedAt.is_null())
        .count(&ctx.db)
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
    let txn = ctx.db.begin().await.map_err(|e| e.to_string())?;
    let mut active: tasks::ActiveModel = task.into();
    active.plan_pid = ActiveValue::set(target);
    // Plan-specific links do not survive the move.
    active.component_pid = ActiveValue::set(None);
    active.sprint_pid = ActiveValue::set(None);
    active.parent_task_pid = ActiveValue::set(None);
    if mapped {
        active.status = ActiveValue::set(status.clone());
        active.status_changed_at = ActiveValue::set(now.into());
        active.done_at =
            ActiveValue::set(crate::workflow::is_done(workflow, &status).then(|| now.into()));
    }
    active.update(&txn).await.map_err(|e| e.to_string())?;
    if mapped {
        crate::tba::transition_row(
            pid,
            target,
            Some(from_status.clone()),
            status.clone(),
            now,
            caller.actor().map(ToString::to_string),
            assignee,
        )
        .insert(&txn)
        .await
        .map_err(|e| e.to_string())?;
    }
    AuditModel::record(
        &txn,
        pid,
        "task_moved_plan",
        caller.actor(),
        Some(serde_json::json!({ "from_plan": old_plan, "to_plan": target })),
    )
    .await
    .map_err(|e| e.to_string())?;
    txn.commit().await.map_err(|e| e.to_string())?;
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
