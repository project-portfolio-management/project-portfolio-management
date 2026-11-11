//! PPM Phase-A governance controllers (spec/15-roadmap PPM-1/3/10/12):
//! the `proposals` work-intake pipeline, per-plan phase-gate
//! reviews, risks, and budget lines, plus the per-item governance
//! summary. Every mutation writes an audit row; the pure rules live in
//! [`crate::governance`]; nothing here ever feeds the matcher (the §8
//! partition rule).

use crate::prelude::*;
use axa::error::ErrorDetail;
use axum::http::StatusCode;
use project_portfolio_management_matcher::{MatchConfig, MatchingEngine, Plan};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::plans::parse_kind_label;
use crate::auth::{self, MaybeAuthUser};
use crate::governance as rules;
use crate::models::_entities::{budget_lines, gate_reviews, plans, proposals, risks};
use crate::models::audit_logs::Model as AuditModel;
use crate::models::governance as gov;
use crate::models::plans::Model as PlanModel;
use crate::streaming;
use crate::validation::MAX_TEXT_LEN;

/// `422` with a joined problem list (family convention).
fn unprocessable(problems: &[String]) -> Error {
    Error::CustomError(
        StatusCode::UNPROCESSABLE_ENTITY,
        ErrorDetail::new("validation", problems.join("; ")),
    )
}

/// One-problem `422` for state-machine refusals.
fn refuse(message: &str) -> Error {
    Error::CustomError(
        StatusCode::UNPROCESSABLE_ENTITY,
        ErrorDetail::new("unprocessable", message),
    )
}

/// A permitted sponsor/approver/owner reference: an EntityRef-shaped
/// URN (`person:` / `worker:` / `organization:` + UUID). Format check
/// only — resolution is the reader's concern.
pub(crate) fn valid_ref(value: &str, schemes: &[&str]) -> bool {
    schemes.iter().any(|scheme| {
        value
            .strip_prefix(scheme)
            .and_then(|rest| rest.strip_prefix(':'))
            .is_some_and(|id| Uuid::parse_str(id).is_ok())
    })
}

/// Pushes a "too long" problem onto `problems` when `value` is longer than
/// [`MAX_TEXT_LEN`] bytes; `field` names the offending input in the message.
fn cap(problems: &mut Vec<String>, field: &str, value: &str) {
    if value.len() > MAX_TEXT_LEN {
        problems.push(format!("{field} exceeds {MAX_TEXT_LEN} characters"));
    }
}

/// Optional-text variant of [`cap`]: an absent value passes, a present one is length-checked.
fn cap_opt(problems: &mut Vec<String>, field: &str, value: Option<&str>) {
    if let Some(v) = value {
        cap(problems, field, v);
    }
}

/// The current time, as the timestamp columns store it.
fn now() -> chrono::DateTime<chrono::FixedOffset> {
    chrono::Utc::now().into()
}

/// A live (not soft-deleted) row of one table by public id, or `404`. The table is a literal.
macro_rules! find_live {
    ($ctx:expr, $table:literal, $pid:expr) => {
        sqlx::query_as(concat!(
            "SELECT * FROM ",
            $table,
            " WHERE pid = $1 AND deleted_at IS NULL LIMIT 1"
        ))
        .bind($pid)
        .fetch_optional(crate::db::pool($ctx))
        .await?
        .ok_or(Error::NotFound)
    };
}

/// A live proposal by public id, or `404`.
async fn find_proposal(ctx: &AppContext, pid: Uuid) -> Result<proposals::Model> {
    find_live!(ctx, "proposals", pid)
}

/// A live risk by public id, or `404`.
async fn find_risk(ctx: &AppContext, pid: Uuid) -> Result<risks::Model> {
    find_live!(ctx, "risks", pid)
}

/// A live budget line by public id, or `404`.
async fn find_budget_line(ctx: &AppContext, pid: Uuid) -> Result<budget_lines::Model> {
    find_live!(ctx, "budget_lines", pid)
}

/// A plan's gate reviews, oldest first.
async fn gate_reviews_for(ctx: &AppContext, plan_pid: Uuid) -> Result<Vec<gate_reviews::Model>> {
    Ok(
        sqlx::query_as("SELECT * FROM gate_reviews WHERE plan_pid = $1 ORDER BY id")
            .bind(plan_pid)
            .fetch_all(crate::db::pool(ctx))
            .await?,
    )
}

/// A plan's live risks, oldest first.
async fn risks_for(ctx: &AppContext, plan_pid: Uuid) -> Result<Vec<risks::Model>> {
    Ok(
        sqlx::query_as(
            "SELECT * FROM risks WHERE plan_pid = $1 AND deleted_at IS NULL ORDER BY id",
        )
        .bind(plan_pid)
        .fetch_all(crate::db::pool(ctx))
        .await?,
    )
}

/// A plan's live budget lines, oldest first.
async fn budget_lines_for(ctx: &AppContext, plan_pid: Uuid) -> Result<Vec<budget_lines::Model>> {
    Ok(sqlx::query_as(
        "SELECT * FROM budget_lines WHERE plan_pid = $1 AND deleted_at IS NULL ORDER BY id",
    )
    .bind(plan_pid)
    .fetch_all(crate::db::pool(ctx))
    .await?)
}

/// `POST /api/proposals` body (also the draft-update shape).
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ProposalPayload {
    /// Proposal title; required (non-blank) and capped at `MAX_TEXT_LEN`.
    title: String,
    /// Free-text summary of the demand; absent means none was given.
    #[serde(default)]
    summary: Option<String>,
    /// The collection an approved proposal becomes (`portfolios` /
    /// `projects` / `products` / `programs`).
    kind_target: String,
    /// Sponsor reference as a `person:` / `worker:` / `organization:` URN; absent means no sponsor named yet.
    #[serde(default)]
    sponsor_ref: Option<String>,
    /// Why the demand matters strategically; absent means not stated.
    #[serde(default)]
    strategic_rationale: Option<String>,
    /// Requested funding in minor currency units; when present a valid `currency` must accompany it. Absent means no amount was asked for (not zero).
    #[serde(default)]
    requested_minor: Option<i64>,
    /// ISO 4217 currency code for `requested_minor`; ignored unless an amount is given.
    #[serde(default)]
    currency: Option<String>,
    /// An external ask from the sponsor (EX-13).
    #[serde(default)]
    external_ask: bool,
    /// What the ask displaces; needed to approve an external ask.
    #[serde(default)]
    tradeoff: Option<String>,
}

/// Collects every validation problem for a proposal body (blank title, length caps, trade-off length, kind label, sponsor URN, currency with amount) so the caller gets one `422` listing them all.
fn validate_proposal(p: &ProposalPayload) -> Vec<String> {
    let mut problems = Vec::new();
    if p.title.trim().is_empty() {
        problems.push("title is required".to_string());
    }
    cap(&mut problems, "title", &p.title);
    cap_opt(&mut problems, "summary", p.summary.as_deref());
    if p.tradeoff
        .as_deref()
        .is_some_and(|t| t.chars().count() > 2000)
    {
        problems.push("tradeoff must be at most 2000 characters".to_string());
    }
    cap_opt(
        &mut problems,
        "strategic_rationale",
        p.strategic_rationale.as_deref(),
    );
    // `kind_target` is now an optional descriptive label for the plan a
    // promoted proposal mints. Blank means "no kind"; a non-blank value
    // must be a recognized kind label.
    if !p.kind_target.trim().is_empty() && parse_kind_label(&p.kind_target).is_none() {
        problems.push(format!(
            "kind_target, when set, must be one of portfolio/project/product/program/practice/process/purpose/pathway/proposal, got {:?}",
            p.kind_target
        ));
    }
    if let Some(sponsor) = p.sponsor_ref.as_deref()
        && !valid_ref(sponsor, &["person", "worker", "organization"])
    {
        problems.push("sponsor_ref must be a person:/worker:/organization: URN".to_string());
    }
    if p.requested_minor.is_some() {
        match p.currency.as_deref() {
            Some(code) if rules::valid_currency(code) => {}
            _ => problems
                .push("currency (ISO 4217, e.g. GBP) is required with requested_minor".to_string()),
        }
    }
    problems
}

/// `POST /api/proposals` — open a demand record (status `draft`).
#[debug_handler]
async fn create_proposal(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Json(payload): Json<ProposalPayload>,
) -> Result<Response> {
    let problems = validate_proposal(&payload);
    if !problems.is_empty() {
        return Err(unprocessable(&problems));
    }
    let mut tx = crate::db::begin(&ctx).await?;
    let (row_pid, status): (Uuid, String) = sqlx::query_as(
        "INSERT INTO proposals
             (pid, title, summary, kind_target, sponsor_ref, strategic_rationale, requested_minor, currency, status,
              promoted_plan_pid, deleted_at, external_ask, tradeoff)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 'draft', NULL, NULL, $9, $10)
         RETURNING pid, status",
    )
    .bind(Uuid::new_v4())
    .bind(payload.title.as_str())
    .bind(payload.summary.as_deref())
    .bind(payload.kind_target.as_str())
    .bind(payload.sponsor_ref.as_deref())
    .bind(payload.strategic_rationale.as_deref())
    .bind(payload.requested_minor)
    .bind(payload.currency.as_deref())
    .bind(payload.external_ask)
    .bind(payload.tradeoff.as_deref().filter(|t| !t.trim().is_empty()))
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(tx.conn(), row_pid, "proposal_created", caller.actor(), None)
        .await
        .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(serde_json::json!({ "pid": row_pid.to_string(), "status": status }))
}

/// `GET /api/proposals?status=` — the intake board (default: every
/// non-promoted, most recent first, capped 200).
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ListParams {
    /// Optional status filter; absent means every proposal that has not yet been promoted.
    #[serde(default)]
    status: Option<String>,
}

/// `GET /api/proposals` handler: lists non-deleted proposals, newest first, capped at 200 rows. Read-only, so nothing is audited.
#[debug_handler]
async fn list_proposals(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(params): Query<ListParams>,
) -> Result<Response> {
    let rows: Vec<proposals::Model> = if let Some(status) = &params.status {
        sqlx::query_as(
            "SELECT * FROM proposals WHERE deleted_at IS NULL AND status = $1 ORDER BY id DESC",
        )
        .bind(status.as_str())
        .fetch_all(crate::db::pool(&ctx))
        .await?
    } else {
        sqlx::query_as(
            "SELECT * FROM proposals WHERE deleted_at IS NULL AND status <> 'promoted' ORDER BY id DESC",
        )
        .fetch_all(crate::db::pool(&ctx))
        .await?
    };
    let rows = super::plan_access::HiddenScope::of(&ctx, &caller)
        .await?
        .keep(rows);
    format::json(rows.into_iter().take(200).collect::<Vec<_>>())
}

/// A proposal the caller may read: one that became a plan is that plan's, so it is `404` to whoever cannot read the plan.
pub(super) async fn readable_proposal(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    pid: &str,
) -> Result<proposals::Model> {
    let row = find_proposal(ctx, gov::parse_pid(pid)?).await?;
    super::plan_access::require_readable_if_any(ctx, caller, row.promoted_plan_pid).await?;
    Ok(row)
}

/// `GET /api/proposals/{pid}`.
#[debug_handler]
async fn get_proposal(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    format::json(readable_proposal(&ctx, &caller, &pid).await?)
}

/// `PUT /api/proposals/{pid}` — edit; drafts only (the pipeline owns
/// everything after submission).
#[debug_handler]
async fn update_proposal(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(payload): Json<ProposalPayload>,
) -> Result<Response> {
    let problems = validate_proposal(&payload);
    if !problems.is_empty() {
        return Err(unprocessable(&problems));
    }
    let row = readable_proposal(&ctx, &caller, &pid).await?;
    if !rules::proposal_editable(&row.status) {
        return Err(refuse(&format!(
            "a {} proposal is no longer editable",
            row.status
        )));
    }
    // Edit only what the payload carries; the audit row is written in the same
    // transaction so a failed audit rolls the edit back.
    let row_pid = row.pid;
    let mut tx = crate::db::begin(&ctx).await?;
    // `fetch_one`: an update that finds no row is an error, as SeaORM's was.
    let row: proposals::Model = sqlx::query_as(
        "UPDATE proposals
         SET title = $1, summary = $2, kind_target = $3, sponsor_ref = $4, strategic_rationale = $5,
             requested_minor = $6, currency = $7, external_ask = $8, tradeoff = $9
         WHERE id = $10
         RETURNING *",
    )
    .bind(payload.title.as_str())
    .bind(payload.summary.as_deref())
    .bind(payload.kind_target.as_str())
    .bind(payload.sponsor_ref.as_deref())
    .bind(payload.strategic_rationale.as_deref())
    .bind(payload.requested_minor)
    .bind(payload.currency.as_deref())
    .bind(payload.external_ask)
    .bind(payload.tradeoff.as_deref().filter(|t| !t.trim().is_empty()))
    .bind(row.id)
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(tx.conn(), row_pid, "proposal_updated", caller.actor(), None)
        .await
        .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(row)
}

/// The pack's section: external asks raised in the period and the trade-off each names (EX-13). Proposals
/// carry no plan, so this is portfolio-wide; it holds titles and trade-offs, no person or amount.
///
/// # Errors
///
/// When a database read fails.
/// An ask already promoted to a plan the pack leaves out (private or marked) is left out with it
/// (CF-5): its title would otherwise name the hidden plan's work. An ask with no plan yet is
/// portfolio-wide and stays.
pub(crate) async fn asks_section(
    ctx: &AppContext,
    period: &str,
    included: &std::collections::HashSet<Uuid>,
) -> Result<serde_json::Value> {
    let rows: Vec<proposals::Model> =
        sqlx::query_as("SELECT * FROM proposals WHERE external_ask = true AND deleted_at IS NULL")
            .fetch_all(crate::db::pool(ctx))
            .await?;
    let mut asks: Vec<serde_json::Value> = rows
        .iter()
        .filter(|p| p.created_at.format("%Y-%m").to_string() == period)
        .filter(|p| {
            p.promoted_plan_pid
                .is_none_or(|plan| included.contains(&plan))
        })
        .map(|p| {
            serde_json::json!({
                "title": p.title, "status": p.status, "tradeoff": p.tradeoff,
                "named_tradeoff": p.tradeoff.as_deref().is_some_and(|t| !t.trim().is_empty()),
            })
        })
        .collect();
    asks.sort_by_key(|a| a["title"].as_str().unwrap_or("").to_string());
    let without = asks.iter().filter(|a| a["named_tradeoff"] == false).count();
    Ok(serde_json::json!({
        "period": period, "asks": asks, "without_named_tradeoff": without,
        "note": "External asks from the sponsor raised in the period, with what each displaces. An ask cannot be approved without a named trade-off; one still in draft or review may not have one yet.",
    }))
}

/// `PUT /api/proposals/{pid}/tradeoff` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct TradeoffBody {
    /// What the ask displaces (10 to 2000 characters).
    tradeoff: String,
}

/// `PUT /api/proposals/{pid}/tradeoff` — name what an external ask displaces (EX-13). Allowed until the
/// proposal is decided, so a reviewer can record it without reopening the draft. Audited.
#[debug_handler]
async fn set_tradeoff(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<TradeoffBody>,
) -> Result<Response> {
    let row = readable_proposal(&ctx, &caller, &pid).await?;
    if !row.external_ask {
        return Err(refuse(
            "only an external ask has a trade-off: tag the proposal as one first",
        ));
    }
    if !["draft", "submitted", "in_review"].contains(&row.status.as_str()) {
        return Err(refuse(&format!(
            "a {} proposal's trade-off can no longer be changed",
            row.status
        )));
    }
    let text = b.tradeoff.trim().to_string();
    if !(10..=2000).contains(&text.chars().count()) {
        return Err(refuse("tradeoff must be 10 to 2000 characters"));
    }
    let row_pid = row.pid;
    let mut tx = crate::db::begin(&ctx).await?;
    let row: proposals::Model =
        sqlx::query_as("UPDATE proposals SET tradeoff = $1 WHERE id = $2 RETURNING *")
            .bind(text)
            .bind(row.id)
            .fetch_one(tx.conn())
            .await?;
    AuditModel::record_on(
        tx.conn(),
        row_pid,
        "proposal_tradeoff_set",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(row)
}

/// Run one pipeline action and audit it.
async fn transition_proposal(
    ctx: &AppContext,
    caller: &MaybeAuthUser,
    pid: &str,
    action: rules::ProposalAction,
) -> Result<proposals::Model> {
    let row = readable_proposal(ctx, caller, pid).await?;
    let next = rules::proposal_transition(&row.status, action).map_err(|e| refuse(&e))?;
    // A new ask from the sponsor must name what it displaces before it is approved (EX-13).
    if matches!(action, rules::ProposalAction::Approve)
        && row.external_ask
        && row
            .tradeoff
            .as_deref()
            .is_none_or(|t| t.trim().chars().count() < 10)
    {
        return Err(refuse(
            "an external ask needs a named trade-off before approval: set tradeoff to what it displaces (10 or more characters)",
        ));
    }
    let row_pid = row.pid;
    let mut tx = crate::db::begin(ctx).await?;
    let row: proposals::Model =
        sqlx::query_as("UPDATE proposals SET status = $1 WHERE id = $2 RETURNING *")
            .bind(next)
            .bind(row.id)
            .fetch_one(tx.conn())
            .await?;
    AuditModel::record_on(tx.conn(), row_pid, action.token(), caller.actor(), None)
        .await
        .map_err(super::audit_failed)?;
    tx.commit().await?;
    Ok(row)
}

/// Generates a thin handler for a pipeline transition (submit, review, approve, reject). Each expansion delegates to `transition_proposal`, which applies the state machine and writes the audit row.
macro_rules! pipeline_handler {
    ($fn_name:ident, $action:expr) => {
        #[debug_handler]
        async fn $fn_name(
            State(ctx): State<AppContext>,
            caller: MaybeAuthUser,
            Path(pid): Path<String>,
        ) -> Result<Response> {
            format::json(transition_proposal(&ctx, &caller, &pid, $action).await?)
        }
    };
}

pipeline_handler!(submit_proposal, rules::ProposalAction::Submit);
pipeline_handler!(review_proposal, rules::ProposalAction::Review);
pipeline_handler!(approve_proposal, rules::ProposalAction::Approve);
pipeline_handler!(reject_proposal, rules::ProposalAction::Reject);

/// `POST /api/proposals/{pid}/promote` — mint the plan from an
/// approved proposal (`provenance` recorded in the audit trail) and
/// mark the proposal `promoted`.
#[debug_handler]
async fn promote_proposal(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let row = readable_proposal(&ctx, &caller, &pid).await?;
    let next = rules::proposal_transition(&row.status, rules::ProposalAction::Promote)
        .map_err(|e| refuse(&e))?;
    let mut plan = Plan {
        kind: parse_kind_label(&row.kind_target),
        name: row.title.clone(),
        ..Plan::default()
    };
    if let Some(summary) = &row.summary {
        plan.keywords = vec![summary.clone()];
    }
    let model = streaming::create_and_emit(&ctx.db, &plan, caller.actor()).await?;
    let proposal_pid = row.pid;
    let mut tx = crate::db::begin(&ctx).await?;
    let updated: proposals::Model = sqlx::query_as(
        "UPDATE proposals SET status = $1, promoted_plan_pid = $2 WHERE id = $3 RETURNING *",
    )
    .bind(next)
    .bind(model.pid)
    .bind(row.id)
    .fetch_one(tx.conn())
    .await?;
    let snapshot = serde_json::json!({
        "plan_pid": model.pid.to_string(),
        "collection": updated.kind_target,
        "provenance": "intake",
    });
    AuditModel::record_on(
        tx.conn(),
        proposal_pid,
        rules::ProposalAction::Promote.token(),
        caller.actor(),
        Some(snapshot),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(serde_json::json!({
        "pid": updated.pid.to_string(),
        "status": updated.status,
        "plan_pid": model.pid.to_string(),
        "collection": updated.kind_target,
    }))
}

/// `GET /api/proposals/forecast` query parameters — the same shape as
/// `GET /plans/{pid}/forecast`'s (T-28i reuses its Monte-Carlo).
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct IntakeForecastQuery {
    /// How many periods ahead to forecast an approved count for.
    #[serde(default)]
    periods: Option<usize>,
    /// Periods of history to sample (default 12).
    #[serde(default)]
    history_periods: Option<usize>,
    /// Calendar days per period (default 7).
    #[serde(default)]
    period_days: Option<i64>,
    /// Simulation trials (default 10 000, capped).
    #[serde(default)]
    trials: Option<usize>,
    /// Seed, so a caller can vary the draw deliberately.
    #[serde(default)]
    seed: Option<u64>,
}

/// `GET /api/proposals/forecast` — "how many approved proposals in
/// the next N periods?" (T-28i), reusing the exact throughput
/// Monte-Carlo behind `GET /plans/{pid}/forecast`
/// ([`crate::tba::forecast_items`]) over the **intake pipeline**
/// instead of a plan's task board: the history it samples is
/// `proposal_approved` events per period, read from the audit trail
/// (`proposals` itself carries no `approved_at` column — the audit
/// row's own timestamp is the only record of when a proposal actually
/// became approved). `arrivals_per_period` (from `proposals.created_at`
/// directly) rides alongside for context; it does not feed the
/// forecast, which answers the *approved* question, not the
/// *submitted* one.
#[debug_handler]
async fn intake_forecast(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(query): Query<IntakeForecastQuery>,
) -> Result<Response> {
    let scope = super::plan_access::HiddenScope::of(&ctx, &caller).await?;
    let period_days = query.period_days.unwrap_or(crate::tba::DEFAULT_PERIOD_DAYS);
    if period_days <= 0 || period_days > 90 {
        return Err(refuse("period_days must be between 1 and 90"));
    }
    let history_periods = query.history_periods.unwrap_or(12);
    if history_periods == 0 || history_periods > 260 {
        return Err(refuse("history_periods must be between 1 and 260"));
    }
    let trials = query.trials.unwrap_or(crate::tba::DEFAULT_TRIALS);
    if trials > crate::tba::MAX_TRIALS {
        return Err(refuse(&format!(
            "trials must be at most {}",
            crate::tba::MAX_TRIALS
        )));
    }

    let now = chrono::Utc::now();
    let period_ms = period_days.saturating_mul(crate::tba::DAY_MS);
    let (from_ms, to_ms) =
        crate::tba::history_window(now.timestamp_millis(), history_periods, period_ms);

    // A proposal that became a hidden plan is not part of the intake the caller may see.
    let visible = scope.keep(
        sqlx::query_as::<_, proposals::Model>("SELECT * FROM proposals")
            .fetch_all(crate::db::pool(&ctx))
            .await?,
    );
    let visible_pids: std::collections::HashSet<Uuid> = visible.iter().map(|p| p.pid).collect();
    let approved: Vec<(Uuid, chrono::DateTime<chrono::FixedOffset>)> =
        sqlx::query_as("SELECT entity_pid, created_at FROM audit_logs WHERE action = $1")
            .bind(rules::ProposalAction::Approve.token())
            .fetch_all(crate::db::pool(&ctx))
            .await?;
    let approved_ms: Vec<i64> = approved
        .iter()
        .filter(|(entity_pid, _)| visible_pids.contains(entity_pid))
        .map(|(_, created_at)| created_at.timestamp_millis())
        .collect();
    let arrival_ms: Vec<i64> = visible
        .iter()
        .map(|row| row.created_at.timestamp_millis())
        .collect();

    let approvals_history = crate::tba::throughput_history(&approved_ms, from_ms, to_ms, period_ms);
    let arrivals_history = crate::tba::throughput_history(&arrival_ms, from_ms, to_ms, period_ms);

    let periods = query.periods.unwrap_or(4);
    let seed = query.seed.unwrap_or(0x5EED_5EED_5EED_5EED);

    format::json(serde_json::json!({
        "as_of": now,
        "note": "forecast samples the intake pipeline's own approval throughput \
                 — how many proposals actually got approved per period — the \
                 same Monte-Carlo GET /plans/{pid}/forecast runs over a task \
                 board's completions. arrivals_per_period is contextual only \
                 and does not feed the forecast.",
        "history_window": {
            "from": chrono::DateTime::from_timestamp_millis(from_ms),
            "to": chrono::DateTime::from_timestamp_millis(to_ms),
            "periods": history_periods,
            "period_days": period_days,
        },
        "arrivals_per_period": arrivals_history,
        "approvals_per_period": approvals_history,
        "forecast": crate::tba::forecast_items(&approvals_history, periods, trials, period_days, seed),
    }))
}

/// A scored duplicate-demand hit.
#[derive(Debug, Serialize)]
struct DemandHit {
    /// Kind of record matched: `"plan"` or `"proposal"`.
    source: &'static str,
    /// Public identifier (pid) of the matched record, as a string.
    pid: String,
    /// Display name of the matched record.
    name: String,
    /// Similarity score from the matcher; higher is closer. Advisory only: the caller decides what to do.
    score: f64,
}

/// `GET /api/proposals/{pid}/duplicates` — duplicate-demand check
/// (the registry heritage applied at intake): match the proposal's
/// title against the live plans of its target collection **and**
/// the other open proposals. A plan the caller cannot read is not a
/// hit: a title must not be a way of finding one.
#[debug_handler]
async fn proposal_duplicates(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let row = readable_proposal(&ctx, &caller, &pid).await?;
    let engine = MatchingEngine::new(MatchConfig::default());
    let query = Plan {
        kind: parse_kind_label(&row.kind_target),
        name: row.title.clone(),
        ..Plan::default()
    };
    let mut hits: Vec<DemandHit> = Vec::new();
    // Live plans (one recursive collection).
    let items = PlanModel::list(&ctx.db, super::plans::CHECK_DUPLICATES_SCAN_CAP).await?;
    let items = super::plan_access::HiddenScope::of(&ctx, &caller)
        .await?
        .keep(items);
    for item in &items {
        let candidate = item.to_plan()?;
        let result = engine.match_plans(&query, &candidate);
        if result.is_match {
            hits.push(DemandHit {
                source: "plan",
                pid: item.pid.to_string(),
                name: item.name.clone(),
                score: result.score,
            });
        }
    }
    // Other open proposals targeting the same collection.
    let siblings: Vec<proposals::Model> = sqlx::query_as(
        "SELECT * FROM proposals
         WHERE deleted_at IS NULL AND kind_target = $1 AND status <> 'promoted' AND pid <> $2",
    )
    .bind(row.kind_target.as_str())
    .bind(row.pid)
    .fetch_all(crate::db::pool(&ctx))
    .await?;
    for sibling in &siblings {
        let candidate = Plan {
            kind: parse_kind_label(&sibling.kind_target),
            name: sibling.title.clone(),
            ..Plan::default()
        };
        let result = engine.match_plans(&query, &candidate);
        if result.is_match {
            hits.push(DemandHit {
                source: "proposal",
                pid: sibling.pid.to_string(),
                name: sibling.title.clone(),
                score: result.score,
            });
        }
    }
    hits.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    format::json(hits)
}

/// Resolve `plans/{pid}` to the stored plan (404 when unknown).
pub(crate) async fn find_item(ctx: &AppContext, pid: &str) -> Result<plans::Model> {
    // An id that is not a UUID names nothing: `404`.
    let pid = gov::parse_pid(pid)?;
    find_live!(ctx, "plans", pid)
}

/// `POST /api/plans/{pid}/gate-reviews` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct GateReviewPayload {
    /// Gate being decided; must be the next gate in order, checked by `rules::apply_gate_review`.
    gate: String,
    /// The gate decision token; only an approving decision advances the stage.
    decision: String,
    /// Conditions attached to the decision, if any; absent means unconditional.
    #[serde(default)]
    conditions: Option<String>,
    /// Approver as a `worker:` / `person:` URN; absent means the decision is recorded without a named approver.
    #[serde(default)]
    approver_ref: Option<String>,
}

/// `POST /api/plans/{pid}/gate-reviews` — record a phase-gate
/// decision; an approving decision advances the item's `stage`
/// (strictly in gate order). Record-level ABAC applies
/// (`resource.stage`), so gate-locking is policy.
#[debug_handler]
async fn create_gate_review(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(payload): Json<GateReviewPayload>,
) -> Result<Response> {
    let mut problems = Vec::new();
    cap_opt(&mut problems, "conditions", payload.conditions.as_deref());
    if let Some(approver) = payload.approver_ref.as_deref()
        && !valid_ref(approver, &["worker", "person"])
    {
        problems.push("approver_ref must be a worker:/person: URN".to_string());
    }
    if !problems.is_empty() {
        return Err(unprocessable(&problems));
    }
    let item = find_item(&ctx, &pid).await?;
    auth::authorize_record(
        &caller,
        authentication_verifier::Action::Write,
        &auth::plan_resource_attrs(item.stage.as_deref()),
    )
    .map_err(super::record_rejection)?;
    // Pure rule: an approving decision is only valid for the next gate in order,
    // so a stage can never skip ahead.
    let new_stage =
        rules::apply_gate_review(item.stage.as_deref(), &payload.gate, &payload.decision)
            .map_err(|e| refuse(&e))?;
    let mut tx = crate::db::begin(&ctx).await?;
    let (review_pid, review_gate, review_decision): (Uuid, String, String) = sqlx::query_as(
        "INSERT INTO gate_reviews (pid, plan_pid, gate, decision, conditions, approver_ref, decided_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7)
         RETURNING pid, gate, decision",
    )
    .bind(Uuid::new_v4())
    .bind(item.pid)
    .bind(payload.gate.as_str())
    .bind(payload.decision.as_str())
    .bind(payload.conditions.as_deref())
    .bind(payload.approver_ref.as_deref())
    .bind(now())
    .fetch_one(tx.conn())
    .await?;
    let item_pid = item.pid;
    let old_stage = item.stage.clone();
    // `fetch_one`: an update that finds no row is an error, as SeaORM's was.
    sqlx::query_scalar::<_, i64>("UPDATE plans SET stage = $1 WHERE id = $2 RETURNING id")
        .bind(new_stage.as_deref())
        .bind(item.id)
        .fetch_one(tx.conn())
        .await?;
    let snapshot = serde_json::json!({
        "gate": payload.gate,
        "decision": payload.decision,
        "from_stage": old_stage,
        "to_stage": new_stage,
        "approver_ref": payload.approver_ref,
    });
    AuditModel::record_on(
        tx.conn(),
        item_pid,
        "gate_reviewed",
        caller.actor(),
        Some(snapshot),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(serde_json::json!({
        "pid": review_pid.to_string(),
        "gate": review_gate,
        "decision": review_decision,
        "stage": new_stage,
    }))
}

/// `GET /api/plans/{pid}/gate-reviews`.
#[debug_handler]
async fn list_gate_reviews(
    State(ctx): State<AppContext>,
    Path(pid): Path<String>,
) -> Result<Response> {
    let item = find_item(&ctx, &pid).await?;
    let reviews = gate_reviews_for(&ctx, item.pid).await?;
    format::json(serde_json::json!({
        "stage": item.stage,
        "next_gate": rules::next_gate(item.stage.as_deref()),
        "reviews": reviews,
    }))
}

/// `POST /api/plans/{pid}/risks` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct RiskPayload {
    /// Risk title; required (non-blank) and length-capped.
    title: String,
    /// Longer description of the risk; absent means none given.
    #[serde(default)]
    description: Option<String>,
    /// Likelihood score; validated together with `impact` by `rules::risk_exposure`.
    probability: i32,
    /// Impact score; exposure is `probability * impact`.
    impact: i32,
    /// Risk owner as a `worker:` / `person:` URN; absent means unowned.
    #[serde(default)]
    owner_ref: Option<String>,
    /// Planned mitigation text; absent means none recorded.
    #[serde(default)]
    mitigation: Option<String>,
    /// Date by which the risk should next be reviewed; absent means no review scheduled.
    #[serde(default)]
    review_date: Option<chrono::NaiveDate>,
    /// Optional category (`delivery` / `tech_debt` / `compliance` /
    /// `security` / `other`); absent reads as `delivery`.
    #[serde(default)]
    category: Option<String>,
}

/// `POST /api/plans/{pid}/risks` — raise a risk (status `open`).
#[debug_handler]
async fn create_risk(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(payload): Json<RiskPayload>,
) -> Result<Response> {
    let mut problems = Vec::new();
    if payload.title.trim().is_empty() {
        problems.push("title is required".to_string());
    }
    cap(&mut problems, "title", &payload.title);
    cap_opt(&mut problems, "description", payload.description.as_deref());
    cap_opt(&mut problems, "mitigation", payload.mitigation.as_deref());
    if let Err(e) = rules::risk_exposure(payload.probability, payload.impact) {
        problems.push(e);
    }
    if let Some(owner) = payload.owner_ref.as_deref()
        && !valid_ref(owner, &["worker", "person"])
    {
        problems.push("owner_ref must be a worker:/person: URN".to_string());
    }
    if let Some(category) = payload.category.as_deref()
        && !rules::is_token(rules::RISK_CATEGORIES, category)
    {
        problems.push(format!(
            "category must be one of {:?}",
            rules::RISK_CATEGORIES
        ));
    }
    if !problems.is_empty() {
        return Err(unprocessable(&problems));
    }
    let item = find_item(&ctx, &pid).await?;
    let mut tx = crate::db::begin(&ctx).await?;
    let row_pid: Uuid = sqlx::query_scalar(
        "INSERT INTO risks
             (pid, plan_pid, title, description, probability, impact, status, owner_ref, mitigation, review_date,
              escalated_at, category, deleted_at)
         VALUES ($1, $2, $3, $4, $5, $6, 'open', $7, $8, $9, NULL, $10, NULL)
         RETURNING pid",
    )
    .bind(Uuid::new_v4())
    .bind(item.pid)
    .bind(payload.title.as_str())
    .bind(payload.description.as_deref())
    .bind(payload.probability)
    .bind(payload.impact)
    .bind(payload.owner_ref.as_deref())
    .bind(payload.mitigation.as_deref())
    .bind(payload.review_date)
    .bind(payload.category.as_deref())
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(tx.conn(), row_pid, "risk_raised", caller.actor(), None)
        .await
        .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(serde_json::json!({ "pid": row_pid.to_string() }))
}

/// `GET /api/plans/{pid}/risks` — active risks with derived
/// exposure, highest first.
#[debug_handler]
async fn list_risks(State(ctx): State<AppContext>, Path(pid): Path<String>) -> Result<Response> {
    let item = find_item(&ctx, &pid).await?;
    let mut rows = risks_for(&ctx, item.pid).await?;
    // Highest exposure first; negated because sort_by_key is ascending.
    rows.sort_by_key(|r| -(r.probability * r.impact));
    let views: Vec<_> = rows
        .into_iter()
        .map(|r| {
            let exposure = r.probability * r.impact;
            serde_json::json!({
                "pid": r.pid.to_string(), "title": r.title, "status": r.status,
                "probability": r.probability, "impact": r.impact, "exposure": exposure,
                "owner_ref": r.owner_ref, "mitigation": r.mitigation,
                "review_date": r.review_date, "escalated_at": r.escalated_at,
                // Simulation inputs (schedule-risk, cost-risk): the event's
                // probability is shared by both.
                "task_pid": r.task_pid, "schedule_impact_days": r.schedule_impact_days,
                "probability_percent": r.schedule_probability_percent,
                "cost_impact_minor": r.cost_impact_minor, "cost_currency": r.cost_currency,
            })
        })
        .collect();
    format::json(views)
}

/// `PUT /api/plans/{pid}/risks/{risk_pid}` — update scoring /
/// status / mitigation.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct RiskUpdate {
    /// New status; must be one of `rules::RISK_STATUSES`. Absent leaves the status unchanged.
    #[serde(default)]
    status: Option<String>,
    /// New likelihood score; absent keeps the stored value.
    #[serde(default)]
    probability: Option<i32>,
    /// New impact score; absent keeps the stored value.
    #[serde(default)]
    impact: Option<i32>,
    /// Replacement mitigation text; absent leaves it unchanged.
    #[serde(default)]
    mitigation: Option<String>,
    /// New review date; absent leaves it unchanged.
    #[serde(default)]
    review_date: Option<chrono::NaiveDate>,
}

/// `PUT /api/plans/{pid}/risks/{risk_pid}` handler: validates the changed fields (the exposure is re-checked against the merged scores), refuses a risk that belongs to another plan with `404`, and audits `risk_updated`.
#[debug_handler]
async fn update_risk(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, risk_pid)): Path<(String, String)>,
    Json(payload): Json<RiskUpdate>,
) -> Result<Response> {
    let item = find_item(&ctx, &pid).await?;
    let risk = find_risk(&ctx, gov::parse_pid(&risk_pid)?).await?;
    if risk.plan_pid != item.pid {
        return Err(Error::NotFound);
    }
    let mut problems = Vec::new();
    if let Some(status) = payload.status.as_deref()
        && !rules::is_token(rules::RISK_STATUSES, status)
    {
        problems.push(format!("status must be one of {:?}", rules::RISK_STATUSES));
    }
    let probability = payload.probability.unwrap_or(risk.probability);
    let impact = payload.impact.unwrap_or(risk.impact);
    if let Err(e) = rules::risk_exposure(probability, impact) {
        problems.push(e);
    }
    cap_opt(&mut problems, "mitigation", payload.mitigation.as_deref());
    if !problems.is_empty() {
        return Err(unprocessable(&problems));
    }
    let risk_row_pid = risk.pid;
    // The update names only what changes: the scores always, the rest when the body carries them.
    let mut update = sqlx::QueryBuilder::<sqlx::Postgres>::new("UPDATE risks SET probability = ");
    update.push_bind(probability);
    update.push(", impact = ").push_bind(impact);
    if let Some(status) = payload.status {
        update.push(", status = ").push_bind(status);
    }
    if let Some(mitigation) = payload.mitigation {
        update.push(", mitigation = ").push_bind(mitigation);
    }
    if let Some(review_date) = payload.review_date {
        update.push(", review_date = ").push_bind(review_date);
    }
    update
        .push(" WHERE id = ")
        .push_bind(risk.id)
        .push(" RETURNING *");
    let mut tx = crate::db::begin(&ctx).await?;
    let row: risks::Model = update.build_query_as().fetch_one(tx.conn()).await?;
    AuditModel::record_on(
        tx.conn(),
        risk_row_pid,
        "risk_updated",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(row)
}

/// `POST /api/plans/{pid}/risks/{risk_pid}/escalate` — a
/// materialized risk: status → `materialized`, stamped. (Conversion
/// into a tracked issue arrives with the issues sub-resource.)
#[debug_handler]
async fn escalate_risk(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, risk_pid)): Path<(String, String)>,
) -> Result<Response> {
    let item = find_item(&ctx, &pid).await?;
    let risk = find_risk(&ctx, gov::parse_pid(&risk_pid)?).await?;
    if risk.plan_pid != item.pid {
        return Err(Error::NotFound);
    }
    if !matches!(risk.status.as_str(), "open" | "mitigating") {
        return Err(refuse(&format!("cannot escalate a {} risk", risk.status)));
    }
    let risk_row_pid = risk.pid;
    let escalated_at = now();
    let mut tx = crate::db::begin(&ctx).await?;
    let row: risks::Model = sqlx::query_as(
        "UPDATE risks SET status = 'materialized', escalated_at = $1 WHERE id = $2 RETURNING *",
    )
    .bind(escalated_at)
    .bind(risk.id)
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        risk_row_pid,
        "risk_escalated",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(row)
}

/// `POST /api/plans/{pid}/budget-lines` body.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct BudgetPayload {
    /// Budget category; must be one of `rules::BUDGET_CATEGORIES`.
    category: String,
    /// What the line pays for; required and length-capped.
    description: String,
    /// ISO 4217 currency code; a plan may hold lines in several currencies, totals are kept per currency.
    currency: String,
    /// Planned amount in minor currency units; must not be negative.
    planned_minor: i64,
    /// First day of the period the line covers; absent means open-ended.
    #[serde(default)]
    period_start: Option<chrono::NaiveDate>,
    /// Last day of the period; must not precede `period_start`. Absent means open-ended.
    #[serde(default)]
    period_end: Option<chrono::NaiveDate>,
    /// Optional funding gate: the line is HELD (no actuals) until the
    /// plan's stage reaches this gate and the line is released.
    #[serde(default)]
    gate: Option<String>,
    /// Where the money comes from (DP-6); optional, and a line with none is reported as such.
    #[serde(default)]
    funding_source_pid: Option<Uuid>,
}

/// `POST /api/plans/{pid}/budget-lines`.
#[debug_handler]
async fn create_budget_line(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(payload): Json<BudgetPayload>,
) -> Result<Response> {
    let mut problems = Vec::new();
    if !rules::is_token(rules::BUDGET_CATEGORIES, &payload.category) {
        problems.push(format!(
            "category must be one of {:?}",
            rules::BUDGET_CATEGORIES
        ));
    }
    if payload.description.trim().is_empty() {
        problems.push("description is required".to_string());
    }
    cap(&mut problems, "description", &payload.description);
    if !rules::valid_currency(&payload.currency) {
        problems.push("currency must be an ISO 4217 code (e.g. GBP)".to_string());
    }
    if payload.planned_minor < 0 {
        problems.push("planned_minor must be non-negative".to_string());
    }
    if let (Some(start), Some(end)) = (payload.period_start, payload.period_end)
        && end < start
    {
        problems.push("period_end is before period_start".to_string());
    }
    if let Some(gate) = payload.gate.as_deref()
        && !rules::is_token(rules::GATES, gate)
    {
        problems.push(format!("gate must be one of {:?}", rules::GATES));
    }
    if !problems.is_empty() {
        return Err(unprocessable(&problems));
    }
    let item = find_item(&ctx, &pid).await?;
    if let Some(s) = payload.funding_source_pid {
        super::funding::check_source(&ctx, s).await?;
    }
    let mut tx = crate::db::begin(&ctx).await?;
    let row_pid: Uuid = sqlx::query_scalar(
        "INSERT INTO budget_lines
             (pid, plan_pid, category, description, currency, planned_minor, actual_minor, period_start, period_end,
              gate, released_at, funding_source_pid, deleted_at)
         VALUES ($1, $2, $3, $4, $5, $6, 0, $7, $8, $9, NULL, $10, NULL)
         RETURNING pid",
    )
    .bind(Uuid::new_v4())
    .bind(item.pid)
    .bind(payload.category.as_str())
    .bind(payload.description.as_str())
    .bind(payload.currency.as_str())
    .bind(payload.planned_minor)
    .bind(payload.period_start)
    .bind(payload.period_end)
    .bind(payload.gate.as_deref())
    .bind(payload.funding_source_pid)
    .fetch_one(tx.conn())
    .await?;
    AuditModel::record_on(
        tx.conn(),
        row_pid,
        "budget_line_created",
        caller.actor(),
        None,
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(serde_json::json!({ "pid": row_pid.to_string() }))
}

/// `GET /api/plans/{pid}/budget-lines` — lines + per-currency
/// totals (planned / actual / variance).
#[debug_handler]
async fn list_budget_lines(
    State(ctx): State<AppContext>,
    Path(pid): Path<String>,
) -> Result<Response> {
    let item = find_item(&ctx, &pid).await?;
    let rows = budget_lines_for(&ctx, item.pid).await?;
    format::json(serde_json::json!({
        "lines": rows,
        "totals": budget_totals(&rows),
    }))
}

/// Per-currency planned / actual / variance rollup.
fn budget_totals(rows: &[budget_lines::Model]) -> Vec<serde_json::Value> {
    let mut currencies: Vec<&str> = rows.iter().map(|r| r.currency.as_str()).collect();
    currencies.sort_unstable();
    currencies.dedup();
    currencies
        .into_iter()
        .map(|currency| {
            let planned: i64 = rows
                .iter()
                .filter(|r| r.currency == currency)
                .map(|r| r.planned_minor)
                .sum();
            let actual: i64 = rows
                .iter()
                .filter(|r| r.currency == currency)
                .map(|r| r.actual_minor)
                .sum();
            serde_json::json!({
                "currency": currency,
                "planned_minor": planned,
                "actual_minor": actual,
                "variance_minor": planned - actual,
            })
        })
        .collect()
}

/// `POST /api/plans/{pid}/budget-lines/{line_pid}/actual` —
/// record spend against a line (accumulates; negative adjustments
/// allowed; overflow refused).
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ActualPayload {
    /// Spend to add, in minor currency units; negative values are adjustments. Overflow is refused.
    amount_minor: i64,
    /// Optional free-text note, kept only in the audit snapshot.
    #[serde(default)]
    note: Option<String>,
}

/// `POST /api/plans/{pid}/budget-lines/{line_pid}/actual` handler: adds spend to a line. Refused while a gated line is still held, and audited as `budget_actual_recorded` with the new running total.
#[debug_handler]
async fn record_actual(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, line_pid)): Path<(String, String)>,
    Json(payload): Json<ActualPayload>,
) -> Result<Response> {
    let item = find_item(&ctx, &pid).await?;
    let line = find_budget_line(&ctx, gov::parse_pid(&line_pid)?).await?;
    if line.plan_pid != item.pid {
        return Err(Error::NotFound);
    }
    // A stage-gated tranche is HELD until released: no actuals may be
    // recorded against it (funding follows governance, not vice versa).
    if let Some(gate) = line.gate.as_deref()
        && line.released_at.is_none()
    {
        return Err(refuse(&format!(
            "budget line is held until gate `{gate}` passes and the line is released"
        )));
    }
    let next = rules::accumulate_actual(line.actual_minor, payload.amount_minor)
        .map_err(|e| refuse(&e))?;
    let line_pid = line.pid;
    let mut tx = crate::db::begin(&ctx).await?;
    let row: budget_lines::Model =
        sqlx::query_as("UPDATE budget_lines SET actual_minor = $1 WHERE id = $2 RETURNING *")
            .bind(next)
            .bind(line.id)
            .fetch_one(tx.conn())
            .await?;
    let snapshot = serde_json::json!({
        "amount_minor": payload.amount_minor,
        "actual_minor": next,
        "note": payload.note,
    });
    AuditModel::record_on(
        tx.conn(),
        line_pid,
        "budget_actual_recorded",
        caller.actor(),
        Some(snapshot),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(row)
}

/// `POST /api/plans/{pid}/budget-lines/{line_pid}/release` —
/// release a stage-gated funding tranche. Refused (`422`) when the
/// line has no gate, is already released, or the plan's stage has
/// not reached the gate (first the governance, then the money).
#[debug_handler]
async fn release_budget_line(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, line_pid)): Path<(String, String)>,
) -> Result<Response> {
    let item = find_item(&ctx, &pid).await?;
    let line = find_budget_line(&ctx, gov::parse_pid(&line_pid)?).await?;
    if line.plan_pid != item.pid {
        return Err(Error::NotFound);
    }
    let Some(gate) = line.gate.clone() else {
        return Err(refuse(
            "budget line has no funding gate; nothing to release",
        ));
    };
    if line.released_at.is_some() {
        return Err(refuse("budget line is already released"));
    }
    if !rules::gate_reached(item.stage.as_deref(), &gate) {
        return Err(refuse(&format!(
            "gate `{gate}` has not passed (current stage: {})",
            item.stage.as_deref().unwrap_or("pre_gate")
        )));
    }
    super::register::guard(&ctx, &caller, item.pid, "releasing a budget line").await?;
    let line_pid = line.pid;
    let released_at = now();
    let mut tx = crate::db::begin(&ctx).await?;
    let row: budget_lines::Model =
        sqlx::query_as("UPDATE budget_lines SET released_at = $1 WHERE id = $2 RETURNING *")
            .bind(released_at)
            .bind(line.id)
            .fetch_one(tx.conn())
            .await?;
    AuditModel::record_on(
        tx.conn(),
        line_pid,
        "budget_line_released",
        caller.actor(),
        Some(serde_json::json!({ "gate": gate })),
    )
    .await
    .map_err(super::audit_failed)?;
    tx.commit().await?;
    format::json(row)
}

/// `GET /api/plans/{pid}/governance` — the per-item summary:
/// stage + next gate, risk posture, and budget totals in one read.
#[debug_handler]
async fn governance_summary(
    State(ctx): State<AppContext>,
    Path(pid): Path<String>,
) -> Result<Response> {
    let item = find_item(&ctx, &pid).await?;
    let reviews = gate_reviews_for(&ctx, item.pid).await?;
    let risks = risks_for(&ctx, item.pid).await?;
    let budgets = budget_lines_for(&ctx, item.pid).await?;
    let open_risks: Vec<_> = risks
        .iter()
        .filter(|r| matches!(r.status.as_str(), "open" | "mitigating"))
        .collect();
    format::json(serde_json::json!({
        "pid": item.pid.to_string(),
        "name": item.name,
        "stage": item.stage,
        "next_gate": rules::next_gate(item.stage.as_deref()),
        "gate_reviews": reviews.len(),
        "latest_review": reviews.last().map(|r| serde_json::json!({
            "gate": r.gate, "decision": r.decision, "decided_at": r.decided_at,
        })),
        "risks": {
            "open": open_risks.len(),
            "materialized": risks.iter().filter(|r| r.status == "materialized").count(),
            "max_exposure": open_risks.iter().map(|r| r.probability * r.impact).max(),
            "total_exposure": open_risks.iter().map(|r| r.probability * r.impact).sum::<i32>(),
        },
        "budget": budget_totals(&budgets),
    }))
}

/// The governance routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/proposals", post(create_proposal))
        .add("/proposals", get(list_proposals))
        .add("/proposals/forecast", get(intake_forecast))
        .add("/proposals/{pid}", get(get_proposal))
        .add("/proposals/{pid}", put(update_proposal))
        .add("/proposals/{pid}/tradeoff", put(set_tradeoff))
        .add("/proposals/{pid}/submit", post(submit_proposal))
        .add("/proposals/{pid}/review", post(review_proposal))
        .add("/proposals/{pid}/approve", post(approve_proposal))
        .add("/proposals/{pid}/reject", post(reject_proposal))
        .add("/proposals/{pid}/promote", post(promote_proposal))
        .add("/proposals/{pid}/duplicates", get(proposal_duplicates))
        .add("/plans/{pid}/gate-reviews", post(create_gate_review))
        .add("/plans/{pid}/gate-reviews", get(list_gate_reviews))
        .add("/plans/{pid}/risks", post(create_risk))
        .add("/plans/{pid}/risks", get(list_risks))
        .add("/plans/{pid}/risks/{risk_pid}", put(update_risk))
        .add(
            "/plans/{pid}/risks/{risk_pid}/escalate",
            post(escalate_risk),
        )
        .add("/plans/{pid}/budget-lines", post(create_budget_line))
        .add("/plans/{pid}/budget-lines", get(list_budget_lines))
        .add(
            "/plans/{pid}/budget-lines/{line_pid}/actual",
            post(record_actual),
        )
        .add(
            "/plans/{pid}/budget-lines/{line_pid}/release",
            post(release_budget_line),
        )
        .add("/plans/{pid}/governance", get(governance_summary))
}
