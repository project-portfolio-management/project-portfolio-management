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

use axum::http::StatusCode;
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use sea_orm::{QueryOrder, QuerySelect};
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::MaybeAuthUser;
use crate::earned_value::{self as ev, DefaultMethod, Method, Period, TaskInput, TaskState};
use crate::models::_entities::{
    budget_baseline_periods, budget_baselines, budget_lines, earned_value_settings, tasks,
};
use crate::models::audit_logs::Model as AuditModel;

fn unprocessable(message: &str) -> Error {
    Error::CustomError(
        StatusCode::UNPROCESSABLE_ENTITY,
        ErrorDetail::new("unprocessable_entity", message),
    )
}

fn db_err(e: sea_orm::DbErr) -> Error {
    Error::Model(ModelError::from(e))
}

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

/// Compute a plan's earned-value figures as of `as_of`.
pub(crate) async fn compute(
    ctx: &AppContext,
    plan: Uuid,
    as_of: chrono::NaiveDate,
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
        let ac = lines
            .iter()
            .filter(|l| l.currency == b.currency)
            .fold(0_i64, |a, l| a.saturating_add(l.actual_minor));
        if let Some(block) = ev::block(&periods, pc.numerator, pc.denominator, Some(ac), as_of) {
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

/// `GET /api/plans/{pid}/earned-value` — the plan's earned-value figures today.
#[debug_handler]
async fn earned_value(State(ctx): State<AppContext>, Path(pid): Path<String>) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let as_of = chrono::Utc::now().date_naive();
    let body = match compute(&ctx, plan.pid, as_of).await? {
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
            "ac_source": "budget_line_actuals",
            "note": "Measured as of today: actuals carry no date and intermediate task progress is not stored by date, so the past is not reconstructed. Planned value is linear within each baseline period. Percent complete is physical (work finished), not elapsed time. Cost per currency is never added across currencies. Actual cost is the recorded budget-line actuals; timesheets x a rate is not offered until cost rates exist.",
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
    AuditModel::record(
        &ctx.db,
        plan.pid,
        "earned_value_method_set",
        caller.actor(),
        Some(serde_json::json!({ "from": was, "to": b.default_method })),
    )
    .await
    .ok();
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
    let t_pid = Uuid::parse_str(&t_pid).map_err(|_| Error::NotFound)?;
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
    AuditModel::record(
        &ctx.db,
        row.pid,
        "percent_complete_method_set",
        caller.actor(),
        Some(
            serde_json::json!({ "from": { "method": was.0, "percent": was.1 },
                                  "to": { "method": b.method, "percent": manual, "note": note } }),
        ),
    )
    .await
    .ok();
    format::json(
        serde_json::json!({ "task_pid": row.pid, "method": row.percent_method,
        "percent": row.percent_manual, "note": row.percent_note }),
    )
}

/// The earned-value routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/earned-value", get(earned_value))
        .add("/plans/{pid}/earned-value-settings", put(put_settings))
        .add(
            "/plans/{pid}/tasks/{t_pid}/percent-complete",
            put(put_percent),
        )
}
