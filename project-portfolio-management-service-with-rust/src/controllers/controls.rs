//! **Controls** — the Controlling-process register (entity spec §5.9.8
//! / FR-38, FR-39). Pure rules live in [`crate::controls`].
//!
//! The four process steps are records here, not habits: set a standard
//! (a `controls` row), measure (a `control_readings` row), compare (the
//! verdict and gap, derived at write from the standard), act (a
//! `control_actions` row).
//!
//! **The timing decides what a failing control may do**, and that is
//! enforced rather than documented: only a `feedforward` control is
//! allowed to block, because acting before the fact is that timing's
//! entire purpose; a `feedback` control judges finished work and may
//! only record.
//!
//! Two honesty rules the read path depends on:
//!
//! - **A control naming a metric this service does not produce is
//!   refused at write** (`422`). Left registered, it would read
//!   `unmeasured` forever — and a check nobody can evaluate is
//!   indistinguishable from one that passes.
//! - **`unmeasured` is a third verdict**, excluded from pass rates
//!   rather than counted as either half. An all-unmeasured control
//!   reports `null`, never `0%`.

use super::unprocessable;
use crate::prelude::*;
use axum::http::HeaderMap;
use serde::Deserialize;
use uuid::Uuid;

use super::find_live_plan as find_plan;
use crate::auth::MaybeAuthUser;
use crate::controls as rules;
use crate::models::_entities::{control_actions, control_readings, controls, plans};
use crate::models::audit_logs::Model as AuditModel;
use crate::validation::MAX_TEXT_LEN;

/// The metrics a control may name. A control's source must resolve to
/// something this service actually produces (FR-38); the list is the
/// derived figures the read surfaces already serve.
pub const KNOWN_METRICS: &[&str] = &[
    "flow_efficiency",
    "cycle_time_p85",
    "lead_time_p85",
    "throughput",
    "work_in_progress",
    "first_pass_yield",
    "burndown_remaining",
    "smart_score",
    "dipp",
    "dipp_progress_index",
    "roi",
    "budget_variance",
    "risk_exposure",
    "gate_readiness",
    "adoption_rate",
    "value_realization_rate",
    "time_to_value",
];

use super::parse_pid as find_plan_pid;

/// `404` unless the caller can read the plan a control belongs to. Controls, readings and actions are reached by their own
/// pids, outside the plan guard, so every lookup here passes this, whether or not the control has been removed.
async fn require_visible(ctx: &AppContext, caller: &MaybeAuthUser, plan: Uuid) -> Result<()> {
    let readable = super::plan_access::readable_plans(ctx, caller, &[plan]).await;
    if readable.contains(&plan) {
        Ok(())
    } else {
        Err(Error::NotFound)
    }
}

/// Load one live (not soft-deleted) control by its public id, if the caller can read its plan; a malformed or unknown id is
/// `404`/bad-request, never a leak of internal keys.
async fn find_control(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    raw: &str,
) -> Result<controls::Model> {
    let pid = find_plan_pid(raw)?;
    let control: controls::Model =
        sqlx::query_as("SELECT * FROM controls WHERE pid = $1 AND deleted_at IS NULL LIMIT 1")
            .bind(pid)
            .fetch_optional(crate::db::pool(ctx))
            .await?
            .ok_or(Error::NotFound)?;
    require_visible(ctx, caller, control.plan_pid).await?;
    Ok(control)
}

/// Load one reading by public id, if the caller can read its control's plan. Readings are append-only history and have no
/// soft-delete column, so there is no `deleted_at` filter.
async fn find_reading(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    raw: &str,
) -> Result<control_readings::Model> {
    let pid = find_plan_pid(raw)?;
    let reading: control_readings::Model =
        sqlx::query_as("SELECT * FROM control_readings WHERE pid = $1 LIMIT 1")
            .bind(pid)
            .fetch_optional(crate::db::pool(ctx))
            .await?
            .ok_or(Error::NotFound)?;
    let control: controls::Model = sqlx::query_as("SELECT * FROM controls WHERE pid = $1 LIMIT 1")
        .bind(reading.control_pid)
        .fetch_optional(crate::db::pool(ctx))
        .await?
        .ok_or(Error::NotFound)?;
    require_visible(ctx, caller, control.plan_pid).await?;
    Ok(reading)
}

/// Load one live (not soft-deleted) control action by public id, if the caller can read its plan.
async fn find_action(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    raw: &str,
) -> Result<control_actions::Model> {
    let pid = find_plan_pid(raw)?;
    let action: control_actions::Model = sqlx::query_as(
        "SELECT * FROM control_actions WHERE pid = $1 AND deleted_at IS NULL LIMIT 1",
    )
    .bind(pid)
    .fetch_optional(crate::db::pool(ctx))
    .await?
    .ok_or(Error::NotFound)?;
    find_reading(ctx, caller, &action.reading_pid.to_string()).await?;
    Ok(action)
}

/// Parse a wire token into a [`rules::Timing`].
fn parse_timing(raw: &str) -> Option<rules::Timing> {
    match raw {
        "feedforward" => Some(rules::Timing::Feedforward),
        "concurrent" => Some(rules::Timing::Concurrent),
        "feedback" => Some(rules::Timing::Feedback),
        _ => None,
    }
}

/// The wire token for a [`rules::Timing`].
const fn timing_token(timing: rules::Timing) -> &'static str {
    match timing {
        rules::Timing::Feedforward => "feedforward",
        rules::Timing::Concurrent => "concurrent",
        rules::Timing::Feedback => "feedback",
    }
}

/// Parse a wire comparator token; `None` means the token is not one of
/// the four supported comparators and the caller must reject it.
fn parse_comparator(raw: &str) -> Option<rules::Comparator> {
    match raw {
        "at_least" => Some(rules::Comparator::AtLeast),
        "at_most" => Some(rules::Comparator::AtMost),
        "within" => Some(rules::Comparator::Within),
        "equals" => Some(rules::Comparator::Equals),
        _ => None,
    }
}

/// The wire token for a [`rules::Verdict`], the inverse of
/// [`parse_verdict`].
const fn verdict_token(verdict: rules::Verdict) -> &'static str {
    match verdict {
        rules::Verdict::Pass => "pass",
        rules::Verdict::Fail => "fail",
        rules::Verdict::Unmeasured => "unmeasured",
    }
}

/// Parse a stored verdict token back into a [`rules::Verdict`]. Total by
/// design: unknown input is `Unmeasured`, never `Pass`.
fn parse_verdict(raw: &str) -> rules::Verdict {
    match raw {
        "pass" => rules::Verdict::Pass,
        "fail" => rules::Verdict::Fail,
        // Anything unrecognized reads as unmeasured rather than as a
        // pass: a typo must never become a green control.
        _ => rules::Verdict::Unmeasured,
    }
}

/// Rebuild the pure standard from a stored row.
fn standard_of(row: &controls::Model) -> rules::Standard {
    rules::Standard {
        metric: row.metric.clone(),
        target_value: row.target_value,
        comparator: parse_comparator(&row.comparator).unwrap_or(rules::Comparator::Equals),
        tolerance: row.tolerance,
    }
}

/// `POST /api/plans/{pid}/controls` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ControlPayload {
    /// Human-readable control name, trimmed on insert and capped at
    /// `MAX_TEXT_LEN`.
    name: String,
    /// Wire token for [`rules::Timing`]: `feedforward`, `concurrent` or
    /// `feedback`. Decides whether a failure may block or only record.
    timing: String,
    /// Metric the control measures; must be one of [`KNOWN_METRICS`].
    metric: String,
    /// Target the metric is compared against, in the metric's own integer
    /// unit (for example basis points or days).
    target_value: i64,
    /// Wire token for the comparison: `at_least`, `at_most`, `within` or
    /// `equals`.
    comparator: String,
    #[serde(default)]
    /// Allowed deviation from `target_value` for `within`; absent when the
    /// comparator needs none.
    tolerance: Option<i64>,
    #[serde(default)]
    /// Display unit of the metric (for example `bps`, `days`); informational
    /// only and absent when unknown.
    unit: Option<String>,
    #[serde(default)]
    /// Currency code when the metric is monetary; absent otherwise.
    currency: Option<String>,
    #[serde(default = "default_source_kind")]
    /// Where readings come from: `metric`, `query` or `manual`. Defaults to
    /// `metric`.
    source_kind: String,
    #[serde(default)]
    /// Reference to the query or external source behind `source_kind`;
    /// absent for plain metrics and manual controls.
    source_ref: Option<String>,
    #[serde(default)]
    /// Expected days between readings; absent means no cadence is promised.
    /// Must be positive when present.
    cadence_days: Option<i64>,
    #[serde(default)]
    /// Reference (not a copy) to the person accountable for the control.
    owner_ref: Option<String>,
}

/// Serde default for [`ControlPayload::source_kind`].
fn default_source_kind() -> String {
    "metric".to_string()
}

/// Why a control payload could not be registered — the same checks
/// `create` and `register_standard` both need, factored once so the
/// two entry points cannot drift.
enum RegisterError {
    /// The timing token was not `feedforward`, `concurrent` or `feedback`.
    BadTiming,
    /// The comparator token was not a supported one.
    BadComparator,
    /// The source kind was not `metric`, `query` or `manual`.
    BadSourceKind,
    /// The name exceeded `MAX_TEXT_LEN`.
    NameTooLong,
    /// `cadence_days` was zero or negative.
    BadCadence,
    /// The standard failed [`rules::validate`] (for example an unknown
    /// metric); carries every problem found so the caller can fix them at once.
    NotEvaluable(Vec<rules::Invalid>),
    /// The row failed to insert — a real `500`, not a `422`: everything
    /// the caller controls already passed [`rules::validate`].
    Db(sqlx::Error),
}

/// Map a [`RegisterError`] to the HTTP error: caller mistakes are `422`,
/// a failed insert is a `500`.
fn register_error(err: RegisterError) -> Error {
    match err {
        RegisterError::Db(e) => Error::from(e),
        RegisterError::BadTiming => {
            unprocessable("timing must be feedforward, concurrent, or feedback")
        }
        RegisterError::BadComparator => {
            unprocessable("comparator must be at_least, at_most, within, or equals")
        }
        RegisterError::BadSourceKind => {
            unprocessable("source_kind must be metric, query, or manual")
        }
        RegisterError::NameTooLong => unprocessable("name is capped"),
        RegisterError::BadCadence => unprocessable("cadence_days must be positive when present"),
        RegisterError::NotEvaluable(problems) => unprocessable(&format!(
            "control is not evaluable: {}",
            serde_json::to_string(&problems).unwrap_or_default()
        )),
    }
}

/// Validate `payload` and insert the `controls` row (no audit — the two
/// callers record different audit event names). Returns the inserted
/// row plus the parsed [`rules::Timing`] (needed for
/// `permitted_response`, and cheaper than re-parsing the stored token).
async fn register_control(
    tx: &mut crate::db::Tx,
    plan_pid: Uuid,
    payload: &ControlPayload,
) -> std::result::Result<(controls::Model, rules::Timing), RegisterError> {
    let timing = parse_timing(&payload.timing).ok_or(RegisterError::BadTiming)?;
    let comparator = parse_comparator(&payload.comparator).ok_or(RegisterError::BadComparator)?;
    if !["metric", "query", "manual"].contains(&payload.source_kind.as_str()) {
        return Err(RegisterError::BadSourceKind);
    }
    if payload.name.len() > MAX_TEXT_LEN {
        return Err(RegisterError::NameTooLong);
    }
    if payload.cadence_days.is_some_and(|d| d <= 0) {
        return Err(RegisterError::BadCadence);
    }

    let standard = rules::Standard {
        metric: payload.metric.clone(),
        target_value: payload.target_value,
        comparator,
        tolerance: payload.tolerance,
    };

    // The whole point of validating here rather than at read time: a
    // control that can never be evaluated must not be registerable.
    rules::validate(&payload.name, &standard, KNOWN_METRICS)
        .map_err(RegisterError::NotEvaluable)?;

    let row: controls::Model = sqlx::query_as(
        "INSERT INTO controls
             (pid, plan_pid, name, timing, metric, target_value, comparator, tolerance, unit, currency,
              source_kind, source_ref, cadence_days, owner_ref, enabled, deleted_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, true, NULL)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(plan_pid)
    .bind(payload.name.trim())
    .bind(timing_token(timing))
    .bind(&payload.metric)
    .bind(payload.target_value)
    .bind(&payload.comparator)
    .bind(payload.tolerance)
    .bind(&payload.unit)
    .bind(&payload.currency)
    .bind(&payload.source_kind)
    .bind(&payload.source_ref)
    .bind(payload.cadence_days)
    .bind(&payload.owner_ref)
    .fetch_one(tx.conn())
    .await
    .map_err(RegisterError::Db)?;

    Ok((row, timing))
}

/// `POST /api/plans/{pid}/controls` — register a control.
#[debug_handler]
async fn create(
    Path(pid): Path<String>,
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(payload): Json<ControlPayload>,
) -> Result<Response> {
    let plan = find_plan(&ctx, &pid).await?;
    let mut tx = crate::db::begin(&ctx).await?;
    let (row, timing) = register_control(&mut tx, plan.pid, &payload)
        .await
        .map_err(register_error)?;

    AuditModel::record_on(
        tx.conn(),
        plan.pid,
        "control_registered",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(serde_json::json!({
        "pid": row.pid.to_string(),
        "permitted_response": rules::permitted_response(timing),
    }))
}

/// `POST /api/plans/{pid}/controls/register-standard` body — PRO-P33
/// (repo `tasks.md`).
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct RegisterStandardPayload {
    /// Required, no default: a WIP limit is plan/team-specific, and
    /// this service has no notion of a universal "reasonable" one —
    /// registering an invented number would be exactly the risk
    /// PRO-P33 warns against, so the caller must supply their own.
    work_in_progress_limit: i64,
    /// Required, no default, for the identical reason as
    /// `work_in_progress_limit`: `src/tba.rs`'s own
    /// `service_level_expectation` takes its calendar-day target as a
    /// caller-supplied parameter rather than a crate-wide constant,
    /// because a cycle-time SLE is a per-plan commitment, not a fact
    /// about the software.
    cycle_time_p85_days: i64,
    /// Optional — unlike the two above, "flag when actual spend departs
    /// budget by more than N%" has a defensible universal default (10%,
    /// in basis points, `Comparator::Within` a target of zero), so this
    /// one may default rather than forcing every caller to supply it.
    #[serde(default = "default_budget_variance_tolerance_bps")]
    budget_variance_tolerance_bps: i64,
}

/// Default tolerance for the budget-variance control: 1,000 basis points
/// (10%).
fn default_budget_variance_tolerance_bps() -> i64 {
    1_000
}

/// The four controls this service can already evaluate (their metrics
/// are in [`KNOWN_METRICS`]) but nothing registers on a plan's behalf.
/// `gate_readiness` gets a fixed target (100%, i.e. fully ready — the
/// one non-arbitrary bar for a readiness gate regardless of how
/// readiness ends up being computed); `budget_variance` gets `payload`'s
/// (defaulted) tolerance; the WIP limit and cycle-time SLE come from
/// `payload` with no default, per its own doc comments.
fn standard_control_payloads(payload: &RegisterStandardPayload) -> [ControlPayload; 4] {
    let no_extra = || ControlPayload {
        name: String::new(),
        timing: String::new(),
        metric: String::new(),
        target_value: 0,
        comparator: String::new(),
        tolerance: None,
        unit: None,
        currency: None,
        source_kind: default_source_kind(),
        source_ref: None,
        cadence_days: None,
        owner_ref: None,
    };
    [
        ControlPayload {
            name: "Gate readiness".to_string(),
            timing: "feedforward".to_string(),
            metric: "gate_readiness".to_string(),
            target_value: 10_000,
            comparator: "at_least".to_string(),
            unit: Some("bps".to_string()),
            ..no_extra()
        },
        ControlPayload {
            name: "Work in progress limit".to_string(),
            timing: "concurrent".to_string(),
            metric: "work_in_progress".to_string(),
            target_value: payload.work_in_progress_limit,
            comparator: "at_most".to_string(),
            unit: Some("items".to_string()),
            ..no_extra()
        },
        ControlPayload {
            name: "Cycle time SLE (p85)".to_string(),
            timing: "concurrent".to_string(),
            metric: "cycle_time_p85".to_string(),
            target_value: payload.cycle_time_p85_days,
            comparator: "at_most".to_string(),
            unit: Some("days".to_string()),
            ..no_extra()
        },
        ControlPayload {
            name: "Budget variance".to_string(),
            timing: "feedback".to_string(),
            metric: "budget_variance".to_string(),
            target_value: 0,
            comparator: "within".to_string(),
            tolerance: Some(payload.budget_variance_tolerance_bps),
            unit: Some("bps".to_string()),
            ..no_extra()
        },
    ]
}

/// `POST /api/plans/{pid}/controls/register-standard` — PRO-P33 (repo
/// `tasks.md`): the four controls whose metrics this service already
/// produces names for (`gate_readiness`, `work_in_progress`,
/// `cycle_time_p85`, `budget_variance` — `KNOWN_METRICS` already
/// contains all four, and `rules::validate` already accepts them) but
/// which nothing registers on a plan's behalf.
///
/// **Opt-in, per plan, never automatic.** A feedforward control's
/// entire design intent (`rules::may_block`) is to be able to refuse a
/// write once something enforces it; silently registering one on every
/// existing plan would be an unrequested behavioural change the moment
/// that enforcement lands, not merely a coverage-report change today.
/// This endpoint exists so an operator can opt a plan in explicitly,
/// with the two genuinely plan-specific numbers (`work_in_progress_limit`,
/// `cycle_time_p85_days`) supplied rather than invented — see
/// [`RegisterStandardPayload`]'s field docs.
///
/// **Retrospectives are deliberately not included.** There is no
/// `KNOWN_METRICS` entry for "a retrospective happened" — inventing one
/// here would be exactly the guessed-default risk this task exists to
/// avoid; it needs its own design (repo `tasks.md` PRO-P33).
///
/// Idempotent: a metric the plan already has an enabled, non-deleted
/// control for is reported `already_registered` rather than duplicated,
/// so calling this twice is harmless.
#[debug_handler]
async fn register_standard(
    Path(pid): Path<String>,
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(payload): Json<RegisterStandardPayload>,
) -> Result<Response> {
    let plan = find_plan(&ctx, &pid).await?;
    if payload.work_in_progress_limit <= 0 {
        return Err(unprocessable("work_in_progress_limit must be positive"));
    }
    if payload.cycle_time_p85_days <= 0 {
        return Err(unprocessable("cycle_time_p85_days must be positive"));
    }
    if payload.budget_variance_tolerance_bps < 0 {
        return Err(unprocessable(
            "budget_variance_tolerance_bps cannot be negative",
        ));
    }

    let mut tx = crate::db::begin(&ctx).await?;
    let mut outcomes = Vec::with_capacity(4);
    for candidate in &standard_control_payloads(&payload) {
        let existing = sqlx::query(
            "SELECT 1 FROM controls WHERE plan_pid = $1 AND metric = $2 AND deleted_at IS NULL LIMIT 1",
        )
        .bind(plan.pid)
        .bind(candidate.metric.as_str())
        .fetch_optional(tx.conn())
        .await?;
        if existing.is_some() {
            outcomes.push(serde_json::json!({
                "metric": candidate.metric,
                "status": "already_registered",
            }));
            continue;
        }
        let (row, _timing) = register_control(&mut tx, plan.pid, candidate)
            .await
            .map_err(register_error)?;
        outcomes.push(serde_json::json!({
            "metric": candidate.metric,
            "status": "registered",
            "pid": row.pid.to_string(),
        }));
    }

    AuditModel::record_on(
        tx.conn(),
        plan.pid,
        "standard_controls_registered",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;

    format::json(serde_json::json!({ "controls": outcomes }))
}

/// `GET /api/plans/{pid}/controls` — the register for one plan.
#[debug_handler]
async fn list(Path(pid): Path<String>, State(ctx): State<AppContext>) -> Result<Response> {
    let plan = find_plan(&ctx, &pid).await?;
    let rows: Vec<controls::Model> = sqlx::query_as(
        "SELECT * FROM controls WHERE plan_pid = $1 AND deleted_at IS NULL ORDER BY name",
    )
    .bind(plan.pid)
    .fetch_all(crate::db::pool(&ctx))
    .await?;
    format::json(serde_json::json!(rows))
}

/// `DELETE /api/controls/{pid}` — soft-delete a control. The readings
/// stay: they are the history of what was measured, and deleting a
/// control does not unmeasure the past.
#[debug_handler]
async fn remove(
    Path(pid): Path<String>,
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
) -> Result<Response> {
    let row = find_control(&ctx, &caller, &pid).await?;
    let plan_pid = row.plan_pid;
    let mut tx = crate::db::begin(&ctx).await?;
    sqlx::query("UPDATE controls SET deleted_at = $1 WHERE id = $2 RETURNING id")
        .bind(chrono::Utc::now().fixed_offset())
        .bind(row.id)
        .fetch_one(tx.conn())
        .await?;
    AuditModel::record_on(
        tx.conn(),
        plan_pid,
        "control_withdrawn",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::empty_json()
}

/// `POST /api/controls/{pid}/readings` body. `value` absent is
/// `unmeasured`, which is deliberate and is not an error.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ReadingPayload {
    #[serde(default)]
    /// Observed value in the metric's unit. Absent records an `unmeasured`
    /// reading, which is valid and is never coerced to zero.
    value: Option<i64>,
    #[serde(default = "default_method")]
    /// How the value was obtained (for example `manual`); defaults to
    /// `manual`.
    method: String,
}

/// Serde default for [`ReadingPayload::method`].
fn default_method() -> String {
    "manual".to_string()
}

/// `POST /api/controls/{pid}/readings` — measure and compare, in one
/// step, because a reading whose verdict is computed later can disagree
/// with the standard that was in force when it was taken.
#[debug_handler]
async fn add_reading(
    Path(pid): Path<String>,
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(payload): Json<ReadingPayload>,
) -> Result<Response> {
    let control = find_control(&ctx, &caller, &pid).await?;
    let (verdict, gap) = rules::compare(&standard_of(&control), payload.value);

    let mut tx = crate::db::begin(&ctx).await?;
    let row_pid: Uuid = sqlx::query_scalar(
        "INSERT INTO control_readings
             (pid, control_pid, value, verdict, gap, method, accepted_at, accepted_reason)
         VALUES ($1, $2, $3, $4, $5, $6, NULL, NULL)
         RETURNING pid",
    )
    .bind(Uuid::new_v4())
    .bind(control.pid)
    .bind(payload.value)
    .bind(verdict_token(verdict))
    .bind(gap)
    .bind(&payload.method)
    .fetch_one(tx.conn())
    .await?;

    AuditModel::record_on(
        tx.conn(),
        control.plan_pid,
        "control_reading_recorded",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;

    format::json(serde_json::json!({
        "pid": row_pid.to_string(),
        "verdict": verdict_token(verdict),
        "gap": gap,
        "permitted_response": rules::permitted_response(
            parse_timing(&control.timing).unwrap_or(rules::Timing::Feedback),
        ),
    }))
}

/// `GET /api/controls/{pid}/readings` — newest first.
#[debug_handler]
async fn list_readings(
    Path(pid): Path<String>,
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
) -> Result<Response> {
    let control = find_control(&ctx, &caller, &pid).await?;
    let rows: Vec<control_readings::Model> = sqlx::query_as(
        "SELECT * FROM control_readings WHERE control_pid = $1 ORDER BY observed_at DESC",
    )
    .bind(control.pid)
    .fetch_all(crate::db::pool(&ctx))
    .await?;
    format::json(serde_json::json!(rows))
}

/// `POST /api/readings/{pid}/actions` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ActionPayload {
    /// Action kind: `correct`, `adjust`, `retrain`, `accept` or `escalate`.
    kind: String,
    /// What was done or decided; required, non-blank and capped.
    description: String,
    #[serde(default)]
    /// Reference to the person who owns the action.
    owner_ref: Option<String>,
    #[serde(default)]
    /// Why the action was taken; used as the acceptance reason when `kind` is
    /// `accept`.
    reason: Option<String>,
}

/// `POST /api/readings/{pid}/actions` — record what a failing reading
/// provoked. `accept` also stamps the reading as explicitly accepted,
/// which is what stops it being reported as **unanswered**.
#[debug_handler]
async fn add_action(
    Path(pid): Path<String>,
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(payload): Json<ActionPayload>,
) -> Result<Response> {
    let reading = find_reading(&ctx, &caller, &pid).await?;
    if !["correct", "adjust", "retrain", "accept", "escalate"].contains(&payload.kind.as_str()) {
        return Err(unprocessable(
            "kind must be correct, adjust, retrain, accept, or escalate",
        ));
    }
    if payload.description.trim().is_empty() || payload.description.len() > MAX_TEXT_LEN {
        return Err(unprocessable("description is required (and capped)"));
    }

    let mut tx = crate::db::begin(&ctx).await?;
    let row_pid: Uuid = sqlx::query_scalar(
        "INSERT INTO control_actions
             (pid, reading_pid, kind, description, owner_ref, due_date, converted_task_pid,
              converted_issue_pid, closed_at, outcome, deleted_at)
         VALUES ($1, $2, $3, $4, $5, NULL, NULL, NULL, NULL, NULL, NULL)
         RETURNING pid",
    )
    .bind(Uuid::new_v4())
    .bind(reading.pid)
    .bind(&payload.kind)
    .bind(payload.description.trim())
    .bind(&payload.owner_ref)
    .fetch_one(tx.conn())
    .await?;

    if payload.kind == "accept" {
        sqlx::query(
            "UPDATE control_readings SET accepted_at = $1, accepted_reason = $2 WHERE id = $3 RETURNING id",
        )
        .bind(chrono::Utc::now().fixed_offset())
        .bind(&payload.reason)
        .bind(reading.id)
        .fetch_one(tx.conn())
        .await?;
    }

    AuditModel::record_on(
        tx.conn(),
        reading.control_pid,
        "control_action_recorded",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(serde_json::json!({ "pid": row_pid.to_string() }))
}

/// `POST /api/actions/{pid}/convert` query.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ConvertParams {
    /// `task` (default) or `issue`.
    #[serde(default)]
    to: Option<String>,
}

/// The issue half of [`convert_action`]: raise the issue and mark the action in one transaction.
async fn convert_to_issue(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    action: &control_actions::Model,
    control: &controls::Model,
) -> Result<Response> {
    let today = crate::tz::deployment_today();
    let title: String = action
        .description
        .trim()
        .chars()
        .take(crate::issues::TITLE_MAX)
        .collect();
    let workflow = super::workflow::in_force_issue(ctx, control.plan_pid, "concern").await?;
    let (status, category) = crate::workflow::initial_state(&workflow).map_or_else(
        || ("open".to_string(), "todo".to_string()),
        |s| (s.key.clone(), s.category.token().to_string()),
    );
    let moment = chrono::Utc::now();
    let times = crate::issues::times_after(
        crate::issues::Times::default(),
        None,
        crate::issues::Standing {
            category: &category,
            terminal: false,
        },
        moment,
    );
    let mut tx = crate::db::begin(ctx).await?;
    let issue_pid: Uuid = sqlx::query_scalar(
        "INSERT INTO issues
             (pid, plan_pid, title, description, kind, severity, priority, status, status_category,
              owner_ref, raised_by, raised_on, due_date, resolution, resolved_on, reported_at,
              acknowledged_at, source_risk_pid, source_task_pid, change_request_pid, deleted_at)
         VALUES ($1, $2, $3, $4, 'concern', 'med', NULL, $5, $6, $7, $8, $9, $10, NULL, NULL, $11, $12,
                 NULL, NULL, NULL, NULL)
         RETURNING pid",
    )
    .bind(Uuid::new_v4())
    .bind(control.plan_pid)
    .bind(title)
    .bind(format!(
        "Converted from a {} action on control \"{}\".\n\n{}",
        action.kind, control.name, action.description
    ))
    .bind(status)
    .bind(category)
    .bind(&action.owner_ref)
    .bind(caller.actor())
    .bind(today)
    .bind(action.due_date)
    .bind(moment.fixed_offset())
    .bind(times.acknowledged_at.map(|t| t.fixed_offset()))
    .fetch_one(tx.conn())
    .await?;
    sqlx::query("UPDATE control_actions SET converted_issue_pid = $1 WHERE id = $2 RETURNING id")
        .bind(issue_pid)
        .bind(action.id)
        .fetch_one(tx.conn())
        .await?;
    AuditModel::record_on(
        tx.conn(),
        control.pid,
        "control_action_converted",
        caller.actor(),
        Some(serde_json::json!({
            "action_pid": action.pid.to_string(),
            "issue_pid": issue_pid.to_string(),
        })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;

    format::json(serde_json::json!({
        "pid": action.pid.to_string(),
        "issue_pid": issue_pid.to_string(),
        "plan_pid": control.plan_pid.to_string(),
    }))
}

/// `POST /api/actions/{pid}/convert` — turn a control action into a
/// task on the control's own plan (T-26: actions convert into the work
/// stores that already exist rather than becoming a fifth one — see
/// `control_actions`' migration doc comment). Created into the
/// **initial** state of the workflow in force for that plan, exactly
/// like an ordinary task create (`engineering::create_task`), so it
/// enters the board and the flow analysis identically to a
/// hand-created one.
///
/// `?to=issue` converts it into an issue on the same plan instead (the
/// issues register, FR-14): kind `concern`, severity `med`, owner and due
/// date from the action; the action records `converted_issue_pid`. The
/// default, `?to=task`, is unchanged. Either way an action converts once.
#[debug_handler]
async fn convert_action(
    Path(pid): Path<String>,
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(params): Query<ConvertParams>,
) -> Result<Response> {
    let to_issue = match params.to.as_deref() {
        None | Some("task") => false,
        Some("issue") => true,
        Some(_) => return Err(unprocessable("to must be task or issue")),
    };
    let action = find_action(&ctx, &caller, &pid).await?;
    if action.converted_task_pid.is_some() || action.converted_issue_pid.is_some() {
        return Err(unprocessable("action is already converted"));
    }
    if action.closed_at.is_some() {
        return Err(unprocessable("action is already closed"));
    }
    let reading: control_readings::Model =
        sqlx::query_as("SELECT * FROM control_readings WHERE pid = $1 LIMIT 1")
            .bind(action.reading_pid)
            .fetch_optional(crate::db::pool(&ctx))
            .await?
            .ok_or(Error::NotFound)?;
    let control: controls::Model =
        sqlx::query_as("SELECT * FROM controls WHERE pid = $1 AND deleted_at IS NULL LIMIT 1")
            .bind(reading.control_pid)
            .fetch_optional(crate::db::pool(&ctx))
            .await?
            .ok_or(Error::NotFound)?;

    if to_issue {
        let standing = super::plan_roles::standing(&ctx, &caller, control.plan_pid).await?;
        super::plan_roles::require(&standing, &[crate::plan_roles::ISSUE_ADD])?;
        return convert_to_issue(&ctx, &caller, &action, &control).await;
    }

    // The vocabulary comes from the workflow in force for the control's
    // plan, exactly like `engineering::create_task` — a plan with a
    // custom task workflow gets the converted task in *its* initial
    // state, not a hardcoded "todo".
    let workflow = super::workflow::in_force(&ctx, control.plan_pid, "task").await?;
    let initial = workflow
        .states
        .iter()
        .find(|state| state.is_initial)
        .map_or("todo", |state| state.key.as_str());

    let now = chrono::Utc::now().fixed_offset();
    // The task and its opening transition commit together with the
    // action's own update, same reasoning as task creation (spec
    // `time-based-analysis.md` §5.1 invariant 3): a converted task
    // without a transition would silently begin its life at its first
    // later move, and an action left unmarked could be converted twice.
    let mut tx = crate::db::begin(&ctx).await?;
    let task_pid: Uuid = sqlx::query_scalar(
        "INSERT INTO tasks
             (pid, plan_pid, sprint_pid, title, description, status, assignee_ref, points,
              status_changed_at, flow_type, done_at, deleted_at)
         VALUES ($1, $2, NULL, $3, $4, $5, $6, NULL, $7, NULL, NULL, NULL)
         RETURNING pid",
    )
    .bind(Uuid::new_v4())
    .bind(control.plan_pid)
    .bind(&action.description)
    .bind(format!(
        "Converted from a {} action on control \"{}\".",
        action.kind, control.name
    ))
    .bind(initial)
    .bind(&action.owner_ref)
    .bind(now)
    .fetch_one(tx.conn())
    .await?;
    // The task's creation in the transition log, as `crate::tba::transition_row` writes it.
    sqlx::query(
        "INSERT INTO task_transitions
             (pid, task_pid, plan_pid, from_status, to_status, at, actor_ref, assignee_ref, backfilled)
         VALUES ($1, $2, $3, NULL, $4, $5, $6, $7, false)",
    )
    .bind(Uuid::new_v4())
    .bind(task_pid)
    .bind(control.plan_pid)
    .bind(initial)
    .bind(now)
    .bind(caller.actor())
    .bind(&action.owner_ref)
    .execute(tx.conn())
    .await?;
    sqlx::query("UPDATE control_actions SET converted_task_pid = $1 WHERE id = $2 RETURNING id")
        .bind(task_pid)
        .bind(action.id)
        .fetch_one(tx.conn())
        .await?;

    AuditModel::record_on(
        tx.conn(),
        control.pid,
        "control_action_converted",
        caller.actor(),
        Some(serde_json::json!({
            "action_pid": action.pid.to_string(),
            "task_pid": task_pid.to_string(),
        })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;

    format::json(serde_json::json!({
        "pid": action.pid.to_string(),
        "task_pid": task_pid.to_string(),
    }))
}

/// Build the coverage facts for a set of controls.
async fn coverage_of(
    ctx: &AppContext,
    control_rows: Vec<controls::Model>,
) -> Result<rules::Coverage> {
    let now = chrono::Utc::now();
    let mut facts = Vec::with_capacity(control_rows.len());

    for control in control_rows {
        let readings: Vec<control_readings::Model> = sqlx::query_as(
            "SELECT * FROM control_readings WHERE control_pid = $1 ORDER BY observed_at DESC",
        )
        .bind(control.pid)
        .fetch_all(crate::db::pool(ctx))
        .await?;

        let mut reading_facts = Vec::with_capacity(readings.len());
        for reading in readings {
            let answered = reading.accepted_at.is_some()
                || sqlx::query(
                    "SELECT 1 FROM control_actions WHERE reading_pid = $1 AND deleted_at IS NULL LIMIT 1",
                )
                .bind(reading.pid)
                .fetch_optional(crate::db::pool(ctx))
                .await?
                .is_some();
            reading_facts.push(rules::ReadingFact {
                age_days: (now - reading.observed_at.with_timezone(&chrono::Utc))
                    .num_days()
                    .max(0),
                verdict: parse_verdict(&reading.verdict),
                answered,
            });
        }

        facts.push(rules::ControlFact {
            timing: parse_timing(&control.timing).unwrap_or(rules::Timing::Feedback),
            cadence_days: control.cadence_days,
            readings: reading_facts,
            enabled: control.enabled,
        });
    }

    Ok(rules::coverage(&facts))
}

/// `GET /api/plans/{pid}/controls/coverage` — what is **not** being
/// controlled on one plan.
#[debug_handler]
async fn plan_coverage(
    Path(pid): Path<String>,
    State(ctx): State<AppContext>,
    headers: HeaderMap,
) -> Result<Response> {
    let plan = find_plan(&ctx, &pid).await?;
    let rows: Vec<controls::Model> =
        sqlx::query_as("SELECT * FROM controls WHERE plan_pid = $1 AND deleted_at IS NULL")
            .bind(plan.pid)
            .fetch_all(crate::db::pool(&ctx))
            .await?;
    let body = serde_json::json!({
        "plan_pid": plan.pid.to_string(),
        "coverage": coverage_of(&ctx, rows).await?,
        "as_of": chrono::Utc::now(),
    });
    let etag = super::etag_of(&body);
    super::conditional_json(&headers, &etag, &body)
}

/// `GET /api/controls/coverage` — the portfolio-wide answer to the same
/// question. Plans with **no controls at all** are named, because an
/// empty cell is the finding a register exists to surface.
#[debug_handler]
async fn portfolio_coverage(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    headers: HeaderMap,
) -> Result<Response> {
    let scope = super::plan_access::HiddenScope::of(&ctx, &caller).await?;
    let rows = scope.keep(
        sqlx::query_as::<_, controls::Model>("SELECT * FROM controls WHERE deleted_at IS NULL")
            .fetch_all(crate::db::pool(&ctx))
            .await?,
    );

    let controlled: std::collections::HashSet<Uuid> =
        rows.iter().map(|control| control.plan_pid).collect();
    let uncontrolled: Vec<String> = scope
        .keep(
            sqlx::query_as::<_, plans::Model>("SELECT * FROM plans WHERE deleted_at IS NULL")
                .fetch_all(crate::db::pool(&ctx))
                .await?,
        )
        .into_iter()
        .filter(|plan| !controlled.contains(&plan.pid))
        .map(|plan| plan.pid.to_string())
        .collect();

    let body = serde_json::json!({
        "coverage": coverage_of(&ctx, rows).await?,
        "plans_with_no_controls": uncontrolled,
        "as_of": chrono::Utc::now(),
    });
    let etag = super::etag_of(&body);
    super::conditional_json(&headers, &etag, &body)
}

/// The control routes (entity spec §9.2c).
#[must_use]
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/controls", post(create))
        .add(
            "/plans/{pid}/controls/register-standard",
            post(register_standard),
        )
        .add("/plans/{pid}/controls", get(list))
        .add("/plans/{pid}/controls/coverage", get(plan_coverage))
        .add("/controls/coverage", get(portfolio_coverage))
        .add("/controls/{pid}", delete(remove))
        .add("/controls/{pid}/readings", post(add_reading))
        .add("/controls/{pid}/readings", get(list_readings))
        .add("/readings/{pid}/actions", post(add_action))
        .add("/actions/{pid}/convert", post(convert_action))
}
