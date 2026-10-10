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
use crate::models::_entities::{issues, plans};
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

/// The issue routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/issues", get(list))
        .add("/plans/{pid}/issues", post(create))
        .add("/plans/{pid}/issues/{issue_pid}", get(get_one))
        .add("/plans/{pid}/issues/{issue_pid}", put(update))
        .add("/plans/{pid}/issues/{issue_pid}", delete(remove))
}
