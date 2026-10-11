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

use axum::http::StatusCode;
use chrono::Utc;
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use sea_orm::{QueryOrder, TransactionTrait};
use serde::Deserialize;
use serde_json::{Value, json};
use uuid::Uuid;

use crate::auth::MaybeAuthUser;
use crate::models::_entities::{
    delivery_exceptions as exceptions, director_packs, plans, programme_thresholds as thresholds,
    programme_triages as triages, threshold_decisions as decisions,
};
use crate::models::audit_logs::Model as AuditModel;
use crate::thresholds as rules;

fn unprocessable(message: &str) -> Error {
    Error::CustomError(
        StatusCode::UNPROCESSABLE_ENTITY,
        ErrorDetail::new("unprocessable_entity", message),
    )
}

fn db_err(e: &sea_orm::DbErr) -> Error {
    Error::string(&e.to_string())
}

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

async fn programme(ctx: &AppContext, pid: &str, what: &str) -> Result<plans::Model> {
    let plan = super::governance::find_item(ctx, pid).await?;
    if plan.kind.as_deref() != Some("Program") {
        return Err(unprocessable(&format!("only a plan of kind Program has {what}")));
    }
    Ok(plan)
}

// ---- triage ---------------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct TriageBody {
    /// `finish`, `fix`, `pause` or `stop`.
    outcome: String,
    reason: String,
}

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
        return Err(unprocessable(&format!("outcome must be one of {:?}", rules::OUTCOMES)));
    }
    if b.reason.trim().chars().count() < 10 {
        return Err(unprocessable("a reason of at least 10 characters is required"));
    }
    let row = triages::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        plan_pid: ActiveValue::set(plan.pid),
        triaged_on: ActiveValue::set(Utc::now().date_naive()),
        outcome: ActiveValue::set(b.outcome.clone()),
        reason: ActiveValue::set(b.reason.trim().to_string()),
        decided_by: ActiveValue::set(caller.actor().map(str::to_string)),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(|e| db_err(&e))?;
    AuditModel::record(&ctx.db, plan.pid, "programme_triaged", caller.actor(), Some(json!({ "outcome": b.outcome })))
        .await
        .ok();
    format::json(triage_view(&row))
}

/// `GET /api/plans/{pid}/triage` — the history, newest first.
#[debug_handler]
async fn get_triage(State(ctx): State<AppContext>, Path(pid): Path<String>) -> Result<Response> {
    let plan = programme(&ctx, &pid, "a triage").await?;
    let rows = triages::Entity::find()
        .filter(triages::Column::PlanPid.eq(plan.pid))
        .order_by_desc(triages::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    format::json(json!({ "triage": rows.first().map(triage_view), "history": rows.iter().map(triage_view).collect::<Vec<_>>() }))
}

// ---- thresholds -----------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct ThresholdBody {
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
    #[serde(default)]
    currency: Option<String>,
    /// Remove this threshold.
    #[serde(default)]
    remove: bool,
}

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
        return Err(unprocessable(&format!("kind must be one of {:?}", rules::KINDS)));
    }
    let existing = thresholds::Entity::find()
        .filter(thresholds::Column::PlanPid.eq(plan.pid))
        .filter(thresholds::Column::Kind.eq(&b.kind))
        .one(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    if b.remove {
        if let Some(e) = existing {
            e.delete(&ctx.db).await.map_err(|e| db_err(&e))?;
        }
        AuditModel::record(&ctx.db, plan.pid, "threshold_removed", caller.actor(), Some(json!({ "kind": b.kind }))).await.ok();
        return format::json(json!({ "removed": b.kind }));
    }
    let (count, date, amount, currency) = match b.kind.as_str() {
        "confidence_red_packs" if b.count.is_some_and(|n| (1..=24).contains(&n)) => (b.count, None, None, None),
        "confidence_red_packs" => return Err(unprocessable("count must be 1 to 24")),
        "forecast_past" if b.date.is_some() => (None, b.date, None, None),
        "forecast_past" => return Err(unprocessable("date is required")),
        _ => match (b.cap_minor, b.currency.as_deref()) {
            (Some(c), Some(cur)) if c > 0 && crate::governance::valid_currency(cur) => (None, None, Some(c), Some(cur.to_string())),
            _ => return Err(unprocessable("cap_minor (above 0) and a currency (ISO 4217) are required")),
        },
    };
    let row = if let Some(e) = existing {
        let mut a: thresholds::ActiveModel = e.into();
        a.count_value = ActiveValue::set(count);
        a.date_value = ActiveValue::set(date);
        a.amount_minor = ActiveValue::set(amount);
        a.currency = ActiveValue::set(currency);
        a.update(&ctx.db).await.map_err(|e| db_err(&e))?
    } else {
        thresholds::ActiveModel {
            pid: ActiveValue::set(Uuid::new_v4()),
            plan_pid: ActiveValue::set(plan.pid),
            kind: ActiveValue::set(b.kind.clone()),
            count_value: ActiveValue::set(count),
            date_value: ActiveValue::set(date),
            amount_minor: ActiveValue::set(amount),
            currency: ActiveValue::set(currency),
            ..Default::default()
        }
        .insert(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
    };
    AuditModel::record(&ctx.db, plan.pid, "threshold_set", caller.actor(), Some(threshold_view(&row))).await.ok();
    format::json(threshold_view(&row))
}

/// `GET /api/plans/{pid}/thresholds`
#[debug_handler]
async fn get_thresholds(State(ctx): State<AppContext>, Path(pid): Path<String>) -> Result<Response> {
    let plan = programme(&ctx, &pid, "thresholds").await?;
    let rows = thresholds::Entity::find()
        .filter(thresholds::Column::PlanPid.eq(plan.pid))
        .order_by_asc(thresholds::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    format::json(json!({ "thresholds": rows.iter().map(threshold_view).collect::<Vec<_>>() }))
}

// ---- the sweep ------------------------------------------------------------------------------

/// For each programme, whether it was red in each issued period, newest first (the latest issued pack of a
/// period that names the programme speaks for the period).
async fn red_history(ctx: &AppContext) -> Result<HashMap<String, Vec<bool>>> {
    let packs = director_packs::Entity::find()
        .filter(director_packs::Column::Status.eq("issued"))
        .order_by_desc(director_packs::Column::Period)
        .order_by_desc(director_packs::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?;
    let mut seen: HashSet<(String, String)> = HashSet::new();
    let mut out: HashMap<String, Vec<bool>> = HashMap::new();
    for p in packs {
        let Ok(body) = serde_json::from_str::<Value>(&p.body) else { continue };
        for g in body["portfolio_map"]["groups"].as_array().into_iter().flatten() {
            for prog in g["programmes"].as_array().into_iter().flatten() {
                let Some(id) = prog["plan_pid"].as_str() else { continue };
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

/// `POST /api/threshold-sweep` — open an exception for each threshold now crossed. Changes no programme.
#[debug_handler]
#[allow(clippy::too_many_lines)]
async fn sweep(State(ctx): State<AppContext>, caller: MaybeAuthUser) -> Result<Response> {
    let today = Utc::now().date_naive();
    let all = thresholds::Entity::find().order_by_asc(thresholds::Column::Id).all(&ctx.db).await.map_err(|e| db_err(&e))?;
    let reds = red_history(&ctx).await?;
    let mut results = Vec::new();
    for t in &all {
        let Some(plan) = plans::Entity::find()
            .filter(plans::Column::Pid.eq(t.plan_pid))
            .filter(plans::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await
            .map_err(|e| db_err(&e))?
        else {
            continue;
        };
        let (crossed, detail) = match t.kind.as_str() {
            "confidence_red_packs" => {
                let n = usize::try_from(t.count_value.unwrap_or(0)).unwrap_or(0);
                let h = reds.get(&plan.pid.to_string()).cloned().unwrap_or_default();
                (rules::confidence_crossed(&h, n), format!("delivery confidence was red in each of the last {n} issued packs"))
            }
            "forecast_past" => {
                let limit = t.date_value.unwrap_or(today);
                let forecast = super::schedule_baselines::current(&ctx, plan.pid).await.ok().and_then(|(_, f, _, _)| f);
                (rules::forecast_crossed(forecast, limit), format!("the forecast finish is after {limit}"))
            }
            _ => {
                let cur = t.currency.clone().unwrap_or_default();
                let tree = super::earned_value::subtree_ids(&ctx, plan.pid).await?;
                let p80 = super::cost_risk::p80_by_currency(&ctx, &tree).await?.and_then(|m| m.get(&cur).copied());
                (
                    rules::p80_crossed(p80, t.amount_minor.unwrap_or(i64::MAX)),
                    format!("the P80 cost is above the cap of {} {cur} (minor units)", t.amount_minor.unwrap_or(0)),
                )
            }
        };
        let mut row = json!({ "plan_pid": plan.pid, "programme": plan.name, "kind": t.kind, "crossed": crossed });
        if !crossed {
            results.push(row);
            continue;
        }
        let active_override = decisions::Entity::find()
            .filter(decisions::Column::PlanPid.eq(plan.pid))
            .filter(decisions::Column::ThresholdKind.eq(&t.kind))
            .filter(decisions::Column::Action.eq("override"))
            .filter(decisions::Column::Expiry.gt(today))
            .one(&ctx.db)
            .await
            .map_err(|e| db_err(&e))?;
        if active_override.is_some() {
            row["outcome"] = json!("suppressed_by_override");
            results.push(row);
            continue;
        }
        let open = exceptions::Entity::find()
            .filter(exceptions::Column::PlanPid.eq(plan.pid))
            .filter(exceptions::Column::Status.eq("open"))
            .one(&ctx.db)
            .await
            .map_err(|e| db_err(&e))?;
        let cause = format!("Threshold crossed: {detail}.");
        let actor = caller.actor().map(str::to_string);
        match open {
            Some(o) if o.threshold_kind.is_some() => {
                row["outcome"] = json!("already_open");
                row["exception_pid"] = json!(o.pid);
            }
            Some(o) => {
                let mut a: exceptions::ActiveModel = o.clone().into();
                a.threshold_kind = ActiveValue::set(Some(t.kind.clone()));
                a.cause = ActiveValue::set(Some(match &o.cause {
                    Some(c) if !c.trim().is_empty() => format!("{c} {cause}"),
                    _ => cause.clone(),
                }));
                a.update(&ctx.db).await.map_err(|e| db_err(&e))?;
                AuditModel::record(&ctx.db, o.pid, "threshold_attached_to_exception", actor.as_deref(), Some(json!({ "kind": t.kind }))).await.ok();
                row["outcome"] = json!("attached_to_open_exception");
                row["exception_pid"] = json!(o.pid);
            }
            None => {
                let now = Utc::now();
                let e = exceptions::ActiveModel {
                    pid: ActiveValue::set(Uuid::new_v4()),
                    plan_pid: ActiveValue::set(plan.pid),
                    status: ActiveValue::set("open".to_string()),
                    opened_at: ActiveValue::set(now.into()),
                    opened_by: ActiveValue::set(actor.clone()),
                    opened_value: ActiveValue::set("threshold".to_string()),
                    worst_value: ActiveValue::set("threshold".to_string()),
                    worsened_count: ActiveValue::set(0),
                    cause: ActiveValue::set(Some(cause)),
                    decision_needed: ActiveValue::set(Some("Pause, reset, stop, or record an override with an expiry".to_string())),
                    threshold_kind: ActiveValue::set(Some(t.kind.clone())),
                    ..Default::default()
                }
                .insert(&ctx.db)
                .await
                .map_err(|e| db_err(&e))?;
                AuditModel::record(&ctx.db, e.pid, "threshold_exception_opened", actor.as_deref(), Some(json!({ "kind": t.kind }))).await.ok();
                row["outcome"] = json!("opened");
                row["exception_pid"] = json!(e.pid);
            }
        }
        results.push(row);
    }
    format::json(json!({
        "results": results,
        "note": "A crossing opens a delivery exception (or is attached to the one already open); a person then records pause, reset, stop or an override with an expiry. Nothing is paused, reset or stopped by this call.",
    }))
}

// ---- the decision ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct DecideBody {
    action: String,
    reason: String,
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
        return Err(unprocessable("this exception was not opened by a threshold: use the ordinary decision"));
    };
    let today = Utc::now().date_naive();
    let problems = rules::decision_problems(&b.action, &b.reason, b.expiry, today);
    if !problems.is_empty() {
        return Err(unprocessable(&problems.join("; ")));
    }
    let actor = caller.actor().unwrap_or("system").to_string();
    let tx = ctx.db.begin().await.map_err(|e| db_err(&e))?;
    let text = format!("{}: {}", b.action, b.reason.trim());
    let mut a: exceptions::ActiveModel = row.clone().into();
    if row.status != "open" {
        return Err(Error::CustomError(StatusCode::CONFLICT, ErrorDetail::new("conflict", "this exception is already closed")));
    }
    a.status = ActiveValue::set("closed".to_string());
    a.closed_at = ActiveValue::set(Some(Utc::now().into()));
    a.decision = ActiveValue::set(Some(text));
    a.decided_by = ActiveValue::set(Some(actor.clone()));
    a.update(&tx).await.map_err(|e| db_err(&e))?;
    let d = decisions::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        exception_pid: ActiveValue::set(row.pid),
        plan_pid: ActiveValue::set(row.plan_pid),
        threshold_kind: ActiveValue::set(kind.clone()),
        action: ActiveValue::set(b.action.clone()),
        expiry: ActiveValue::set(b.expiry),
        reason: ActiveValue::set(b.reason.trim().to_string()),
        decided_by: ActiveValue::set(Some(actor.clone())),
        decided_on: ActiveValue::set(today),
        ..Default::default()
    }
    .insert(&tx)
    .await
    .map_err(|e| db_err(&e))?;
    AuditModel::record(&tx, row.pid, "threshold_decision", Some(&actor), Some(json!({ "action": b.action, "kind": kind, "expiry": b.expiry })))
        .await
        .map_err(|e| Error::string(&e.to_string()))?;
    tx.commit().await.map_err(|e| db_err(&e))?;
    format::json(json!({ "pid": d.pid, "exception_pid": row.pid, "action": d.action, "expiry": d.expiry, "decided_on": d.decided_on }))
}

// ---- the pack -------------------------------------------------------------------------------

/// The pack's section: the latest triage of each programme, the decisions of the period by kind, and the
/// overrides still in force (with their expiry, not who decided).
///
/// # Errors
///
/// When a database read fails.
pub(crate) async fn pack_section(ctx: &AppContext, include: &HashSet<Uuid>, period: &str) -> Result<Value> {
    let names: HashMap<Uuid, String> = plans::Entity::find()
        .filter(plans::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .into_iter()
        .filter(|p| include.contains(&p.pid) && p.kind.as_deref() == Some("Program"))
        .map(|p| (p.pid, p.name))
        .collect();
    let mut latest: HashMap<Uuid, triages::Model> = HashMap::new();
    for t in triages::Entity::find().order_by_asc(triages::Column::Id).all(&ctx.db).await.map_err(|e| db_err(&e))? {
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
    let today = Utc::now().date_naive();
    let decided: Vec<decisions::Model> = decisions::Entity::find()
        .order_by_asc(decisions::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .into_iter()
        .filter(|d| names.contains_key(&d.plan_pid))
        .collect();
    let in_period = |a: &str| decided.iter().filter(|d| d.action == a && d.decided_on.format("%Y-%m").to_string() == period).count();
    let overrides: Vec<Value> = decided
        .iter()
        .filter(|d| d.action == "override" && d.expiry.is_some_and(|e| e > today))
        .map(|d| json!({ "programme": names[&d.plan_pid], "threshold": d.threshold_kind, "expiry": d.expiry }))
        .collect();
    let open = exceptions::Entity::find()
        .filter(exceptions::Column::Status.eq("open"))
        .filter(exceptions::Column::ThresholdKind.is_not_null())
        .all(&ctx.db)
        .await
        .map_err(|e| db_err(&e))?
        .into_iter()
        .filter(|e| names.contains_key(&e.plan_pid))
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
