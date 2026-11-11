//! Tasks: the Kanban board CRUD and the status move with flow stamps.

use crate::auth::MaybeAuthUser;
use crate::engineering as rules;
use crate::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

use super::{find_task, refuse, valid_person_ref};
use crate::models::_entities::tasks;
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
        "flow_type": task.flow_type,
        "created_at": task.created_at.to_utc(),
        "status_changed_at": task.status_changed_at.to_utc(),
        "done_at": task.done_at.map(|at| at.to_utc()),
        "blocked_days": blocked_days,
    })
}

/// `POST /api/plans/{pid}/tasks` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct TaskPayload {
    /// Task title.
    title: String,
    /// Optional free-text description.
    #[serde(default)]
    description: Option<String>,
    /// Initial workflow state; must be one the plan's workflow declares. Absent means the workflow's initial state.
    #[serde(default)]
    status: Option<String>,
    /// Assignee as a person reference (worker or person URN); absent means unassigned.
    #[serde(default)]
    assignee_ref: Option<String>,
    /// Sprint of this plan the task is scheduled into; absent means none.
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
    /// defaulted to `feature`. On **update**, absent keeps the declared
    /// type, `null` withdraws it (the task is unclassified again), and a
    /// value replaces it: unlike the other fields, leaving it out of a
    /// `PUT` does not clear it, as a client that never showed it would
    /// otherwise erase a declaration it did not know about.
    #[serde(default, deserialize_with = "FlowTypeField::present")]
    #[schemars(with = "Option<String>")]
    flow_type: FlowTypeField,
}

/// What a task body says about `flow_type`: nothing (the field is absent), withdraw it (`null`), or this type.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
enum FlowTypeField {
    /// Absent: on update the declared type is kept; on create there is none.
    #[default]
    Keep,
    /// `null`: no type (on update, the declaration is withdrawn).
    Clear,
    /// A type, checked by [`validate_task`].
    Set(String),
}

impl FlowTypeField {
    /// Read a present field: `null` is [`Self::Clear`]. An absent field never reaches this; it is the default.
    fn present<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Option::<String>::deserialize(d)?.map_or(Self::Clear, Self::Set))
    }

    /// The type this field declares, if any.
    fn declared(&self) -> Option<&str> {
        match self {
            Self::Set(flow_type) => Some(flow_type),
            Self::Keep | Self::Clear => None,
        }
    }
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
    if let Some(flow_type) = payload.flow_type.declared()
        && !["feature", "defect", "risk", "debt"].contains(&flow_type)
    {
        return Err(refuse(
            "flow_type must be feature, defect, risk or debt; omit it to leave the \
             item unclassified",
        ));
    }
    if let Some(component_pid) = payload.component_pid {
        let found = sqlx::query(
            "SELECT 1 FROM components
             WHERE pid = $1 AND plan_pid = $2 AND deleted_at IS NULL
             LIMIT 1",
        )
        .bind(component_pid)
        .bind(plan_pid)
        .fetch_optional(crate::db::pool(ctx))
        .await?;
        if found.is_none() {
            return Err(refuse("component_pid must name a component of this plan"));
        }
    }
    if let Some(sprint_pid) = payload.sprint_pid {
        let sprint_plan: Option<Uuid> = sqlx::query_scalar(
            "SELECT plan_pid FROM sprints WHERE pid = $1 AND deleted_at IS NULL LIMIT 1",
        )
        .bind(sprint_pid)
        .fetch_optional(crate::db::pool(ctx))
        .await?;
        if sprint_plan.is_none_or(|p| p != plan_pid) {
            return Err(refuse("sprint_pid must name a sprint of this plan"));
        }
    }
    Ok((status, workflow))
}

/// `POST /api/plans/{pid}/tasks` — create a task.
///
/// The default status is the workflow's **initial** state, and creating
/// straight into a state whose category is `done` stamps `done_at`. Where the
/// workflow restricts moves, a new task must start in the initial state, so that
/// nobody creates one past a gate.
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
    if !workflow.transitions.is_empty()
        && crate::workflow::initial_state(&workflow).is_some_and(|s| s.key != status)
    {
        return Err(refuse(
            "this workflow restricts moves, so a new task starts in its initial state",
        ));
    }
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
    let at = chrono::DateTime::<chrono::FixedOffset>::from(now);
    let mut tx = crate::db::begin(&ctx).await?;
    let row: tasks::Model = sqlx::query_as(
        "INSERT INTO tasks
             (pid, plan_pid, sprint_pid, title, description, status, assignee_ref, points, due_date,
              start_date, duration_days, component_pid, parent_task_pid, status_changed_at, flow_type,
              done_at, deleted_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, NULL)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(item.pid)
    .bind(payload.sprint_pid)
    .bind(&payload.title)
    .bind(payload.description.as_deref())
    .bind(&status)
    .bind(payload.assignee_ref.as_deref())
    .bind(payload.points)
    .bind(payload.due_date)
    .bind(payload.start_date)
    .bind(payload.duration_days)
    .bind(payload.component_pid)
    .bind(payload.parent_task_pid)
    .bind(at)
    .bind(payload.flow_type.declared())
    // Stamped from the state's **category**, not from the literal
    // string "done". A custom vocabulary finishes in whatever it
    // calls finished (`shipped`, `closed`, …), and matching on the
    // name would leave the burndown blind to every board that
    // renamed its final column — the exact failure the mandatory
    // category exists to prevent.
    .bind(crate::workflow::is_done(&workflow, &status).then_some(at))
    .fetch_one(tx.conn())
    .await?;
    super::super::custom_fields::store_on(&mut tx, "task", row.pid, &custom).await?;
    crate::tba::insert_transition_on(
        &mut tx,
        crate::tba::NewTransition {
            task_pid: row.pid,
            plan_pid: item.pid,
            from_status: None,
            to_status: &status,
            at: now,
            actor_ref: caller.actor(),
            assignee_ref: payload.assignee_ref.as_deref(),
            blocked_reason: None,
        },
    )
    .await?;
    AuditModel::record_on(tx.conn(), row.pid, "task_created", caller.actor(), None)
        .await
        .map_err(super::super::audit_failed)?;
    tx.commit().await?;

    if let Some(sub) = caller.actor() {
        super::super::watchers::ensure_watching(&ctx, "task", row.pid, sub).await;
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
    /// Component filter: a component pid, or `unassigned` for tasks with none.
    #[serde(default)]
    component: Option<String>,
}

/// The largest page of tasks, and the page when no `limit` is asked for, so one huge plan cannot make one response
/// unbounded. Past it the answer says `truncated: true`, and `Link: rel="next"` names the next page.
pub const UNPAGED_TASK_CAP: u64 = 2_000;

/// The page for a task list: [`UNPAGED_TASK_CAP`] tasks unless `limit` asks for fewer, from the signed cursor.
///
/// # Errors
///
/// `422` for a bad limit or cursor; `400` for a cursor past [`super::super::pagination::MAX_OFFSET`].
pub fn task_window(
    cursor: &super::super::pagination::Cursor,
) -> Result<super::super::pagination::Page> {
    cursor.page_up_to(UNPAGED_TASK_CAP, UNPAGED_TASK_CAP)
}

/// `GET /api/plans/{pid}/tasks` — the item's live tasks plus
/// per-status counts (board columns).
#[debug_handler]
pub(super) async fn list_tasks(
    State(ctx): State<AppContext>,
    Path(pid): Path<String>,
    Query(params): Query<TaskListParams>,
    cursor: super::super::pagination::Cursor,
) -> Result<Response> {
    let item = super::super::governance::find_item(&ctx, &pid).await?;
    // `?component=<pid>` narrows to one component; `?component=unassigned`
    // to the tasks with none (never silently dropped from a grouping).
    let component = match params.component.as_deref() {
        None => ComponentFilter::Any,
        Some("unassigned") => ComponentFilter::Unassigned,
        Some(raw) => ComponentFilter::One(
            Uuid::parse_str(raw)
                .map_err(|_| refuse("component must be a component pid or \"unassigned\""))?,
        ),
    };
    let filtered = |select: &str| {
        let mut query = sqlx::QueryBuilder::<sqlx::Postgres>::new(select);
        query
            .push(" FROM tasks WHERE plan_pid = ")
            .push_bind(item.pid)
            .push(" AND deleted_at IS NULL");
        match component {
            ComponentFilter::Any => {}
            ComponentFilter::Unassigned => {
                query.push(" AND component_pid IS NULL");
            }
            ComponentFilter::One(c) => {
                query.push(" AND component_pid = ").push_bind(c);
            }
        }
        query
    };
    let page = task_window(&cursor)?;
    // The per-status counts cover every task that matches, not just this page (board columns).
    let mut counted = filtered("SELECT status, COUNT(id) AS n");
    counted.push(" GROUP BY status");
    let by_status: Vec<(String, i64)> = counted
        .build_query_as()
        .fetch_all(crate::db::pool(&ctx))
        .await?;
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
    let mut listed = filtered("SELECT *");
    listed
        .push(" ORDER BY sibling_position, id LIMIT ")
        .push_bind(i64::try_from(page.fetch()).unwrap_or(i64::MAX))
        .push(" OFFSET ")
        .push_bind(i64::try_from(page.offset()).unwrap_or(i64::MAX));
    let rows: Vec<tasks::Model> = listed
        .build_query_as()
        .fetch_all(crate::db::pool(&ctx))
        .await?;
    let (rows, truncated) = page.split(rows);
    // An object, not a bare array, with the next page in `Link` all the same. The counts and total are of one plan
    // the caller can already read, so they count nothing hidden from them.
    let mut response = format::json(serde_json::json!({
        "tasks": rows.iter().map(task_view).collect::<Vec<_>>(),
        "counts": counts,
        "total": total,
        "truncated": truncated,
        "limit": page.limit(),
    }))?;
    page.stamp(&mut response, truncated);
    Ok(response)
}

/// What `?component=` asks a task list for.
#[derive(Clone, Copy)]
enum ComponentFilter {
    /// Every task.
    Any,
    /// The tasks with no component.
    Unassigned,
    /// The tasks of one component.
    One(Uuid),
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
    // `flow_type` is written only when the body names it (absent keeps it; `null` clears it; a value sets it).
    let sql = if payload.flow_type == FlowTypeField::Keep {
        "UPDATE tasks
         SET title = $1, description = $2, assignee_ref = $3, points = $4, due_date = $5,
             component_pid = $6, parent_task_pid = $7, sprint_pid = $8
         WHERE id = $9
         RETURNING *"
    } else {
        "UPDATE tasks
         SET title = $1, description = $2, assignee_ref = $3, points = $4, due_date = $5,
             component_pid = $6, parent_task_pid = $7, sprint_pid = $8, flow_type = $10
         WHERE id = $9
         RETURNING *"
    };
    let mut tx = crate::db::begin(&ctx).await?;
    let mut update = sqlx::query_as(sql)
        .bind(&payload.title)
        .bind(payload.description.as_deref())
        .bind(payload.assignee_ref.as_deref())
        .bind(payload.points)
        .bind(payload.due_date)
        .bind(payload.component_pid)
        .bind(payload.parent_task_pid)
        .bind(payload.sprint_pid)
        .bind(task.id);
    if payload.flow_type != FlowTypeField::Keep {
        update = update.bind(payload.flow_type.declared());
    }
    // `fetch_one`: an update that finds no row is an error, as before SQ-1.
    let row: tasks::Model = update.fetch_one(tx.conn()).await?;
    AuditModel::record_on(tx.conn(), task_pid, "task_updated", caller.actor(), None)
        .await
        .map_err(super::super::audit_failed)?;
    tx.commit().await?;
    crate::live::publish(
        item.pid,
        "task_updated",
        serde_json::json!({ "task_pid": task_pid }),
    );
    format::json(task_view(&row))
}

/// RM-2: a move may be restricted to named roles. Like the other refusals
/// in the move path this writes **no** transition row. `roles` are the token's
/// `role` attribute (a vocabulary the deployment declares, as with `view`) and
/// any held on the plan (OC-3); a caller with none holds no role.
pub(crate) fn enforce_move_roles(
    workflow: &crate::workflow::WorkflowDef,
    from: &str,
    to: &str,
    roles: &[String],
) -> Result<()> {
    crate::workflow::may_transition_as(workflow, from, to, roles).map_err(|denied| {
        Error::CustomError(
            axum::http::StatusCode::FORBIDDEN,
            axa::error::ErrorDetail::new(
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
    /// Target workflow state of the move; the workflow, WIP and blocker rules decide whether it is allowed.
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
    let occupancy: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM tasks WHERE plan_pid = $1 AND status = $2 AND deleted_at IS NULL",
    )
    .bind(plan_pid)
    .bind(status)
    .fetch_one(crate::db::pool(ctx))
    .await?;
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
    let standing = super::super::plan_roles::standing(&ctx, &caller, item.pid).await?;
    enforce_move_roles(&workflow, &task.status, &payload.status, &standing.roles)?;
    // LT-10: a task cannot be finished while a `blocks` relation's blocker is open.
    if crate::workflow::is_done(&workflow, &payload.status) {
        super::super::task_relations::ensure_not_blocked(&ctx, &caller, task.pid).await?;
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
    let at = chrono::DateTime::<chrono::FixedOffset>::from(now);
    let mut tx = crate::db::begin(&ctx).await?;
    // `done_at` is written only on the first finish (kept thereafter).
    let sql = if first_done {
        "UPDATE tasks SET status = $1, status_changed_at = $2, done_at = $2 WHERE id = $3 RETURNING *"
    } else {
        "UPDATE tasks SET status = $1, status_changed_at = $2 WHERE id = $3 RETURNING *"
    };
    // `fetch_one`: an update that finds no row is an error, as before SQ-1.
    let row: tasks::Model = sqlx::query_as(sql)
        .bind(&payload.status)
        .bind(at)
        .bind(task.id)
        .fetch_one(tx.conn())
        .await?;
    crate::tba::insert_transition_on(
        &mut tx,
        crate::tba::NewTransition {
            task_pid,
            plan_pid: item.pid,
            from_status: Some(&from),
            to_status: &payload.status,
            at: now,
            actor_ref: caller.actor(),
            assignee_ref: assignee.as_deref(),
            blocked_reason: payload.blocked_reason.as_deref(),
        },
    )
    .await?;
    AuditModel::record_on(
        tx.conn(),
        task_pid,
        "task_moved",
        caller.actor(),
        Some(serde_json::json!({ "from": from, "to": payload.status })),
    )
    .await
    .map_err(super::super::audit_failed)?;
    tx.commit().await?;

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
    let mut tx = crate::db::begin(&ctx).await?;
    // `fetch_one`: an update that finds no row is an error, as before SQ-1.
    sqlx::query_scalar::<_, i32>("UPDATE tasks SET deleted_at = $1 WHERE id = $2 RETURNING id")
        .bind(chrono::DateTime::<chrono::FixedOffset>::from(
            chrono::Utc::now(),
        ))
        .bind(task.id)
        .fetch_one(tx.conn())
        .await?;
    AuditModel::record_on(tx.conn(), task_pid, "task_deleted", caller.actor(), None)
        .await
        .map_err(super::super::audit_failed)?;
    tx.commit().await?;
    format::empty_json()
}

#[cfg(test)]
mod tests {
    use super::{FlowTypeField, TaskPayload};

    fn flow_type(body: serde_json::Value) -> FlowTypeField {
        serde_json::from_value::<TaskPayload>(body)
            .expect("a task body")
            .flow_type
    }

    #[test]
    fn an_absent_flow_type_is_keep_null_is_clear_and_a_value_is_set() {
        assert_eq!(
            flow_type(serde_json::json!({ "title": "t" })),
            FlowTypeField::Keep
        );
        assert_eq!(
            flow_type(serde_json::json!({ "title": "t", "flow_type": null })),
            FlowTypeField::Clear
        );
        assert_eq!(
            flow_type(serde_json::json!({ "title": "t", "flow_type": "debt" })),
            FlowTypeField::Set("debt".into())
        );
        assert_eq!(FlowTypeField::Set("debt".into()).declared(), Some("debt"));
        assert_eq!(FlowTypeField::Clear.declared(), None);
        assert_eq!(FlowTypeField::Keep.declared(), None);
    }
}
