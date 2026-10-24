//! Cost-risk and joint cost-and-schedule Monte Carlo: the inputs and the runs.
//! The maths is [`crate::cost_risk`] and [`crate::joint_risk`]; this is the
//! wiring.
//!
//! * `PUT …/budget-lines/{l_pid}/estimate` — a line's low and high final cost
//!   (its likely cost is `planned_minor`);
//! * `PUT …/risks/{r_pid}/cost-impact` — the cost a register risk adds, and (when
//!   given) the **shared** probability of the event;
//! * `GET /api/plans/{pid}/cost-risk` — one plan; `GET /api/cost-risk` — every
//!   plan the caller can read, per currency and never across;
//! * `GET /api/plans/{pid}/joint-risk` — schedule and cost together, with the
//!   joint confidence.
//!
//! One register risk has one probability. A risk with both a schedule impact and
//! a cost impact is a single event: in the joint run it strikes once and drives
//! both.

use super::db_err;
use std::collections::{HashMap, HashSet};

use loco_rs::prelude::*;
use sea_orm::QueryOrder;
use serde::Deserialize;
use uuid::Uuid;

use super::unprocessable_entity as unprocessable;
use crate::auth::MaybeAuthUser;
use crate::cost_risk::{self as sim, CostEvent, Line};
use crate::joint_risk::{self as joint, JointError};
use crate::models::_entities::{budget_lines, plans, risks};
use crate::models::audit_logs::Model as AuditModel;
use crate::schedule_risk::{self, Distribution, RunError};

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct EstimateBody {
    /// Low final cost in minor units; `null` clears.
    #[serde(default)]
    low_minor: Option<i64>,
    /// High final cost in minor units; `null` clears.
    #[serde(default)]
    high_minor: Option<i64>,
}

/// `PUT /api/plans/{pid}/budget-lines/{l_pid}/estimate` — set a line's low and
/// high final cost. Low must not exceed the planned amount, nor high be below it.
#[debug_handler]
async fn set_line_estimate(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, l_pid)): Path<(String, String)>,
    Json(b): Json<EstimateBody>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let l_pid = super::parse_pid(&l_pid)?;
    let line = budget_lines::Entity::find()
        .filter(budget_lines::Column::Pid.eq(l_pid))
        .filter(budget_lines::Column::PlanPid.eq(plan.pid))
        .filter(budget_lines::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    let mut problems = Vec::new();
    if let Some(lo) = b.low_minor {
        if lo < 0 {
            problems.push("low_minor must not be negative".to_string());
        } else if lo > line.planned_minor {
            problems.push(format!(
                "low_minor is above the planned amount ({})",
                line.planned_minor
            ));
        }
    }
    if b.high_minor.is_some_and(|hi| hi < line.planned_minor) {
        problems.push(format!(
            "high_minor is below the planned amount ({})",
            line.planned_minor
        ));
    }
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let was = serde_json::json!({ "low_minor": line.low_minor, "high_minor": line.high_minor });
    let mut a: budget_lines::ActiveModel = line.into();
    a.low_minor = ActiveValue::set(b.low_minor);
    a.high_minor = ActiveValue::set(b.high_minor);
    let txn = ctx.db.begin().await.map_err(super::db_err)?;
    let row = a.update(&txn).await.map_err(db_err)?;
    AuditModel::record(
        &txn,
        row.pid,
        "budget_line_estimate_set",
        caller.actor(),
        Some(serde_json::json!({ "from": was, "to": { "low_minor": row.low_minor, "high_minor": row.high_minor } })),
    ).await
    .map_err(super::audit_failed)?;
    txn.commit().await.map_err(super::db_err)?;
    format::json(
        serde_json::json!({ "line_pid": row.pid, "low_minor": row.low_minor,
        "planned_minor": row.planned_minor, "high_minor": row.high_minor }),
    )
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct CostImpactBody {
    /// The cost it adds, in minor units (positive); `null` clears both this and the currency.
    #[serde(default)]
    cost_impact_minor: Option<i64>,
    /// ISO 4217 code, required with an impact.
    #[serde(default)]
    cost_currency: Option<String>,
    /// The event's probability in percent, shared with the schedule simulation.
    /// Left out: unchanged.
    #[serde(default)]
    probability_percent: Option<i32>,
}

/// `PUT /api/plans/{pid}/risks/{r_pid}/cost-impact` — make a register risk a cost
/// event. The probability is the one the schedule simulation uses, so one event
/// strikes once and drives both.
#[debug_handler]
async fn set_cost_impact(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, r_pid)): Path<(String, String)>,
    Json(b): Json<CostImpactBody>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let r_pid = super::parse_pid(&r_pid)?;
    let risk = risks::Entity::find()
        .filter(risks::Column::Pid.eq(r_pid))
        .filter(risks::Column::PlanPid.eq(plan.pid))
        .filter(risks::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    let currency = b
        .cost_currency
        .as_deref()
        .map(str::trim)
        .map(str::to_ascii_uppercase);
    let mut problems = Vec::new();
    match (b.cost_impact_minor, &currency) {
        (None, None) => {}
        (Some(i), Some(c)) => {
            if i <= 0 {
                problems.push("cost_impact_minor must be positive".to_string());
            }
            if c.len() != 3 || !c.chars().all(|ch| ch.is_ascii_uppercase()) {
                problems.push("cost_currency must be a 3-letter code such as GBP".to_string());
            }
        }
        _ => problems.push("cost_impact_minor and cost_currency go together".to_string()),
    }
    if b.probability_percent
        .is_some_and(|p| !(0..=100).contains(&p))
    {
        problems.push("probability_percent must be 0 to 100".to_string());
    }
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let was = serde_json::json!({ "cost_impact_minor": risk.cost_impact_minor,
        "cost_currency": risk.cost_currency, "probability_percent": risk.schedule_probability_percent });
    let mut a: risks::ActiveModel = risk.into();
    a.cost_impact_minor = ActiveValue::set(b.cost_impact_minor);
    a.cost_currency = ActiveValue::set(currency);
    if let Some(p) = b.probability_percent {
        a.schedule_probability_percent = ActiveValue::set(Some(p));
    }
    let txn = ctx.db.begin().await.map_err(super::db_err)?;
    let row = a.update(&txn).await.map_err(db_err)?;
    let now = serde_json::json!({ "cost_impact_minor": row.cost_impact_minor,
        "cost_currency": row.cost_currency, "probability_percent": row.schedule_probability_percent });
    AuditModel::record(
        &txn,
        row.pid,
        "risk_cost_impact_set",
        caller.actor(),
        Some(serde_json::json!({ "from": was, "to": now })),
    )
    .await
    .map_err(super::audit_failed)?;
    txn.commit().await.map_err(super::db_err)?;
    format::json(serde_json::json!({ "risk_pid": row.pid, "cost": now }))
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct RunParams {
    #[serde(default)]
    iterations: Option<usize>,
    #[serde(default)]
    seed: Option<u64>,
    /// `triangular` (default) or `pert`.
    #[serde(default)]
    distribution: Option<String>,
    /// Band for a line (or task) with no estimate, in percent.
    #[serde(default)]
    fallback_band_percent: Option<i64>,
    /// Joint run only: the currency (needed when the plan budgets in several).
    #[serde(default)]
    currency: Option<String>,
    /// Joint run only: cost of each business day the finish is late, minor units.
    #[serde(default)]
    delay_cost_per_day_minor: Option<i64>,
}

struct Common {
    distribution: Distribution,
    seed: u64,
    band: i64,
}

fn common(q: &RunParams) -> Result<Common> {
    let distribution = match q.distribution.as_deref() {
        None => Distribution::Triangular,
        Some(d) => Distribution::parse(d)
            .ok_or_else(|| unprocessable("distribution must be triangular or pert"))?,
    };
    let band = q.fallback_band_percent.unwrap_or(20);
    if !(0..=100).contains(&band) {
        return Err(unprocessable("fallback_band_percent must be 0 to 100"));
    }
    Ok(Common {
        distribution,
        seed: q.seed.unwrap_or(1),
        band,
    })
}

fn line_of(l: &budget_lines::Model) -> Line {
    Line {
        pid: l.pid,
        plan: l.plan_pid,
        currency: l.currency.clone(),
        actual: l.actual_minor,
        likely: l.planned_minor,
        low: l.low_minor,
        high: l.high_minor,
    }
}

/// The register's cost events: usable, skipped (named), or not cost risks at all.
fn classify_cost_events(
    register: &[risks::Model],
    currencies: &HashSet<String>,
) -> (
    Vec<CostEvent>,
    Vec<serde_json::Value>,
    Vec<serde_json::Value>,
    usize,
) {
    let (mut events, mut used, mut skipped, mut none) =
        (Vec::new(), Vec::new(), Vec::new(), 0_usize);
    for r in register {
        let (Some(impact), Some(currency)) = (r.cost_impact_minor, r.cost_currency.as_ref()) else {
            none += 1;
            continue;
        };
        let reason = if !matches!(r.status.as_str(), "open" | "mitigating") {
            Some("closed")
        } else if r.schedule_probability_percent.is_none() {
            Some("no_probability")
        } else if !currencies.contains(currency) {
            Some("no_budget_in_currency")
        } else {
            None
        };
        match (reason, r.schedule_probability_percent) {
            (None, Some(p)) => {
                events.push(CostEvent {
                    risk: r.pid,
                    plan: r.plan_pid,
                    currency: currency.clone(),
                    probability_bp: i64::from(p) * 100,
                    impact_minor: impact,
                });
                used.push(serde_json::json!({ "risk_pid": r.pid, "plan_pid": r.plan_pid, "title": r.title,
                    "probability_percent": p, "cost_impact_minor": impact, "currency": currency,
                    "also_a_schedule_event": r.task_pid.is_some() && r.schedule_impact_days.is_some() }));
            }
            (reason, _) => skipped.push(serde_json::json!({ "risk_pid": r.pid, "title": r.title,
                "reason": reason.unwrap_or("incomplete") })),
        }
    }
    (events, used, skipped, none)
}

fn assumptions(c: &Common, iterations: usize, max_iterations: usize) -> serde_json::Value {
    serde_json::json!({
        "seed": c.seed,
        "iterations": iterations,
        "max_iterations": max_iterations,
        "distribution": c.distribution,
        "fallback_band_percent": c.band,
        "correlation": "none",
        "independence": "Every line cost and every risk event is drawn independently: there is no correlation between them, so a shared cause (inflation moving every line together) is not modelled and the spread is narrower than a correlated one would be.",
        "floor": "What is already spent is never given back: a line's simulated final cost is never below its recorded actual.",
        "currencies": "Each currency is simulated and reported on its own; currencies are never added or converted.",
        "budget": "The budget is the sum of the lines' planned amounts, which are also each line's likely cost.",
        "fallback": "A line with no low or high estimate is simulated at its planned amount plus and minus fallback_band_percent, and is marked source=fallback; it is never treated as certain.",
    })
}

fn pct_json(v: &[(u32, i64)]) -> Vec<serde_json::Value> {
    v.iter()
        .map(|(p, m)| serde_json::json!({ "p": p, "minor": m }))
        .collect()
}

async fn load_lines(
    ctx: &AppContext,
    plan_pids: Option<&[Uuid]>,
) -> Result<Vec<budget_lines::Model>> {
    let mut q = budget_lines::Entity::find().filter(budget_lines::Column::DeletedAt.is_null());
    if let Some(p) = plan_pids {
        q = q.filter(budget_lines::Column::PlanPid.is_in(p.to_vec()));
    }
    q.order_by_asc(budget_lines::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)
}

async fn load_register(ctx: &AppContext, plan_pids: &[Uuid]) -> Result<Vec<risks::Model>> {
    if plan_pids.is_empty() {
        return Ok(Vec::new());
    }
    risks::Entity::find()
        .filter(risks::Column::PlanPid.is_in(plan_pids.to_vec()))
        .filter(risks::Column::DeletedAt.is_null())
        .order_by_asc(risks::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)
}

/// Shape the engine's outcome with the lines' and risks' own words.
fn shape(
    outcome: &sim::Outcome,
    line_rows: &HashMap<Uuid, &budget_lines::Model>,
    register: &[risks::Model],
    plan_names: &HashMap<Uuid, String>,
) -> Vec<serde_json::Value> {
    let titles: HashMap<Uuid, &str> = register.iter().map(|r| (r.pid, r.title.as_str())).collect();
    outcome
        .currencies
        .iter()
        .map(|c| {
            serde_json::json!({
                "currency": c.currency,
                "budget_minor": c.budget_minor,
                "spent_minor": c.spent_minor,
                "mean_minor": c.mean_minor,
                "percentiles": pct_json(&c.percentiles),
                "budget_confidence_bp": c.budget_confidence_bp,
                "min_minor": c.min_minor,
                "max_minor": c.max_minor,
                "histogram": c.histogram,
                "lines": c.lines.iter().map(|l| {
                    let mut v = serde_json::to_value(l).unwrap_or_default();
                    if let Some(row) = line_rows.get(&l.pid) {
                        v["description"] = serde_json::json!(row.description);
                        v["category"] = serde_json::json!(row.category);
                    }
                    v
                }).collect::<Vec<_>>(),
                "events": c.events.iter().map(|e| {
                    let mut v = serde_json::to_value(e).unwrap_or_default();
                    v["title"] = serde_json::json!(titles.get(&e.risk));
                    v
                }).collect::<Vec<_>>(),
                "plans": c.plans.iter().map(|p| {
                    let mut v = serde_json::to_value(p).unwrap_or_default();
                    v["name"] = serde_json::json!(plan_names.get(&p.plan));
                    v
                }).collect::<Vec<_>>(),
            })
        })
        .collect()
}

fn run_error(e: sim::RunError) -> Error {
    match e {
        sim::RunError::NoLines => unprocessable(
            "there are no budget lines to simulate: add budget lines (with a planned amount) first",
        ),
        sim::RunError::TooMuchWork(m) => unprocessable(&m),
    }
}

/// The P80 cost per currency over the given plans, with the default seed, distribution and band; `None` when
/// they have no budget line to simulate. Used by the threshold sweep (EX-7).
///
/// # Errors
///
/// When a database read fails.
pub(crate) async fn p80_by_currency(
    ctx: &AppContext,
    plan_pids: &[Uuid],
) -> Result<Option<HashMap<String, i64>>> {
    let c = common(&RunParams {
        iterations: None,
        seed: None,
        distribution: None,
        fallback_band_percent: None,
        currency: None,
        delay_cost_per_day_minor: None,
    })?;
    let params = sim::Params {
        iterations: sim::DEFAULT_ITERATIONS,
        seed: c.seed,
        distribution: c.distribution,
        fallback_band_percent: c.band,
    };
    let rows = load_lines(ctx, Some(plan_pids)).await?;
    let lines: Vec<Line> = rows.iter().map(line_of).collect();
    if lines.is_empty() {
        return Ok(None);
    }
    let register = load_register(ctx, plan_pids).await?;
    let currencies: HashSet<String> = lines.iter().map(|l| l.currency.clone()).collect();
    let (events, _, _, _) = classify_cost_events(&register, &currencies);
    let outcome = match sim::simulate(&lines, &events, params) {
        Ok(o) => o,
        Err(sim::RunError::NoLines) => return Ok(None),
        Err(e) => return Err(run_error(e)),
    };
    Ok(Some(
        outcome
            .currencies
            .iter()
            .filter_map(|cu| {
                cu.percentiles
                    .iter()
                    .find(|(p, _)| *p == 80)
                    .map(|(_, v)| (cu.currency.clone(), *v))
            })
            .collect(),
    ))
}

async fn run_cost(
    ctx: &AppContext,
    plan_pids: &[Uuid],
    scope: serde_json::Value,
    q: &RunParams,
) -> Result<Response> {
    let c = common(q)?;
    let params = sim::Params {
        iterations: q.iterations.unwrap_or(sim::DEFAULT_ITERATIONS),
        seed: c.seed,
        distribution: c.distribution,
        fallback_band_percent: c.band,
    };
    let rows = load_lines(ctx, Some(plan_pids)).await?;
    let lines: Vec<Line> = rows.iter().map(line_of).collect();
    let register = load_register(ctx, plan_pids).await?;
    let currencies: HashSet<String> = lines.iter().map(|l| l.currency.clone()).collect();
    let (events, used, skipped, not_cost) = classify_cost_events(&register, &currencies);
    let outcome = sim::simulate(&lines, &events, params).map_err(run_error)?;
    let plan_names: HashMap<Uuid, String> = plans::Entity::find()
        .filter(plans::Column::Pid.is_in(plan_pids.to_vec()))
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|p| (p.pid, p.name))
        .collect();
    let by_pid: HashMap<Uuid, &budget_lines::Model> = rows.iter().map(|r| (r.pid, r)).collect();
    let fallback: Vec<Uuid> = outcome
        .currencies
        .iter()
        .flat_map(|cu| cu.lines.iter())
        .filter(|l| l.source == sim::Source::Fallback)
        .map(|l| l.pid)
        .collect();
    let mut a = assumptions(&c, params.iterations, sim::MAX_ITERATIONS);
    a["lines_using_fallback"] = serde_json::json!(fallback);
    format::json(serde_json::json!({
        "scope": scope,
        "assumptions": a,
        "currencies": shape(&outcome, &by_pid, &register, &plan_names),
        "events": { "used": used, "skipped": skipped, "risks_without_cost_fields": not_cost },
        "note": "Percentiles are the cost that share of iterations came in at or under (nearest rank). budget_confidence_bp is the share at or under the budget. sensitivity is the correlation of a line's cost with the currency's total. A risk with no probability, or in a currency with no budget line, is left out and named under events.skipped.",
    }))
}

/// `GET /api/plans/{pid}/cost-risk` — simulate one plan's cost.
#[debug_handler]
async fn plan_cost_risk(
    State(ctx): State<AppContext>,
    Path(pid): Path<String>,
    Query(q): Query<RunParams>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    run_cost(
        &ctx,
        &[plan.pid],
        serde_json::json!({ "kind": "plan", "plan_pid": plan.pid, "plans": 1 }),
        &q,
    )
    .await
}

/// `GET /api/cost-risk` — simulate the cost of every plan the caller can read,
/// together. Plans the caller cannot read are left out without a trace.
#[debug_handler]
async fn portfolio_cost_risk(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(q): Query<RunParams>,
) -> Result<Response> {
    let all = load_lines(&ctx, None).await?;
    let mut pids: Vec<Uuid> = all.iter().map(|l| l.plan_pid).collect();
    pids.sort();
    pids.dedup();
    let readable = super::plan_access::readable_plans(&ctx, &caller, &pids).await;
    let mut plans: Vec<Uuid> = pids.into_iter().filter(|p| readable.contains(p)).collect();
    plans.sort();
    let n = plans.len();
    run_cost(
        &ctx,
        &plans,
        serde_json::json!({ "kind": "portfolio", "plans": n }),
        &q,
    )
    .await
}

/// `GET /api/plans/{pid}/joint-risk` — schedule and cost simulated together.
#[debug_handler]
#[expect(clippy::too_many_lines)]
async fn joint_risk(
    State(ctx): State<AppContext>,
    Path(pid): Path<String>,
    Query(q): Query<RunParams>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let c = common(&q)?;
    let rate = q.delay_cost_per_day_minor.unwrap_or(0);
    if !(0..=1_000_000_000_000).contains(&rate) {
        return Err(unprocessable(
            "delay_cost_per_day_minor must not be negative",
        ));
    }
    let params = schedule_risk::Params {
        iterations: q.iterations.unwrap_or(schedule_risk::DEFAULT_ITERATIONS),
        seed: c.seed,
        distribution: c.distribution,
        fallback_band_percent: c.band,
    };
    let rows = load_lines(&ctx, Some(&[plan.pid])).await?;
    let mut currencies: Vec<String> = rows.iter().map(|l| l.currency.clone()).collect();
    currencies.sort();
    currencies.dedup();
    let currency = match (
        q.currency.as_deref().map(str::to_ascii_uppercase),
        currencies.as_slice(),
    ) {
        (Some(c), _) if currencies.contains(&c) => c,
        (Some(c), _) => {
            return Err(unprocessable(&format!(
                "the plan has no budget lines in {c}"
            )));
        }
        (None, [only]) => only.clone(),
        (None, []) => {
            return Err(unprocessable(
                "the plan has no budget lines: add some first",
            ));
        }
        (None, many) => {
            return Err(unprocessable(&format!(
                "the plan budgets in several currencies ({}); choose one with currency=",
                many.join(", ")
            )));
        }
    };
    let mine: Vec<&budget_lines::Model> = rows.iter().filter(|l| l.currency == currency).collect();
    let lines: Vec<Line> = mine.iter().map(|l| line_of(l)).collect();
    let register = load_register(&ctx, &[plan.pid]).await?;
    let (cost_events, cost_used, cost_skipped, not_cost) =
        classify_cost_events(&register, &HashSet::from([currency.clone()]));

    let net = super::cpm::load_network(&ctx, plan.pid).await?;
    let estimates = super::schedule_risk::estimates_of(&net.rows);
    let in_network: HashSet<Uuid> = net.nodes.iter().map(|n| n.pid).collect();
    let (sched_events, sched_used, sched_skipped, not_sched) =
        super::schedule_risk::classify_events(&register, &in_network);
    let shared: Vec<Uuid> = sched_events
        .iter()
        .filter(|e| cost_events.iter().any(|c| c.risk == e.risk))
        .map(|e| e.risk)
        .collect();

    let out = joint::simulate(
        &net.nodes,
        &net.edges,
        &net.plan_cal,
        &net.own,
        &estimates,
        &sched_events,
        &lines,
        &cost_events,
        rate,
        params,
    )
    .map_err(|e| match e {
        JointError::NoLines => unprocessable("the plan has no budget lines in that currency"),
        JointError::Schedule(RunError::Cycle(on)) => unprocessable(&format!(
            "the dependencies contain a cycle, so nothing can be simulated (on or behind it: {})",
            on.iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        )),
        JointError::Schedule(RunError::NothingScheduled) => unprocessable(
            "no task can be scheduled: give tasks a duration and a start date or a predecessor",
        ),
        JointError::Schedule(RunError::TooMuchWork(m)) => unprocessable(&m),
    })?;

    let by_pid: HashMap<Uuid, &budget_lines::Model> = rows.iter().map(|r| (r.pid, r)).collect();
    let titles: HashMap<Uuid, String> = net.rows.iter().map(|t| (t.pid, t.title.clone())).collect();
    let mut a = assumptions(
        &c,
        params.iterations,
        schedule_risk::MAX_ITERATIONS.min(sim::MAX_ITERATIONS),
    );
    a["delay_cost"] = serde_json::json!({
        "per_day_minor": rate,
        "currency": currency,
        "rule": "Each business day the finish is later than the critical-path finish adds this much; finishing early earns nothing back. At zero the cost and the schedule are simulated together but the delay costs nothing.",
    });
    a["shared_events"] = serde_json::json!(shared);
    a["shared_events_rule"] = serde_json::json!(
        "A risk with both a schedule impact and a cost impact is one event with one probability: it strikes once and drives both."
    );
    format::json(serde_json::json!({
        "plan_pid": plan.pid,
        "currency": currency,
        "other_currencies_not_simulated": currencies.iter().filter(|x| **x != currency).collect::<Vec<_>>(),
        "calendar": net.cal_source,
        "assumptions": a,
        "schedule": {
            "cpm_finish": out.schedule.cpm_finish,
            "percentiles": out.schedule.percentiles,
            "earliest": out.schedule.earliest,
            "latest": out.schedule.latest,
            "tasks": out.schedule.tasks.iter().map(|t| {
                let mut v = serde_json::to_value(t).unwrap_or_default();
                v["title"] = serde_json::json!(titles.get(&t.pid));
                v
            }).collect::<Vec<_>>(),
        },
        "cost": {
            "budget_minor": out.budget_minor,
            "spent_minor": out.spent_minor,
            "mean_minor": out.mean_cost_minor,
            "mean_delay_cost_minor": out.mean_delay_cost_minor,
            "percentiles": pct_json(&out.cost_percentiles),
            "lines": out.lines.iter().map(|l| {
                let mut v = serde_json::to_value(l).unwrap_or_default();
                if let Some(row) = by_pid.get(&l.pid) {
                    v["description"] = serde_json::json!(row.description);
                }
                v
            }).collect::<Vec<_>>(),
        },
        "confidence": {
            "budget_bp": out.budget_confidence_bp,
            "finish_bp": out.finish_confidence_bp,
            "joint_bp": out.joint_confidence_bp,
        },
        "quadrants": out.quadrants,
        "points": out.points.iter().map(|(d, c)| serde_json::json!({ "days_late": d, "cost_minor": c })).collect::<Vec<_>>(),
        "events": {
            "schedule": { "used": sched_used, "skipped": sched_skipped, "risks_without_schedule_fields": not_sched },
            "cost": { "used": cost_used, "skipped": cost_skipped, "risks_without_cost_fields": not_cost },
        },
        "note": "Joint confidence is the share of iterations both within budget and not later than the critical-path finish; it is never above the smaller of the two on their own. Cost includes delay cost. Points are a thinned sample for a scatter: business days late (negative is early) against total cost.",
    }))
}

/// The cost-risk routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/cost-risk", get(plan_cost_risk))
        .add("/cost-risk", get(portfolio_cost_risk))
        .add("/plans/{pid}/joint-risk", get(joint_risk))
        .add(
            "/plans/{pid}/budget-lines/{l_pid}/estimate",
            put(set_line_estimate),
        )
        .add(
            "/plans/{pid}/risks/{r_pid}/cost-impact",
            put(set_cost_impact),
        )
}
