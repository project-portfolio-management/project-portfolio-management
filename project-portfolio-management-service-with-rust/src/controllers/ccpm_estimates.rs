//! Aggressive and safe estimates (CC-2). The rules are [`crate::ccpm_estimates`]; this is the wiring.
//!
//! * `PUT /api/plans/{pid}/tasks/{t_pid}/ccpm-estimate` — record or clear a task's two figures (audited
//!   with what they were and what they became);
//! * `GET /api/plans/{pid}/ccpm-estimates` — every task's figures with where each came from, the rule in
//!   force, and the tasks that are missing one.

use super::db_err;
use std::collections::HashMap;

use axum::http::StatusCode;
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use sea_orm::QueryOrder;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::auth::MaybeAuthUser;
use crate::ccpm_estimates::{self as est, State as EstState};
use crate::models::_entities::{task_ccpm_estimates as rows, tasks};
use crate::models::audit_logs::Model as AuditModel;

fn unprocessable(message: &str) -> Error {
    Error::CustomError(
        StatusCode::UNPROCESSABLE_ENTITY,
        ErrorDetail::new("unprocessable_entity", message),
    )
}

/// Most tasks one answer lists.
const MAX_LIMIT: u64 = 1_000;

#[derive(Debug, Deserialize)]
struct Body {
    /// Business days at about 50 percent confidence; `null` clears.
    #[serde(default)]
    aggressive_days: Option<i32>,
    /// Business days at about 90 percent confidence; `null` clears.
    #[serde(default)]
    safe_days: Option<i32>,
}

/// `PUT /api/plans/{pid}/tasks/{t_pid}/ccpm-estimate`
#[debug_handler]
async fn set(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, t_pid)): Path<(String, String)>,
    Json(b): Json<Body>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let t_pid = Uuid::parse_str(&t_pid).map_err(|_| Error::NotFound)?;
    let task = tasks::Entity::find()
        .filter(tasks::Column::Pid.eq(t_pid))
        .filter(tasks::Column::PlanPid.eq(plan.pid))
        .filter(tasks::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    let problems = est::problems(
        task.duration_days.map(i64::from),
        b.aggressive_days.map(i64::from),
        b.safe_days.map(i64::from),
    );
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let existing = rows::Entity::find()
        .filter(rows::Column::TaskPid.eq(t_pid))
        .one(&ctx.db)
        .await
        .map_err(db_err)?;
    let was = existing.as_ref().map(|r| (r.aggressive_days, r.safe_days));
    match (
        existing,
        b.aggressive_days.is_none() && b.safe_days.is_none(),
    ) {
        (Some(r), true) => {
            r.delete(&ctx.db).await.map_err(db_err)?;
        }
        (None, true) => {}
        (Some(r), false) => {
            let mut a: rows::ActiveModel = r.into();
            a.aggressive_days = ActiveValue::set(b.aggressive_days);
            a.safe_days = ActiveValue::set(b.safe_days);
            a.update(&ctx.db).await.map_err(db_err)?;
        }
        (None, false) => {
            rows::ActiveModel {
                pid: ActiveValue::set(Uuid::new_v4()),
                plan_pid: ActiveValue::set(plan.pid),
                task_pid: ActiveValue::set(t_pid),
                aggressive_days: ActiveValue::set(b.aggressive_days),
                safe_days: ActiveValue::set(b.safe_days),
                ..Default::default()
            }
            .insert(&ctx.db)
            .await
            .map_err(db_err)?;
        }
    }
    AuditModel::record(
        &ctx.db,
        t_pid,
        "task_ccpm_estimate_set",
        caller.actor(),
        Some(json!({
            "from": was.map(|w| json!({ "aggressive_days": w.0, "safe_days": w.1 })),
            "to": { "aggressive_days": b.aggressive_days, "safe_days": b.safe_days },
        })),
    )
    .await
    .ok();
    format::json(json!({
        "task_pid": t_pid, "aggressive_days": b.aggressive_days, "safe_days": b.safe_days,
        "likely_days": task.duration_days,
    }))
}

/// Every task's resolved figures, under the rule in force.
pub(crate) async fn resolved(
    ctx: &AppContext,
    plan: Uuid,
    rows_of_plan: &[tasks::Model],
) -> Result<HashMap<Uuid, est::Resolved>> {
    let (rule, _) = est::configured_rule();
    let recorded: HashMap<Uuid, rows::Model> = rows::Entity::find()
        .filter(rows::Column::PlanPid.eq(plan))
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|r| (r.task_pid, r))
        .collect();
    Ok(rows_of_plan
        .iter()
        .map(|t| (t.pid, resolve_task(t, recorded.get(&t.pid), rule)))
        .collect())
}

fn resolve_task(t: &tasks::Model, rec: Option<&rows::Model>, rule: est::Rule) -> est::Resolved {
    est::resolve(
        t.duration_days.map(i64::from),
        (
            rec.and_then(|r| r.aggressive_days).map(i64::from),
            rec.and_then(|r| r.safe_days).map(i64::from),
        ),
        (
            t.duration_optimistic_days.map(i64::from),
            t.duration_pessimistic_days.map(i64::from),
        ),
        rule,
    )
}

#[derive(Debug, Deserialize)]
struct Page {
    #[serde(default)]
    limit: Option<u64>,
    #[serde(default)]
    offset: Option<u64>,
}

/// `GET /api/plans/{pid}/ccpm-estimates`
#[debug_handler]
async fn list(
    State(ctx): State<AppContext>,
    Path(pid): Path<String>,
    Query(q): Query<Page>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let limit = q.limit.unwrap_or(200);
    if !(1..=MAX_LIMIT).contains(&limit) {
        return Err(unprocessable(&format!("limit must be 1 to {MAX_LIMIT}")));
    }
    let (rule, unusable) = est::configured_rule();
    let all = tasks::Entity::find()
        .filter(tasks::Column::PlanPid.eq(plan.pid))
        .filter(tasks::Column::DeletedAt.is_null())
        .filter(tasks::Column::IsHammock.eq(false))
        .order_by_asc(tasks::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let recorded: HashMap<Uuid, rows::Model> = rows::Entity::find()
        .filter(rows::Column::PlanPid.eq(plan.pid))
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|r| (r.task_pid, r))
        .collect();
    let mut counts: HashMap<&'static str, i64> = HashMap::new();
    let mut missing: Vec<Uuid> = Vec::new();
    let mut items = Vec::new();
    for t in &all {
        let rec = recorded.get(&t.pid);
        let r = resolve_task(t, rec, rule);
        let name = serde_json::to_value(r.state)
            .ok()
            .and_then(|v| v.as_str().map(str::to_string))
            .unwrap_or_default();
        *counts
            .entry(match r.state {
                EstState::Complete => "complete",
                EstState::Derived => "derived",
                EstState::Missing => "missing",
                EstState::Inconsistent => "inconsistent",
                EstState::Milestone => "milestone",
                EstState::NoDuration => "no_duration",
            })
            .or_default() += 1;
        if matches!(r.state, EstState::Missing | EstState::Inconsistent) {
            missing.push(t.pid);
        }
        items.push(json!({
            "pid": t.pid, "title": t.title, "likely_days": t.duration_days,
            "aggressive": r.aggressive, "safe": r.safe, "state": name,
        }));
    }
    let total = items.len();
    let offset = usize::try_from(q.offset.unwrap_or(0)).unwrap_or(usize::MAX);
    let page: Vec<_> = items
        .into_iter()
        .skip(offset)
        .take(usize::try_from(limit).unwrap_or(0))
        .collect();
    format::json(json!({
        "rule": rule.name(),
        "rule_setting_unusable": unusable,
        "total": total, "limit": limit, "offset": offset,
        "counts": counts,
        "needing_attention": missing,
        "tasks": page,
        "note": "The likely duration is the task's duration. An aggressive estimate is about 50 percent confidence and a safe one about 90 percent. Each figure is the one recorded for it, else the optimistic or pessimistic estimate of the schedule-risk inputs, else (only under the rule in force) a derivation, and says which. A task with a figure missing is listed under needing_attention, never given one silently.",
    }))
}

/// The routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/tasks/{t_pid}/ccpm-estimate", put(set))
        .add("/plans/{pid}/ccpm-estimates", get(list))
}
