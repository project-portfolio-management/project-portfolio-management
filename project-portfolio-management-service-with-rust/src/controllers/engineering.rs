//! Engineering-team endpoints — the spec-§13 operational **tasks**
//! sub-resource (Kanban board: CRUD + a PATCH status move with honest
//! flow stamps), **sprints** + the honest **burndown** (real `done_at`
//! completions only), the per-item **standup digest** (audit-derived
//! last-24h changes), and the estate views: **blocked-work aging**,
//! the **`MoSCoW`** scope cut (`moscow:<band>` tags), the
//! **delivery-links** panel (external tracker identifiers), and the
//! **milestone calendar** (demo / release / checkpoint kinds).
//!
//! Tasks and sprints are operational data: deliberately **not** part
//! of the matcher payload (the spec's partition rule) — nothing here
//! feeds matching.

use axum::http::HeaderMap;
use loco_rs::prelude::*;
use sea_orm::{PaginatorTrait, QueryOrder, QuerySelect};
use serde::Deserialize;
use uuid::Uuid;

use super::insights::item_ref;
use crate::auth::MaybeAuthUser;
use crate::engineering as rules;
use crate::models::_entities::{
    audit_logs, devops_events, milestones, plans, sprint_notes, sprints, tasks,
};
use crate::models::audit_logs::Model as AuditModel;

/// Load all live rows of one entity (soft-deleted excluded).
macro_rules! live {
    ($module:ident, $db:expr, $scope:expr) => {
        $scope.keep(
            $module::Entity::find()
                .filter($module::Column::DeletedAt.is_null())
                .all($db)
                .await
                .map_err(|e| Error::Model(ModelError::from(e)))?,
        )
    };
}

/// `422` with a reason.
fn refuse(reason: &str) -> Error {
    Error::CustomError(
        axum::http::StatusCode::UNPROCESSABLE_ENTITY,
        loco_rs::controller::ErrorDetail::new("unprocessable_entity", reason),
    )
}

/// A `worker:`/`person:` URN check (mirrors governance's rule).
fn valid_person_ref(value: &str) -> bool {
    value.split_once(':').is_some_and(|(scheme, id)| {
        matches!(scheme, "worker" | "person") && Uuid::parse_str(id).is_ok()
    })
}

/// Find one live task of one plan, or 404 (the shared definition, CQ-7).
pub(crate) use super::find_live_task as find_task;

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
async fn validate_task(
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
    let workflow = super::workflow::in_force(ctx, plan_pid, "task").await?;
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
async fn create_task(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(payload): Json<TaskPayload>,
) -> Result<Response> {
    let item = super::governance::find_item(&ctx, &pid).await?;
    let schedule_problems =
        crate::cpm::schedule_problems(payload.start_date, payload.duration_days);
    if !schedule_problems.is_empty() {
        return Err(refuse(&schedule_problems.join("; ")));
    }
    let (status, workflow) = validate_task(&ctx, item.pid, &payload).await?;
    // Required custom fields block creation; validated before any write.
    let custom = super::custom_fields::prepare(
        &ctx,
        "task",
        None,
        &payload.custom.clone().unwrap_or_default(),
        true,
    )
    .await?;
    super::task_tree::check_parent(&ctx, item.pid, None, payload.parent_task_pid).await?;
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
    super::custom_fields::store(&txn, "task", row.pid, &custom).await?;
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
        .map_err(super::audit_failed)?;
    txn.commit()
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?;

    if let Some(sub) = caller.actor() {
        super::watchers::ensure_watching(&ctx.db, "task", row.pid, sub).await;
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
        super::pagination::Page { limit, offset }.resolve(100)
    } else {
        (UNPAGED_TASK_CAP, 0)
    }
}

/// `GET /api/plans/{pid}/tasks` — the item's live tasks plus
/// per-status counts (board columns).
#[debug_handler]
async fn list_tasks(
    State(ctx): State<AppContext>,
    Path(pid): Path<String>,
    Query(params): Query<TaskListParams>,
) -> Result<Response> {
    let item = super::governance::find_item(&ctx, &pid).await?;
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
    super::pagination::Page {
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
    Ok(super::pagination::with_page_headers(
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
async fn update_task(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, t_pid)): Path<(String, String)>,
    Json(payload): Json<TaskPayload>,
) -> Result<Response> {
    let item = super::governance::find_item(&ctx, &pid).await?;
    let task = find_task(&ctx, item.pid, &t_pid).await?;
    if payload.status.as_deref().is_some_and(|s| s != task.status) {
        return Err(refuse("status changes go through PATCH (the board move)"));
    }
    validate_task(&ctx, item.pid, &payload).await?;
    super::task_tree::check_parent(&ctx, item.pid, Some(task.pid), payload.parent_task_pid).await?;
    if payload.due_date != task.due_date {
        super::task_relations::check_due_change(&ctx, task.pid, payload.due_date).await?;
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
    let txn = ctx.db.begin().await.map_err(super::db_err)?;
    let row = active
        .update(&txn)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?;
    AuditModel::record(&txn, task_pid, "task_updated", caller.actor(), None)
        .await
        .map_err(super::audit_failed)?;
    txn.commit().await.map_err(super::db_err)?;
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
fn enforce_move_roles(
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
async fn check_wip(
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
async fn move_task(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, t_pid)): Path<(String, String)>,
    Json(payload): Json<MovePayload>,
) -> Result<Response> {
    let item = super::governance::find_item(&ctx, &pid).await?;
    let task = find_task(&ctx, item.pid, &t_pid).await?;

    // The workflow in force decides both the vocabulary and the legal
    // moves (FR-26). A plan with none falls back to the built-in, whose
    // transition set is empty — meaning unconstrained — so existing
    // boards behave exactly as before.
    let workflow = super::workflow::in_force(&ctx, item.pid, "task").await?;
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
        super::task_relations::ensure_not_blocked(&ctx, task.pid).await?;
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
    .map_err(super::audit_failed)?;
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
        super::task_relations::close_duplicates(&ctx, task_pid, caller.actor()).await;
    }
    // Watchers (RM-3) are told once, the mover never; best-effort.
    super::watchers::notify_watchers(
        &ctx,
        "task",
        task_pid,
        caller.actor(),
        "A task you follow moved to a new status",
    )
    .await;
    // Workflow automation: the move is already committed, so a rule
    // that fails is logged as a failed run and never undoes the move.
    super::automation::fire(
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
async fn delete_task(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, t_pid)): Path<(String, String)>,
) -> Result<Response> {
    let item = super::governance::find_item(&ctx, &pid).await?;
    let task = find_task(&ctx, item.pid, &t_pid).await?;
    let task_pid = task.pid;
    let mut active: tasks::ActiveModel = task.into();
    active.deleted_at = sea_orm::ActiveValue::set(Some(chrono::Utc::now().into()));
    let txn = ctx.db.begin().await.map_err(super::db_err)?;
    active
        .update(&txn)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?;
    AuditModel::record(&txn, task_pid, "task_deleted", caller.actor(), None)
        .await
        .map_err(super::audit_failed)?;
    txn.commit().await.map_err(super::db_err)?;
    format::empty_json()
}

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
async fn create_sprint(
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
    let item = super::governance::find_item(&ctx, &pid).await?;
    let txn = ctx.db.begin().await.map_err(super::db_err)?;
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
        .map_err(super::audit_failed)?;
    txn.commit().await.map_err(super::db_err)?;
    format::json(row)
}

/// `GET /api/plans/{pid}/sprints` — the item's sprints, newest
/// first.
#[debug_handler]
async fn list_sprints(State(ctx): State<AppContext>, Path(pid): Path<String>) -> Result<Response> {
    let item = super::governance::find_item(&ctx, &pid).await?;
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
async fn burndown(
    axum::extract::Query(query): axum::extract::Query<BurndownQuery>,
    State(ctx): State<AppContext>,
    Path(pid): Path<String>,
    headers: HeaderMap,
) -> Result<Response> {
    let item = super::governance::find_item(&ctx, &pid).await?;
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
async fn standup(
    State(ctx): State<AppContext>,
    Path(pid): Path<String>,
    headers: HeaderMap,
) -> Result<Response> {
    let item = super::governance::find_item(&ctx, &pid).await?;
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
async fn find_sprint(ctx: &AppContext, plan_pid: Uuid, s_pid: &str) -> Result<sprints::Model> {
    let s_pid = super::parse_pid(s_pid)?;
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
async fn create_note(
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
    let item = super::governance::find_item(&ctx, &pid).await?;
    let sprint = find_sprint(&ctx, item.pid, &s_pid).await?;
    let txn = ctx.db.begin().await.map_err(super::db_err)?;
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
        .map_err(super::audit_failed)?;
    txn.commit().await.map_err(super::db_err)?;
    format::json(row)
}

/// `GET /api/plans/{pid}/sprints/{s_pid}/notes` — the sprint's
/// notes, grouped-friendly (oldest first).
#[debug_handler]
async fn list_notes(
    State(ctx): State<AppContext>,
    Path((pid, s_pid)): Path<(String, String)>,
) -> Result<Response> {
    let item = super::governance::find_item(&ctx, &pid).await?;
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
async fn convert_note(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, s_pid, n_pid)): Path<(String, String, String)>,
) -> Result<Response> {
    let item = super::governance::find_item(&ctx, &pid).await?;
    let sprint = find_sprint(&ctx, item.pid, &s_pid).await?;
    let n_pid = super::parse_pid(&n_pid)?;
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
    let txn = ctx.db.begin().await.map_err(super::db_err)?;
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
    .map_err(super::audit_failed)?;
    txn.commit().await.map_err(super::db_err)?;
    format::json(serde_json::json!({ "note": updated, "task": task_view(&task) }))
}

/// `GET /api/plans/{pid}/velocity` — per-sprint completed
/// counts and story-point sums, from real `done_at` stamps only.
/// Points are **team-local**: the served note says never to compare
/// them across teams or items.
#[debug_handler]
async fn velocity(
    State(ctx): State<AppContext>,
    Path(pid): Path<String>,
    headers: HeaderMap,
) -> Result<Response> {
    let item = super::governance::find_item(&ctx, &pid).await?;
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

// ─── DevOps events ──────────────────────────────────────────────────────────

/// `POST /api/devops/events` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct DevopsEventPayload {
    plan_pid: Uuid,
    kind: String,
    #[serde(default)]
    environment: Option<String>,
    #[serde(default)]
    version: Option<String>,
    #[serde(default)]
    reference: Option<String>,
    #[serde(default)]
    incident_pid: Option<Uuid>,
    #[serde(default)]
    caused_by_deploy_pid: Option<Uuid>,
    #[serde(default)]
    occurred_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// `POST /api/devops/events` — ingest one deploy / incident /
/// recovery event. A `deploy` requires an `environment`; a `recovery`
/// must reference its `incident`; an `incident` may declare the
/// deploy that caused it. The DORA-style metrics derive **only** from
/// events ingested here.
#[debug_handler]
async fn ingest_devops_event(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(payload): Json<DevopsEventPayload>,
) -> Result<Response> {
    if !rules::DEVOPS_EVENT_KINDS.contains(&payload.kind.as_str()) {
        return Err(refuse(&format!(
            "kind must be one of {:?}",
            rules::DEVOPS_EVENT_KINDS
        )));
    }
    let item = plans::Entity::find()
        .filter(plans::Column::Pid.eq(payload.plan_pid))
        .filter(plans::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?
        .ok_or(Error::NotFound)?;
    match payload.kind.as_str() {
        "deploy" if payload.environment.as_deref().is_none_or(str::is_empty) => {
            return Err(refuse("a deploy event requires an environment"));
        }
        "recovery" => {
            let Some(incident_pid) = payload.incident_pid else {
                return Err(refuse("a recovery event must reference its incident_pid"));
            };
            let incident = devops_events::Entity::find()
                .filter(devops_events::Column::Pid.eq(incident_pid))
                .one(&ctx.db)
                .await
                .map_err(|e| Error::Model(ModelError::from(e)))?;
            if incident.is_none_or(|e| e.kind != "incident") {
                return Err(refuse("incident_pid must name an ingested incident event"));
            }
        }
        _ => {}
    }
    if payload.caused_by_deploy_pid.is_some() && payload.kind != "incident" {
        return Err(refuse("caused_by_deploy_pid belongs on incident events"));
    }
    if let Some(deploy_pid) = payload.caused_by_deploy_pid {
        let deploy = devops_events::Entity::find()
            .filter(devops_events::Column::Pid.eq(deploy_pid))
            .one(&ctx.db)
            .await
            .map_err(|e| Error::Model(ModelError::from(e)))?;
        if deploy.is_none_or(|e| e.kind != "deploy") {
            return Err(refuse(
                "caused_by_deploy_pid must name an ingested deploy event",
            ));
        }
    }
    let txn = ctx.db.begin().await.map_err(super::db_err)?;
    let row = devops_events::ActiveModel {
        pid: sea_orm::ActiveValue::set(Uuid::new_v4()),
        plan_pid: sea_orm::ActiveValue::set(item.pid),
        kind: sea_orm::ActiveValue::set(payload.kind.clone()),
        environment: sea_orm::ActiveValue::set(payload.environment.clone()),
        version: sea_orm::ActiveValue::set(payload.version.clone()),
        reference: sea_orm::ActiveValue::set(payload.reference.clone()),
        incident_pid: sea_orm::ActiveValue::set(payload.incident_pid),
        caused_by_deploy_pid: sea_orm::ActiveValue::set(payload.caused_by_deploy_pid),
        occurred_at: sea_orm::ActiveValue::set(
            payload.occurred_at.unwrap_or_else(chrono::Utc::now).into(),
        ),
        ..Default::default()
    }
    .insert(&txn)
    .await
    .map_err(|e| Error::Model(ModelError::from(e)))?;
    AuditModel::record(
        &txn,
        row.pid,
        "devops_event_ingested",
        caller.actor(),
        Some(serde_json::json!({ "kind": row.kind })),
    )
    .await
    .map_err(super::audit_failed)?;
    txn.commit().await.map_err(super::db_err)?;
    format::json(row)
}

/// Query for the devops metrics window.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct DevopsMetricsQuery {
    /// Trailing months (default 3, cap 24).
    months: Option<u32>,
}

/// `GET /api/devops/metrics?months=` — DORA-style metrics derived
/// **only** from ingested events: deploys per month (+ per
/// environment), incidents per month, MTTR over linked
/// incident→recovery pairs (unresolved incidents counted, never
/// timed), and change-failure over **declared-cause** incidents only
/// (undeclared causation is not guessed).
#[debug_handler]
async fn devops_metrics(
    axum::extract::Query(query): axum::extract::Query<DevopsMetricsQuery>,
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    headers: HeaderMap,
) -> Result<Response> {
    let scope = super::plan_access::HiddenScope::of(&ctx, &caller).await?;
    let months = i64::from(query.months.unwrap_or(3).clamp(1, 24));
    let now = chrono::Utc::now();
    let window_start = now - chrono::Days::new(u64::try_from(months * 31).unwrap_or(93));
    let rows = scope.keep(
        devops_events::Entity::find()
            .all(&ctx.db)
            .await
            .map_err(|e| Error::Model(ModelError::from(e)))?,
    );
    let in_window = |at: chrono::DateTime<chrono::Utc>| at >= window_start && at <= now;

    let mut deploys_by_month: std::collections::BTreeMap<String, usize> =
        std::collections::BTreeMap::new();
    let mut deploys_by_environment: std::collections::BTreeMap<String, usize> =
        std::collections::BTreeMap::new();
    let mut incidents_by_month: std::collections::BTreeMap<String, usize> =
        std::collections::BTreeMap::new();
    let mut deploy_count = 0usize;
    let mut incident_count = 0usize;
    let mut declared_cause_incidents = 0usize;
    for row in &rows {
        let at = row.occurred_at.to_utc();
        if !in_window(at) {
            continue;
        }
        let month = at.format("%Y-%m").to_string();
        match row.kind.as_str() {
            "deploy" => {
                deploy_count += 1;
                *deploys_by_month.entry(month).or_default() += 1;
                if let Some(environment) = row.environment.as_deref() {
                    *deploys_by_environment
                        .entry(environment.to_string())
                        .or_default() += 1;
                }
            }
            "incident" => {
                incident_count += 1;
                *incidents_by_month.entry(month).or_default() += 1;
                if row.caused_by_deploy_pid.is_some() {
                    declared_cause_incidents += 1;
                }
            }
            _ => {}
        }
    }

    // MTTR: linked incident→recovery pairs whose incident is in-window.
    let recovery_of: std::collections::BTreeMap<Uuid, chrono::DateTime<chrono::Utc>> = rows
        .iter()
        .filter(|row| row.kind == "recovery")
        .filter_map(|row| row.incident_pid.map(|pid| (pid, row.occurred_at.to_utc())))
        .collect();
    let mut recovery_hours: Vec<i64> = Vec::new();
    let mut unresolved = 0usize;
    for row in rows.iter().filter(|row| row.kind == "incident") {
        let at = row.occurred_at.to_utc();
        if !in_window(at) {
            continue;
        }
        match recovery_of.get(&row.pid) {
            Some(recovered_at) => recovery_hours.push((*recovered_at - at).num_hours()),
            None => unresolved += 1,
        }
    }
    recovery_hours.sort_unstable();
    let median_recovery_hours = if recovery_hours.is_empty() {
        None
    } else {
        Some(recovery_hours[recovery_hours.len() / 2])
    };

    #[expect(clippy::cast_precision_loss)] // display ratio, not money math
    let change_failure_rate = if deploy_count == 0 {
        None
    } else {
        Some(declared_cause_incidents as f64 / deploy_count as f64)
    };
    let body = serde_json::json!({
        "as_of": now,
        "window_months": months,
        "derivation": "derived only from ingested devops events; MTTR = linked \
                       incident->recovery pairs (unresolved counted, never timed); \
                       change_failure_rate = declared-cause incidents / deploys \
                       (undeclared causation is not guessed)",
        "deploys": deploy_count,
        "deploys_by_month": deploys_by_month,
        "deploys_by_environment": deploys_by_environment,
        "incidents": incident_count,
        "incidents_by_month": incidents_by_month,
        "resolved_incidents": recovery_hours.len(),
        "unresolved_incidents": unresolved,
        "median_recovery_hours": median_recovery_hours,
        "declared_cause_incidents": declared_cause_incidents,
        "change_failure_rate": change_failure_rate,
    });
    conditional(&headers, &body)
}

/// `GET /api/devops/releases` — the release register: ingested deploy
/// events newest first (version / environment / item), cap 200.
#[debug_handler]
async fn devops_releases(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    headers: HeaderMap,
) -> Result<Response> {
    let scope = super::plan_access::HiddenScope::of(&ctx, &caller).await?;
    let items = live!(plans, &ctx.db, &scope);
    let by_pid: std::collections::BTreeMap<Uuid, &plans::Model> =
        items.iter().map(|i| (i.pid, i)).collect();
    let rows = devops_events::Entity::find()
        .filter(devops_events::Column::Kind.eq("deploy"))
        .order_by_desc(devops_events::Column::OccurredAt)
        .limit(200)
        .all(&ctx.db)
        .await
        .map_err(|e| Error::Model(ModelError::from(e)))?;
    let rows = scope.keep(rows);
    let releases: Vec<serde_json::Value> = rows
        .iter()
        .map(|row| {
            serde_json::json!({
                "pid": row.pid,
                "occurred_at": row.occurred_at.to_utc(),
                "environment": row.environment,
                "version": row.version,
                "reference": row.reference,
                "item": by_pid.get(&row.plan_pid).map(|i| item_ref(i)),
            })
        })
        .collect();
    let body = serde_json::json!({
        "as_of": chrono::Utc::now(),
        "releases": releases,
    });
    conditional(&headers, &body)
}

// ─── Estate views ───────────────────────────────────────────────────────────

/// `GET /api/engineering/blocked` — every blocked task estate-wide,
/// oldest blockage first, with age in calendar days (from `status_changed_at`).
#[debug_handler]
async fn blocked(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    headers: HeaderMap,
) -> Result<Response> {
    let scope = super::plan_access::HiddenScope::of(&ctx, &caller).await?;
    let items = live!(plans, &ctx.db, &scope);
    let by_pid: std::collections::BTreeMap<Uuid, &plans::Model> =
        items.iter().map(|i| (i.pid, i)).collect();
    let mut rows: Vec<&tasks::Model> = Vec::new();
    let all_tasks = scope.keep(
        tasks::Entity::find()
            .filter(tasks::Column::Status.eq("blocked"))
            .filter(tasks::Column::DeletedAt.is_null())
            .all(&ctx.db)
            .await
            .map_err(|e| Error::Model(ModelError::from(e)))?,
    );
    for task in &all_tasks {
        rows.push(task);
    }
    rows.sort_by_key(|task| task.status_changed_at);
    let blocked: Vec<serde_json::Value> = rows
        .iter()
        .map(|task| {
            let mut view = task_view(task);
            if let Some(map) = view.as_object_mut() {
                map.insert(
                    "item".to_string(),
                    by_pid
                        .get(&task.plan_pid)
                        .map_or(serde_json::Value::Null, |i| item_ref(i)),
                );
            }
            view
        })
        .collect();
    let body = serde_json::json!({
        "as_of": chrono::Utc::now(),
        "derivation": "age = calendar days since the task entered blocked (status_changed_at)",
        "blocked": blocked,
    });
    conditional(&headers, &body)
}

/// `GET /api/engineering/moscow` — the `MoSCoW` scope cut from
/// `moscow:<band>` tags (convention disclosed; untagged items listed
/// separately, never guessed into a band).
#[debug_handler]
async fn moscow(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    headers: HeaderMap,
) -> Result<Response> {
    let scope = super::plan_access::HiddenScope::of(&ctx, &caller).await?;
    let items = live!(plans, &ctx.db, &scope);
    let mut bands: std::collections::BTreeMap<&str, Vec<serde_json::Value>> = rules::MOSCOW_BANDS
        .iter()
        .map(|band| (*band, Vec::new()))
        .collect();
    let mut untagged = 0usize;
    for item in &items {
        let tags = item
            .data
            .get("tags")
            .and_then(|v| v.as_array())
            .map_or(&[][..], Vec::as_slice);
        let band = tags
            .iter()
            .filter_map(|tag| tag.as_str())
            .find_map(rules::parse_moscow_tag);
        match band {
            Some(band) => bands
                .get_mut(band)
                .expect("known band")
                .push(item_ref(item)),
            None => untagged += 1,
        }
    }
    let body = serde_json::json!({
        "as_of": chrono::Utc::now(),
        "convention": "tag `moscow:<must|should|could|wont>`; the first moscow tag wins; \
                       untagged items are counted, never guessed into a band",
        "bands": bands,
        "untagged": untagged,
    });
    conditional(&headers, &body)
}

/// `GET /api/engineering/delivery-links` — which items are tracked in
/// which external delivery tool (the `identifiers` schemes the matcher
/// already short-circuits on), plus the untracked list.
#[debug_handler]
async fn delivery_links(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    headers: HeaderMap,
) -> Result<Response> {
    const TRACKER_SCHEMES: [&str; 6] = [
        "JiraProjectKey",
        "AsanaGid",
        "TrelloBoardId",
        "MsProjectId",
        "GitHubProjectId",
        "LinearId",
    ];
    let scope = super::plan_access::HiddenScope::of(&ctx, &caller).await?;
    let items = live!(plans, &ctx.db, &scope);
    let mut tracked = Vec::new();
    let mut untracked = Vec::new();
    for item in &items {
        let identifiers = item
            .data
            .get("identifiers")
            .and_then(|v| v.as_array())
            .map_or(&[][..], Vec::as_slice);
        let links: Vec<serde_json::Value> = identifiers
            .iter()
            .filter_map(|identifier| {
                let scheme = identifier.get("scheme").and_then(|v| v.as_str())?;
                TRACKER_SCHEMES.contains(&scheme).then(|| {
                    serde_json::json!({
                        "scheme": scheme,
                        "value": identifier.get("value"),
                    })
                })
            })
            .collect();
        if links.is_empty() {
            untracked.push(item_ref(item));
        } else {
            tracked.push(serde_json::json!({ "item": item_ref(item), "links": links }));
        }
    }
    let body = serde_json::json!({
        "as_of": chrono::Utc::now(),
        "schemes": TRACKER_SCHEMES,
        "tracked": tracked,
        "untracked": untracked,
    });
    conditional(&headers, &body)
}

/// Query for the milestone calendar: optional kind filter.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct CalendarQuery {
    kind: Option<String>,
}

/// `GET /api/engineering/milestone-calendar?kind=` — estate milestones
/// with their kinds (absent reads `milestone`), due-date order, for
/// the demo / release calendar.
#[debug_handler]
async fn milestone_calendar(
    axum::extract::Query(query): axum::extract::Query<CalendarQuery>,
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    headers: HeaderMap,
) -> Result<Response> {
    let scope = super::plan_access::HiddenScope::of(&ctx, &caller).await?;
    if let Some(kind) = query.kind.as_deref()
        && !rules::MILESTONE_KINDS.contains(&kind)
    {
        return Err(refuse(&format!(
            "kind must be one of {:?}",
            rules::MILESTONE_KINDS
        )));
    }
    let items = live!(plans, &ctx.db, &scope);
    let by_pid: std::collections::BTreeMap<Uuid, &plans::Model> =
        items.iter().map(|i| (i.pid, i)).collect();
    let mut rows = live!(milestones, &ctx.db, &scope);
    rows.sort_by_key(|m| m.due);
    let entries: Vec<serde_json::Value> = rows
        .iter()
        .filter(|m| {
            query
                .kind
                .as_deref()
                .is_none_or(|kind| m.kind.as_deref().unwrap_or("milestone") == kind)
        })
        .map(|m| {
            serde_json::json!({
                "pid": m.pid,
                "name": m.name,
                "kind": m.kind.as_deref().unwrap_or("milestone"),
                "due": m.due,
                "done": m.done,
                "item": by_pid.get(&m.plan_pid).map(|i| item_ref(i)),
            })
        })
        .collect();
    let body = serde_json::json!({
        "as_of": chrono::Utc::now(),
        "kinds": rules::MILESTONE_KINDS,
        "milestones": entries,
    });
    conditional(&headers, &body)
}

/// ETag-conditional JSON response; the tag excludes `as_of`.
fn conditional(headers: &HeaderMap, body: &serde_json::Value) -> Result<Response> {
    let mut fingerprint = body.clone();
    if let Some(map) = fingerprint.as_object_mut() {
        map.remove("as_of");
    }
    super::conditional_json(headers, &super::etag_of(&fingerprint), body)
}

/// The engineering routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/tasks", post(create_task))
        .add("/plans/{pid}/tasks", get(list_tasks))
        .add("/plans/{pid}/tasks/{t_pid}", put(update_task))
        .add("/plans/{pid}/tasks/{t_pid}", patch(move_task))
        .add("/plans/{pid}/tasks/{t_pid}", delete(delete_task))
        .add("/plans/{pid}/sprints", post(create_sprint))
        .add("/plans/{pid}/sprints", get(list_sprints))
        .add("/plans/{pid}/burndown", get(burndown))
        .add("/plans/{pid}/velocity", get(velocity))
        .add("/plans/{pid}/sprints/{s_pid}/notes", post(create_note))
        .add("/plans/{pid}/sprints/{s_pid}/notes", get(list_notes))
        .add(
            "/plans/{pid}/sprints/{s_pid}/notes/{n_pid}/convert",
            post(convert_note),
        )
        .add("/plans/{pid}/standup", get(standup))
        .add("/devops/events", post(ingest_devops_event))
        .add("/devops/metrics", get(devops_metrics))
        .add("/devops/releases", get(devops_releases))
        .add("/engineering/blocked", get(blocked))
        .add("/engineering/moscow", get(moscow))
        .add("/engineering/delivery-links", get(delivery_links))
        .add("/engineering/milestone-calendar", get(milestone_calendar))
}

#[cfg(test)]
mod task_window_tests {
    use super::{UNPAGED_TASK_CAP, task_window};

    #[test]
    fn no_paging_parameters_means_the_first_capped_slice() {
        assert_eq!(task_window(None, None), (UNPAGED_TASK_CAP, 0));
    }

    #[test]
    fn a_limit_or_an_offset_means_a_standard_page() {
        assert_eq!(task_window(Some(10), None), (10, 0));
        assert_eq!(
            task_window(None, Some(40)),
            (100, 40),
            "the default page is 100"
        );
        assert_eq!(
            task_window(Some(10_000), Some(5)),
            (500, 5),
            "a big limit is clamped to 500"
        );
        assert_eq!(
            task_window(Some(0), None),
            (100, 0),
            "zero falls back to the default"
        );
    }
}
