//! Schedule-risk Monte Carlo (SC-8): the inputs and the run. The maths is
//! [`crate::schedule_risk`]; this is the wiring.
//!
//! * `PUT …/tasks/{pid}/estimate` — a task's optimistic and pessimistic
//!   duration (its likely one is `duration_days`);
//! * `PUT …/risks/{pid}/schedule-impact` — the task a register risk threatens,
//!   the working days it adds, and its probability in percent;
//! * `GET …/schedule-risk` — the seeded simulation. Every assumption is echoed
//!   in the response, including that inputs are treated as independent.

use std::collections::HashMap;

use axum::http::StatusCode;
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use sea_orm::QueryOrder;
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::MaybeAuthUser;
use crate::models::_entities::{risks, tasks};
use crate::models::audit_logs::Model as AuditModel;
use crate::schedule_risk::{self as sim, Distribution, Estimate, Event, Params, RunError};

fn unprocessable(message: &str) -> Error {
    Error::CustomError(
        StatusCode::UNPROCESSABLE_ENTITY,
        ErrorDetail::new("unprocessable_entity", message),
    )
}

fn db_err(e: sea_orm::DbErr) -> Error {
    Error::Model(ModelError::from(e))
}

#[derive(Debug, Deserialize)]
struct EstimateBody {
    /// Optimistic working days; `null` clears.
    #[serde(default)]
    optimistic_days: Option<i32>,
    /// Pessimistic working days; `null` clears.
    #[serde(default)]
    pessimistic_days: Option<i32>,
}

/// `PUT /api/plans/{pid}/tasks/{t_pid}/estimate` — set a task's optimistic and
/// pessimistic duration. The task needs a likely duration first, and
/// optimistic <= likely <= pessimistic.
#[debug_handler]
async fn set_estimate(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, t_pid)): Path<(String, String)>,
    Json(b): Json<EstimateBody>,
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
    let mut problems = Vec::new();
    for (name, v) in [
        ("optimistic_days", b.optimistic_days),
        ("pessimistic_days", b.pessimistic_days),
    ] {
        if v.is_some_and(|d| !(0..=3650).contains(&d)) {
            problems.push(format!("{name} must be 0 to 3650"));
        }
    }
    if b.optimistic_days.is_some() || b.pessimistic_days.is_some() {
        match task.duration_days {
            None => {
                problems.push("set the task's likely duration (duration_days) first".to_string());
            }
            Some(m) => {
                if b.optimistic_days.is_some_and(|o| o > m) {
                    problems.push(format!(
                        "optimistic_days is above the likely duration ({m})"
                    ));
                }
                if b.pessimistic_days.is_some_and(|p| p < m) {
                    problems.push(format!(
                        "pessimistic_days is below the likely duration ({m})"
                    ));
                }
            }
        }
    }
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let was = (
        task.duration_optimistic_days,
        task.duration_pessimistic_days,
    );
    let mut a: tasks::ActiveModel = task.into();
    a.duration_optimistic_days = ActiveValue::set(b.optimistic_days);
    a.duration_pessimistic_days = ActiveValue::set(b.pessimistic_days);
    let row = a.update(&ctx.db).await.map_err(db_err)?;
    AuditModel::record(
        &ctx.db,
        row.pid,
        "task_estimate_set",
        caller.actor(),
        Some(serde_json::json!({
            "from": { "optimistic_days": was.0, "pessimistic_days": was.1 },
            "to": { "optimistic_days": b.optimistic_days, "pessimistic_days": b.pessimistic_days },
        })),
    )
    .await
    .ok();
    format::json(serde_json::json!({
        "task_pid": row.pid, "optimistic_days": row.duration_optimistic_days,
        "likely_days": row.duration_days, "pessimistic_days": row.duration_pessimistic_days,
    }))
}

#[derive(Debug, Deserialize)]
struct ImpactBody {
    /// The task it threatens; `null` clears the schedule fields.
    #[serde(default)]
    task_pid: Option<Uuid>,
    /// Working days it adds if it happens (1-365).
    #[serde(default)]
    impact_days: Option<i32>,
    /// Chance it happens, in percent (0-100).
    #[serde(default)]
    probability_percent: Option<i32>,
}

/// `PUT /api/plans/{pid}/risks/{r_pid}/schedule-impact` — make a register risk a
/// schedule event: which task, how many days, how likely. Any field left out
/// stays unset, and a risk missing one is skipped (and named) by the simulation.
#[debug_handler]
async fn set_impact(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, r_pid)): Path<(String, String)>,
    Json(b): Json<ImpactBody>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let r_pid = Uuid::parse_str(&r_pid).map_err(|_| Error::NotFound)?;
    let risk = risks::Entity::find()
        .filter(risks::Column::Pid.eq(r_pid))
        .filter(risks::Column::PlanPid.eq(plan.pid))
        .filter(risks::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    let mut problems = Vec::new();
    if b.impact_days.is_some_and(|d| !(1..=365).contains(&d)) {
        problems.push("impact_days must be 1 to 365".to_string());
    }
    if b.probability_percent
        .is_some_and(|p| !(0..=100).contains(&p))
    {
        problems.push("probability_percent must be 0 to 100".to_string());
    }
    if let Some(t) = b.task_pid {
        let known = tasks::Entity::find()
            .filter(tasks::Column::Pid.eq(t))
            .filter(tasks::Column::PlanPid.eq(plan.pid))
            .filter(tasks::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await
            .map_err(db_err)?
            .is_some();
        if !known {
            problems.push("task_pid is not a task of this plan".to_string());
        }
    }
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let was = serde_json::json!({ "task_pid": risk.task_pid, "impact_days": risk.schedule_impact_days,
        "probability_percent": risk.schedule_probability_percent });
    let mut a: risks::ActiveModel = risk.into();
    a.task_pid = ActiveValue::set(b.task_pid);
    a.schedule_impact_days = ActiveValue::set(b.impact_days);
    a.schedule_probability_percent = ActiveValue::set(b.probability_percent);
    let row = a.update(&ctx.db).await.map_err(db_err)?;
    let now = serde_json::json!({ "task_pid": row.task_pid, "impact_days": row.schedule_impact_days,
        "probability_percent": row.schedule_probability_percent });
    AuditModel::record(
        &ctx.db,
        row.pid,
        "risk_schedule_impact_set",
        caller.actor(),
        Some(serde_json::json!({ "from": was, "to": now })),
    )
    .await
    .ok();
    format::json(serde_json::json!({ "risk_pid": row.pid, "schedule": now }))
}

#[derive(Debug, Deserialize)]
struct RunParams {
    #[serde(default)]
    iterations: Option<usize>,
    #[serde(default)]
    seed: Option<u64>,
    /// `triangular` (default) or `pert`.
    #[serde(default)]
    distribution: Option<String>,
    /// Band for a task with no estimate, in percent of its likely duration.
    #[serde(default)]
    fallback_band_percent: Option<i64>,
}

fn bp(percent: i32) -> i64 {
    i64::from(percent) * 100
}

/// Each task's optimistic / pessimistic estimate, for those that have one and a
/// likely duration to sit around.
pub(crate) fn estimates_of(rows: &[tasks::Model]) -> HashMap<Uuid, Estimate> {
    rows.iter()
        .filter_map(|t| {
            match (
                t.duration_optimistic_days,
                t.duration_pessimistic_days,
                t.duration_days,
            ) {
                (None, None, _) | (_, _, None) => None,
                (o, p, Some(m)) => Some((
                    t.pid,
                    Estimate {
                        optimistic: i64::from(o.unwrap_or(m)),
                        pessimistic: i64::from(p.unwrap_or(m)),
                    },
                )),
            }
        })
        .collect()
}

/// The register's schedule events: those that can be simulated, those skipped
/// with a named reason, and the count of risks with no schedule fields at all.
pub(crate) fn classify_events(
    register: &[risks::Model],
    in_network: &std::collections::HashSet<Uuid>,
) -> (
    Vec<Event>,
    Vec<serde_json::Value>,
    Vec<serde_json::Value>,
    usize,
) {
    let mut events = Vec::new();
    let mut used = Vec::new();
    let mut skipped = Vec::new();
    let mut not_schedule = 0_usize;
    for r in register {
        if r.task_pid.is_none()
            && r.schedule_impact_days.is_none()
            && r.schedule_probability_percent.is_none()
        {
            not_schedule += 1;
            continue;
        }
        let reason = if !matches!(r.status.as_str(), "open" | "mitigating") {
            Some("closed")
        } else if r.schedule_probability_percent.is_none() {
            Some("no_probability")
        } else if r.task_pid.is_none() {
            Some("no_task")
        } else if r.schedule_impact_days.is_none() {
            Some("no_impact")
        } else if r.task_pid.is_some_and(|t| !in_network.contains(&t)) {
            Some("task_not_scheduled")
        } else {
            None
        };
        match (
            reason,
            r.task_pid,
            r.schedule_impact_days,
            r.schedule_probability_percent,
        ) {
            (None, Some(task), Some(days), Some(p)) => {
                events.push(Event {
                    risk: r.pid,
                    task,
                    probability_bp: bp(p),
                    impact_days: i64::from(days),
                });
                used.push(
                    serde_json::json!({ "risk_pid": r.pid, "title": r.title, "task_pid": task,
                    "probability_percent": p, "impact_days": days }),
                );
            }
            (reason, ..) => skipped.push(serde_json::json!({ "risk_pid": r.pid, "title": r.title,
                "reason": reason.unwrap_or("incomplete") })),
        }
    }
    (events, used, skipped, not_schedule)
}

/// `GET /api/plans/{pid}/schedule-risk` — simulate the plan's task network.
#[debug_handler]
#[allow(clippy::too_many_lines)]
async fn schedule_risk(
    State(ctx): State<AppContext>,
    Path(pid): Path<String>,
    Query(q): Query<RunParams>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let distribution = match q.distribution.as_deref() {
        None => Distribution::Triangular,
        Some(d) => Distribution::parse(d)
            .ok_or_else(|| unprocessable("distribution must be triangular or pert"))?,
    };
    let band = q.fallback_band_percent.unwrap_or(sim::DEFAULT_BAND_PERCENT);
    if !(0..=100).contains(&band) {
        return Err(unprocessable("fallback_band_percent must be 0 to 100"));
    }
    let params = Params {
        iterations: q.iterations.unwrap_or(sim::DEFAULT_ITERATIONS),
        seed: q.seed.unwrap_or(sim::DEFAULT_SEED),
        distribution,
        fallback_band_percent: band,
    };
    let net = super::cpm::load_network(&ctx, plan.pid).await?;
    let estimates = estimates_of(&net.rows);

    // The register's schedule events: used, skipped (named), or not schedule
    // risks at all (counted).
    let register = risks::Entity::find()
        .filter(risks::Column::PlanPid.eq(plan.pid))
        .filter(risks::Column::DeletedAt.is_null())
        .order_by_asc(risks::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let in_network: std::collections::HashSet<Uuid> = net.nodes.iter().map(|n| n.pid).collect();
    let (events, used, skipped, not_schedule) = classify_events(&register, &in_network);

    let outcome = sim::simulate(
        &net.nodes,
        &net.edges,
        &net.plan_cal,
        &net.own,
        &estimates,
        &events,
        params,
    )
    .map_err(|e| match e {
        RunError::Cycle(on) => unprocessable(&format!(
            "the dependencies contain a cycle, so nothing can be simulated (on or behind it: {})",
            on.iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        )),
        RunError::NothingScheduled => unprocessable(
            "no task can be scheduled: give tasks a duration and a start date or a predecessor",
        ),
        RunError::TooMuchWork(m) => unprocessable(&m),
    })?;

    let by: HashMap<Uuid, &tasks::Model> = net.rows.iter().map(|t| (t.pid, t)).collect();
    let unscheduled = {
        let base = net.analyze(0).ok();
        base.map(|r| {
            r.unscheduled
                .iter()
                .map(|u| serde_json::json!({ "pid": u.pid, "title": by.get(&u.pid).map(|t| t.title.clone()), "reason": u.reason }))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
    };
    let fallback_tasks: Vec<Uuid> = outcome
        .tasks
        .iter()
        .filter(|t| t.source == sim::Source::Fallback)
        .map(|t| t.pid)
        .collect();
    format::json(serde_json::json!({
        "plan_pid": plan.pid,
        "calendar": net.cal_source,
        "assumptions": {
            "seed": params.seed,
            "iterations": params.iterations,
            "max_iterations": sim::MAX_ITERATIONS,
            "distribution": distribution,
            "fallback_band_percent": band,
            "correlation": "none",
            "independence": "Every task duration and every risk event is drawn independently: there is no correlation between them, so a shared cause (one team slipping everything at once) is not modelled and the spread is narrower than a correlated one would be.",
            "durations": "Whole working days of the plan calendar; each iteration re-runs the critical-path analysis (constraints, calendars and relationship types included).",
            "tasks_using_fallback": fallback_tasks,
            "fallback": "A task with a likely duration but no optimistic or pessimistic estimate is simulated at its likely duration plus and minus fallback_band_percent, and is marked source=fallback; it is never treated as certain.",
        },
        "cpm_finish": outcome.cpm_finish,
        "percentiles": outcome.percentiles,
        "cpm_finish_confidence_bp": outcome.cpm_finish_confidence_bp,
        "earliest": outcome.earliest,
        "latest": outcome.latest,
        "histogram": outcome.histogram.iter().map(|(d, c)| serde_json::json!({ "date": d, "count": c })).collect::<Vec<_>>(),
        "tasks": outcome.tasks.iter().map(|t| {
            let mut v = serde_json::to_value(t).unwrap_or_default();
            v["title"] = serde_json::json!(by.get(&t.pid).map(|r| r.title.clone()));
            v
        }).collect::<Vec<_>>(),
        "events": { "used": used, "skipped": skipped, "risks_without_schedule_fields": not_schedule },
        "unscheduled": unscheduled,
        "note": "P50, P80 and P90 are the finish dates that share of iterations finished on or before (nearest rank). cpm_finish_confidence_bp is the share finishing on or before the deterministic critical-path finish, in basis points. criticality_bp is the share of iterations in which a task had no float; sensitivity is the correlation of its duration with the finish date. A risk event with no probability, task or impact is skipped and named under events.skipped.",
    }))
}

/// The schedule-risk routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/schedule-risk", get(schedule_risk))
        .add("/plans/{pid}/tasks/{t_pid}/estimate", put(set_estimate))
        .add(
            "/plans/{pid}/risks/{r_pid}/schedule-impact",
            put(set_impact),
        )
}
