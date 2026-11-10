//! **Realized gains** and **strategic performance** — the surface over
//! [`crate::value`] (entity spec §9.2c / FR-33, FR-34, FR-36).
//!
//! Every figure is derived on read. Nothing here is stored, so
//! recording a value point corrects every number resting on it rather
//! than leaving a stale one behind.
//!
//! The rule that runs through all of it: **absent evidence reports
//! `null` with a reason and sorts last, never `0`**. A plan with no
//! value points has not failed to deliver — it has not been measured,
//! and those are different findings.

use super::unprocessable;
use crate::prelude::*;
use axum::http::HeaderMap;
use serde::Deserialize;
use uuid::Uuid;

use super::find_live_plan as find_plan;
use crate::auth::MaybeAuthUser;
use crate::models::_entities::{
    adoption_snapshots, budget_lines, business_case_targets, satisfaction_responses, value_points,
};
use crate::models::audit_logs::Model as AuditModel;
use crate::value as rules;

/// Upper bound on rows read for a rollup query, so a runaway table cannot produce an unbounded response.
const MAX_ROWS: i64 = 5000;

/// `POST /api/plans/{pid}/business-case` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct TargetPayload {
    /// Name of the measure being promised; required (blank is refused).
    metric: String,
    /// Value at approval time, in the metric's unit.
    baseline_value: i64,
    /// Promised value, in the metric's unit.
    target_value: i64,
    /// Optional unit label for the metric.
    #[serde(default)]
    unit: Option<String>,
    /// ISO 4217 code when the metric is monetary; `None` for non-monetary metrics.
    #[serde(default)]
    currency: Option<String>,
    /// Where the promise was made: `charter` or `gate_review`; `None` means `charter`.
    #[serde(default)]
    source: Option<String>,
}

/// `POST /api/plans/{pid}/business-case` — record a promised target.
///
/// `approved_at` is stamped here and has no update path: it is the
/// Time-to-Value clock start, and a clock start that can move is not a
/// measurement.
#[debug_handler]
async fn create_target(
    Path(pid): Path<String>,
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(payload): Json<TargetPayload>,
) -> Result<Response> {
    let plan = find_plan(&ctx, &pid).await?;
    let source = payload.source.unwrap_or_else(|| "charter".to_string());
    if !["charter", "gate_review"].contains(&source.as_str()) {
        return Err(unprocessable("source must be charter or gate_review"));
    }
    if payload.metric.trim().is_empty() {
        return Err(unprocessable("metric is required"));
    }
    let mut tx = crate::db::begin(&ctx).await?;
    let row_pid: Uuid = sqlx::query_scalar(
        "INSERT INTO business_case_targets
             (pid, plan_pid, metric, baseline_value, target_value, unit, currency, promised_by, source,
              approved_by_ref, deleted_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, NULL, $8, $9, NULL)
         RETURNING pid",
    )
    .bind(Uuid::new_v4())
    .bind(plan.pid)
    .bind(payload.metric.trim())
    .bind(payload.baseline_value)
    .bind(payload.target_value)
    .bind(&payload.unit)
    .bind(&payload.currency)
    .bind(&source)
    .bind(caller.actor())
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        plan.pid,
        "business_case_target_set",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(serde_json::json!({ "pid": row_pid.to_string() }))
}

/// `POST /api/plans/{pid}/value-points` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ValuePayload {
    /// Observed value in minor units of `currency` (or the metric's own unit when not monetary).
    value: i64,
    /// How the value was measured; an unrecognized or absent method falls back to the default method token.
    #[serde(default)]
    method: Option<String>,
    /// ISO 4217 code for a monetary value; `None` when not monetary.
    #[serde(default)]
    currency: Option<String>,
    /// Marks the first measurable value, which stops the Time-to-Value clock; at most one per plan.
    #[serde(default)]
    is_first_measurable: bool,
    /// Optional reference to the evidence behind the value (a link or document pid).
    #[serde(default)]
    evidence_ref: Option<String>,
}

/// `POST /api/plans/{pid}/value-points` — record observed value.
#[debug_handler]
async fn create_value_point(
    Path(pid): Path<String>,
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(payload): Json<ValuePayload>,
) -> Result<Response> {
    let plan = find_plan(&ctx, &pid).await?;
    let mut tx = crate::db::begin(&ctx).await?;
    let row_pid: Uuid = sqlx::query_scalar(
        "INSERT INTO value_points
             (pid, plan_pid, benefit_pid, value, currency, is_first_measurable, method, evidence_ref, actor,
              deleted_at)
         VALUES ($1, $2, NULL, $3, $4, $5, $6, $7, $8, NULL)
         RETURNING pid",
    )
    .bind(Uuid::new_v4())
    .bind(plan.pid)
    .bind(payload.value)
    .bind(&payload.currency)
    .bind(payload.is_first_measurable)
    .bind(rules::Method::parse(payload.method.as_deref()).token())
    .bind(&payload.evidence_ref)
    .bind(caller.actor())
    .fetch_one(tx.conn())
    .await
    .map_err(|e| {
        // The partial unique index refuses a second first-measurable
        // point: the clock stops once.
        if e.to_string().contains("value_points_first") {
            unprocessable(
                "this plan already has a first-measurable value point: the \
                 Time-to-Value clock stops once",
            )
        } else {
            Error::from(e)
        }
    })?;
    AuditModel::record_on(tx.conn(), plan.pid, "value_observed", caller.actor(), None)
        .await
        .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(serde_json::json!({ "pid": row_pid.to_string() }))
}

/// `POST /api/plans/{pid}/adoption` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct AdoptionPayload {
    /// Users active in the window; must not be negative.
    active_users: i64,
    /// Intended number of users; must be positive so the adoption rate has a non-zero denominator.
    target_users: i64,
    /// Length of the measurement window in days; must be positive.
    window_days: i32,
    /// What counted as an "active user", stored beside the rate so later readings stay comparable; required.
    definition: String,
}

/// `POST /api/plans/{pid}/adoption` — record an adoption snapshot.
#[debug_handler]
async fn create_adoption(
    Path(pid): Path<String>,
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(payload): Json<AdoptionPayload>,
) -> Result<Response> {
    let plan = find_plan(&ctx, &pid).await?;
    if payload.target_users <= 0 {
        return Err(unprocessable(
            "target_users must be positive: a rate with a zero denominator is \
             refused here rather than divided at read",
        ));
    }
    if payload.active_users < 0 || payload.window_days <= 0 {
        return Err(unprocessable(
            "active_users must not be negative and window_days must be positive",
        ));
    }
    if payload.definition.trim().is_empty() {
        return Err(unprocessable(
            "definition is required: `active user` is the term most easily \
             redefined between two readings, so it is stored beside the rate",
        ));
    }
    let mut tx = crate::db::begin(&ctx).await?;
    let row_pid: Uuid = sqlx::query_scalar(
        "INSERT INTO adoption_snapshots
             (pid, plan_pid, active_users, target_users, window_days, definition, deleted_at)
         VALUES ($1, $2, $3, $4, $5, $6, NULL)
         RETURNING pid",
    )
    .bind(Uuid::new_v4())
    .bind(plan.pid)
    .bind(payload.active_users)
    .bind(payload.target_users)
    .bind(payload.window_days)
    .bind(payload.definition.trim())
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        plan.pid,
        "adoption_recorded",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(serde_json::json!({ "pid": row_pid.to_string() }))
}

/// `POST /api/plans/{pid}/satisfaction` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct SatisfactionPayload {
    /// Survey instrument, `nps` or `csat`.
    instrument: String,
    /// Score from 0 to 10 inclusive.
    score: i16,
    /// Respondent role: `sponsor`, `user`, `team` or `other`.
    respondent_role: String,
    /// Optional free-text comment from the respondent.
    #[serde(default)]
    comment: Option<String>,
}

/// `POST /api/plans/{pid}/satisfaction` — record a response.
#[debug_handler]
async fn create_satisfaction(
    Path(pid): Path<String>,
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(payload): Json<SatisfactionPayload>,
) -> Result<Response> {
    let plan = find_plan(&ctx, &pid).await?;
    if !["nps", "csat"].contains(&payload.instrument.as_str()) {
        return Err(unprocessable("instrument must be nps or csat"));
    }
    if !(0..=10).contains(&payload.score) {
        return Err(unprocessable("score must be between 0 and 10"));
    }
    if !["sponsor", "user", "team", "other"].contains(&payload.respondent_role.as_str()) {
        return Err(unprocessable(
            "respondent_role must be sponsor, user, team or other",
        ));
    }
    let mut tx = crate::db::begin(&ctx).await?;
    let row_pid: Uuid = sqlx::query_scalar(
        "INSERT INTO satisfaction_responses
             (pid, plan_pid, instrument, score, respondent_role, comment, deleted_at)
         VALUES ($1, $2, $3, $4, $5, $6, NULL)
         RETURNING pid",
    )
    .bind(Uuid::new_v4())
    .bind(plan.pid)
    .bind(&payload.instrument)
    .bind(payload.score)
    .bind(&payload.respondent_role)
    .bind(&payload.comment)
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        plan.pid,
        "satisfaction_recorded",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(serde_json::json!({ "pid": row_pid.to_string() }))
}

/// `GET /api/plans/{pid}/value-realization` — the realized-gains view.
#[debug_handler]
async fn value_realization(
    Path(pid): Path<String>,
    State(ctx): State<AppContext>,
    headers: HeaderMap,
) -> Result<Response> {
    let plan = find_plan(&ctx, &pid).await?;

    let points: Vec<value_points::Model> = sqlx::query_as(
        "SELECT * FROM value_points WHERE plan_pid = $1 AND deleted_at IS NULL LIMIT $2",
    )
    .bind(plan.pid)
    .bind(MAX_ROWS)
    .fetch_all(crate::db::pool(&ctx))
    .await?;
    let targets: Vec<business_case_targets::Model> = sqlx::query_as(
        "SELECT * FROM business_case_targets WHERE plan_pid = $1 AND deleted_at IS NULL",
    )
    .bind(plan.pid)
    .fetch_all(crate::db::pool(&ctx))
    .await?;
    let budgets: Vec<budget_lines::Model> =
        sqlx::query_as("SELECT * FROM budget_lines WHERE plan_pid = $1 AND deleted_at IS NULL")
            .bind(plan.pid)
            .fetch_all(crate::db::pool(&ctx))
            .await?;
    let adoption: Option<adoption_snapshots::Model> = sqlx::query_as(
        "SELECT * FROM adoption_snapshots WHERE plan_pid = $1 AND deleted_at IS NULL
         ORDER BY observed_at DESC LIMIT 1",
    )
    .bind(plan.pid)
    .fetch_optional(crate::db::pool(&ctx))
    .await?;

    let approved_at = targets.iter().map(|t| t.approved_at).min();
    let facts: Vec<rules::ValueFact> = points
        .iter()
        .map(|row| rules::ValueFact {
            value: row.value,
            method: rules::Method::parse(Some(&row.method)),
            is_first_measurable: row.is_first_measurable,
            days_from_approval: approved_at.map(|start| {
                (row.observed_at.with_timezone(&chrono::Utc) - start.with_timezone(&chrono::Utc))
                    .num_days()
            }),
        })
        .collect();

    // Investment is **actual** cost, not planned: ROI on money not yet
    // spent is a forecast, and this view reports what happened.
    //
    // Budget lines carry a currency each, so a mixed-currency plan has
    // no single investment figure. This service converts nowhere, so
    // the ROI is withheld with `mixed_currency` rather than silently
    // adding pounds to euros — the restriction Smart Score already
    // carries.
    let currencies: std::collections::BTreeSet<&str> =
        budgets.iter().map(|line| line.currency.as_str()).collect();
    let mixed = currencies.len() > 1;
    let investment: i64 = budgets
        .iter()
        .fold(0_i64, |acc, line| acc.saturating_add(line.actual_minor));
    let roi = if mixed {
        rules::Roi {
            basis_points: None,
            absent: Some(rules::Absent::MixedCurrency),
            realized_minor: facts.iter().fold(0_i64, |a, f| a.saturating_add(f.value)),
            investment_minor: investment,
            measured_share_basis_points: None,
        }
    } else {
        rules::roi(&facts, investment)
    };

    let adoption_view = adoption.as_ref().map_or_else(
        || serde_json::json!({ "measured": false, "reason": "no adoption snapshot recorded" }),
        |snap| {
            let rate = rules::adoption_rate(snap.active_users, snap.target_users);
            serde_json::json!({
                "measured": rate.is_ok(),
                "basis_points": rate.ok(),
                "absent": rate.err(),
                "active_users": snap.active_users,
                "target_users": snap.target_users,
                // Returned with the rate, because a rate whose
                // definition is not visible cannot be compared.
                "definition": snap.definition,
                "window_days": snap.window_days,
            })
        },
    );

    let body = serde_json::json!({
        "plan_pid": plan.pid.to_string(),
        "transformation_roi": roi,
        "time_to_value": rules::time_to_value(&facts),
        "adoption": adoption_view,
        "performance_to_business_case": targets.iter().map(|t| serde_json::json!({
            "metric": t.metric,
            "baseline_value": t.baseline_value,
            "target_value": t.target_value,
            "promised_by": t.promised_by,
            "source": t.source,
            "approved_at": t.approved_at,
        })).collect::<Vec<_>>(),
        "as_of": chrono::Utc::now(),
        "asserted": true,
    });
    let etag = super::etag_of(&body);
    super::conditional_json(&headers, &etag, &body)
}

/// `GET /api/plans/{pid}/performance` — the six-dimension view.
#[debug_handler]
async fn performance(
    Path(pid): Path<String>,
    State(ctx): State<AppContext>,
    headers: HeaderMap,
) -> Result<Response> {
    let plan = find_plan(&ctx, &pid).await?;
    let responses: Vec<satisfaction_responses::Model> = sqlx::query_as(
        "SELECT * FROM satisfaction_responses
         WHERE plan_pid = $1 AND deleted_at IS NULL AND instrument = 'nps'
         LIMIT $2",
    )
    .bind(plan.pid)
    .bind(MAX_ROWS)
    .fetch_all(crate::db::pool(&ctx))
    .await?;
    let scores: Vec<u8> = responses
        .iter()
        .filter_map(|r| u8::try_from(r.score).ok())
        .collect();

    // SC-4: schedule and cost performance are real earned-value figures now. They are
    // `null` with an accurate reason whenever they cannot be stated — never defaulted
    // to 1.0 ("exactly on plan").
    let as_of = crate::tz::deployment_today();
    let (spi, cpi, absent, reason): (Option<i64>, Option<i64>, Option<&str>, Option<String>) =
        match super::earned_value::compute(&ctx, plan.pid, as_of).await? {
            super::earned_value::Outcome::NoBaseline => (
                None,
                None,
                Some("no_baseline"),
                Some("no phased budget baseline: a plan without one is unmeasured, not on track".to_string()),
            ),
            super::earned_value::Outcome::NoPercent => (
                None,
                None,
                Some("no_weighted_tasks"),
                Some("a baseline exists, but there is no task to measure percent complete against".to_string()),
            ),
            super::earned_value::Outcome::Figures { blocks, .. } => match blocks.as_slice() {
                [(_, _, b)] => (
                    b.spi.bp,
                    b.cpi.bp,
                    if b.spi.bp.is_none() || b.cpi.bp.is_none() { Some("not_yet_defined") } else { None },
                    None,
                ),
                _ => (
                    None,
                    None,
                    Some("several_currencies"),
                    Some("the plan has baselines in several currencies; see /earned-value for one block per currency".to_string()),
                ),
            },
        };

    let body = serde_json::json!({
        "plan_pid": plan.pid.to_string(),
        "stakeholder": { "nps": rules::nps(&scores) },
        "schedule": {
            "spi": spi,
            "unit": "basis_points",
            "absent": absent,
            "reason": reason,
        },
        "financial": {
            "cpi": cpi,
            "unit": "basis_points",
            "absent": absent,
            "reason": reason,
        },
        "detail": format!("/api/plans/{}/earned-value", plan.pid),
        "as_of": chrono::Utc::now(),
    });
    let etag = super::etag_of(&body);
    super::conditional_json(&headers, &etag, &body)
}

/// The value and performance routes.
#[must_use]
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/business-case", post(create_target))
        .add("/plans/{pid}/value-points", post(create_value_point))
        .add("/plans/{pid}/adoption", post(create_adoption))
        .add("/plans/{pid}/satisfaction", post(create_satisfaction))
        .add("/plans/{pid}/value-realization", get(value_realization))
        .add("/plans/{pid}/performance", get(performance))
}
