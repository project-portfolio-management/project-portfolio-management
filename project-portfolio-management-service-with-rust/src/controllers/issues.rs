//! The issues register of a plan (IR-1). Rules in [`crate::issues`]; this is the
//! wiring. Plan-scoped, so the plan-access and per-plan module middleware apply
//! as for tasks and risks. Every write is audited. Deleting is a soft delete.
//!
//! An issue that has been `closed` must carry a resolution (`422` otherwise; the
//! database enforces it too). `resolved_on` is set by the service, never typed.

use axum::http::StatusCode;
use chrono::{NaiveDate, Utc};
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use sea_orm::QueryOrder;
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::MaybeAuthUser;
use crate::issues as rules;
use crate::models::_entities::{issues, plans, risks, tasks};
use crate::models::audit_logs::Model as AuditModel;

fn unprocessable(message: &str) -> Error {
    Error::CustomError(
        StatusCode::UNPROCESSABLE_ENTITY,
        ErrorDetail::new("unprocessable_entity", message),
    )
}

fn db_err(e: sea_orm::DbErr) -> Error {
    Error::Model(ModelError::from(e))
}

fn today() -> NaiveDate {
    Utc::now().date_naive()
}

fn view(i: &issues::Model, today: NaiveDate) -> serde_json::Value {
    serde_json::json!({
        "pid": i.pid, "plan_pid": i.plan_pid, "title": i.title,
        "description": i.description, "kind": i.kind, "severity": i.severity,
        "priority": i.priority, "status": i.status, "owner_ref": i.owner_ref,
        "raised_by": i.raised_by, "raised_on": i.raised_on, "due_date": i.due_date,
        "resolution": i.resolution, "resolved_on": i.resolved_on,
        "source_risk_pid": i.source_risk_pid, "source_task_pid": i.source_task_pid,
        "change_request_pid": i.change_request_pid,
        "age_calendar_days": rules::age_calendar_days(i.raised_on, i.resolved_on, today),
        "overdue": rules::is_overdue(&i.status, i.due_date, today),
    })
}

async fn require_plan(ctx: &AppContext, plan: &str) -> Result<Uuid> {
    let pid = Uuid::parse_str(plan).map_err(|_| Error::NotFound)?;
    plans::Entity::find()
        .filter(plans::Column::Pid.eq(pid))
        .filter(plans::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    Ok(pid)
}

async fn find(ctx: &AppContext, plan: Uuid, pid: &str) -> Result<issues::Model> {
    let pid = Uuid::parse_str(pid).map_err(|_| Error::NotFound)?;
    issues::Entity::find()
        .filter(issues::Column::Pid.eq(pid))
        .filter(issues::Column::PlanPid.eq(plan))
        .filter(issues::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)
}

#[derive(Debug, Deserialize)]
struct ListParams {
    #[serde(default)]
    status: Option<String>,
    #[serde(default)]
    severity: Option<String>,
    #[serde(default)]
    owner: Option<String>,
    /// `true` lists only the issues that are not resolved or closed.
    #[serde(default)]
    open: Option<bool>,
}

/// `GET /api/plans/{pid}/issues` — the plan's register, most serious and oldest first.
#[debug_handler]
async fn list(
    State(ctx): State<AppContext>,
    Path(plan): Path<String>,
    Query(q): Query<ListParams>,
) -> Result<Response> {
    let plan = require_plan(&ctx, &plan).await?;
    for (name, value, allowed) in [
        ("status", q.status.as_deref(), rules::STATUSES),
        ("severity", q.severity.as_deref(), rules::SEVERITIES),
    ] {
        if value.is_some_and(|v| !allowed.contains(&v)) {
            return Err(unprocessable(&format!("{name} must be one of {allowed:?}")));
        }
    }
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
        find = find.filter(issues::Column::Status.is_in(["open", "in_progress"]));
    }
    let mut rows = find
        .order_by_asc(issues::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    rows.sort_by(|a, b| {
        rules::severity_rank(&b.severity)
            .cmp(&rules::severity_rank(&a.severity))
            .then(a.raised_on.cmp(&b.raised_on))
            .then(a.id.cmp(&b.id))
    });
    let now = today();
    let open = rows.iter().filter(|i| !rules::is_done(&i.status)).count();
    let overdue = rows
        .iter()
        .filter(|i| rules::is_overdue(&i.status, i.due_date, now))
        .count();
    format::json(serde_json::json!({
        "plan_pid": plan,
        "issues": rows.iter().map(|i| view(i, now)).collect::<Vec<_>>(),
        "total": rows.len(), "open": open, "overdue": overdue,
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

#[derive(Debug, Deserialize)]
struct Input {
    title: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(default = "default_kind")]
    kind: String,
    #[serde(default = "default_severity")]
    severity: String,
    #[serde(default)]
    priority: Option<String>,
    #[serde(default = "default_status")]
    status: String,
    #[serde(default)]
    owner_ref: Option<String>,
    /// Default today.
    #[serde(default)]
    raised_on: Option<NaiveDate>,
    #[serde(default)]
    due_date: Option<NaiveDate>,
    #[serde(default)]
    resolution: Option<String>,
    #[serde(default)]
    source_risk_pid: Option<Uuid>,
    #[serde(default)]
    source_task_pid: Option<Uuid>,
}

fn default_kind() -> String {
    "concern".to_string()
}
fn default_severity() -> String {
    "med".to_string()
}
fn default_status() -> String {
    "open".to_string()
}

fn validate(b: &Input, raised_on: NaiveDate) -> Result<()> {
    let problems = rules::problems(&rules::Draft {
        title: &b.title,
        description: b.description.as_deref(),
        kind: &b.kind,
        severity: &b.severity,
        status: &b.status,
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

fn trimmed(s: Option<&String>) -> Option<String> {
    s.map(|v| v.trim().to_string()).filter(|v| !v.is_empty())
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
    let now = today();
    let raised_on = b.raised_on.unwrap_or(now);
    validate(&b, raised_on)?;
    let row = issues::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        plan_pid: ActiveValue::set(plan),
        title: ActiveValue::set(b.title.trim().to_string()),
        description: ActiveValue::set(trimmed(b.description.as_ref())),
        kind: ActiveValue::set(b.kind.clone()),
        severity: ActiveValue::set(b.severity.clone()),
        priority: ActiveValue::set(trimmed(b.priority.as_ref())),
        status: ActiveValue::set(b.status.clone()),
        owner_ref: ActiveValue::set(trimmed(b.owner_ref.as_ref())),
        raised_by: ActiveValue::set(caller.actor().map(str::to_string)),
        raised_on: ActiveValue::set(raised_on),
        due_date: ActiveValue::set(b.due_date),
        resolution: ActiveValue::set(trimmed(b.resolution.as_ref())),
        resolved_on: ActiveValue::set(rules::resolved_on(&b.status, None, now)),
        source_risk_pid: ActiveValue::set(b.source_risk_pid),
        source_task_pid: ActiveValue::set(b.source_task_pid),
        change_request_pid: ActiveValue::set(None),
        deleted_at: ActiveValue::set(None),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(db_err)?;
    AuditModel::record(
        &ctx.db,
        row.pid,
        "issue_raised",
        caller.actor(),
        Some(serde_json::json!({
            "plan_pid": plan, "title": row.title, "kind": row.kind,
            "severity": row.severity, "status": row.status,
        })),
    )
    .await
    .ok();
    format::json(view(&row, now))
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
    validate(&b, raised_on)?;
    let was = serde_json::json!({
        "title": row.title, "severity": row.severity, "status": row.status,
        "owner_ref": row.owner_ref,
    });
    let resolved_on = rules::resolved_on(&b.status, row.resolved_on, now);
    let mut a: issues::ActiveModel = row.into();
    a.title = ActiveValue::set(b.title.trim().to_string());
    a.description = ActiveValue::set(trimmed(b.description.as_ref()));
    a.kind = ActiveValue::set(b.kind.clone());
    a.severity = ActiveValue::set(b.severity.clone());
    a.priority = ActiveValue::set(trimmed(b.priority.as_ref()));
    a.status = ActiveValue::set(b.status.clone());
    a.owner_ref = ActiveValue::set(trimmed(b.owner_ref.as_ref()));
    a.raised_on = ActiveValue::set(raised_on);
    a.due_date = ActiveValue::set(b.due_date);
    a.resolution = ActiveValue::set(trimmed(b.resolution.as_ref()));
    a.resolved_on = ActiveValue::set(resolved_on);
    a.source_risk_pid = ActiveValue::set(b.source_risk_pid);
    a.source_task_pid = ActiveValue::set(b.source_task_pid);
    let row = a.update(&ctx.db).await.map_err(db_err)?;
    AuditModel::record(
        &ctx.db,
        row.pid,
        "issue_changed",
        caller.actor(),
        Some(serde_json::json!({ "from": was,
            "to": { "title": row.title, "severity": row.severity, "status": row.status,
                    "owner_ref": row.owner_ref } })),
    )
    .await
    .ok();
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
    let row = find(&ctx, plan, &pid).await?;
    let pid = row.pid;
    let mut a: issues::ActiveModel = row.into();
    a.deleted_at = ActiveValue::set(Some(Utc::now().into()));
    a.update(&ctx.db).await.map_err(db_err)?;
    AuditModel::record(&ctx.db, pid, "issue_removed", caller.actor(), None)
        .await
        .ok();
    format::empty_json()
}

#[derive(Debug, Deserialize)]
struct RegisterParams {
    #[serde(default)]
    plan: Option<Uuid>,
    #[serde(default)]
    status: Option<String>,
    #[serde(default)]
    severity: Option<String>,
    #[serde(default)]
    owner: Option<String>,
    #[serde(default)]
    open: Option<bool>,
    /// Only issues at least this many calendar days old.
    #[serde(default)]
    min_age_days: Option<i64>,
    #[serde(default)]
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
    for (name, value, allowed) in [
        ("status", q.status.as_deref(), rules::STATUSES),
        ("severity", q.severity.as_deref(), rules::SEVERITIES),
    ] {
        if value.is_some_and(|v| !allowed.contains(&v)) {
            return Err(unprocessable(&format!("{name} must be one of {allowed:?}")));
        }
    }
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
    let mut by_status: std::collections::BTreeMap<&str, usize> =
        rules::STATUSES.iter().map(|s| (*s, 0)).collect();
    let mut overdue = 0;
    for i in &rows {
        if let Some(c) = by_severity.get_mut(i.severity.as_str()) {
            *c += 1;
        }
        if let Some(c) = by_status.get_mut(i.status.as_str()) {
            *c += 1;
        }
        if rules::is_overdue(&i.status, i.due_date, now) {
            overdue += 1;
        }
    }
    let oldest_open = rows
        .iter()
        .filter(|i| !rules::is_done(&i.status))
        .max_by_key(|i| rules::age_calendar_days(i.raised_on, i.resolved_on, now))
        .map(|i| view(i, now));

    rows.retain(|i| {
        q.status.as_ref().is_none_or(|s| &i.status == s)
            && q.severity.as_ref().is_none_or(|s| &i.severity == s)
            && (q.open != Some(true) || !rules::is_done(&i.status))
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
        "issues": rows.iter().take(limit).map(|i| view(i, now)).collect::<Vec<_>>(),
        "matching": matching,
        "truncated": matching > limit,
        "summary": { "by_severity": by_severity, "by_status": by_status, "overdue": overdue,
                     "oldest_open": oldest_open },
        "as_of": now,
        "note": "Most serious first, then oldest. Age is in calendar days since raised, until resolved. The summary counts every issue visible to the caller, whatever the filters; plans the caller cannot read are left out.",
    }))
}

/// A live, unfinished issue already raised from this source, if any.
async fn existing_open(
    ctx: &AppContext,
    column: issues::Column,
    source: Uuid,
) -> Result<Option<issues::Model>> {
    issues::Entity::find()
        .filter(column.eq(source))
        .filter(issues::Column::DeletedAt.is_null())
        .filter(issues::Column::Status.is_in(["open", "in_progress"]))
        .one(&ctx.db)
        .await
        .map_err(db_err)
}

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

#[allow(clippy::too_many_arguments)]
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
    let row = issues::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        plan_pid: ActiveValue::set(plan),
        title: ActiveValue::set(title.chars().take(rules::TITLE_MAX).collect()),
        description: ActiveValue::set(description),
        kind: ActiveValue::set(kind.to_string()),
        severity: ActiveValue::set(severity.to_string()),
        priority: ActiveValue::set(None),
        status: ActiveValue::set("open".to_string()),
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
    .insert(&ctx.db)
    .await
    .map_err(db_err)?;
    AuditModel::record(
        &ctx.db,
        row.pid,
        "issue_raised",
        caller.actor(),
        Some(serde_json::json!({
            "plan_pid": plan, "title": row.title, "kind": row.kind, "severity": row.severity,
            "source_risk_pid": source_risk_pid, "source_task_pid": source_task_pid,
        })),
    )
    .await
    .ok();
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
    let risk_pid = Uuid::parse_str(&risk_pid).map_err(|_| Error::NotFound)?;
    let risk = risks::Entity::find()
        .filter(risks::Column::Pid.eq(risk_pid))
        .filter(risks::Column::PlanPid.eq(plan))
        .filter(risks::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    if let Some(found) = existing_open(&ctx, issues::Column::SourceRiskPid, risk.pid).await? {
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
    let task_pid = Uuid::parse_str(&task_pid).map_err(|_| Error::NotFound)?;
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
    if let Some(found) = existing_open(&ctx, issues::Column::SourceTaskPid, task.pid).await? {
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
