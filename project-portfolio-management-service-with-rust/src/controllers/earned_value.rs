//! Earned value (SC-4): PV, EV, AC, SV, CV, SPI, CPI and earned-schedule SPI(t)
//! per currency, from the plan's phased budget baseline (T-28b), its tasks'
//! percent complete, and its recorded actual cost. Pure rules in
//! [`crate::earned_value`].
//!
//! **What it is built from, said plainly.** *Planned value* follows the budget
//! baseline (the latest version per currency). *Percent complete* is physical: each
//! task is counted 0/100, 0/50/100, or by a person-entered percent (always flagged,
//! always with a note), weighted equally or by story points. *Actual cost* is the
//! plan's recorded budget-line actuals in that currency; timesheets × a rate is not
//! offered because cost rates do not exist yet (PJ-2).
//!
//! **Measured as of today (UTC date).** Actuals carry no date and a task's
//! intermediate progress is not stored by date, so the past is not reconstructed;
//! `as_of` is always today and is stated in the response. (Period-by-period
//! history for an S-curve comes with the EVM analytics, SC-5.)
//!
//! **Currencies never merge.** One block per baseline currency; budget lines in a
//! currency with no baseline are listed, not added in.

use super::db_err;
use axum::http::{StatusCode, header};
use std::fmt::Write as _;

use axum::response::IntoResponse;
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use sea_orm::{QueryOrder, QuerySelect};
use serde::Deserialize;
use uuid::Uuid;

use super::unprocessable_entity as unprocessable;
use crate::auth::MaybeAuthUser;
use crate::earned_value::{self as ev, DefaultMethod, Method, Period, TaskInput, TaskState};
use crate::evm::{self, Thresholds};
use crate::models::_entities::{
    budget_baseline_periods, budget_baselines, budget_lines, earned_value_settings,
    earned_value_snapshots, plans, tasks, time_entry_costs,
};
use crate::models::audit_logs::Model as AuditModel;

async fn default_method(ctx: &AppContext, plan: Uuid) -> Result<DefaultMethod> {
    Ok(earned_value_settings::Entity::find()
        .filter(earned_value_settings::Column::PlanPid.eq(plan))
        .filter(earned_value_settings::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .and_then(|s| DefaultMethod::parse(&s.default_method))
        .unwrap_or(DefaultMethod::ZeroHundred))
}

async fn task_inputs(ctx: &AppContext, plan: Uuid) -> Result<Vec<TaskInput>> {
    let workflow = super::workflow::in_force(ctx, plan, "task").await?;
    Ok(tasks::Entity::find()
        .filter(tasks::Column::PlanPid.eq(plan))
        .filter(tasks::Column::DeletedAt.is_null())
        .order_by_asc(tasks::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|t| TaskInput {
            pid: t.pid,
            points: t.points,
            state: match crate::workflow::category_of(&workflow, &t.status) {
                Some(crate::workflow::Category::Done) => TaskState::Done,
                Some(crate::workflow::Category::Active | crate::workflow::Category::Waiting) => {
                    TaskState::Started
                }
                Some(crate::workflow::Category::Todo) | None => TaskState::Todo,
            },
            method: t.percent_method.as_deref().and_then(Method::parse),
            manual_bp: t.percent_manual.map(|p| i64::from(p) * 100),
        })
        .collect())
}

/// What the figures came to.
pub(crate) enum Outcome {
    /// No phased budget baseline exists.
    NoBaseline,
    /// A baseline exists but nothing carries weight to be a percent of.
    NoPercent,
    /// The figures.
    Figures {
        default: DefaultMethod,
        pc: ev::PercentComplete,
        blocks: Vec<(String, i32, ev::Block)>,
        unbaselined_currencies: Vec<String>,
    },
}

/// Where actual cost (AC) comes from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AcSource {
    /// The plan's recorded budget-line actuals (the default, and what every other
    /// endpoint uses).
    BudgetLines,
    /// Recorded labour cost (PJ-2): time entries priced at the rate in force, with
    /// the rate stored. Finance data.
    Labour,
    /// Both added: **only right when budget lines do not already include labour**,
    /// which the API cannot know — the caller says so by asking for it.
    Both,
}

impl AcSource {
    fn parse(s: &str) -> Option<Self> {
        match s {
            "budget_lines" => Some(Self::BudgetLines),
            "labour" => Some(Self::Labour),
            "both" => Some(Self::Both),
            _ => None,
        }
    }
    fn token(self) -> &'static str {
        match self {
            Self::BudgetLines => "budget_line_actuals",
            Self::Labour => "labour_costs",
            Self::Both => "budget_line_actuals_plus_labour_costs",
        }
    }
}

/// Compute a plan's earned-value figures as of `as_of`, from budget-line actuals.
pub(crate) async fn compute(
    ctx: &AppContext,
    plan: Uuid,
    as_of: chrono::NaiveDate,
) -> Result<Outcome> {
    compute_with(ctx, plan, as_of, AcSource::BudgetLines).await
}

/// Compute a plan's earned-value figures as of `as_of`, with the AC from `source`.
pub(crate) async fn compute_with(
    ctx: &AppContext,
    plan: Uuid,
    as_of: chrono::NaiveDate,
    source: AcSource,
) -> Result<Outcome> {
    let baselines = budget_baselines::Entity::find()
        .filter(budget_baselines::Column::PlanPid.eq(plan))
        .order_by_desc(budget_baselines::Column::Version)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    if baselines.is_empty() {
        return Ok(Outcome::NoBaseline);
    }
    let default = default_method(ctx, plan).await?;
    let Some(pc) = ev::percent_complete(default, &task_inputs(ctx, plan).await?) else {
        return Ok(Outcome::NoPercent);
    };
    // The latest version of each currency's baseline (newest first, so the first seen wins).
    let mut seen: Vec<&budget_baselines::Model> = Vec::new();
    for b in &baselines {
        if !seen.iter().any(|s| s.currency == b.currency) {
            seen.push(b);
        }
    }
    let lines = budget_lines::Entity::find()
        .filter(budget_lines::Column::PlanPid.eq(plan))
        .filter(budget_lines::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let labour_rows = if source == AcSource::BudgetLines {
        Vec::new()
    } else {
        time_entry_costs::Entity::find()
            .filter(time_entry_costs::Column::PlanPid.eq(plan))
            .all(&ctx.db)
            .await
            .map_err(db_err)?
    };
    let mut blocks = Vec::new();
    for b in seen {
        let periods: Vec<Period> = budget_baseline_periods::Entity::find()
            .filter(budget_baseline_periods::Column::BaselinePid.eq(b.pid))
            .order_by_asc(budget_baseline_periods::Column::PeriodStart)
            .limit(5_000)
            .all(&ctx.db)
            .await
            .map_err(db_err)?
            .into_iter()
            .map(|p| Period {
                start: p.period_start,
                end: p.period_end,
                planned_minor: p.planned_minor,
            })
            .collect();
        let budget_ac = lines
            .iter()
            .filter(|l| l.currency == b.currency)
            .fold(0_i64, |a, l| a.saturating_add(l.actual_minor));
        let labour: Vec<i64> = labour_rows
            .iter()
            .filter(|c| c.currency == b.currency)
            .map(|c| c.cost_minor)
            .collect();
        let labour_ac = labour.iter().fold(0_i64, |a, c| a.saturating_add(*c));
        // No labour recorded in this currency is "no actuals" (null), never a zero.
        let ac = match source {
            AcSource::BudgetLines => Some(budget_ac),
            AcSource::Labour => (!labour.is_empty()).then_some(labour_ac),
            AcSource::Both => Some(budget_ac.saturating_add(labour_ac)),
        };
        if let Some(block) = ev::block(&periods, pc.numerator, pc.denominator, ac, as_of) {
            blocks.push((b.currency.clone(), b.version, block));
        }
    }
    let mut unbaselined: Vec<String> = lines
        .iter()
        .map(|l| l.currency.clone())
        .filter(|c| !blocks.iter().any(|(bc, _, _)| bc == c))
        .collect();
    unbaselined.sort();
    unbaselined.dedup();
    Ok(Outcome::Figures {
        default,
        pc,
        blocks,
        unbaselined_currencies: unbaselined,
    })
}

fn pc_view(default: DefaultMethod, pc: &ev::PercentComplete) -> serde_json::Value {
    serde_json::json!({
        "percent_bp": pc.percent_bp,
        "default_method": default,
        "weighting": pc.weighting,
        "tasks_counted": pc.tasks.len(),
        "unweighted_tasks": pc.unweighted,
        "manual_tasks": pc.manual,
        "manual": !pc.manual.is_empty(),
        "tasks": pc.tasks,
    })
}

/// `?ac=` on `GET …/earned-value`.
#[derive(Debug, Deserialize)]
struct EarnedValueParams {
    /// `budget_lines` (default), `labour` or `both`.
    #[serde(default)]
    ac: Option<String>,
}

/// `GET /api/plans/{pid}/earned-value` — the plan's earned-value figures today.
#[debug_handler]
async fn earned_value(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Query(q): Query<EarnedValueParams>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let source = match q.ac.as_deref() {
        None => AcSource::BudgetLines,
        Some(a) => AcSource::parse(a).ok_or_else(|| {
            Error::CustomError(
                StatusCode::UNPROCESSABLE_ENTITY,
                ErrorDetail::new("unprocessable", "ac must be budget_lines, labour or both"),
            )
        })?,
    };
    // Labour cost is finance data: asking for it needs the same admission as the rates.
    if source != AcSource::BudgetLines && !super::cost_rates::may_see_rates(&caller) {
        return Err(Error::CustomError(
            StatusCode::FORBIDDEN,
            ErrorDetail::new("forbidden", "actual cost from labour needs finance access"),
        ));
    }
    let as_of = chrono::Utc::now().date_naive();
    let body = match compute_with(&ctx, plan.pid, as_of, source).await? {
        Outcome::NoBaseline => serde_json::json!({
            "plan_pid": plan.pid, "as_of": as_of, "blocks": null, "absent": "no_baseline",
            "reason": "no phased budget baseline: a plan without one is unmeasured, not on track",
        }),
        Outcome::NoPercent => serde_json::json!({
            "plan_pid": plan.pid, "as_of": as_of, "blocks": null, "absent": "no_weighted_tasks",
            "reason": "there is nothing to measure percent complete against: the plan has no tasks (or, counting by story points, no task has points)",
        }),
        Outcome::Figures {
            default,
            pc,
            blocks,
            unbaselined_currencies,
        } => serde_json::json!({
            "plan_pid": plan.pid,
            "as_of": as_of,
            "percent_complete": pc_view(default, &pc),
            "blocks": blocks.iter().map(|(currency, version, b)| {
                let mut v = serde_json::to_value(b).unwrap_or_default();
                v["currency"] = serde_json::json!(currency);
                v["baseline_version"] = serde_json::json!(version);
                v
            }).collect::<Vec<_>>(),
            "unbaselined_currencies": unbaselined_currencies,
            "ac_source": source.token(),
            "note": "Measured as of today: actuals carry no date and intermediate task progress is not stored by date, so the past is not reconstructed. Planned value is linear within each baseline period. Percent complete is physical (work finished), not elapsed time. Cost per currency is never added across currencies. Actual cost defaults to the recorded budget-line actuals; ac=labour uses recorded labour cost (time entries x the rate in force, rate stored) and ac=both adds the two, which is only right when the budget lines do not already include labour. A currency with no recorded labour has no actual cost (null), never zero. The forecast, S-curve, snapshots and portfolio figures still use budget-line actuals.",
        }),
    };
    format::json(body)
}

/// `PUT …/earned-value-settings` body.
#[derive(Debug, Deserialize)]
struct SettingsBody {
    /// `zero_hundred`, `fifty_fifty` or `points`.
    default_method: String,
}

/// `PUT /api/plans/{pid}/earned-value-settings` — how the plan's tasks are counted by default.
#[debug_handler]
async fn put_settings(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<SettingsBody>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    if DefaultMethod::parse(&b.default_method).is_none() {
        return Err(unprocessable(
            "default_method must be zero_hundred, fifty_fifty or points",
        ));
    }
    let existing = earned_value_settings::Entity::find()
        .filter(earned_value_settings::Column::PlanPid.eq(plan.pid))
        .filter(earned_value_settings::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?;
    let was = existing.as_ref().map(|e| e.default_method.clone());
    match existing {
        Some(row) => {
            let mut a: earned_value_settings::ActiveModel = row.into();
            a.default_method = ActiveValue::set(b.default_method.clone());
            a.update(&ctx.db).await.map_err(db_err)?;
        }
        None => {
            earned_value_settings::ActiveModel {
                pid: ActiveValue::set(Uuid::new_v4()),
                plan_pid: ActiveValue::set(plan.pid),
                default_method: ActiveValue::set(b.default_method.clone()),
                deleted_at: ActiveValue::set(None),
                ..Default::default()
            }
            .insert(&ctx.db)
            .await
            .map_err(db_err)?;
        }
    }
    AuditModel::record_or_log(
        &ctx.db,
        plan.pid,
        "earned_value_method_set",
        caller.actor(),
        Some(serde_json::json!({ "from": was, "to": b.default_method })),
    )
    .await;
    format::json(serde_json::json!({ "plan_pid": plan.pid, "default_method": b.default_method }))
}

/// `PUT …/tasks/{pid}/percent-complete` body.
#[derive(Debug, Deserialize)]
struct PercentBody {
    /// `zero_hundred`, `fifty_fifty` or `manual`; absent or null clears the override.
    #[serde(default)]
    method: Option<String>,
    /// 0–100; required with `manual`.
    #[serde(default)]
    percent: Option<i32>,
    /// Why; required with `manual`.
    #[serde(default)]
    note: Option<String>,
}

/// `PUT /api/plans/{pid}/tasks/{t_pid}/percent-complete` — count this task another way.
#[debug_handler]
async fn put_percent(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, t_pid)): Path<(String, String)>,
    Json(b): Json<PercentBody>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let t_pid = super::parse_pid(&t_pid)?;
    let task = tasks::Entity::find()
        .filter(tasks::Column::Pid.eq(t_pid))
        .filter(tasks::Column::PlanPid.eq(plan.pid))
        .filter(tasks::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    let method =
        match b.method.as_deref() {
            None => None,
            Some(m) => Some(Method::parse(m).ok_or_else(|| {
                unprocessable("method must be zero_hundred, fifty_fifty or manual")
            })?),
        };
    let (manual, note) = if method == Some(Method::Manual) {
        let percent = b
            .percent
            .ok_or_else(|| unprocessable("a manual percent needs percent, 0 to 100"))?;
        if !(0..=100).contains(&percent) {
            return Err(unprocessable("percent must be 0 to 100"));
        }
        let note = b.note.as_deref().map(str::trim).filter(|n| !n.is_empty());
        let Some(note) = note else {
            return Err(unprocessable(
                "a manual percent needs a note saying why it is what it is",
            ));
        };
        if note.len() > crate::validation::MAX_TEXT_LEN {
            return Err(unprocessable("note exceeds the text cap"));
        }
        (Some(percent), Some(note.to_string()))
    } else {
        if b.percent.is_some() || b.note.is_some() {
            return Err(unprocessable("percent and note go with method manual only"));
        }
        (None, None)
    };
    let was = (task.percent_method.clone(), task.percent_manual);
    let mut a: tasks::ActiveModel = task.into();
    a.percent_method = ActiveValue::set(b.method.clone());
    a.percent_manual = ActiveValue::set(manual);
    a.percent_note = ActiveValue::set(note.clone());
    let row = a.update(&ctx.db).await.map_err(db_err)?;
    AuditModel::record_or_log(
        &ctx.db,
        row.pid,
        "percent_complete_method_set",
        caller.actor(),
        Some(
            serde_json::json!({ "from": { "method": was.0, "percent": was.1 },
                                  "to": { "method": b.method, "percent": manual, "note": note } }),
        ),
    )
    .await;
    format::json(
        serde_json::json!({ "task_pid": row.pid, "method": row.percent_method,
        "percent": row.percent_manual, "note": row.percent_note }),
    )
}

// ───────────────────────── SC-5: forecasts, series, roll-up ─────────────────────────

fn thresholds() -> (Thresholds, bool) {
    evm::parse_thresholds(
        std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_EV_THRESHOLDS")
            .ok()
            .as_deref(),
    )
}

fn rag_json(r: evm::Rag) -> serde_json::Value {
    serde_json::to_value(r).unwrap_or_default()
}

/// One block's forecast: every EAC variant named by its formula, the rule-chosen
/// one marked, ETC, VAC, TCPI to the budget and to the chosen forecast, lights.
fn forecast_view(
    currency: &str,
    version: i32,
    b: &ev::Block,
    bottom_up_eac: Option<i64>,
    t: &Thresholds,
) -> serde_json::Value {
    let variants = evm::eac_variants(
        b.bac_minor,
        b.ev_minor,
        b.ac_minor,
        b.cpi.bp,
        b.spi.bp,
        bottom_up_eac,
    );
    let (chosen, why) = evm::recommended(b.cpi.bp, b.spi.bp, t);
    let chosen_eac = variants
        .iter()
        .find(|v| v.variant == chosen)
        .and_then(|v| v.eac.minor);
    let tcpi_budget = evm::tcpi(b.bac_minor, b.ev_minor, b.ac_minor, b.bac_minor);
    let tcpi_forecast = chosen_eac.map(|e| evm::tcpi(b.bac_minor, b.ev_minor, b.ac_minor, e));
    serde_json::json!({
        "currency": currency,
        "baseline_version": version,
        "spi": { "bp": b.spi.bp, "rag": rag_json(evm::rag_ratio(b.spi.bp, t)) },
        "cpi": { "bp": b.cpi.bp, "rag": rag_json(evm::rag_ratio(b.cpi.bp, t)) },
        "variants": variants,
        "recommended": { "variant": chosen, "why": why, "eac_minor": chosen_eac },
        "tcpi_to_budget": { "tcpi": tcpi_budget, "rag": rag_json(evm::rag_tcpi(tcpi_budget.bp, t)) },
        "tcpi_to_recommended_eac": tcpi_forecast.map(|x| serde_json::json!({ "tcpi": x, "rag": rag_json(evm::rag_tcpi(x.bp, t)) })),
        "bac_minor": b.bac_minor, "ev_minor": b.ev_minor, "ac_minor": b.ac_minor,
    })
}

/// `GET /api/plans/{pid}/earned-value/forecast` — where the plan is heading.
#[debug_handler]
async fn forecast(State(ctx): State<AppContext>, Path(pid): Path<String>) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let as_of = chrono::Utc::now().date_naive();
    let (t, defaulted) = thresholds();
    let body = match compute(&ctx, plan.pid, as_of).await? {
        Outcome::NoBaseline => {
            serde_json::json!({ "plan_pid": plan.pid, "as_of": as_of, "forecasts": null,
            "absent": "no_baseline", "reason": "no phased budget baseline, so there is nothing to forecast against" })
        }
        Outcome::NoPercent => {
            serde_json::json!({ "plan_pid": plan.pid, "as_of": as_of, "forecasts": null,
            "absent": "no_weighted_tasks", "reason": "nothing to measure percent complete against, so no forecast" })
        }
        Outcome::Figures { blocks, .. } => {
            let own = super::financials::forecast_of(&ctx, plan.pid).await?;
            let forecasts: Vec<serde_json::Value> = blocks
                .iter()
                .map(|(cur, ver, b)| {
                    // Only a real estimate counts as bottom-up (a TPC cost-estimate-to-complete).
                    // The baseline's not-yet-elapsed periods is a schedule-driven fallback, not
                    // someone re-estimating, so it is not offered as one.
                    let bottom_up = (own.currency.as_deref() == Some(cur.as_str())
                        && own.etc_source == Some(crate::financials::EtcSource::Tpc))
                    .then_some(own.estimate_at_completion_minor)
                    .flatten();
                    forecast_view(cur, *ver, b, bottom_up, &t)
                })
                .collect();
            serde_json::json!({
                "plan_pid": plan.pid, "as_of": as_of, "forecasts": forecasts,
                "thresholds": { "values": t, "source": if defaulted { "default" } else { "PROJECT_PORTFOLIO_MANAGEMENT_EV_THRESHOLDS" } },
                "note": "Every EAC names its formula; none is the answer, because which is right depends on whether past overruns will continue. `recommended` is a stated rule, and the others are returned beside it. A TCPI above 1.00 means the remaining work must be done more efficiently than the work so far; a target already spent is budget_exhausted, not infinity. Currencies are never added together.",
            })
        }
    };
    format::json(body)
}

/// Record today's figures for a plan (one row per currency; a second call the
/// same day replaces the first). Returns how many currencies were recorded.
pub(crate) async fn snapshot_plan(ctx: &AppContext, plan: Uuid) -> Result<usize> {
    let today = chrono::Utc::now().date_naive();
    let Outcome::Figures { blocks, .. } = compute(ctx, plan, today).await? else {
        return Ok(0);
    };
    // One statement per currency, so two snapshots on the same day cannot race into a
    // unique-index error: the second replaces the first (CQ-11).
    let now: chrono::DateTime<chrono::FixedOffset> = chrono::Utc::now().into();
    for (currency, version, b) in &blocks {
        use earned_value_snapshots::Column as C;
        earned_value_snapshots::Entity::insert(earned_value_snapshots::ActiveModel {
            pid: ActiveValue::set(Uuid::new_v4()),
            plan_pid: ActiveValue::set(plan),
            currency: ActiveValue::set(currency.clone()),
            as_of: ActiveValue::set(today),
            baseline_version: ActiveValue::set(*version),
            bac_minor: ActiveValue::set(b.bac_minor),
            pv_minor: ActiveValue::set(b.pv_minor),
            ev_minor: ActiveValue::set(b.ev_minor),
            ac_minor: ActiveValue::set(b.ac_minor),
            updated_at: ActiveValue::set(now),
            ..Default::default()
        })
        .on_conflict(
            sea_orm::sea_query::OnConflict::columns([C::PlanPid, C::Currency, C::AsOf])
                .update_columns([
                    C::BaselineVersion,
                    C::BacMinor,
                    C::PvMinor,
                    C::EvMinor,
                    C::AcMinor,
                    C::UpdatedAt,
                ])
                .to_owned(),
        )
        .exec(&ctx.db)
        .await
        .map_err(db_err)?;
    }
    Ok(blocks.len())
}

/// Snapshot every plan that has a baseline and no snapshot yet today. The
/// scheduler calls this; it is idempotent and cheap on the second pass of a day.
///
/// # Errors
///
/// Propagates database errors from finding the plans.
pub async fn snapshot_all(ctx: &AppContext) -> Result<usize> {
    let today = chrono::Utc::now().date_naive();
    let with_baseline: std::collections::BTreeSet<Uuid> = budget_baselines::Entity::find()
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|b| b.plan_pid)
        .collect();
    let done: std::collections::BTreeSet<Uuid> = earned_value_snapshots::Entity::find()
        .filter(earned_value_snapshots::Column::AsOf.eq(today))
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|s| s.plan_pid)
        .collect();
    let mut n = 0;
    for plan in with_baseline.difference(&done) {
        n += snapshot_plan(ctx, *plan).await.unwrap_or(0);
    }
    Ok(n)
}

/// `POST /api/plans/{pid}/earned-value/snapshot` — record today's figures now.
#[debug_handler]
async fn snapshot(State(ctx): State<AppContext>, Path(pid): Path<String>) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let n = snapshot_plan(&ctx, plan.pid).await?;
    format::json(
        serde_json::json!({ "plan_pid": plan.pid, "as_of": chrono::Utc::now().date_naive(), "snapshotted": n }),
    )
}

#[derive(Debug, Deserialize)]
struct SeriesParams {
    /// `csv` for a spreadsheet.
    #[serde(default)]
    format: Option<String>,
}

#[allow(clippy::too_many_lines)]
/// `GET /api/plans/{pid}/earned-value/series[?format=csv]` — the S-curve data: the
/// planned-value curve from the baseline (complete), and earned value and actual
/// cost on the days they were **recorded** (never filled in between).
#[debug_handler]
async fn series(
    State(ctx): State<AppContext>,
    Path(pid): Path<String>,
    Query(p): Query<SeriesParams>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let baselines = budget_baselines::Entity::find()
        .filter(budget_baselines::Column::PlanPid.eq(plan.pid))
        .order_by_desc(budget_baselines::Column::Version)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let snaps = earned_value_snapshots::Entity::find()
        .filter(earned_value_snapshots::Column::PlanPid.eq(plan.pid))
        .order_by_asc(earned_value_snapshots::Column::AsOf)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let mut seen: Vec<&budget_baselines::Model> = Vec::new();
    for b in &baselines {
        if !seen.iter().any(|s| s.currency == b.currency) {
            seen.push(b);
        }
    }
    let mut curves = Vec::new();
    for b in seen {
        let periods: Vec<Period> = budget_baseline_periods::Entity::find()
            .filter(budget_baseline_periods::Column::BaselinePid.eq(b.pid))
            .order_by_asc(budget_baseline_periods::Column::PeriodStart)
            .limit(5_000)
            .all(&ctx.db)
            .await
            .map_err(db_err)?
            .into_iter()
            .map(|r| Period {
                start: r.period_start,
                end: r.period_end,
                planned_minor: r.planned_minor,
            })
            .collect();
        let Some(start) = ev::baseline_start(&periods) else {
            continue;
        };
        let mine: Vec<&earned_value_snapshots::Model> =
            snaps.iter().filter(|s| s.currency == b.currency).collect();
        let mut dates: Vec<chrono::NaiveDate> = periods
            .iter()
            .map(|r| r.end)
            .chain(mine.iter().map(|s| s.as_of))
            .collect();
        dates.push(start - chrono::Duration::days(1));
        dates.sort_unstable();
        dates.dedup();
        let points: Vec<serde_json::Value> = dates
            .iter()
            .map(|d| {
                let t = if *d < start {
                    0
                } else {
                    (*d - start).num_days() + 1
                };
                let snap = mine.iter().find(|s| s.as_of == *d);
                serde_json::json!({
                    "date": d,
                    "pv_minor": ev::pv_after(&periods, start, t),
                    "ev_minor": snap.map(|s| s.ev_minor),
                    "ac_minor": snap.and_then(|s| s.ac_minor),
                })
            })
            .collect();
        curves.push(serde_json::json!({
            "currency": b.currency, "baseline_version": b.version, "bac_minor": ev::bac(&periods),
            "snapshots": mine.len(), "points": points,
        }));
    }
    if p.format.as_deref() == Some("csv") {
        let mut out = String::from("currency,date,pv_minor,ev_minor,ac_minor\n");
        for c in &curves {
            for pt in c["points"].as_array().into_iter().flatten() {
                let cell = |k: &str| pt[k].as_i64().map_or(String::new(), |n| n.to_string());
                let _ = writeln!(
                    out,
                    "{},{},{},{},{}",
                    c["currency"].as_str().unwrap_or(""),
                    pt["date"].as_str().unwrap_or(""),
                    cell("pv_minor"),
                    cell("ev_minor"),
                    cell("ac_minor")
                );
            }
        }
        return Ok((
            [
                (header::CONTENT_TYPE, "text/csv; charset=utf-8"),
                (
                    header::CONTENT_DISPOSITION,
                    "attachment; filename=\"earned-value-series.csv\"",
                ),
            ],
            out,
        )
            .into_response());
    }
    format::json(serde_json::json!({
        "plan_pid": plan.pid,
        "curves": curves,
        "note": "The planned-value curve is the whole baseline. Earned value and actual cost appear only on the days a snapshot was recorded (daily by the scheduler, or POST …/earned-value/snapshot): they carry no date at source, so the past cannot be reconstructed and is never filled in between.",
    }))
}

/// A plan and every plan under it (bounded), as pids.
pub(crate) async fn subtree_ids(ctx: &AppContext, root: Uuid) -> Result<Vec<Uuid>> {
    descendants(ctx, root).await
}

async fn descendants(ctx: &AppContext, root: Uuid) -> Result<Vec<Uuid>> {
    let mut all = vec![root];
    let mut frontier = vec![root];
    for _ in 0..10 {
        if frontier.is_empty() || all.len() >= 500 {
            break;
        }
        let kids: Vec<Uuid> = plans::Entity::find()
            .filter(plans::Column::ParentPid.is_in(frontier.clone()))
            .filter(plans::Column::DeletedAt.is_null())
            .all(&ctx.db)
            .await
            .map_err(db_err)?
            .into_iter()
            .map(|p| p.pid)
            .filter(|p| !all.contains(p))
            .collect();
        all.extend(kids.iter().copied());
        frontier = kids;
    }
    all.truncate(500);
    Ok(all)
}

/// The figures for `plans`, summed **per currency**: one JSON entry per currency, and how many plans had
/// no figures. The one place the roll-up is computed, so the live roll-up, the portfolio view and the
/// director pack cannot differ. With `bac_thresholds`, a currency whose summed BAC is under its threshold
/// is reported as below the earned-value threshold, with no ratios.
pub(crate) async fn rollup_over(
    ctx: &AppContext,
    plans: &[Uuid],
    as_of: chrono::NaiveDate,
    t: &Thresholds,
    source: AcSource,
    bac_thresholds: Option<&std::collections::BTreeMap<String, i64>>,
) -> Result<(Vec<serde_json::Value>, usize)> {
    let mut totals = std::collections::BTreeMap::new();
    let mut eac: std::collections::BTreeMap<String, (i64, usize)> =
        std::collections::BTreeMap::new();
    let mut variants_used: std::collections::BTreeMap<
        String,
        std::collections::BTreeSet<&'static str>,
    > = std::collections::BTreeMap::new();
    let mut without = 0_usize;
    for plan in plans {
        match compute_with(ctx, *plan, as_of, source).await? {
            Outcome::Figures { blocks, .. } => {
                for (cur, _, b) in &blocks {
                    evm::add(
                        &mut totals,
                        cur,
                        b.bac_minor,
                        b.pv_minor,
                        b.ev_minor,
                        b.ac_minor,
                    );
                    let variants = evm::eac_variants(
                        b.bac_minor,
                        b.ev_minor,
                        b.ac_minor,
                        b.cpi.bp,
                        b.spi.bp,
                        None,
                    );
                    let (chosen, _) = evm::recommended(b.cpi.bp, b.spi.bp, t);
                    if let Some(e) = variants
                        .iter()
                        .find(|v| v.variant == chosen)
                        .and_then(|v| v.eac.minor)
                    {
                        let slot = eac.entry(cur.clone()).or_insert((0, 0));
                        slot.0 = slot.0.saturating_add(e);
                        slot.1 += 1;
                        variants_used.entry(cur.clone()).or_default().insert(chosen);
                    }
                }
            }
            _ => without += 1,
        }
    }
    let currencies = totals
        .iter()
        .map(|(cur, tot)| {
            if let Some(map) = bac_thresholds
                && !evm::meets_bac_threshold(tot.bac_minor, cur, map)
            {
                return serde_json::json!({
                    "currency": cur, "state": "below EVM threshold",
                    "bac_minor": tot.bac_minor, "threshold_minor": map.get(cur),
                    "reason": "the budget at completion is under the inclusion threshold for this currency, so no ratios are shown",
                });
            }
            let spi = evm::ratio_bp(tot.ev_minor, tot.pv_minor);
            let cpi = (tot.plans_without_ac == 0)
                .then(|| evm::ratio_bp(tot.ev_minor, tot.ac_minor))
                .flatten();
            let (eac_minor, eac_plans) = eac.get(cur).copied().unwrap_or((0, 0));
            serde_json::json!({
                "currency": cur, "state": "included", "totals": tot,
                "ac_source": source.token(),
                "spi": { "bp": spi, "rag": rag_json(evm::rag_ratio(spi, t)) },
                "cpi": { "bp": cpi, "rag": rag_json(evm::rag_ratio(cpi, t)),
                         "absent": (cpi.is_none()).then_some("cost_unknown_for_some_plans_or_zero") },
                "eac": { "minor": (eac_plans == tot.plans).then_some(eac_minor), "plans_counted": eac_plans, "plans_total": tot.plans,
                         "variants_used": variants_used.get(cur).map(|v| v.iter().copied().collect::<Vec<_>>()).unwrap_or_default(),
                         "note": "the sum of each plan's own recommended EAC; absent unless every plan has one" },
                "vac_minor": (eac_plans == tot.plans).then(|| tot.bac_minor - eac_minor),
            })
        })
        .collect();
    Ok((currencies, without))
}

/// `GET /api/plans/{pid}/earned-value/rollup` — the plan and everything beneath it,
/// summed **per currency**. Plans the caller cannot read are left out without a
/// trace; plans with no figures are counted, not guessed at.
#[debug_handler]
async fn rollup(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let root = super::governance::find_item(&ctx, &pid).await?;
    let as_of = chrono::Utc::now().date_naive();
    let (t, defaulted) = thresholds();
    let all = descendants(&ctx, root.pid).await?;
    let readable = super::plan_access::readable_plans(&ctx, &caller, &all).await;
    let plans: Vec<Uuid> = all
        .iter()
        .filter(|p| readable.contains(p))
        .copied()
        .collect();
    let (currencies, without) =
        rollup_over(&ctx, &plans, as_of, &t, AcSource::BudgetLines, None).await?;
    format::json(serde_json::json!({
        "root_plan_pid": root.pid, "as_of": as_of, "currencies": currencies,
        "plans_without_figures": without,
        "thresholds": { "values": t, "source": if defaulted { "default" } else { "PROJECT_PORTFOLIO_MANAGEMENT_EV_THRESHOLDS" } },
        "note": "Summed per currency, never across currencies. A partial cost (some plans with no recorded cost) gives no CPI rather than a flattering one. Plans you cannot read are not included.",
    }))
}

/// Where the pack takes actual cost from: `PROJECT_PORTFOLIO_MANAGEMENT_EVM_AC_SOURCE`
/// (`budget_lines`, `labour` or `both`), default `budget_lines`.
pub(crate) fn pack_ac_source() -> AcSource {
    std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_EVM_AC_SOURCE")
        .ok()
        .as_deref()
        .and_then(AcSource::parse)
        .unwrap_or(AcSource::BudgetLines)
}

/// Every plan beneath `root` (and `root`) that is in `include`.
async fn subtree_in(
    ctx: &AppContext,
    root: Uuid,
    include: &std::collections::HashSet<Uuid>,
) -> Result<Vec<Uuid>> {
    Ok(descendants(ctx, root)
        .await?
        .into_iter()
        .filter(|p| include.contains(p))
        .collect())
}

/// The earned-value section over the programmes in `include`: for each, its roll-up per currency,
/// with currencies under the inclusion threshold reported as below it. Figures come from
/// [`rollup_over`], the same code as `GET …/earned-value/rollup`.
///
/// # Errors
///
/// When a database read fails.
pub(crate) async fn portfolio_section(
    ctx: &AppContext,
    include: &std::collections::HashSet<Uuid>,
    as_of: chrono::NaiveDate,
    source: AcSource,
) -> Result<serde_json::Value> {
    let (t, defaulted) = thresholds();
    let (bac, bac_bad) = evm::parse_bac_thresholds(
        std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_EVM_BAC_THRESHOLDS")
            .ok()
            .as_deref(),
    );
    let mut progs: Vec<crate::models::_entities::plans::Model> =
        crate::models::_entities::plans::Entity::find()
            .filter(crate::models::_entities::plans::Column::DeletedAt.is_null())
            .all(&ctx.db)
            .await
            .map_err(db_err)?
            .into_iter()
            .filter(|p| p.kind.as_deref() == Some("Program") && include.contains(&p.pid))
            .collect();
    progs.sort_by(|a, b| a.name.cmp(&b.name).then(a.pid.cmp(&b.pid)));
    let methods: std::collections::HashMap<Uuid, String> =
        crate::models::_entities::programme_profiles::Entity::find()
            .all(&ctx.db)
            .await
            .map_err(db_err)?
            .into_iter()
            .map(|r| (r.plan_pid, r.method))
            .collect();
    let mut out = Vec::new();
    for p in &progs {
        let plans = subtree_in(ctx, p.pid, include).await?;
        let mut entry = serde_json::json!({ "plan_pid": p.pid, "name": p.name });
        // A product programme reports burn-up and flow, not earned value (EX-14).
        if methods.get(&p.pid).map(String::as_str) == Some("product") {
            entry["method"] = "product".into();
            if let Some(f) = product_flow(ctx, &plans, as_of).await? {
                entry["state"] = "product".into();
                entry["flow"] = serde_json::to_value(&f).unwrap_or_default();
            } else {
                entry["state"] = "no_flow_data".into();
                entry["reason"] = "this product programme has no tasks yet, so there is no burn-up or flow to show".into();
            }
            out.push(entry);
            continue;
        }
        let (currencies, without) = rollup_over(ctx, &plans, as_of, &t, source, Some(&bac)).await?;
        entry["plans_without_figures"] = without.into();
        entry["method"] = "evm".into();
        if currencies.is_empty() {
            entry["state"] = "no_baseline".into();
            entry["reason"] = "no plan in the programme has a phased budget baseline with something to measure percent complete against, so there are no earned-value figures".into();
        } else {
            entry["currencies"] = currencies.into();
        }
        out.push(entry);
    }
    Ok(serde_json::json!({
        "as_of": as_of,
        "ac_source": source.token(),
        "bac_inclusion_threshold": {
            "minor_by_currency": bac,
            "configured": !bac.is_empty(),
            "setting_unusable_so_none_applied": bac_bad,
            "note": "A programme whose summed budget at completion in a currency is under its threshold is shown as below the earned-value threshold, with no ratios. With no threshold set, every programme with figures is included.",
        },
        "ratio_thresholds": { "values": t, "source": if defaulted { "default" } else { "PROJECT_PORTFOLIO_MANAGEMENT_EV_THRESHOLDS" } },
        "programmes": out,
        "note": "Per programme, summed per currency, never across currencies. The figures are those of GET /api/plans/{pid}/earned-value/rollup for the same date, restricted to plans that are neither private nor marked in the pack. A programme with no baseline is no_baseline, not zero. A programme whose reporting method is product (EX-14) shows burn-up and throughput of its tasks instead of earned value, never a blank.",
    }))
}

/// The burn-up and flow of the tasks of `plans` (live, not deleted, not hammocks).
async fn product_flow(
    ctx: &AppContext,
    plans: &[Uuid],
    as_of: chrono::NaiveDate,
) -> Result<Option<crate::product_flow::Flow>> {
    use crate::models::_entities::tasks;
    let rows = tasks::Entity::find()
        .filter(tasks::Column::PlanPid.is_in(plans.to_vec()))
        .filter(tasks::Column::DeletedAt.is_null())
        .filter(tasks::Column::IsHammock.eq(false))
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let items: Vec<crate::product_flow::Item> = rows
        .iter()
        .map(|t| crate::product_flow::Item {
            created: t.created_at.date_naive(),
            done: t.done_at.map(|d| d.date_naive()),
        })
        .collect();
    Ok(crate::product_flow::flow(&items, as_of, 8))
}

/// `GET /api/portfolio-earned-value[?ac=]` — earned value over the threshold for the programmes you can read.
#[debug_handler]
async fn portfolio(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(q): Query<EarnedValueParams>,
) -> Result<Response> {
    let source = match q.ac.as_deref() {
        None => AcSource::BudgetLines,
        Some(a) => AcSource::parse(a)
            .ok_or_else(|| unprocessable("ac must be budget_lines, labour or both"))?,
    };
    let all: Vec<Uuid> = crate::models::_entities::plans::Entity::find()
        .filter(crate::models::_entities::plans::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|p| p.pid)
        .collect();
    let readable = super::plan_access::readable_plans(&ctx, &caller, &all).await;
    format::json(portfolio_section(&ctx, &readable, chrono::Utc::now().date_naive(), source).await?)
}

/// The earned-value routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/earned-value", get(earned_value))
        .add("/plans/{pid}/earned-value/forecast", get(forecast))
        .add("/plans/{pid}/earned-value/series", get(series))
        .add("/plans/{pid}/earned-value/rollup", get(rollup))
        .add("/portfolio-earned-value", get(portfolio))
        .add("/plans/{pid}/earned-value/snapshot", post(snapshot))
        .add("/plans/{pid}/earned-value-settings", put(put_settings))
        .add(
            "/plans/{pid}/tasks/{t_pid}/percent-complete",
            put(put_percent),
        )
}
