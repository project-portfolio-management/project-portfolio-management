//! Tasks: the Kanban board CRUD and the status move with flow stamps.

use crate::auth::MaybeAuthUser;
use crate::engineering as rules;
use loco_rs::prelude::*;
use sea_orm::{PaginatorTrait, QueryOrder, QuerySelect};
use serde::Deserialize;
use uuid::Uuid;

use super::{find_task, refuse, valid_person_ref};
use crate::models::_entities::{sprints, tasks};
use crate::models::audit_logs::Model as AuditModel;

/// The task wire view (adds blocked-age calendar days to the stored row).
pub(crate) fn task_view(task: &tasks::Model) -> serde_json::Value {
    let blocked_days = (task.status == "blocked")
        .then(|| (chrono::Utc::now() - task.status_changed_at.to_utc()).num_days());
    serde_json::json!({
        "pid": task.pid,
        "title": task.title,
        "description": task.description,
        "status": task.status,
        "assignee_ref": task.assignee_ref,
        "points": task.points,
        "due_date": task.due_date,
        "deadline": task.deadline,
        "start_date": task.start_date,
        "duration_days": task.duration_days,
        "duration_optimistic_days": task.duration_optimistic_days,
        "duration_pessimistic_days": task.duration_pessimistic_days,
        "work_minutes": task.work_minutes,
        "units_percent": task.units_percent,
        "scheduling_rule": task.scheduling_rule,
        "contour": task.contour,
        "contour_custom": task.contour_custom,
        "component_pid": task.component_pid,
        "parent_task_pid": task.parent_task_pid,
        "sibling_position": task.sibling_position,
        "sprint_pid": task.sprint_pid,
        "created_at": task.created_at.to_utc(),
        "status_changed_at": task.status_changed_at.to_utc(),
        "done_at": task.done_at.map(|at| at.to_utc()),
        "blocked_days": blocked_days,
    })
}

/// `POST /api/plans/{pid}/tasks` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct TaskPayload {
    title: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    status: Option<String>,
    #[serde(default)]
    assignee_ref: Option<String>,
    #[serde(default)]
    sprint_pid: Option<Uuid>,
    /// Optional due **calendar date** (`YYYY-MM-DD`, no zone).
    #[serde(default)]
    due_date: Option<chrono::NaiveDate>,
    /// Optional planned start **calendar date** (PJ-1), honoured on **create**
    /// only; change it afterwards with `PUT …/tasks/{pid}/schedule`, which also
    /// reports what moved.
    #[serde(default)]
    start_date: Option<chrono::NaiveDate>,
    /// Optional business-day duration (PJ-1; `0` is a milestone), on create only.
    #[serde(default)]
    duration_days: Option<i32>,
    /// Optional component (grouping) of this plan.
    #[serde(default)]
    component_pid: Option<Uuid>,
    /// Optional parent task of this plan (a subtask); see `task_tree`.
    #[serde(default)]
    parent_task_pid: Option<Uuid>,
    /// Custom-field values (RM-1), honoured on **create** only; every
    /// required task field must be supplied there. Edit them afterwards
    /// through `PUT /api/custom-fields/values`.
    #[serde(default)]
    custom: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Optional story points (team-local; 0–100).
    #[serde(default)]
    points: Option<i32>,
    /// The declared Flow Framework work-item type — `feature`,
    /// `defect`, `risk` or `debt` (FR-31). Absent means **nobody
    /// declared one**, which Flow Distribution reports as
    /// `unclassified` and counts separately; it is deliberately not
    /// defaulted to `feature`.
    #[serde(default)]
    flow_type: Option<String>,
}

/// Validate the shared task fields; returns the resolved status.
pub(super) async fn validate_task(
    ctx: &AppContext,
    plan_pid: Uuid,
    payload: &TaskPayload,
) -> Result<(String, crate::workflow::WorkflowDef)> {
    if payload.title.trim().is_empty() {
        return Err(refuse("title is required"));
    }
    if payload.title.len() > crate::validation::MAX_TEXT_LEN {
        return Err(refuse("title exceeds the text cap"));
    }
    // The vocabulary comes from the workflow in force, not from a
    // compile-time constant: a plan with a custom workflow is validated
    // against *its* states, and a plan with none falls back to the
    // built-in vocabulary, so existing boards are unaffected (FR-26).
    let workflow = super::super::workflow::in_force(ctx, plan_pid, "task").await?;
    let initial = workflow
        .states
        .iter()
        .find(|state| state.is_initial)
        .map_or("todo", |state| state.key.as_str());
    let status = payload
        .status
        .clone()
        .unwrap_or_else(|| initial.to_string());
    if crate::workflow::category_of(&workflow, &status).is_none() {
        let declared: Vec<&str> = workflow
            .states
            .iter()
            .map(|state| state.key.as_str())
            .collect();
        return Err(refuse(&format!(
            "status must be one of {declared:?} (the workflow in force for this plan)"
        )));
    }
    if let Some(assignee) = payload.assignee_ref.as_deref()
        && !valid_person_ref(assignee)
    {
        return Err(refuse("assignee_ref must be a worker:/person: URN"));
    }
    if let Some(points) = payload.points
        && !(0..=100).contains(&points)
    {
        return Err(refuse("points must be between 0 and 100"));
    }
    // `unclassified` is deliberately not accepted: it is the *absence*
    // of a declaration, and letting a caller spell it would allow a row
    // to claim it had been classified as unclassified.
    if let Some(flow_type) = payload.flow_type.as_deref()
        && !["feature", "defect", "risk", "debt"].contains(&flow_type)
    {
        return Err(refuse(
            "flow_type must be feature, defect, risk or debt; omit it to leave the \
             item unclassified",
        ));
    }
    if let Some(component_pid) = payload.component_pid {
        let found = crate::models::_entities::components::Entity::find()
            .filter(crate::models::_entities::components::Column::Pid.eq(component_pid))
            .filter(crate::models::_entities::components::Column::PlanPid.eq(plan_pid))
            .filter(crate::models::_entities::components::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await
            .map_err(|e| Error::Model(ModelError::from(e)))?;
        if found.is_none() {
            return Err(refuse("component_pid must name a component of this plan"));
        }
    }
    if let Some(sprint_pid) = payload.sprint_pid {
        let sprint = sprints::Entity::find()
            .filter(sprints::Column::Pid.eq(sprint_pid))
            .filter(sprints::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await
            .map_err(|e| Error::Model(ModelError::from(e)))?;
        if sprint.is_none_or(|s| s.plan_pid != plan_pid) {
            return Err(refuse("sprint_pid must name a sprint of this plan"));
        }
    }
    Ok((status, workflow))
}

/// `POST /api/plans/{pid}/tasks` — create a task.
///
/// The default status is the workflow's **initial** state, and creating
/// straight into a state whose category is `done` stamps `done_at`.
#[debug_handler]
pub(super) async fn create_task(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(payload): Json<TaskPayload>,
) -> Result<Response> {
    let item = super::super::governance::find_item(&ctx, &pid).await?;
    let schedule_problems =
        crate::cpm::schedule_problems(payload.start_date, payload.duration_days);
    if !schedule_problems.is_empty() {
        return Err(refuse(&schedule_problems.join("; ")));
    }
    let (status, workflow) = validate_task(&ctx, item.pid, &payload).await?;
    // Required custom fields block creation; validated before any write.
    let custom = super::super::custom_fields::prepare(
        &ctx,
        "task",
        None,
        &payload.custom.clone().unwrap_or_default(),
        true,
    )
    .await?;
    super::super::task_tree::check_parent(&ctx, item.pid, None, payload.parent_task_pid).await?;
    let now = chrono::Utc::now();
    // The task and its opening transition commit together: a task
    // without one would have no measurable start, and the time-based
    // analysis would silently begin its life at the first later move
    // (spec `time-based-analysis.md` §5.1 invariant 3).
    let txn = ctx
        .db
        .begin()
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?;
    let row = tasks::ActiveModel {
        pid: sea_orm::ActiveValue::set(Uuid::new_v4()),
        plan_pid: sea_orm::ActiveValue::set(item.pid),
        sprint_pid: sea_orm::ActiveValue::set(payload.sprint_pid),
        title: sea_orm::ActiveValue::set(payload.title.clone()),
        description: sea_orm::ActiveValue::set(payload.description.clone()),
        status: sea_orm::ActiveValue::set(status.clone()),
        assignee_ref: sea_orm::ActiveValue::set(payload.assignee_ref.clone()),
        points: sea_orm::ActiveValue::set(payload.points),
        due_date: sea_orm::ActiveValue::set(payload.due_date),
        start_date: sea_orm::ActiveValue::set(payload.start_date),
        duration_days: sea_orm::ActiveValue::set(payload.duration_days),
        component_pid: sea_orm::ActiveValue::set(payload.component_pid),
        parent_task_pid: sea_orm::ActiveValue::set(payload.parent_task_pid),
        status_changed_at: sea_orm::ActiveValue::set(now.into()),
        flow_type: sea_orm::ActiveValue::set(payload.flow_type.clone()),
        // Stamped from the state's **category**, not from the literal
        // string "done". A custom vocabulary finishes in whatever it
        // calls finished (`shipped`, `closed`, …), and matching on the
        // name would leave the burndown blind to every board that
        // renamed its final column — the exact failure the mandatory
        // category exists to prevent.
        done_at: sea_orm::ActiveValue::set(
            crate::workflow::is_done(&workflow, &status).then(|| now.into()),
        ),
        deleted_at: sea_orm::ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&txn)
    .await
    .map_err(|e| Error::Model(ModelError::from(e)))?;
    super::super::custom_fields::store(&txn, "task", row.pid, &custom).await?;
    crate::tba::transition_row(
        row.pid,
        item.pid,
        None,
        status.clone(),
        now,
        caller.actor().map(ToString::to_string),
        payload.assignee_ref.clone(),
    )
    .insert(&txn)
    .await
    .map_err(|e| Error::Model(ModelError::from(e)))?;
    AuditModel::record(&txn, row.pid, "task_created", caller.actor(), None)
        .await
        .map_err(super::super::audit_failed)?;
    txn.commit()
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?;

    if let Some(sub) = caller.actor() {
        super::super::watchers::ensure_watching(&ctx.db, "task", row.pid, sub).await;
    }
    crate::live::publish(
        item.pid,
        "task_created",
        serde_json::json!({ "task_pid": row.pid, "title": row.title, "status": row.status }),
    );
    format::json(task_view(&row))
}

/// `GET /api/plans/{pid}/tasks` query.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct TaskListParams {
    #[serde(default)]
    component: Option<String>,
    /// Page size (at most 500). With `limit` or `offset` the answer is one page.
    #[serde(default)]
    limit: Option<u64>,
    /// Rows to skip.
    #[serde(default)]
    offset: Option<u64>,
}

/// Tasks returned when a caller sends no paging parameters (the old behaviour, now with a ceiling so
/// one huge plan cannot make one response unbounded). Past it the answer says `truncated: true` and
/// the caller pages with `limit` and `offset`.
pub const UNPAGED_TASK_CAP: u64 = 2_000;

/// The `(limit, offset)` for a task list: the standard page when the caller asked for one, else the
/// first [`UNPAGED_TASK_CAP`] tasks. Pure, so it is unit-tested.
#[must_use]
pub fn task_window(limit: Option<u64>, offset: Option<u64>) -> (u64, u64) {
    if limit.is_some() || offset.is_some() {
        super::super::pagination::Page { limit, offset }.resolve(100)
    } else {
        (UNPAGED_TASK_CAP, 0)
    }
}

/// `GET /api/plans/{pid}/tasks` — the item's live tasks plus
/// per-status counts (board columns).
#[debug_handler]
pub(super) async fn list_tasks(
    State(ctx): State<AppContext>,
    Path(pid): Path<String>,
    Query(params): Query<TaskListParams>,
) -> Result<Response> {
    let item = super::super::governance::find_item(&ctx, &pid).await?;
    let mut query = tasks::Entity::find()
        .filter(tasks::Column::PlanPid.eq(item.pid))
        .filter(tasks::Column::DeletedAt.is_null());
    // `?component=<pid>` narrows to one component; `?component=unassigned`
    // to the tasks with none (never silently dropped from a grouping).
    match params.component.as_deref() {
        None => {}
        Some("unassigned") => query = query.filter(tasks::Column::ComponentPid.is_null()),
        Some(raw) => {
            let component = Uuid::parse_str(raw)
                .map_err(|_| refuse("component must be a component pid or \"unassigned\""))?;
            query = query.filter(tasks::Column::ComponentPid.eq(component));
        }
    }
    super::super::pagination::Page {
        limit: params.limit,
        offset: params.offset,
    }
    .check_offset()?;
    let (limit, offset) = task_window(params.limit, params.offset);
    // The per-status counts cover every task that matches, not just this page (board columns).
    let by_status: Vec<(String, i64)> = query
        .clone()
        .select_only()
        .column(tasks::Column::Status)
        .column_as(tasks::Column::Id.count(), "n")
        .group_by(tasks::Column::Status)
        .into_tuple()
        .all(&ctx.db)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?;
    let mut counts: std::collections::BTreeMap<&str, usize> = rules::TASK_STATUSES
        .iter()
        .map(|status| (*status, 0))
        .collect();
    let mut total: u64 = 0;
    for (status, n) in &by_status {
        let n = usize::try_from(*n).unwrap_or(0);
        total += n as u64;
        if let Some(entry) = counts.get_mut(status.as_str()) {
            *entry += n;
        }
    }
    // Positioned siblings first (Postgres sorts NULL last), then by age.
    let rows = query
        .order_by_asc(tasks::Column::SiblingPosition)
        .order_by_asc(tasks::Column::Id)
        .offset(offset)
        .limit(limit)
        .all(&ctx.db)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?;
    let truncated = offset + (rows.len() as u64) < total;
    Ok(super::super::pagination::with_page_headers(
        format::json(serde_json::json!({
            "tasks": rows.iter().map(task_view).collect::<Vec<_>>(),
            "counts": counts,
            "total": total,
            "truncated": truncated,
            "limit": limit,
            "offset": offset,
        }))?,
        total,
        limit,
        offset,
    ))
}

/// `PUT /api/plans/{pid}/tasks/{t_pid}` — update fields
/// (status changes route through PATCH so the flow stamps stay true).
#[debug_handler]
pub(super) async fn update_task(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, t_pid)): Path<(String, String)>,
    Json(payload): Json<TaskPayload>,
) -> Result<Response> {
    let item = super::super::governance::find_item(&ctx, &pid).await?;
    let task = find_task(&ctx, item.pid, &t_pid).await?;
    if payload.status.as_deref().is_some_and(|s| s != task.status) {
        return Err(refuse("status changes go through PATCH (the board move)"));
    }
    validate_task(&ctx, item.pid, &payload).await?;
    super::super::task_tree::check_parent(&ctx, item.pid, Some(task.pid), payload.parent_task_pid)
        .await?;
    if payload.due_date != task.due_date {
        super::super::task_relations::check_due_change(&ctx, task.pid, payload.due_date).await?;
    }
    let task_pid = task.pid;
    let mut active: tasks::ActiveModel = task.into();
    active.title = sea_orm::ActiveValue::set(payload.title.clone());
    active.description = sea_orm::ActiveValue::set(payload.description.clone());
    active.assignee_ref = sea_orm::ActiveValue::set(payload.assignee_ref.clone());
    active.points = sea_orm::ActiveValue::set(payload.points);
    active.due_date = sea_orm::ActiveValue::set(payload.due_date);
    active.component_pid = sea_orm::ActiveValue::set(payload.component_pid);
    active.parent_task_pid = sea_orm::ActiveValue::set(payload.parent_task_pid);
    active.sprint_pid = sea_orm::ActiveValue::set(payload.sprint_pid);
    let txn = ctx.db.begin().await.map_err(super::super::db_err)?;
    let row = active
        .update(&txn)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?;
    AuditModel::record(&txn, task_pid, "task_updated", caller.actor(), None)
        .await
        .map_err(super::super::audit_failed)?;
    txn.commit().await.map_err(super::super::db_err)?;
    crate::live::publish(
        item.pid,
        "task_updated",
        serde_json::json!({ "task_pid": task_pid }),
    );
    format::json(task_view(&row))
}

/// RM-2: a move may be restricted to named roles. Like the other refusals
/// in the move path this writes **no** transition row. The caller's roles
/// are the token's `role` attribute (a vocabulary the deployment declares, as
/// with `view`); a caller with none holds no role.
pub(super) fn enforce_move_roles(
    workflow: &crate::workflow::WorkflowDef,
    from: &str,
    to: &str,
    caller: &MaybeAuthUser,
) -> Result<()> {
    let roles = caller
        .claims()
        .and_then(|c| c.attrs.get("role").cloned())
        .unwrap_or_default();
    crate::workflow::may_transition_as(workflow, from, to, &roles).map_err(|denied| {
        Error::CustomError(
            axum::http::StatusCode::FORBIDDEN,
            loco_rs::controller::ErrorDetail::new(
                "forbidden",
                format!(
                    "moving `{from}` -> `{to}` is restricted to the role(s): {}",
                    denied.allowed.join(", ")
                ),
            ),
        )
    })
}

/// `PATCH /api/plans/{pid}/tasks/{t_pid}` body: the move.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct MovePayload {
    status: String,
    /// Why the task is blocked; only on a move into `blocked`
    /// (`crate::tba::BLOCKED_REASONS`).
    #[serde(default)]
    blocked_reason: Option<String>,
}

/// Refuse a move into a capped column that is already full.
///
/// Caps declared on the workflow's own states win over the
/// deployment-wide env map: a plan that configured its board said
/// something more specific than the environment did. No cap from
/// either source means no limit.
///
/// A refused move writes **no** transition row — the log must not
/// record work that did not happen.
pub(super) async fn check_wip(
    ctx: &AppContext,
    plan_pid: Uuid,
    status: &str,
    workflow: &crate::workflow::WorkflowDef,
) -> Result<()> {
    let workflow_caps = crate::workflow::wip_limits(workflow);
    let env_caps = rules::parse_wip_limits(
        std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_WIP_LIMITS")
            .ok()
            .as_deref(),
    );
    let Some(cap) = workflow_caps
        .get(status)
        .or_else(|| env_caps.as_ref().and_then(|l| l.get(status)))
    else {
        return Ok(());
    };
    let occupancy = tasks::Entity::find()
        .filter(tasks::Column::PlanPid.eq(plan_pid))
        .filter(tasks::Column::Status.eq(status))
        .filter(tasks::Column::DeletedAt.is_null())
        .count(&ctx.db)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?;
    if usize::try_from(occupancy).unwrap_or(usize::MAX) >= *cap {
        return Err(refuse(&format!(
            "WIP limit reached: `{status}` is capped at {cap} for this item"
        )));
    }
    Ok(())
}

/// `PATCH /api/plans/{pid}/tasks/{t_pid}` — the board move.
/// Stamps `status_changed_at`; first entry into a state whose
/// **category** is `done` stamps `done_at` (kept thereafter — the
/// completion history stays true).
#[debug_handler]
#[expect(clippy::too_many_lines)] // the move pipeline: validate, refuse, commit, then side effects
pub(super) async fn move_task(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, t_pid)): Path<(String, String)>,
    Json(payload): Json<MovePayload>,
) -> Result<Response> {
    let item = super::super::governance::find_item(&ctx, &pid).await?;
    let task = find_task(&ctx, item.pid, &t_pid).await?;

    // The workflow in force decides both the vocabulary and the legal
    // moves (FR-26). A plan with none falls back to the built-in, whose
    // transition set is empty — meaning unconstrained — so existing
    // boards behave exactly as before.
    let workflow = super::super::workflow::in_force(&ctx, item.pid, "task").await?;
    if crate::workflow::category_of(&workflow, &payload.status).is_none() {
        let declared: Vec<&str> = workflow
            .states
            .iter()
            .map(|state| state.key.as_str())
            .collect();
        return Err(refuse(&format!(
            "status must be one of {declared:?} (the workflow in force for this plan)"
        )));
    }
    crate::tba::check_blocked_reason(&payload.status, payload.blocked_reason.as_deref())
        .map_err(|problem| refuse(&problem))?;
    if task.status == payload.status {
        return format::json(task_view(&task));
    }
    if !crate::workflow::may_transition(&workflow, &task.status, &payload.status) {
        // A refused move writes **no** transition row — the log must
        // not record work that did not happen
        // (`spec/time-based-analysis.md` §6), the same rule the WIP-limit
        // refusal below already honours.
        return Err(refuse(&format!(
            "the workflow in force does not permit `{}` -> `{}`",
            task.status, payload.status
        )));
    }
    enforce_move_roles(&workflow, &task.status, &payload.status, &caller)?;
    // LT-10: a task cannot be finished while a `blocks` relation's blocker is open.
    if crate::workflow::is_done(&workflow, &payload.status) {
        super::super::task_relations::ensure_not_blocked(&ctx, task.pid).await?;
    }
    // WIP limits (env-configured, per plan board): refuse a move
    // into a capped column that is already full. No config ⇒ no caps.
    check_wip(&ctx, item.pid, &payload.status, &workflow).await?;
    let from = task.status.clone();
    let task_pid = task.pid;
    let assignee = task.assignee_ref.clone();
    // Category, not the literal string — see `create_task`.
    let first_done = task.done_at.is_none() && crate::workflow::is_done(&workflow, &payload.status);
    let now = chrono::Utc::now();
    // The move and its transition commit together. A refused move (an
    // unknown status, a full WIP column) has already returned above, so
    // the log never records a move that did not happen.
    let txn = ctx
        .db
        .begin()
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?;
    let mut active: tasks::ActiveModel = task.into();
    active.status = sea_orm::ActiveValue::set(payload.status.clone());
    active.status_changed_at = sea_orm::ActiveValue::set(now.into());
    if first_done {
        active.done_at = sea_orm::ActiveValue::set(Some(now.into()));
    }
    let row = active
        .update(&txn)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?;
    let mut transition = crate::tba::transition_row(
        task_pid,
        item.pid,
        Some(from.clone()),
        payload.status.clone(),
        now,
        caller.actor().map(ToString::to_string),
        assignee,
    );
    transition.blocked_reason = sea_orm::ActiveValue::set(payload.blocked_reason.clone());
    transition
        .insert(&txn)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?;
    AuditModel::record(
        &txn,
        task_pid,
        "task_moved",
        caller.actor(),
        Some(serde_json::json!({ "from": from, "to": payload.status })),
    )
    .await
    .map_err(super::super::audit_failed)?;
    txn.commit()
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?;

    crate::live::publish(
        item.pid,
        "task_moved",
        serde_json::json!({ "task_pid": task_pid, "from": from, "to": payload.status }),
    );
    // LT-10: finishing a primary closes the tasks that duplicate it.
    if first_done {
        super::super::task_relations::close_duplicates(&ctx, task_pid, caller.actor()).await;
    }
    // Watchers (RM-3) are told once, the mover never; best-effort.
    super::super::watchers::notify_watchers(
        &ctx,
        "task",
        task_pid,
        caller.actor(),
        "A task you follow moved to a new status",
    )
    .await;
    // Workflow automation: the move is already committed, so a rule
    // that fails is logged as a failed run and never undoes the move.
    super::super::automation::fire(
        &ctx,
        &crate::automation::TriggerFact {
            kind: "task_moved".to_string(),
            plan_pid: item.pid,
            from_status: Some(from),
            to_status: Some(payload.status.clone()),
        },
        "task",
        task_pid,
        caller.actor(),
    )
    .await;
    format::json(task_view(&row))
}

/// `DELETE /api/plans/{pid}/tasks/{t_pid}` — soft delete.
#[debug_handler]
pub(super) async fn delete_task(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, t_pid)): Path<(String, String)>,
) -> Result<Response> {
    let item = super::super::governance::find_item(&ctx, &pid).await?;
    let task = find_task(&ctx, item.pid, &t_pid).await?;
    let task_pid = task.pid;
    let mut active: tasks::ActiveModel = task.into();
    active.deleted_at = sea_orm::ActiveValue::set(Some(chrono::Utc::now().into()));
    let txn = ctx.db.begin().await.map_err(super::super::db_err)?;
    active
        .update(&txn)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?;
    AuditModel::record(&txn, task_pid, "task_deleted", caller.actor(), None)
        .await
        .map_err(super::super::audit_failed)?;
    txn.commit().await.map_err(super::super::db_err)?;
    format::empty_json()
}
