//! Aggressive and safe estimates (CC-2). The rules are [`crate::ccpm_estimates`]; this is the wiring.
//!
//! * `PUT /api/plans/{pid}/tasks/{t_pid}/ccpm-estimate` — record or clear a task's two figures (audited
//!   with what they were and what they became);
//! * `GET /api/plans/{pid}/ccpm-estimates` — every task's figures with where each came from, the rule in
//!   force, and the tasks that are missing one.

use std::collections::HashMap;

use crate::prelude::*;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use super::unprocessable_entity as unprocessable;
use crate::auth::MaybeAuthUser;
use crate::ccpm_estimates::{self as est, State as EstState};
use crate::models::_entities::{task_ccpm_estimates as rows, tasks};
use crate::models::audit_logs::Model as AuditModel;

/// Most tasks one answer lists.
const MAX_LIMIT: u64 = 1_000;

/// Request body setting the two CCPM estimates for a task.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct Body {
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
    let t_pid = super::parse_pid(&t_pid)?;
    let task: tasks::Model = sqlx::query_as(
        "SELECT * FROM tasks WHERE pid = $1 AND plan_pid = $2 AND deleted_at IS NULL LIMIT 1",
    )
    .bind(t_pid)
    .bind(plan.pid)
    .fetch_optional(crate::db::pool(&ctx))
    .await?
    .ok_or(Error::NotFound)?;
    let problems = est::problems(
        task.duration_days.map(i64::from),
        b.aggressive_days.map(i64::from),
        b.safe_days.map(i64::from),
    );
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let existing: Option<rows::Model> =
        sqlx::query_as("SELECT * FROM task_ccpm_estimates WHERE task_pid = $1 LIMIT 1")
            .bind(t_pid)
            .fetch_optional(crate::db::pool(&ctx))
            .await?;
    let was = existing.as_ref().map(|r| (r.aggressive_days, r.safe_days));
    let mut tx = crate::db::begin(&ctx).await?;
    match (
        existing,
        b.aggressive_days.is_none() && b.safe_days.is_none(),
    ) {
        (Some(r), true) => {
            sqlx::query("DELETE FROM task_ccpm_estimates WHERE id = $1")
                .bind(r.id)
                .execute(tx.conn())
                .await?;
        }
        (None, true) => {}
        (Some(r), false) => {
            // `fetch_one`: an update that finds no row is an error, as before SQ-1.
            sqlx::query_scalar::<_, i64>(
                "UPDATE task_ccpm_estimates SET aggressive_days = $1, safe_days = $2 WHERE id = $3 RETURNING id",
            )
            .bind(b.aggressive_days)
            .bind(b.safe_days)
            .bind(r.id)
            .fetch_one(tx.conn())
            .await?;
        }
        (None, false) => {
            sqlx::query(
                "INSERT INTO task_ccpm_estimates (pid, plan_pid, task_pid, aggressive_days, safe_days)
                 VALUES ($1, $2, $3, $4, $5)",
            )
            .bind(Uuid::new_v4())
            .bind(plan.pid)
            .bind(t_pid)
            .bind(b.aggressive_days)
            .bind(b.safe_days)
            .execute(tx.conn())
            .await?;
        }
    }
    AuditModel::record_on(
        tx.conn(),
        t_pid,
        "task_ccpm_estimate_set",
        caller.actor(),
        Some(json!({
            "from": was.map(|w| json!({ "aggressive_days": w.0, "safe_days": w.1 })),
            "to": { "aggressive_days": b.aggressive_days, "safe_days": b.safe_days },
        })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(json!({
        "task_pid": t_pid, "aggressive_days": b.aggressive_days, "safe_days": b.safe_days,
        "likely_days": task.duration_days,
    }))
}

/// Every estimate recorded on a plan's tasks (unordered, as before).
async fn recorded_of(ctx: &AppContext, plan: Uuid) -> Result<Vec<rows::Model>> {
    Ok(
        sqlx::query_as("SELECT * FROM task_ccpm_estimates WHERE plan_pid = $1")
            .bind(plan)
            .fetch_all(crate::db::pool(ctx))
            .await?,
    )
}

/// Every task's resolved figures, under the rule in force.
pub(crate) async fn resolved(
    ctx: &AppContext,
    plan: Uuid,
    rows_of_plan: &[tasks::Model],
) -> Result<HashMap<Uuid, est::Resolved>> {
    let (rule, _) = est::configured_rule();
    let recorded: HashMap<Uuid, rows::Model> = recorded_of(ctx, plan)
        .await?
        .into_iter()
        .map(|r| (r.task_pid, r))
        .collect();
    Ok(rows_of_plan
        .iter()
        .map(|t| (t.pid, resolve_task(t, recorded.get(&t.pid), rule)))
        .collect())
}

/// Resolve a task's duration estimates from its recorded CCPM estimate, falling back to its optimistic/pessimistic days and its duration, under the given rule.
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

/// `GET /api/plans/{pid}/ccpm-estimates`
#[debug_handler]
async fn list(
    State(ctx): State<AppContext>,
    Path(pid): Path<String>,
    cursor: super::pagination::Cursor,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let page = cursor.page_up_to(200, MAX_LIMIT)?;
    let (rule, unusable) = est::configured_rule();
    let all: Vec<tasks::Model> = sqlx::query_as(
        "SELECT * FROM tasks
         WHERE plan_pid = $1 AND deleted_at IS NULL AND is_hammock = $2
         ORDER BY id",
    )
    .bind(plan.pid)
    .bind(false)
    .fetch_all(crate::db::pool(&ctx))
    .await?;
    let recorded: HashMap<Uuid, rows::Model> = recorded_of(&ctx, plan.pid)
        .await?
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
    let (tasks, more) = page.split(page.slice(items));
    let mut response = format::json(json!({
        "rule": rule.name(),
        "rule_setting_unusable": unusable,
        "total": total, "limit": page.limit(),
        "counts": counts,
        "needing_attention": missing,
        "tasks": tasks,
        "note": "The likely duration is the task's duration. An aggressive estimate is about 50 percent confidence and a safe one about 90 percent. Each figure is the one recorded for it, else the optimistic or pessimistic estimate of the schedule-risk inputs, else (only under the rule in force) a derivation, and says which. A task with a figure missing is listed under needing_attention, never given one silently.",
    }))?;
    page.stamp(&mut response, more);
    Ok(response)
}

/// The routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/tasks/{t_pid}/ccpm-estimate", put(set))
        .add("/plans/{pid}/ccpm-estimates", get(list))
}
