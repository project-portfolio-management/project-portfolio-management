//! Planned gate dates, gate decisions in a period, and programme status (DP-5). Rules in
//! [`crate::gate_decisions`].
//!
//! A programme is active until a **recorded decision** pauses or stops it: the decision names who made
//! it, why, when, and the gate review or exception it follows from. `stopped` is terminal and keeps the
//! record. A pause flags the programme's allocations in the pack and the status read; it deletes none.

use super::refuse;
use std::collections::{BTreeMap, HashMap, HashSet};

use axum::http::StatusCode;
use chrono::{DateTime, NaiveDate, Utc};
use loco_rs::prelude::*;
use sea_orm::QueryOrder;
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::{MaybeAuthUser, require_auth};
use crate::gate_decisions::{self as rules, Decision, Planned, Review};
use crate::governance as gov_rules;
use crate::models::_entities::{
    allocations, benefits, budget_lines, delivery_exceptions, gate_reviews, planned_gates, plans,
    programme_status_decisions as decisions,
};
use crate::models::audit_logs::Model as AuditModel;

fn unprocessable(message: &str) -> Error {
    refuse(
        StatusCode::UNPROCESSABLE_ENTITY,
        "unprocessable_entity",
        message,
    )
}

fn db_err(e: &sea_orm::DbErr) -> Error {
    let text = e.to_string();
    // The database enforces the transition rules too; a race lands here.
    if text.contains("already has that status") || text.contains("stays stopped") {
        return refuse(
            StatusCode::CONFLICT,
            "conflict",
            "the programme's status changed meanwhile",
        );
    }
    Error::string(&text)
}

/// Pausing or stopping a programme is a decision of its own: `access=admin` or `programme=decider`.
/// With login off everyone is trusted, as for every other write.
fn may_decide(caller: &MaybeAuthUser) -> bool {
    if !require_auth() {
        return true;
    }
    super::plan_access::is_admin(caller.claims())
        || caller
            .claims()
            .and_then(|c| c.attrs.get("programme"))
            .is_some_and(|v| v.iter().any(|x| x == "decider"))
}

/// The routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/planned-gates", get(list_planned))
        .add("/plans/{pid}/planned-gates/{gate}", put(set_planned))
        .add("/plans/{pid}/programme-status", get(get_status))
        .add("/plans/{pid}/programme-status", post(decide_status))
}

async fn history_of(ctx: &AppContext, plan: Uuid) -> Result<Vec<decisions::Model>> {
    decisions::Entity::find()
        .filter(decisions::Column::PlanPid.eq(plan))
        .order_by_asc(decisions::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))
}

fn as_rules(h: &[decisions::Model]) -> Vec<Decision> {
    h.iter()
        .map(|d| Decision {
            status: d.status.clone(),
            decided_on: d.decided_on,
        })
        .collect()
}

/// A plan and every plan beneath it.
fn subtree(all: &[plans::Model], root: Uuid) -> HashSet<Uuid> {
    let mut by_parent: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
    for p in all {
        if let Some(parent) = p.parent_pid {
            by_parent.entry(parent).or_default().push(p.pid);
        }
    }
    let mut out = HashSet::from([root]);
    let mut stack = vec![root];
    while let Some(n) = stack.pop() {
        for c in by_parent.get(&n).into_iter().flatten() {
            if out.insert(*c) {
                stack.push(*c);
            }
        }
    }
    out
}

/// What a paused or stopped programme still has: allocations flagged (counted, never deleted), money per
/// currency and benefits, all reportable.
async fn kept(ctx: &AppContext, tree: &HashSet<Uuid>) -> Result<serde_json::Value> {
    let allocs = allocations::Entity::find()
        .filter(allocations::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .into_iter()
        .filter(|a| tree.contains(&a.plan_pid))
        .count();
    let mut money: BTreeMap<String, (i64, i64)> = BTreeMap::new();
    for b in budget_lines::Entity::find()
        .filter(budget_lines::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .into_iter()
        .filter(|b| tree.contains(&b.plan_pid))
    {
        let e = money.entry(b.currency).or_default();
        e.0 += b.planned_minor;
        e.1 += b.actual_minor;
    }
    let benefit_rows: Vec<benefits::Model> = benefits::Entity::find()
        .filter(benefits::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .into_iter()
        .filter(|b| tree.contains(&b.plan_pid))
        .collect();
    Ok(serde_json::json!({
        "allocations_flagged": allocs,
        "money_by_currency": money.iter().map(|(c, (p, a))| serde_json::json!({
            "currency": c, "planned_minor": p, "actual_minor": a })).collect::<Vec<_>>(),
        "benefits": benefit_rows.len(),
    }))
}

// ---- planned gate dates ----------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct PlannedBody {
    planned_on: NaiveDate,
}

/// `PUT /api/plans/{pid}/planned-gates/{gate}` — the date the plan expects its review at a gate.
#[debug_handler]
async fn set_planned(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, gate)): Path<(String, String)>,
    Json(b): Json<PlannedBody>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    if !gov_rules::is_token(gov_rules::GATES, &gate) {
        return Err(unprocessable(&format!(
            "gate must be one of {:?}",
            gov_rules::GATES
        )));
    }
    let existing = planned_gates::Entity::find()
        .filter(planned_gates::Column::PlanPid.eq(plan.pid))
        .filter(planned_gates::Column::Gate.eq(&gate))
        .one(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    let row = if let Some(e) = existing {
        let mut a: planned_gates::ActiveModel = e.into();
        a.planned_on = ActiveValue::set(b.planned_on);
        a.set_by = ActiveValue::set(caller.actor().map(str::to_string));
        a.update(&ctx.db).await.map_err(|e| db_err(&e))?
    } else {
        planned_gates::ActiveModel {
            pid: ActiveValue::set(Uuid::new_v4()),
            plan_pid: ActiveValue::set(plan.pid),
            gate: ActiveValue::set(gate.clone()),
            planned_on: ActiveValue::set(b.planned_on),
            set_by: ActiveValue::set(caller.actor().map(str::to_string)),
            ..Default::default()
        }
        .insert(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
    };
    AuditModel::record(
        &ctx.db,
        plan.pid,
        "gate_planned",
        caller.actor(),
        Some(serde_json::json!({ "gate": gate, "planned_on": b.planned_on })),
    )
    .await
    .map_err(|e| Error::string(&e.to_string()))?;
    format::json(
        serde_json::json!({ "plan_pid": plan.pid, "gate": row.gate, "planned_on": row.planned_on }),
    )
}

/// `GET /api/plans/{pid}/planned-gates`.
#[debug_handler]
async fn list_planned(State(ctx): State<AppContext>, Path(pid): Path<String>) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let rows = planned_gates::Entity::find()
        .filter(planned_gates::Column::PlanPid.eq(plan.pid))
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    let mut out: Vec<serde_json::Value> = rows
        .iter()
        .map(|r| serde_json::json!({ "gate": r.gate, "planned_on": r.planned_on }))
        .collect();
    out.sort_by(|a, b| a["gate"].as_str().cmp(&b["gate"].as_str()));
    format::json(serde_json::json!({ "plan_pid": plan.pid, "planned_gates": out }))
}

// ---- programme status ------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct StatusBody {
    status: String,
    reason: String,
    #[serde(default)]
    gate_review_pid: Option<Uuid>,
    #[serde(default)]
    exception_pid: Option<Uuid>,
}

fn history_json(h: &[decisions::Model]) -> Vec<serde_json::Value> {
    h.iter()
        .map(|d| {
            serde_json::json!({
                "status": d.status, "reason": d.reason, "decided_by": d.decided_by,
                "decided_on": d.decided_on, "gate_review_pid": d.gate_review_pid,
                "exception_pid": d.exception_pid,
            })
        })
        .collect()
}

/// `POST /api/plans/{pid}/programme-status` — pause, resume or stop a programme by a recorded decision.
#[debug_handler]
async fn decide_status(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<StatusBody>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    if !may_decide(&caller) {
        return Err(refuse(
            StatusCode::FORBIDDEN,
            "forbidden",
            "pausing, resuming or stopping a programme needs access=admin or programme=decider",
        ));
    }
    if plan.kind.as_deref() != Some("Program") {
        return Err(unprocessable(
            "only a plan of kind Program has a programme status",
        ));
    }
    if b.reason.trim().chars().count() < rules::MIN_REASON {
        return Err(unprocessable("the reason must be at least 10 characters"));
    }
    let history = history_of(&ctx, plan.pid).await?;
    let today = Utc::now().date_naive();
    let current = rules::status_at(&as_rules(&history), today);
    // The database's last row is the truth even if its date is in the future; the date is today's.
    let current = history.last().map_or(current, |d| match d.status.as_str() {
        "paused" => "paused",
        "stopped" => "stopped",
        _ => "active",
    });
    if let Err(why) = rules::check_transition(current, &b.status) {
        return Err(refuse(StatusCode::CONFLICT, "conflict", why));
    }
    if let Some(g) = b.gate_review_pid {
        let ok = gate_reviews::Entity::find()
            .filter(gate_reviews::Column::Pid.eq(g))
            .filter(gate_reviews::Column::PlanPid.eq(plan.pid))
            .one(&ctx.db)
            .await
            .map_err(|e| db_err(&e))?
            .is_some();
        if !ok {
            return Err(unprocessable(
                "gate_review_pid is not a gate review of this programme",
            ));
        }
    }
    if let Some(x) = b.exception_pid {
        let ok = delivery_exceptions::Entity::find()
            .filter(delivery_exceptions::Column::Pid.eq(x))
            .filter(delivery_exceptions::Column::PlanPid.eq(plan.pid))
            .one(&ctx.db)
            .await
            .map_err(|e| db_err(&e))?
            .is_some();
        if !ok {
            return Err(unprocessable(
                "exception_pid is not an exception of this programme",
            ));
        }
    }
    let actor = caller.actor().unwrap_or("system").to_string();
    let row = decisions::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        plan_pid: ActiveValue::set(plan.pid),
        status: ActiveValue::set(b.status.clone()),
        reason: ActiveValue::set(b.reason.trim().to_string()),
        decided_by: ActiveValue::set(Some(actor)),
        decided_on: ActiveValue::set(today),
        gate_review_pid: ActiveValue::set(b.gate_review_pid),
        exception_pid: ActiveValue::set(b.exception_pid),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(|e| db_err(&e))?;
    let action = match (current, b.status.as_str()) {
        (_, "stopped") => "programme_stopped",
        (_, "paused") => "programme_paused",
        _ => "programme_resumed",
    };
    AuditModel::record(
        &ctx.db,
        plan.pid,
        action,
        caller.actor(),
        Some(serde_json::json!({ "from": current, "to": b.status, "reason": row.reason })),
    )
    .await
    .map_err(|e| Error::string(&e.to_string()))?;
    format::json(
        serde_json::json!({ "plan_pid": plan.pid, "status": row.status, "decision": row.pid }),
    )
}

/// `GET /api/plans/{pid}/programme-status` — the status, the decisions behind it, and what a paused or
/// stopped programme still has.
#[debug_handler]
async fn get_status(State(ctx): State<AppContext>, Path(pid): Path<String>) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let history = history_of(&ctx, plan.pid).await?;
    let status = rules::status_at(&as_rules(&history), Utc::now().date_naive());
    let kept_now = if status == "active" {
        serde_json::Value::Null
    } else {
        let all = plans::Entity::find()
            .filter(plans::Column::DeletedAt.is_null())
            .all(&ctx.db)
            .await
            .map_err(|e| db_err(&e))?;
        kept(&ctx, &subtree(&all, plan.pid)).await?
    };
    format::json(serde_json::json!({
        "plan_pid": plan.pid, "status": status, "history": history_json(&history),
        "still_held": kept_now,
        "note": "A programme is active until a recorded decision pauses or stops it. Stopped is terminal and keeps the record. A pause flags the programme's allocations; it deletes none.",
    }))
}

// ---- the same, for the pack and the map ------------------------------------------------------

/// Each programme's status at `at`, for the programmes with any decision.
pub(crate) async fn status_map(
    ctx: &AppContext,
    at: NaiveDate,
) -> Result<HashMap<Uuid, &'static str>> {
    let rows = decisions::Entity::find()
        .order_by_asc(decisions::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    let mut by: HashMap<Uuid, Vec<decisions::Model>> = HashMap::new();
    for r in rows {
        by.entry(r.plan_pid).or_default().push(r);
    }
    Ok(by
        .into_iter()
        .map(|(k, v)| (k, rules::status_at(&as_rules(&v), at)))
        .collect())
}

/// The pack's gate view: gates passed, failed and deferred, and the programmes paused and stopped.
#[allow(clippy::too_many_lines, clippy::similar_names)]
pub(crate) async fn gates_section(
    ctx: &AppContext,
    include: &HashSet<Uuid>,
    period: &str,
    cut_off: DateTime<Utc>,
) -> Result<serde_json::Value> {
    let at = cut_off.date_naive();
    let all = plans::Entity::find()
        .filter(plans::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    let programmes: Vec<&plans::Model> = all
        .iter()
        .filter(|p| p.kind.as_deref() == Some("Program") && include.contains(&p.pid))
        .collect();
    let reviews = gate_reviews::Entity::find()
        .order_by_asc(gate_reviews::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    let plans_planned = planned_gates::Entity::find()
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    let history = decisions::Entity::find()
        .order_by_asc(decisions::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    let start = crate::delivery_confidence::period_start(period);
    let (mut passed, mut failed, mut deferred) = (Vec::new(), Vec::new(), Vec::new());
    let (mut paused, mut stopped, mut changes) = (Vec::new(), Vec::new(), Vec::new());
    for p in &programmes {
        let mine: Vec<Review> = reviews
            .iter()
            .filter(|r| r.plan_pid == p.pid)
            .map(|r| Review {
                gate: r.gate.clone(),
                decision: r.decision.clone(),
                decided_on: DateTime::<Utc>::from(r.decided_at).date_naive(),
            })
            .collect();
        let planned: Vec<Planned> = plans_planned
            .iter()
            .filter(|g| g.plan_pid == p.pid)
            .map(|g| Planned {
                gate: g.gate.clone(),
                planned_on: g.planned_on,
            })
            .collect();
        let view = rules::classify(&mine, &planned, period, at);
        let line = |l: &rules::Line| {
            serde_json::json!({
                "plan_pid": p.pid, "plan": p.name, "gate": l.gate, "outcome": l.outcome,
                "decided_on": l.decided_on, "planned_on": l.planned_on,
                "reason": l.reason, "days_late": l.days_late,
            })
        };
        passed.extend(view.passed.iter().map(line));
        failed.extend(view.failed.iter().map(line));
        deferred.extend(view.deferred.iter().map(line));
        let h: Vec<&decisions::Model> = history.iter().filter(|d| d.plan_pid == p.pid).collect();
        let decided: Vec<Decision> = h
            .iter()
            .map(|d| Decision {
                status: d.status.clone(),
                decided_on: d.decided_on,
            })
            .collect();
        for d in &h {
            if d.decided_on <= at
                && start.is_some_and(|s| d.decided_on >= s)
                && d.decided_on.format("%Y-%m").to_string() == period
            {
                changes.push(serde_json::json!({
                    "plan_pid": p.pid, "plan": p.name, "to": d.status, "reason": d.reason,
                    "decided_on": d.decided_on,
                    "follows_from": if d.gate_review_pid.is_some() { Some("a gate review") }
                        else if d.exception_pid.is_some() { Some("a delivery exception") } else { None },
                }));
            }
        }
        let status = rules::status_at(&decided, at);
        if status != "active" {
            let since = h.iter().rfind(|d| d.decided_on <= at);
            let held = kept(ctx, &subtree(&all, p.pid)).await?;
            let item = serde_json::json!({
                "plan_pid": p.pid, "plan": p.name,
                "since": since.map(|d| d.decided_on), "reason": since.map(|d| d.reason.clone()),
                "still_held": held,
            });
            if status == "paused" {
                paused.push(item);
            } else {
                stopped.push(item);
            }
        }
    }
    Ok(serde_json::json!({
        "period": period,
        "passed": passed, "failed": failed, "deferred": deferred,
        "paused": paused, "stopped": stopped, "status_changes_in_period": changes,
        "note": "Passed and failed are the gate reviews decided in the period (approved and approved with conditions are passed; rejected is failed). Deferred is what is outstanding at the cut-off: the latest review is a hold, or the planned date passed with no passing or failing review. Paused and stopped programmes come only from recorded decisions; a stopped programme stays stopped and its money and benefits remain reportable, and a paused programme's allocations are counted, not deleted. Only programmes (plans of kind Program) you may read are included.",
    }))
}
