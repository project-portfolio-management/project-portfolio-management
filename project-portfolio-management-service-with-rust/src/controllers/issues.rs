//! The issues register of a plan (IR-1). Rules in [`crate::issues`]; this is the
//! wiring. Plan-scoped, so the plan-access and per-plan module middleware apply
//! as for tasks and risks. Every write is audited. Deleting is a soft delete.
//!
//! An issue that has been `closed` must carry a resolution (`422` otherwise; the
//! database enforces it too). `resolved_on` is set by the service, never typed.

use super::db_err;
use axum::http::StatusCode;
use chrono::{NaiveDate, Utc};
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use sea_orm::{QueryOrder, QuerySelect};
use serde::Deserialize;
use uuid::Uuid;

use super::unprocessable_entity as unprocessable;
use crate::auth::MaybeAuthUser;
use crate::issues as rules;
use crate::models::_entities::{issues, plans, risks, tasks, workflow_states, workflows};
use crate::models::audit_logs::Model as AuditModel;

/// Today's date in the deployment's configured time zone, so ages and due dates agree with what users see.
fn today() -> NaiveDate {
    crate::tz::deployment_today()
}

/// JSON shape of one issue, with `age_calendar_days` and `overdue` derived from `today` rather than stored.
fn view(i: &issues::Model, today: NaiveDate) -> serde_json::Value {
    serde_json::json!({
        "pid": i.pid, "plan_pid": i.plan_pid, "title": i.title,
        "description": i.description, "kind": i.kind, "severity": i.severity,
        "priority": i.priority, "status": i.status, "status_category": i.status_category,
        "owner_ref": i.owner_ref,
        "raised_by": i.raised_by, "raised_on": i.raised_on, "due_date": i.due_date,
        "resolution": i.resolution, "resolved_on": i.resolved_on,
        "source_risk_pid": i.source_risk_pid, "source_task_pid": i.source_task_pid,
        "change_request_pid": i.change_request_pid,
        "age_calendar_days": rules::age_calendar_days(i.raised_on, i.resolved_on, today),
        "overdue": rules::is_overdue(&i.status_category, i.due_date, today),
    })
}

/// Parses the plan pid and confirms the plan exists and is not soft-deleted; `404` otherwise.
async fn require_plan(ctx: &AppContext, plan: &str) -> Result<Uuid> {
    let pid = super::parse_pid(plan)?;
    plans::Entity::find()
        .filter(plans::Column::Pid.eq(pid))
        .filter(plans::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    Ok(pid)
}

/// Loads one live (not soft-deleted) issue of the given plan; `404` when the pid is unknown or belongs to another plan.
async fn find(ctx: &AppContext, plan: Uuid, pid: &str) -> Result<issues::Model> {
    let pid = super::parse_pid(pid)?;
    issues::Entity::find()
        .filter(issues::Column::Pid.eq(pid))
        .filter(issues::Column::PlanPid.eq(plan))
        .filter(issues::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)
}

/// The most issues one plan's list returns; `truncated` says when there were more.
const PLAN_LIST_MAX: u64 = 5_000;

#[derive(Debug, Deserialize, schemars::JsonSchema)]
/// Query filters for the plan-scoped issue list; all are optional and combine with AND.
pub(crate) struct ListParams {
    #[serde(default)]
    /// Only issues in this status: a built-in one, one a deployment-wide or this plan's workflow declared, or one an issue here carries.
    status: Option<String>,
    #[serde(default)]
    /// Only issues of this severity; must be one of the rules module's severities (`422` otherwise).
    severity: Option<String>,
    #[serde(default)]
    /// Only issues whose owner reference equals this person reference exactly.
    owner: Option<String>,
    /// `true` lists only the issues that are not resolved or closed.
    #[serde(default)]
    open: Option<bool>,
}

/// `GET /api/plans/{pid}/issues` — the plan's register, most serious and oldest first.
#[debug_handler]
async fn list(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(plan): Path<String>,
    Query(q): Query<ListParams>,
) -> Result<Response> {
    let plan = require_plan(&ctx, &plan).await?;
    check_filters(
        &ctx,
        &caller,
        Some(plan),
        q.status.as_deref(),
        q.severity.as_deref(),
    )
    .await?;
    let mut find = issues::Entity::find()
        .filter(issues::Column::PlanPid.eq(plan))
        .filter(issues::Column::DeletedAt.is_null());
    if let Some(s) = &q.status {
        find = find.filter(issues::Column::Status.eq(s.as_str()));
    }
    if let Some(s) = &q.severity {
        find = find.filter(issues::Column::Severity.eq(s.as_str()));
    }
    if let Some(o) = &q.owner {
        find = find.filter(issues::Column::OwnerRef.eq(o.as_str()));
    }
    if q.open == Some(true) {
        find = find.filter(issues::Column::StatusCategory.ne("done"));
    }
    // At most PLAN_LIST_MAX + 1 rows are read, so one huge plan cannot make one answer unbounded.
    let mut rows = find
        .order_by_asc(issues::Column::Id)
        .limit(PLAN_LIST_MAX + 1)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let truncated = rows.len() as u64 > PLAN_LIST_MAX;
    rows.truncate(usize::try_from(PLAN_LIST_MAX).unwrap_or(5000));
    rows.sort_by(|a, b| {
        rules::severity_rank(&b.severity)
            .cmp(&rules::severity_rank(&a.severity))
            .then(a.raised_on.cmp(&b.raised_on))
            .then(a.id.cmp(&b.id))
    });
    let now = today();
    let open = rows
        .iter()
        .filter(|i| !rules::is_done(&i.status_category))
        .count();
    let overdue = rows
        .iter()
        .filter(|i| rules::is_overdue(&i.status_category, i.due_date, now))
        .count();
    format::json(serde_json::json!({
        "plan_pid": plan,
        "issues": rows.iter().map(|i| view(i, now)).collect::<Vec<_>>(),
        "total": rows.len(), "open": open, "overdue": overdue,
        "truncated": truncated,
        "as_of": now,
    }))
}

/// `GET /api/plans/{pid}/issues/{issue_pid}` — one issue.
#[debug_handler]
async fn get_one(
    State(ctx): State<AppContext>,
    Path((plan, pid)): Path<(String, String)>,
) -> Result<Response> {
    let plan = require_plan(&ctx, &plan).await?;
    let row = find(&ctx, plan, &pid).await?;
    format::json(view(&row, today()))
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
/// Request body for creating or replacing an issue (`POST` and `PUT`); validated by [`validate`] before any write.
pub(crate) struct Input {
    /// Short title; trimmed, required, length-limited by the rules module.
    title: String,
    #[serde(default)]
    /// Optional longer text; blank is stored as absent.
    description: Option<String>,
    #[serde(default)]
    /// Issue type, from `/api/issue-types`, not retired and offered by this plan. Absent means `concern` on a new issue and no change on an
    /// existing one. Changing the type of an issue under a workflow that restricts moves needs an administrator.
    kind: Option<String>,
    #[serde(default = "default_severity")]
    /// Severity; defaults to `med` and must be a known severity.
    severity: String,
    #[serde(default)]
    /// Optional free-text priority label; blank is stored as absent.
    priority: Option<String>,
    #[serde(default)]
    /// Status: a state of the workflow in force for this type. Absent means the workflow's initial state on a new issue,
    /// and no change on an existing one. A terminal state must carry a resolution.
    status: Option<String>,
    #[serde(default)]
    /// Person reference of the owner; absent means unowned, and it must look like a person reference.
    owner_ref: Option<String>,
    /// Default today.
    #[serde(default)]
    raised_on: Option<NaiveDate>,
    #[serde(default)]
    /// Optional date the issue should be dealt with by; absent means no due date, so it is never overdue.
    due_date: Option<NaiveDate>,
    #[serde(default)]
    /// How the issue was resolved; required in a terminal status (`closed` by default), blank is stored as absent.
    resolution: Option<String>,
    #[serde(default)]
    /// Risk this issue was raised from, when there is one; recorded as given, the risk is not changed.
    source_risk_pid: Option<Uuid>,
    #[serde(default)]
    /// Task this issue was raised from, when there is one; recorded as given, the task is not changed.
    source_task_pid: Option<Uuid>,
}

/// Serde default for [`Input::severity`].
fn default_severity() -> String {
    "med".to_string()
}
/// Whether `status` means anything to this caller: a built-in status, a state of a deployment-wide issue workflow, or a
/// status a plan they can read has declared or still carries. A withdrawn workflow counts, so an issue left in its status
/// stays findable, and `plan` narrows the plans to that one. A private plan's words are never confirmed to an outsider.
async fn status_is_known(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    plan: Option<Uuid>,
    status: &str,
) -> Result<bool> {
    if crate::workflow::built_in_issue()
        .states
        .iter()
        .any(|s| s.key == status)
    {
        return Ok(true);
    }
    let declaring: Vec<Uuid> = workflow_states::Entity::find()
        .filter(workflow_states::Column::StateKey.eq(status))
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|s| s.workflow_pid)
        .collect();
    let declared = workflows::Entity::find()
        .filter(workflows::Column::AppliesTo.eq("issue"))
        .filter(workflows::Column::Pid.is_in(declaring))
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    if declared.iter().any(|w| w.plan_pid.is_none()) {
        return Ok(true);
    }
    let carrying: Vec<Uuid> = issues::Entity::find()
        .select_only()
        .column(issues::Column::PlanPid)
        .distinct()
        .filter(issues::Column::Status.eq(status))
        .filter(issues::Column::DeletedAt.is_null())
        .into_tuple()
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let mut candidates: Vec<Uuid> = declared.iter().filter_map(|w| w.plan_pid).collect();
    candidates.extend(carrying);
    candidates.retain(|c| plan.is_none_or(|p| p == *c));
    candidates.sort();
    candidates.dedup();
    let readable = super::plan_access::readable_plans(ctx, caller, &candidates).await;
    Ok(!readable.is_empty())
}

/// `422` for a status the caller has no reason to know ([`status_is_known`]), or an unknown severity, so a typo is refused,
/// not answered with an empty list. The answer is the same for a word only a hidden plan uses as for one nobody uses.
async fn check_filters(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    plan: Option<Uuid>,
    status: Option<&str>,
    severity: Option<&str>,
) -> Result<()> {
    if let Some(status) = status
        && !status_is_known(ctx, caller, plan, status).await?
    {
        return Err(unprocessable(
            "status is not one any issue workflow you can see declares",
        ));
    }
    if severity.is_some_and(|s| !rules::SEVERITIES.contains(&s)) {
        return Err(unprocessable(&format!(
            "severity must be one of {:?}",
            rules::SEVERITIES
        )));
    }
    Ok(())
}

/// Where an issue stands in its workflow, as the service stores it.
struct Placed {
    /// The state's key.
    status: String,
    /// What the state means.
    category: String,
    /// Whether the state is terminal, so the issue needs a resolution.
    needs_resolution: bool,
}

/// The state with this key, as stored on an issue; `422` naming the choices if the workflow does not declare it.
fn placed(def: &crate::workflow::WorkflowDef, key: &str) -> Result<Placed> {
    let Some(state) = def.states.iter().find(|s| s.key == key) else {
        let keys: Vec<&str> = def.states.iter().map(|s| s.key.as_str()).collect();
        return Err(unprocessable(&format!("status must be one of {keys:?}")));
    };
    Ok(Placed {
        status: state.key.clone(),
        category: state.category.token().to_string(),
        needs_resolution: state.is_terminal,
    })
}

/// The state a new issue starts in, for the workflow in force. The built-in vocabulary is the fallback for a workflow
/// with no initial state, which [`crate::workflow::validate`] does not let anyone register.
fn initial(def: &crate::workflow::WorkflowDef) -> Placed {
    crate::workflow::initial_state(def).map_or_else(
        || Placed {
            status: "open".to_string(),
            category: "todo".to_string(),
            needs_resolution: false,
        },
        |s| Placed {
            status: s.key.clone(),
            category: s.category.token().to_string(),
            needs_resolution: s.is_terminal,
        },
    )
}

/// `403` unless the caller may bring an issue from a status its workflow does not declare into `to`.
fn enforce_entry_roles(
    def: &crate::workflow::WorkflowDef,
    to: &str,
    roles: &[String],
) -> Result<()> {
    crate::workflow::may_enter_as(def, to, roles).map_err(|denied| {
        super::forbidden(&format!(
            "entering `{to}` is restricted to the role(s): {}",
            denied.allowed.join(", ")
        ))
    })
}

/// Runs the rules module's checks on a request body and turns any problems into one `422` listing them all.
fn validate(b: &Input, raised_on: NaiveDate, needs_resolution: bool) -> Result<()> {
    let problems = rules::problems(&rules::Draft {
        title: &b.title,
        description: b.description.as_deref(),
        severity: &b.severity,
        needs_resolution,
        owner_ref: b.owner_ref.as_deref(),
        raised_on,
        due_date: b.due_date,
        resolution: b.resolution.as_deref(),
    });
    if problems.is_empty() {
        Ok(())
    } else {
        Err(unprocessable(&problems.join("; ")))
    }
}

/// Trims an optional string and maps blank to `None`, so empty input never becomes a stored empty value.
fn trimmed(s: Option<&String>) -> Option<String> {
    s.map(|v| v.trim().to_string()).filter(|v| !v.is_empty())
}

/// The fields a save changes, by the names field rules use. Type, dates of record and source links are listed too, so
/// [`crate::plan_roles`] sees every edit; a rule can only name [`crate::workflow::ISSUE_FIELDS`].
fn changed_fields(
    row: &issues::Model,
    b: &Input,
    kind: &str,
    raised_on: NaiveDate,
) -> Vec<&'static str> {
    [
        ("title", b.title.trim() != row.title),
        (
            "description",
            trimmed(b.description.as_ref()) != row.description,
        ),
        ("kind", kind != row.kind),
        ("severity", b.severity != row.severity),
        ("priority", trimmed(b.priority.as_ref()) != row.priority),
        ("owner_ref", trimmed(b.owner_ref.as_ref()) != row.owner_ref),
        ("raised_on", raised_on != row.raised_on),
        ("due_date", b.due_date != row.due_date),
        ("source_risk_pid", b.source_risk_pid != row.source_risk_pid),
        ("source_task_pid", b.source_task_pid != row.source_task_pid),
        (
            "resolution",
            trimmed(b.resolution.as_ref()) != row.resolution,
        ),
    ]
    .into_iter()
    .filter_map(|(name, changed)| changed.then_some(name))
    .collect()
}

/// `422` naming any field `state` requires of this caller that the body leaves empty.
fn check_required(
    def: &crate::workflow::WorkflowDef,
    state: &str,
    roles: &[String],
    b: &Input,
) -> Result<()> {
    let filled = |field: &str| match field {
        "description" => trimmed(b.description.as_ref()).is_some(),
        "priority" => trimmed(b.priority.as_ref()).is_some(),
        "owner_ref" => trimmed(b.owner_ref.as_ref()).is_some(),
        "due_date" => b.due_date.is_some(),
        "resolution" => trimmed(b.resolution.as_ref()).is_some(),
        // A title and a severity are always there.
        _ => true,
    };
    let missing = crate::workflow::missing_required(def, state, roles, &filled);
    if missing.is_empty() {
        Ok(())
    } else {
        Err(unprocessable(&format!(
            "the status `{state}` requires: {}",
            missing.join(", ")
        )))
    }
}

/// `POST /api/plans/{pid}/issues` — raise an issue.
#[debug_handler]
async fn create(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(plan): Path<String>,
    Json(b): Json<Input>,
) -> Result<Response> {
    let plan = require_plan(&ctx, &plan).await?;
    let standing = super::plan_roles::standing(&ctx, &caller, plan).await?;
    super::plan_roles::require(&standing, &[crate::plan_roles::ISSUE_ADD])?;
    let now = today();
    let raised_on = b.raised_on.unwrap_or(now);
    let kind = b.kind.clone().unwrap_or_else(|| "concern".to_string());
    super::issue_configuration::require_offered_type(&ctx, plan, &kind).await?;
    let def = super::workflow::in_force_issue(&ctx, plan, &kind).await?;
    let start = initial(&def);
    let at = match &b.status {
        None => start,
        // Where moves are constrained, every issue starts where the workflow says, so no one raises one past a gate.
        Some(s) if !def.transitions.is_empty() && *s != start.status => {
            return Err(unprocessable(&format!(
                "a new issue of this type starts in `{}`",
                start.status
            )));
        }
        Some(s) => placed(&def, s)?,
    };
    validate(&b, raised_on, at.needs_resolution)?;
    check_required(&def, &at.status, &standing.roles, &b)?;
    let txn = ctx.db.begin().await.map_err(super::db_err)?;
    let row = issues::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        plan_pid: ActiveValue::set(plan),
        title: ActiveValue::set(b.title.trim().to_string()),
        description: ActiveValue::set(trimmed(b.description.as_ref())),
        kind: ActiveValue::set(kind),
        severity: ActiveValue::set(b.severity.clone()),
        priority: ActiveValue::set(trimmed(b.priority.as_ref())),
        status: ActiveValue::set(at.status.clone()),
        status_category: ActiveValue::set(at.category.clone()),
        owner_ref: ActiveValue::set(trimmed(b.owner_ref.as_ref())),
        raised_by: ActiveValue::set(caller.actor().map(str::to_string)),
        raised_on: ActiveValue::set(raised_on),
        due_date: ActiveValue::set(b.due_date),
        resolution: ActiveValue::set(trimmed(b.resolution.as_ref())),
        resolved_on: ActiveValue::set(rules::resolved_on(&at.category, None, now)),
        source_risk_pid: ActiveValue::set(b.source_risk_pid),
        source_task_pid: ActiveValue::set(b.source_task_pid),
        change_request_pid: ActiveValue::set(None),
        deleted_at: ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&txn)
    .await
    .map_err(db_err)?;
    AuditModel::record(
        &txn,
        row.pid,
        "issue_raised",
        caller.actor(),
        Some(serde_json::json!({
            "plan_pid": plan, "title": row.title, "kind": row.kind,
            "severity": row.severity, "status": row.status,
        })),
    )
    .await
    .map_err(super::audit_failed)?;
    txn.commit().await.map_err(super::db_err)?;
    format::json(view(&row, now))
}

/// Whether the caller may change an issue's type. The new type must be usable, and because a type decides the workflow,
/// a change could step round a gate or a field rule in either workflow: where one restricts saves, it needs an administrator (`403`).
async fn check_type_change(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    plan: Uuid,
    from: &str,
    to: &str,
    def: &crate::workflow::WorkflowDef,
) -> Result<()> {
    super::issue_configuration::require_offered_type(ctx, plan, to).await?;
    let before = super::workflow::in_force_issue(ctx, plan, from).await?;
    let gated = crate::workflow::restricts_saves(&before) || crate::workflow::restricts_saves(def);
    if gated && !super::plan_access::is_admin(caller.claims()) {
        return Err(super::forbidden(
            "only an administrator can change the type of an issue under a workflow that restricts moves or fields",
        ));
    }
    Ok(())
}

/// Where a save leaves an issue. Not a move keeps the meaning it was stored with, even if the workflow has changed since:
/// it cannot lose a resolution it has, and the database requires one on `closed`. A move must be one the workflow allows
/// (`422`) and this caller may make (`403`), and an issue from outside the workflow must still pass any gate into its target.
fn placed_after_save(
    def: &crate::workflow::WorkflowDef,
    row: &issues::Model,
    wanted: &str,
    roles: &[String],
) -> Result<Placed> {
    if wanted == row.status {
        return Ok(Placed {
            status: row.status.clone(),
            category: row.status_category.clone(),
            needs_resolution: row.resolution.is_some() || row.status == "closed",
        });
    }
    let at = placed(def, wanted)?;
    if !crate::workflow::may_move_issue(def, &row.status, wanted) {
        return Err(unprocessable(&format!(
            "the workflow in force does not permit `{}` -> `{wanted}`",
            row.status
        )));
    }
    if def.states.iter().any(|s| s.key == row.status) {
        super::engineering::enforce_move_roles(def, &row.status, wanted, roles)?;
    } else {
        enforce_entry_roles(def, wanted, roles)?;
    }
    Ok(at)
}

/// `PUT /api/plans/{pid}/issues/{issue_pid}` — replace the fields of an issue.
#[debug_handler]
async fn update(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((plan, pid)): Path<(String, String)>,
    Json(b): Json<Input>,
) -> Result<Response> {
    let plan = require_plan(&ctx, &plan).await?;
    let row = find(&ctx, plan, &pid).await?;
    let now = today();
    let raised_on = b.raised_on.unwrap_or(row.raised_on);
    let kind = b.kind.clone().unwrap_or_else(|| row.kind.clone());
    let def = super::workflow::in_force_issue(&ctx, plan, &kind).await?;
    if kind != row.kind {
        check_type_change(&ctx, &caller, plan, &row.kind, &kind, &def).await?;
    }
    let wanted = b.status.clone().unwrap_or_else(|| row.status.clone());
    let moved = wanted != row.status;
    let standing = super::plan_roles::standing(&ctx, &caller, plan).await?;
    let changed = changed_fields(&row, &b, &kind, raised_on);
    // A resolution belongs to a move that finishes the issue, so it is not an edit there.
    let finishes = moved
        && (crate::workflow::is_done(&def, &wanted)
            || def.states.iter().any(|s| s.key == wanted && s.is_terminal));
    let edited = changed.iter().any(|f| !(finishes && *f == "resolution"));
    super::plan_roles::require(
        &standing,
        &crate::plan_roles::needed_for_save(moved, edited),
    )?;
    let at = placed_after_save(&def, &row, &wanted, &standing.roles)?;
    if !standing.admin {
        let current = super::workflow::in_force_issue(&ctx, plan, &row.kind).await?;
        let locked =
            crate::workflow::read_only_changes(&current, &row.status, &standing.roles, &changed);
        if !locked.is_empty() {
            return Err(super::forbidden(&format!(
                "in the status `{}` these are read-only for you: {}",
                row.status,
                locked.join(", ")
            )));
        }
    }
    validate(&b, raised_on, at.needs_resolution)?;
    check_required(&def, &at.status, &standing.roles, &b)?;
    let was = serde_json::json!({
        "title": row.title, "kind": row.kind, "severity": row.severity, "status": row.status,
        "status_category": row.status_category, "owner_ref": row.owner_ref,
    });
    let (issue_pid, read_status, read_kind) = (row.pid, row.status.clone(), row.kind.clone());
    let resolved_on = rules::resolved_on(&at.category, row.resolved_on, now);
    let mut a: issues::ActiveModel = row.into();
    a.title = ActiveValue::set(b.title.trim().to_string());
    a.description = ActiveValue::set(trimmed(b.description.as_ref()));
    a.kind = ActiveValue::set(kind);
    a.severity = ActiveValue::set(b.severity.clone());
    a.priority = ActiveValue::set(trimmed(b.priority.as_ref()));
    a.status = ActiveValue::set(at.status);
    a.status_category = ActiveValue::set(at.category);
    a.owner_ref = ActiveValue::set(trimmed(b.owner_ref.as_ref()));
    a.raised_on = ActiveValue::set(raised_on);
    a.due_date = ActiveValue::set(b.due_date);
    a.resolution = ActiveValue::set(trimmed(b.resolution.as_ref()));
    a.resolved_on = ActiveValue::set(resolved_on);
    a.source_risk_pid = ActiveValue::set(b.source_risk_pid);
    a.source_task_pid = ActiveValue::set(b.source_task_pid);
    let txn = ctx.db.begin().await.map_err(super::db_err)?;
    // Every check above read the issue as it was. If a move landed since, saving would undo it past its gate.
    let saved = issues::Entity::update_many()
        .set(a)
        .filter(issues::Column::Pid.eq(issue_pid))
        .filter(issues::Column::Status.eq(read_status))
        .filter(issues::Column::Kind.eq(read_kind))
        .exec(&txn)
        .await
        .map_err(db_err)?;
    if saved.rows_affected == 0 {
        return Err(super::conflict(
            "this issue moved while it was being saved; reload it and try again",
        ));
    }
    let row = issues::Entity::find()
        .filter(issues::Column::Pid.eq(issue_pid))
        .one(&txn)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    AuditModel::record(
        &txn,
        row.pid,
        "issue_changed",
        caller.actor(),
        Some(serde_json::json!({ "plan_pid": plan, "from": was,
            "to": { "title": row.title, "kind": row.kind, "severity": row.severity,
                    "status": row.status, "status_category": row.status_category,
                    "owner_ref": row.owner_ref } })),
    )
    .await
    .map_err(super::audit_failed)?;
    txn.commit().await.map_err(super::db_err)?;
    format::json(view(&row, now))
}

/// `DELETE /api/plans/{pid}/issues/{issue_pid}` — remove an issue (soft delete, audited).
#[debug_handler]
async fn remove(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((plan, pid)): Path<(String, String)>,
) -> Result<Response> {
    let plan = require_plan(&ctx, &plan).await?;
    let standing = super::plan_roles::standing(&ctx, &caller, plan).await?;
    super::plan_roles::require(&standing, &[crate::plan_roles::ISSUE_DELETE])?;
    let row = find(&ctx, plan, &pid).await?;
    // Removing an issue and raising another would otherwise step round its workflow's gates and field rules.
    let def = super::workflow::in_force_issue(&ctx, plan, &row.kind).await?;
    if !standing.enforced && !standing.admin && crate::workflow::restricts_saves(&def) {
        return Err(super::forbidden(
            "only an administrator can remove an issue under a workflow that restricts moves or fields",
        ));
    }
    let pid = row.pid;
    let detail = serde_json::json!({
        "plan_pid": plan, "title": row.title, "kind": row.kind, "status": row.status,
    });
    let mut a: issues::ActiveModel = row.into();
    a.deleted_at = ActiveValue::set(Some(Utc::now().into()));
    let txn = ctx.db.begin().await.map_err(super::db_err)?;
    a.update(&txn).await.map_err(db_err)?;
    AuditModel::record(&txn, pid, "issue_removed", caller.actor(), Some(detail))
        .await
        .map_err(super::audit_failed)?;
    txn.commit().await.map_err(super::db_err)?;
    format::empty_json()
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
/// Query filters for the cross-plan register (`GET /api/issues`); all optional and combined with AND.
pub(crate) struct RegisterParams {
    #[serde(default)]
    /// Restrict to one plan; plans the caller cannot read stay hidden regardless.
    plan: Option<Uuid>,
    #[serde(default)]
    /// Only issues in this status: a built-in one, or one a deployment-wide workflow or a plan the caller can read has used.
    status: Option<String>,
    #[serde(default)]
    /// Only issues of this severity (validated against the known severities).
    severity: Option<String>,
    #[serde(default)]
    /// Only issues owned by exactly this person reference.
    owner: Option<String>,
    #[serde(default)]
    /// `true` hides resolved and closed issues.
    open: Option<bool>,
    /// Only issues at least this many calendar days old.
    #[serde(default)]
    min_age_days: Option<i64>,
    #[serde(default)]
    /// Rows to return; defaults to 200 and is clamped to 1 through `REGISTER_MAX`.
    limit: Option<u64>,
}

/// The most rows the register returns in one answer.
const REGISTER_MAX: u64 = 500;

/// `GET /api/issues` — the register across every plan the caller can read,
/// most serious first, then oldest. Plans the caller cannot read are left out
/// without a trace; the summary counts what the caller can see, whatever the
/// filters.
#[debug_handler]
async fn register(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(q): Query<RegisterParams>,
) -> Result<Response> {
    check_filters(
        &ctx,
        &caller,
        q.plan,
        q.status.as_deref(),
        q.severity.as_deref(),
    )
    .await?;
    if q.min_age_days.is_some_and(|d| d < 0) {
        return Err(unprocessable("min_age_days cannot be negative"));
    }
    let mut find = issues::Entity::find().filter(issues::Column::DeletedAt.is_null());
    if let Some(p) = q.plan {
        find = find.filter(issues::Column::PlanPid.eq(p));
    }
    if let Some(o) = &q.owner {
        find = find.filter(issues::Column::OwnerRef.eq(o.as_str()));
    }
    let mut rows = find
        .order_by_asc(issues::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let mut plan_pids: Vec<Uuid> = rows.iter().map(|i| i.plan_pid).collect();
    plan_pids.sort();
    plan_pids.dedup();
    let readable = super::plan_access::readable_plans(&ctx, &caller, &plan_pids).await;
    rows.retain(|i| readable.contains(&i.plan_pid));

    let now = today();
    let mut by_severity: std::collections::BTreeMap<&str, usize> =
        rules::SEVERITIES.iter().map(|s| (*s, 0)).collect();
    // The built-in statuses always appear, so a dashboard has its columns; any other status appears once it is used.
    let built_in = crate::workflow::built_in_issue();
    let mut by_status: std::collections::BTreeMap<String, usize> =
        built_in.states.iter().map(|s| (s.key.clone(), 0)).collect();
    let mut overdue = 0;
    for i in &rows {
        if let Some(c) = by_severity.get_mut(i.severity.as_str()) {
            *c += 1;
        }
        *by_status.entry(i.status.clone()).or_insert(0) += 1;
        if rules::is_overdue(&i.status_category, i.due_date, now) {
            overdue += 1;
        }
    }
    let book = super::workflow::StatusBook::load(&ctx).await?;
    let worded = |i: &issues::Model| {
        let mut out = view(i, now);
        if let Some(words) = book.words(i.plan_pid, &i.kind, &i.status) {
            out["status_label"] = serde_json::json!(words.label);
            out["status_labels"] = words.labels.clone();
        }
        out
    };
    // Words for the summary's counts. Two plans can word one status differently; the first seen speaks for both.
    let mut status_words = serde_json::Map::new();
    for i in &rows {
        if let Some(words) = book.words(i.plan_pid, &i.kind, &i.status) {
            status_words.entry(i.status.clone()).or_insert_with(
                || serde_json::json!({ "label": words.label, "labels": words.labels }),
            );
        }
    }
    let oldest_open = rows
        .iter()
        .filter(|i| !rules::is_done(&i.status_category))
        .max_by_key(|i| rules::age_calendar_days(i.raised_on, i.resolved_on, now))
        .map(worded);

    rows.retain(|i| {
        q.status.as_ref().is_none_or(|s| &i.status == s)
            && q.severity.as_ref().is_none_or(|s| &i.severity == s)
            && (q.open != Some(true) || !rules::is_done(&i.status_category))
            && q.min_age_days
                .is_none_or(|d| rules::age_calendar_days(i.raised_on, i.resolved_on, now) >= d)
    });
    rows.sort_by(|a, b| {
        rules::severity_rank(&b.severity)
            .cmp(&rules::severity_rank(&a.severity))
            .then(a.raised_on.cmp(&b.raised_on))
            .then(a.id.cmp(&b.id))
    });
    let limit = usize::try_from(q.limit.unwrap_or(200).clamp(1, REGISTER_MAX)).unwrap_or(200);
    let matching = rows.len();
    format::json(serde_json::json!({
        "issues": rows.iter().take(limit).map(worded).collect::<Vec<_>>(),
        "matching": matching,
        "truncated": matching > limit,
        "summary": { "by_severity": by_severity, "by_status": by_status, "overdue": overdue,
                     "oldest_open": oldest_open, "status_words": status_words },
        "as_of": now,
        "note": "Most serious first, then oldest. Age is in calendar days since raised, until resolved. The summary counts every issue visible to the caller, whatever the filters; plans the caller cannot read are left out.",
    }))
}

/// A live, unfinished issue on this plan already raised from this source, if any. Only this plan's: an issue on another
/// plan can name any pid, and answering for it would disclose that issue.
async fn existing_open(
    ctx: &AppContext,
    plan: Uuid,
    column: issues::Column,
    source: Uuid,
) -> Result<Option<issues::Model>> {
    issues::Entity::find()
        .filter(issues::Column::PlanPid.eq(plan))
        .filter(column.eq(source))
        .filter(issues::Column::DeletedAt.is_null())
        .filter(issues::Column::StatusCategory.ne("done"))
        .one(&ctx.db)
        .await
        .map_err(db_err)
}

/// The `409` returned when an open issue already exists for the same source risk or task; names the existing issue pid.
fn already(existing: &issues::Model) -> Error {
    Error::CustomError(
        StatusCode::CONFLICT,
        ErrorDetail::new(
            "conflict",
            format!(
                "an open issue already exists for this source: {}",
                existing.pid
            ),
        ),
    )
}

#[expect(clippy::too_many_arguments)]
/// Inserts an `open` issue derived from a risk or task and audits `issue_raised` in the same transaction. The owner is kept only if it is a valid person reference, and the title is cut to the maximum length.
async fn insert_from_source(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    plan: Uuid,
    title: String,
    description: Option<String>,
    kind: &str,
    severity: &str,
    owner_ref: Option<String>,
    source_risk_pid: Option<Uuid>,
    source_task_pid: Option<Uuid>,
) -> Result<issues::Model> {
    let now = today();
    let start = initial(&super::workflow::in_force_issue(ctx, plan, kind).await?);
    let txn = ctx.db.begin().await.map_err(super::db_err)?;
    let row = issues::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        plan_pid: ActiveValue::set(plan),
        title: ActiveValue::set(title.chars().take(rules::TITLE_MAX).collect()),
        description: ActiveValue::set(description),
        kind: ActiveValue::set(kind.to_string()),
        severity: ActiveValue::set(severity.to_string()),
        priority: ActiveValue::set(None),
        status: ActiveValue::set(start.status),
        status_category: ActiveValue::set(start.category),
        owner_ref: ActiveValue::set(owner_ref.filter(|o| rules::is_person_ref(o))),
        raised_by: ActiveValue::set(caller.actor().map(str::to_string)),
        raised_on: ActiveValue::set(now),
        due_date: ActiveValue::set(None),
        resolution: ActiveValue::set(None),
        resolved_on: ActiveValue::set(None),
        source_risk_pid: ActiveValue::set(source_risk_pid),
        source_task_pid: ActiveValue::set(source_task_pid),
        change_request_pid: ActiveValue::set(None),
        deleted_at: ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&txn)
    .await
    .map_err(db_err)?;
    AuditModel::record(
        &txn,
        row.pid,
        "issue_raised",
        caller.actor(),
        Some(serde_json::json!({
            "plan_pid": plan, "title": row.title, "kind": row.kind, "severity": row.severity,
            "source_risk_pid": source_risk_pid, "source_task_pid": source_task_pid,
        })),
    )
    .await
    .map_err(super::audit_failed)?;
    txn.commit().await.map_err(super::db_err)?;
    Ok(row)
}

/// `POST /api/plans/{pid}/issues/from-risk/{risk_pid}` — raise an issue from a risk
/// that has happened. The risk is never changed. One open issue per risk (`409`).
#[debug_handler]
async fn from_risk(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((plan, risk_pid)): Path<(String, String)>,
) -> Result<Response> {
    let plan = require_plan(&ctx, &plan).await?;
    let standing = super::plan_roles::standing(&ctx, &caller, plan).await?;
    super::plan_roles::require(&standing, &[crate::plan_roles::ISSUE_ADD])?;
    let risk_pid = super::parse_pid(&risk_pid)?;
    let risk = risks::Entity::find()
        .filter(risks::Column::Pid.eq(risk_pid))
        .filter(risks::Column::PlanPid.eq(plan))
        .filter(risks::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    if let Some(found) = existing_open(&ctx, plan, issues::Column::SourceRiskPid, risk.pid).await? {
        return Err(already(&found));
    }
    let severity = rules::severity_from_exposure(risk.probability * risk.impact);
    let row = insert_from_source(
        &ctx,
        &caller,
        plan,
        risk.title.clone(),
        risk.description.clone(),
        "risk",
        severity,
        risk.owner_ref.clone(),
        Some(risk.pid),
        None,
    )
    .await?;
    format::json(view(&row, today()))
}

/// `POST /api/plans/{pid}/issues/from-task/{task_pid}` — raise a blocker issue from a
/// blocked task (`422` if the task is not blocked). The task is never changed. One open
/// issue per task (`409`).
#[debug_handler]
async fn from_task(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((plan, task_pid)): Path<(String, String)>,
) -> Result<Response> {
    let plan = require_plan(&ctx, &plan).await?;
    let standing = super::plan_roles::standing(&ctx, &caller, plan).await?;
    super::plan_roles::require(&standing, &[crate::plan_roles::ISSUE_ADD])?;
    let task_pid = super::parse_pid(&task_pid)?;
    let task = tasks::Entity::find()
        .filter(tasks::Column::Pid.eq(task_pid))
        .filter(tasks::Column::PlanPid.eq(plan))
        .filter(tasks::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    if task.status != "blocked" {
        return Err(unprocessable(
            "only a blocked task can be raised as an issue",
        ));
    }
    if let Some(found) = existing_open(&ctx, plan, issues::Column::SourceTaskPid, task.pid).await? {
        return Err(already(&found));
    }
    let row = insert_from_source(
        &ctx,
        &caller,
        plan,
        format!("Blocked: {}", task.title),
        task.description.clone(),
        "blocker",
        "med",
        task.assignee_ref.clone(),
        None,
        Some(task.pid),
    )
    .await?;
    format::json(view(&row, today()))
}

/// The issue routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/issues/from-risk/{risk_pid}", post(from_risk))
        .add("/plans/{pid}/issues/from-task/{task_pid}", post(from_task))
        .add("/issues", get(register))
        .add("/plans/{pid}/issues", get(list))
        .add("/plans/{pid}/issues", post(create))
        .add("/plans/{pid}/issues/{issue_pid}", get(get_one))
        .add("/plans/{pid}/issues/{issue_pid}", put(update))
        .add("/plans/{pid}/issues/{issue_pid}", delete(remove))
}
