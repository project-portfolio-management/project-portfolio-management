//! Triage and pause / reset / stop thresholds (EX-7). Rules in [`crate::thresholds`].
//!
//! * `PUT`/`GET /api/plans/{pid}/triage`: a dated triage of a programme (Finish, Fix, Pause, Stop, with the
//!   reason), append-only;
//! * `PUT`/`GET /api/plans/{pid}/thresholds`: up to three thresholds a programme carries;
//! * `POST /api/threshold-sweep`: looks at every programme's thresholds and **opens an exception** for each
//!   crossing; it changes no programme;
//! * `POST /api/exceptions/{pid}/threshold-decision`: pause, reset, stop or an override with an expiry.
//!   This records a decision; it never pauses, resets or stops anything itself.

use std::collections::{HashMap, HashSet};

use crate::prelude::*;
use axa::error::ErrorDetail;
use axum::http::StatusCode;
use chrono::Utc;
use serde::Deserialize;
use serde_json::{Value, json};
use uuid::Uuid;

use super::unprocessable_entity as unprocessable;
use crate::auth::MaybeAuthUser;
use crate::models::_entities::{
    delivery_exceptions as exceptions, director_packs, plans, programme_thresholds as thresholds,
    programme_triages as triages, threshold_decisions as decisions,
};
use crate::models::audit_logs::Model as AuditModel;
use crate::thresholds as rules;

/// The routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/triage", put(set_triage))
        .add("/plans/{pid}/triage", get(get_triage))
        .add("/plans/{pid}/thresholds", put(set_threshold))
        .add("/plans/{pid}/thresholds", get(get_thresholds))
        .add("/threshold-sweep", post(sweep))
        .add("/exceptions/{pid}/threshold-decision", post(decide))
}

/// Loads a plan and requires it to be of kind `Program`; `what` completes the `422` message ("only a plan of kind Program has ...").
async fn programme(ctx: &AppContext, pid: &str, what: &str) -> Result<plans::Model> {
    let plan = super::governance::find_item(ctx, pid).await?;
    if plan.kind.as_deref() != Some("Program") {
        return Err(unprocessable(&format!(
            "only a plan of kind Program has {what}"
        )));
    }
    Ok(plan)
}

// ---- triage ---------------------------------------------------------------------------------

/// `PUT /api/plans/{pid}/triage` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct TriageBody {
    /// `finish`, `fix`, `pause` or `stop`.
    outcome: String,
    /// Why the outcome was chosen; at least 10 characters after trimming.
    reason: String,
}

/// Renders one triage row for the API.
fn triage_view(t: &triages::Model) -> Value {
    json!({ "pid": t.pid, "triaged_on": t.triaged_on, "outcome": t.outcome, "reason": t.reason })
}

/// `PUT /api/plans/{pid}/triage` — record today's triage (append-only; it never changes the programme).
#[debug_handler]
async fn set_triage(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<TriageBody>,
) -> Result<Response> {
    let plan = programme(&ctx, &pid, "a triage").await?;
    if !rules::OUTCOMES.contains(&b.outcome.as_str()) {
        return Err(unprocessable(&format!(
            "outcome must be one of {:?}",
            rules::OUTCOMES
        )));
    }
    if b.reason.trim().chars().count() < 10 {
        return Err(unprocessable(
            "a reason of at least 10 characters is required",
        ));
    }
    let mut tx = crate::db::begin(&ctx).await?;
    let row: triages::Model = sqlx::query_as(
        "INSERT INTO programme_triages (pid, plan_pid, triaged_on, outcome, reason, decided_by)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(plan.pid)
    .bind(crate::tz::deployment_today())
    .bind(&b.outcome)
    .bind(b.reason.trim())
    .bind(caller.actor())
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        plan.pid,
        "programme_triaged",
        caller.actor(),
        Some(json!({ "outcome": b.outcome })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(triage_view(&row))
}

/// `GET /api/plans/{pid}/triage` — the history, newest first.
#[debug_handler]
async fn get_triage(State(ctx): State<AppContext>, Path(pid): Path<String>) -> Result<Response> {
    let plan = programme(&ctx, &pid, "a triage").await?;
    let rows: Vec<triages::Model> =
        sqlx::query_as("SELECT * FROM programme_triages WHERE plan_pid = $1 ORDER BY id DESC")
            .bind(plan.pid)
            .fetch_all(crate::db::pool(&ctx))
            .await?;
    format::json(
        json!({ "triage": rows.first().map(triage_view), "history": rows.iter().map(triage_view).collect::<Vec<_>>() }),
    )
}

// ---- thresholds -----------------------------------------------------------------------------

/// `PUT /api/plans/{pid}/thresholds` body; which of the other fields apply depends on `kind`.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ThresholdBody {
    /// Threshold kind, such as `confidence_red_packs`, `forecast_past` or `p80_cap`; validated by the rules.
    kind: String,
    /// `confidence_red_packs`: how many issued packs in a row at red (1 to 24).
    #[serde(default)]
    count: Option<i32>,
    /// `forecast_past`: the date the forecast finish must not pass.
    #[serde(default)]
    date: Option<chrono::NaiveDate>,
    /// `p80_cap`: the cap on the P80 cost, minor units, with `currency`.
    #[serde(default)]
    cap_minor: Option<i64>,
    /// Currency of `cap_minor`; needed with a `p80_cap` threshold.
    #[serde(default)]
    currency: Option<String>,
    /// Remove this threshold.
    #[serde(default)]
    remove: bool,
}

/// Renders one threshold, showing only the parameter that kind uses (the others are null, not zero).
fn threshold_view(t: &thresholds::Model) -> Value {
    json!({ "kind": t.kind, "count": t.count_value, "date": t.date_value, "cap_minor": t.amount_minor, "currency": t.currency })
}

/// `PUT /api/plans/{pid}/thresholds` — set (or with `remove`, delete) one threshold.
#[debug_handler]
async fn set_threshold(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<ThresholdBody>,
) -> Result<Response> {
    let plan = programme(&ctx, &pid, "thresholds").await?;
    if !rules::KINDS.contains(&b.kind.as_str()) {
        return Err(unprocessable(&format!(
            "kind must be one of {:?}",
            rules::KINDS
        )));
    }
    let mut tx = crate::db::begin(&ctx).await?;
    let existing: Option<thresholds::Model> = sqlx::query_as(
        "SELECT * FROM programme_thresholds WHERE plan_pid = $1 AND kind = $2 LIMIT 1",
    )
    .bind(plan.pid)
    .bind(&b.kind)
    .fetch_optional(tx.conn())
    .await?;
    if b.remove {
        if let Some(e) = existing {
            sqlx::query("DELETE FROM programme_thresholds WHERE id = $1")
                .bind(e.id)
                .execute(tx.conn())
                .await?;
        }
        AuditModel::record_on(
            tx.conn(),
            plan.pid,
            "threshold_removed",
            caller.actor(),
            Some(json!({ "kind": b.kind })),
        )
        .await
        .map_err(super::audit_failed)?;
        tx.commit().await?;
        return format::json(json!({ "removed": b.kind }));
    }
    let (count, date, amount, currency) = match b.kind.as_str() {
        "confidence_red_packs" if b.count.is_some_and(|n| (1..=24).contains(&n)) => {
            (b.count, None, None, None)
        }
        "confidence_red_packs" => return Err(unprocessable("count must be 1 to 24")),
        "forecast_past" if b.date.is_some() => (None, b.date, None, None),
        "forecast_past" => return Err(unprocessable("date is required")),
        _ => match (b.cap_minor, b.currency.as_deref()) {
            (Some(c), Some(cur)) if c > 0 && crate::governance::valid_currency(cur) => {
                (None, None, Some(c), Some(cur.to_string()))
            }
            _ => {
                return Err(unprocessable(
                    "cap_minor (above 0) and a currency (ISO 4217) are required",
                ));
            }
        },
    };
    let row: thresholds::Model = if let Some(e) = existing {
        sqlx::query_as(
            "UPDATE programme_thresholds SET count_value = $1, date_value = $2, amount_minor = $3, currency = $4
             WHERE id = $5
             RETURNING *",
        )
        .bind(count)
        .bind(date)
        .bind(amount)
        .bind(&currency)
        .bind(e.id)
        .fetch_one(tx.conn())
        .await?
    } else {
        sqlx::query_as(
            "INSERT INTO programme_thresholds (pid, plan_pid, kind, count_value, date_value, amount_minor, currency)
             VALUES ($1, $2, $3, $4, $5, $6, $7)
             RETURNING *",
        )
        .bind(Uuid::new_v4())
        .bind(plan.pid)
        .bind(&b.kind)
        .bind(count)
        .bind(date)
        .bind(amount)
        .bind(&currency)
        .fetch_one(tx.conn())
        .await?
    };
    AuditModel::record_on(
        tx.conn(),
        plan.pid,
        "threshold_set",
        caller.actor(),
        Some(threshold_view(&row)),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(threshold_view(&row))
}

/// `GET /api/plans/{pid}/thresholds`
#[debug_handler]
async fn get_thresholds(
    State(ctx): State<AppContext>,
    Path(pid): Path<String>,
) -> Result<Response> {
    let plan = programme(&ctx, &pid, "thresholds").await?;
    let rows: Vec<thresholds::Model> =
        sqlx::query_as("SELECT * FROM programme_thresholds WHERE plan_pid = $1 ORDER BY id")
            .bind(plan.pid)
            .fetch_all(crate::db::pool(&ctx))
            .await?;
    format::json(json!({ "thresholds": rows.iter().map(threshold_view).collect::<Vec<_>>() }))
}

// ---- the sweep ------------------------------------------------------------------------------

/// For each programme, whether it was red in each issued period, newest first (the latest issued pack of a
/// period that names the programme speaks for the period).
async fn red_history(ctx: &AppContext) -> Result<HashMap<String, Vec<bool>>> {
    let packs: Vec<director_packs::Model> = sqlx::query_as(
        "SELECT * FROM director_packs WHERE status = 'issued' ORDER BY period DESC, id DESC",
    )
    .fetch_all(crate::db::pool(ctx))
    .await?;
    let mut seen: HashSet<(String, String)> = HashSet::new();
    let mut out: HashMap<String, Vec<bool>> = HashMap::new();
    for p in packs {
        let Ok(body) = serde_json::from_str::<Value>(&p.body) else {
            continue;
        };
        for g in body["portfolio_map"]["groups"]
            .as_array()
            .into_iter()
            .flatten()
        {
            for prog in g["programmes"].as_array().into_iter().flatten() {
                let Some(id) = prog["plan_pid"].as_str() else {
                    continue;
                };
                if seen.insert((p.period.clone(), id.to_string())) {
                    out.entry(id.to_string())
                        .or_default()
                        .push(prog["confidence"]["value"] == "red");
                }
            }
        }
    }
    Ok(out)
}

/// `POST /api/threshold-sweep` — open an exception for each threshold now crossed, on the plans the caller can read; the
/// scheduled sweep covers the rest. Changes no programme.
#[debug_handler]
async fn sweep(State(ctx): State<AppContext>, caller: MaybeAuthUser) -> Result<Response> {
    let scope = super::plan_access::HiddenScope::of(&ctx, &caller).await?;
    format::json(run_sweep(&ctx, caller.actor(), Some(&scope)).await?)
}

/// The sweep itself, as `actor` (`None` for a scheduled run): what the endpoint and the scheduler both run. `hidden` leaves
/// out what a caller cannot read; the scheduler passes `None`.
///
/// # Errors
///
/// When a database statement fails.
#[expect(clippy::too_many_lines)]
pub(crate) async fn run_sweep(
    ctx: &AppContext,
    actor: Option<&str>,
    hidden: Option<&super::plan_access::HiddenScope>,
) -> Result<Value> {
    let today = crate::tz::deployment_today();
    let all: Vec<thresholds::Model> =
        sqlx::query_as("SELECT * FROM programme_thresholds ORDER BY id")
            .fetch_all(crate::db::pool(ctx))
            .await?;
    let reds = red_history(ctx).await?;
    // One query for every threshold's plan, not one per threshold (CQ-11).
    let live_plans: std::collections::HashMap<Uuid, plans::Model> =
        sqlx::query_as::<_, plans::Model>(
            "SELECT * FROM plans WHERE pid = ANY($1) AND deleted_at IS NULL",
        )
        .bind(all.iter().map(|t| t.plan_pid).collect::<Vec<_>>())
        .fetch_all(crate::db::pool(ctx))
        .await?
        .into_iter()
        .map(|p| (p.pid, p))
        .collect();
    let mut results = Vec::new();
    for t in &all {
        let Some(plan) = live_plans.get(&t.plan_pid) else {
            continue;
        };
        if hidden.is_some_and(|scope| scope.plan(&plan.pid)) {
            continue;
        }
        let (crossed, detail) = match t.kind.as_str() {
            "confidence_red_packs" => {
                let n = usize::try_from(t.count_value.unwrap_or(0)).unwrap_or(0);
                let h = reds.get(&plan.pid.to_string()).cloned().unwrap_or_default();
                (
                    rules::confidence_crossed(&h, n),
                    format!("delivery confidence was red in each of the last {n} issued packs"),
                )
            }
            "forecast_past" => {
                let limit = t.date_value.unwrap_or(today);
                let forecast = super::schedule_baselines::current(ctx, plan.pid)
                    .await
                    .ok()
                    .and_then(|(_, f, _, _)| f);
                (
                    rules::forecast_crossed(forecast, limit),
                    format!("the forecast finish is after {limit}"),
                )
            }
            _ => {
                let cur = t.currency.clone().unwrap_or_default();
                let tree = super::earned_value::subtree_ids(ctx, plan.pid).await?;
                let p80 = super::cost_risk::p80_by_currency(ctx, &tree)
                    .await?
                    .and_then(|m| m.get(&cur).copied());
                (
                    rules::p80_crossed(p80, t.amount_minor.unwrap_or(i64::MAX)),
                    format!(
                        "the P80 cost is above the cap of {} {cur} (minor units)",
                        t.amount_minor.unwrap_or(0)
                    ),
                )
            }
        };
        let mut row = json!({ "plan_pid": plan.pid, "programme": plan.name, "kind": t.kind, "crossed": crossed });
        if !crossed {
            results.push(row);
            continue;
        }
        let active_override: Option<i64> = sqlx::query_scalar(
            "SELECT id FROM threshold_decisions
             WHERE plan_pid = $1 AND threshold_kind = $2 AND action = 'override' AND expiry > $3
             LIMIT 1",
        )
        .bind(plan.pid)
        .bind(&t.kind)
        .bind(today)
        .fetch_optional(crate::db::pool(ctx))
        .await?;
        if active_override.is_some() {
            row["outcome"] = json!("suppressed_by_override");
            results.push(row);
            continue;
        }
        let open: Option<exceptions::Model> = sqlx::query_as(
            "SELECT * FROM delivery_exceptions WHERE plan_pid = $1 AND status = 'open' LIMIT 1",
        )
        .bind(plan.pid)
        .fetch_optional(crate::db::pool(ctx))
        .await?;
        let cause = format!("Threshold crossed: {detail}.");
        let actor = actor.map(str::to_string);
        match open {
            Some(o) if o.threshold_kind.is_some() => {
                row["outcome"] = json!("already_open");
                row["exception_pid"] = json!(o.pid);
            }
            Some(o) => {
                let joined = match &o.cause {
                    Some(c) if !c.trim().is_empty() => format!("{c} {cause}"),
                    _ => cause.clone(),
                };
                let mut tx = crate::db::begin(ctx).await?;
                sqlx::query(
                    "UPDATE delivery_exceptions SET threshold_kind = $1, cause = $2 WHERE id = $3 RETURNING id",
                )
                .bind(&t.kind)
                .bind(joined)
                .bind(o.id)
                .fetch_one(tx.conn())
                .await?;
                AuditModel::record_on(
                    tx.conn(),
                    o.pid,
                    "threshold_attached_to_exception",
                    actor.as_deref(),
                    Some(json!({ "kind": t.kind })),
                )
                .await
                .map_err(super::audit_failed)?;
                tx.commit().await?;
                row["outcome"] = json!("attached_to_open_exception");
                row["exception_pid"] = json!(o.pid);
            }
            None => {
                let now = Utc::now();
                let mut tx = crate::db::begin(ctx).await?;
                let e_pid: Uuid = sqlx::query_scalar(
                    "INSERT INTO delivery_exceptions
                         (pid, plan_pid, status, opened_at, opened_by, opened_value, worst_value, worsened_count,
                          cause, decision_needed, threshold_kind)
                     VALUES ($1, $2, 'open', $3, $4, 'threshold', 'threshold', 0, $5, $6, $7)
                     RETURNING pid",
                )
                .bind(Uuid::new_v4())
                .bind(plan.pid)
                .bind(now.fixed_offset())
                .bind(&actor)
                .bind(cause)
                .bind("Pause, reset, stop, or record an override with an expiry")
                .bind(&t.kind)
                .fetch_one(tx.conn())
                .await?;
                AuditModel::record_on(
                    tx.conn(),
                    e_pid,
                    "threshold_exception_opened",
                    actor.as_deref(),
                    Some(json!({ "kind": t.kind })),
                )
                .await
                .map_err(super::audit_failed)?;
                tx.commit().await?;
                row["outcome"] = json!("opened");
                row["exception_pid"] = json!(e_pid);
            }
        }
        results.push(row);
    }
    Ok(json!({
        "results": results,
        "note": "A crossing opens a delivery exception (or is attached to the one already open); a person then records pause, reset, stop or an override with an expiry. Nothing is paused, reset or stopped by this call.",
    }))
}

// ---- the decision ---------------------------------------------------------------------------

/// `POST /api/exceptions/{pid}/threshold-decision` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct DecideBody {
    /// One of `pause`, `reset`, `stop` or `override`. This only records a decision; nothing is paused, reset or stopped automatically.
    action: String,
    /// Why the decision was made; required.
    reason: String,
    /// Expiry date, required for an override (in the future, at most a year out) and refused for the other actions.
    #[serde(default)]
    expiry: Option<chrono::NaiveDate>,
}

/// `POST /api/exceptions/{pid}/threshold-decision` — record pause, reset, stop or an override; closes it.
#[debug_handler]
async fn decide(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<DecideBody>,
) -> Result<Response> {
    let row = super::confidence::find_exception(&ctx, &caller, &pid).await?;
    let Some(kind) = row.threshold_kind.clone() else {
        return Err(unprocessable(
            "this exception was not opened by a threshold: use the ordinary decision",
        ));
    };
    let today = crate::tz::deployment_today();
    let problems = rules::decision_problems(&b.action, &b.reason, b.expiry, today);
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let actor = caller.actor().unwrap_or("system").to_string();
    let mut tx = crate::db::begin(&ctx).await?;
    let text = format!("{}: {}", b.action, b.reason.trim());
    if row.status != "open" {
        return Err(Error::CustomError(
            StatusCode::CONFLICT,
            ErrorDetail::new("conflict", "this exception is already closed"),
        ));
    }
    sqlx::query(
        "UPDATE delivery_exceptions SET status = 'closed', closed_at = $1, decision = $2, decided_by = $3
         WHERE id = $4
         RETURNING id",
    )
    .bind(Utc::now().fixed_offset())
    .bind(text)
    .bind(&actor)
    .bind(row.id)
    .fetch_one(tx.conn())
    .await?;
    let d: decisions::Model = sqlx::query_as(
        "INSERT INTO threshold_decisions
             (pid, exception_pid, plan_pid, threshold_kind, action, expiry, reason, decided_by, decided_on)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(row.pid)
    .bind(row.plan_pid)
    .bind(&kind)
    .bind(&b.action)
    .bind(b.expiry)
    .bind(b.reason.trim())
    .bind(&actor)
    .bind(today)
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        row.pid,
        "threshold_decision",
        Some(&actor),
        Some(json!({ "action": b.action, "kind": kind, "expiry": b.expiry })),
    )
    .await
    .map_err(|e| Error::string(&e.to_string()))?;
    tx.commit().await?;
    format::json(
        json!({ "pid": d.pid, "exception_pid": row.pid, "action": d.action, "expiry": d.expiry, "decided_on": d.decided_on }),
    )
}

// ---- the pack -------------------------------------------------------------------------------

/// The pack's section: the latest triage of each programme, the decisions of the period by kind, and the
/// overrides still in force (with their expiry, not who decided).
///
/// # Errors
///
/// When a database read fails.
pub(crate) async fn pack_section(
    ctx: &AppContext,
    include: &HashSet<Uuid>,
    period: &str,
) -> Result<Value> {
    let names: HashMap<Uuid, String> =
        sqlx::query_as::<_, plans::Model>("SELECT * FROM plans WHERE deleted_at IS NULL")
            .fetch_all(crate::db::pool(ctx))
            .await?
            .into_iter()
            .filter(|p| include.contains(&p.pid) && p.kind.as_deref() == Some("Program"))
            .map(|p| (p.pid, p.name))
            .collect();
    let mut latest: HashMap<Uuid, triages::Model> = HashMap::new();
    let all_triages: Vec<triages::Model> =
        sqlx::query_as("SELECT * FROM programme_triages ORDER BY id")
            .fetch_all(crate::db::pool(ctx))
            .await?;
    for t in all_triages {
        if names.contains_key(&t.plan_pid) {
            latest.insert(t.plan_pid, t);
        }
    }
    let mut triage: Vec<Value> = latest
        .values()
        .map(|t| json!({ "programme": names[&t.plan_pid], "outcome": t.outcome, "triaged_on": t.triaged_on, "reason": t.reason }))
        .collect();
    triage.sort_by_key(|v| v["programme"].as_str().unwrap_or("").to_string());
    let count_of = |o: &str| latest.values().filter(|t| t.outcome == o).count();
    let today = crate::tz::deployment_today();
    let decided: Vec<decisions::Model> =
        sqlx::query_as::<_, decisions::Model>("SELECT * FROM threshold_decisions ORDER BY id")
            .fetch_all(crate::db::pool(ctx))
            .await?
            .into_iter()
            .filter(|d| names.contains_key(&d.plan_pid))
            .collect();
    let in_period = |a: &str| {
        decided
            .iter()
            .filter(|d| d.action == a && d.decided_on.format("%Y-%m").to_string() == period)
            .count()
    };
    let overrides: Vec<Value> = decided
        .iter()
        .filter(|d| d.action == "override" && d.expiry.is_some_and(|e| e > today))
        .map(|d| json!({ "programme": names[&d.plan_pid], "threshold": d.threshold_kind, "expiry": d.expiry }))
        .collect();
    let open = sqlx::query_scalar::<_, Uuid>(
        "SELECT plan_pid FROM delivery_exceptions WHERE status = 'open' AND threshold_kind IS NOT NULL",
    )
    .fetch_all(crate::db::pool(ctx))
    .await?
    .into_iter()
    .filter(|p| names.contains_key(p))
    .count();
    Ok(json!({
        "triage": triage,
        "triage_counts": { "finish": count_of("finish"), "fix": count_of("fix"), "pause": count_of("pause"), "stop": count_of("stop") },
        "decisions_in_period": { "pause": in_period("pause"), "reset": in_period("reset"), "stop": in_period("stop"), "override": in_period("override") },
        "overrides_in_force": overrides,
        "open_threshold_exceptions": open,
        "note": "Triage is the latest dated triage of each programme. A threshold crossing opens an exception that needs a pause, reset, stop or an override with an expiry; nothing happens automatically. The decisions are counted in the period they were made.",
    }))
}
