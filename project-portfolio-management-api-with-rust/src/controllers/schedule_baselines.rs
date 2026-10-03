//! Schedule baselines (SC-3): numbered, append-only snapshots of a plan's task
//! schedule, and variance against them. Pure rules in [`crate::baselines`].
//!
//! **Append-only.** There is no update and no delete: a re-baseline is the next
//! version with a reason, and the earlier one stays. A bounded number are kept per
//! plan (11 unless `PROJECT_PORTFOLIO_MANAGEMENT_MAX_SCHEDULE_BASELINES` says
//! otherwise); at the limit a new one is refused with `409` rather than silently
//! overwriting history.
//!
//! The snapshot is the **critical-path engine's early dates** (SC-1) at that
//! moment, so a baseline and the variance against it are measured on the same
//! footing, in working days of the plan's calendar.

use axum::http::StatusCode;
use chrono::Utc;
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use sea_orm::{QueryOrder, TransactionTrait};
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::MaybeAuthUser;
use crate::baselines::{self, Snap};
use crate::models::_entities::{schedule_baseline_tasks, schedule_baselines};
use crate::models::audit_logs::Model as AuditModel;

fn unprocessable(message: &str) -> Error {
    Error::CustomError(
        StatusCode::UNPROCESSABLE_ENTITY,
        ErrorDetail::new("unprocessable_entity", message),
    )
}

fn conflict(message: &str) -> Error {
    Error::CustomError(StatusCode::CONFLICT, ErrorDetail::new("conflict", message))
}

fn db_err(e: sea_orm::DbErr) -> Error {
    Error::Model(ModelError::from(e))
}

fn max_baselines() -> i64 {
    std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_MAX_SCHEDULE_BASELINES")
        .ok()
        .and_then(|v| v.trim().parse::<i64>().ok())
        .filter(|n| *n >= 1)
        .unwrap_or(baselines::DEFAULT_MAX_BASELINES)
}

/// The plan's current snapshot: one [`Snap`] per live task, its early dates when
/// the engine could place it, `None` when it could not (with the reason).
async fn current(
    ctx: &AppContext,
    plan_pid: Uuid,
) -> Result<(
    Vec<(Snap, Option<i32>, Option<&'static str>)>,
    Option<chrono::NaiveDate>,
    crate::cpm::WorkCalendar,
    &'static str,
)> {
    let net = super::cpm::load_network(ctx, plan_pid).await?;
    let result = net.analyze(5).map_err(|crate::cpm::CpmError::Cycle(on)| {
        unprocessable(&format!(
            "the dependencies contain a cycle, so there is no schedule to baseline (on or behind it: {})",
            on.iter().map(ToString::to_string).collect::<Vec<_>>().join(", ")
        ))
    })?;
    let snaps = net
        .rows
        .iter()
        .map(|t| {
            let placed = result.nodes.iter().find(|n| n.pid == t.pid);
            let why = result
                .unscheduled
                .iter()
                .find(|u| u.pid == t.pid)
                .map(|u| u.reason);
            (
                Snap {
                    task_pid: t.pid,
                    title: t.title.clone(),
                    start: placed.map(|n| n.early_start),
                    finish: placed.map(|n| n.early_finish),
                    duration: placed.and(t.duration_days.map(i64::from)),
                },
                placed.and_then(|n| i32::try_from(n.total_float).ok()),
                why,
            )
        })
        .collect();
    Ok((snaps, result.project_finish, net.plan_cal, net.cal_source))
}

/// `POST …/schedule-baselines` body.
#[derive(Debug, Deserialize)]
struct BaselineBody {
    /// Why the baseline is being taken (required, up to 500 characters).
    reason: String,
}

fn view(b: &schedule_baselines::Model) -> serde_json::Value {
    serde_json::json!({
        "pid": b.pid, "version": b.version, "reason": b.reason, "created_by": b.created_by,
        "created_at": b.created_at, "task_count": b.task_count, "project_finish": b.project_finish,
    })
}

/// `POST /api/plans/{pid}/schedule-baselines` — take the next baseline.
#[debug_handler]
async fn create(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<BaselineBody>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let reason = b.reason.trim().to_string();
    if reason.is_empty() || reason.chars().count() > 500 {
        return Err(unprocessable(
            "a baseline needs a reason, up to 500 characters",
        ));
    }
    let (snaps, finish, _, _) = current(&ctx, plan.pid).await?;
    if finish.is_none() {
        return Err(unprocessable(
            "nothing is scheduled yet, so there is nothing to baseline: give tasks a duration and a start date",
        ));
    }
    let existing = schedule_baselines::Entity::find()
        .filter(schedule_baselines::Column::PlanPid.eq(plan.pid))
        .order_by_desc(schedule_baselines::Column::Version)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    if i64::try_from(existing.len()).unwrap_or(i64::MAX) >= max_baselines() {
        return Err(conflict(&format!(
            "this plan already holds {} baselines, the most it keeps; baselines are append-only and are never overwritten",
            existing.len()
        )));
    }
    let version = existing.first().map_or(1, |l| l.version + 1);
    let txn = ctx.db.begin().await.map_err(db_err)?;
    let row = schedule_baselines::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        plan_pid: ActiveValue::set(plan.pid),
        version: ActiveValue::set(version),
        reason: ActiveValue::set(reason.clone()),
        created_by: ActiveValue::set(caller.actor().map(str::to_string)),
        task_count: ActiveValue::set(i32::try_from(snaps.len()).unwrap_or(i32::MAX)),
        project_finish: ActiveValue::set(finish),
        ..Default::default()
    }
    .insert(&txn)
    .await
    .map_err(|e| {
        // A concurrent baseline took the same version: the unique index says so.
        if e.to_string().contains("schedule_baselines_version") {
            conflict("another baseline was taken at the same moment; try again")
        } else {
            db_err(e)
        }
    })?;
    for (s, total_float, why) in &snaps {
        schedule_baseline_tasks::ActiveModel {
            pid: ActiveValue::set(Uuid::new_v4()),
            baseline_pid: ActiveValue::set(row.pid),
            task_pid: ActiveValue::set(s.task_pid),
            title: ActiveValue::set(s.title.clone()),
            start_date: ActiveValue::set(s.start),
            finish_date: ActiveValue::set(s.finish),
            duration_days: ActiveValue::set(s.duration.and_then(|d| i32::try_from(d).ok())),
            total_float: ActiveValue::set(*total_float),
            unscheduled_reason: ActiveValue::set(why.map(str::to_string)),
            ..Default::default()
        }
        .insert(&txn)
        .await
        .map_err(db_err)?;
    }
    txn.commit().await.map_err(db_err)?;
    AuditModel::record(
        &ctx.db,
        row.pid,
        "schedule_baselined",
        caller.actor(),
        Some(
            serde_json::json!({ "plan_pid": plan.pid, "version": version, "reason": reason,
                                  "tasks": snaps.len(), "at": Utc::now() }),
        ),
    )
    .await
    .ok();
    format::json(view(&row))
}

/// `GET /api/plans/{pid}/schedule-baselines` — the plan's baselines, newest first.
#[debug_handler]
async fn list(State(ctx): State<AppContext>, Path(pid): Path<String>) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let rows = schedule_baselines::Entity::find()
        .filter(schedule_baselines::Column::PlanPid.eq(plan.pid))
        .order_by_desc(schedule_baselines::Column::Version)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    format::json(rows.iter().map(view).collect::<Vec<_>>())
}

#[derive(Debug, Deserialize)]
struct VarianceParams {
    /// A baseline version; defaults to the latest.
    #[serde(default)]
    baseline: Option<i32>,
}

/// `GET /api/plans/{pid}/schedule-variance?baseline=` — how the schedule now
/// differs from a baseline.
#[debug_handler]
async fn variance(
    State(ctx): State<AppContext>,
    Path(pid): Path<String>,
    Query(p): Query<VarianceParams>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let mut q =
        schedule_baselines::Entity::find().filter(schedule_baselines::Column::PlanPid.eq(plan.pid));
    q = match p.baseline {
        Some(v) => q.filter(schedule_baselines::Column::Version.eq(v)),
        None => q.order_by_desc(schedule_baselines::Column::Version),
    };
    let Some(base) = q.one(&ctx.db).await.map_err(db_err)? else {
        if p.baseline.is_some() {
            return Err(Error::NotFound);
        }
        // No baseline at all: variance is unknown, not zero.
        return format::json(serde_json::json!({
            "baseline": null, "variance": null, "reason": "no_baseline",
            "note": "no baseline has been taken for this plan, so nothing can be said about slip",
        }));
    };
    let then_rows = schedule_baseline_tasks::Entity::find()
        .filter(schedule_baseline_tasks::Column::BaselinePid.eq(base.pid))
        .order_by_asc(schedule_baseline_tasks::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let then: Vec<Snap> = then_rows
        .iter()
        .map(|t| Snap {
            task_pid: t.task_pid,
            title: t.title.clone(),
            start: t.start_date,
            finish: t.finish_date,
            duration: t.duration_days.map(i64::from),
        })
        .collect();
    let (now, now_finish, cal, source) = current(&ctx, plan.pid).await?;
    let now: Vec<Snap> = now.into_iter().map(|(s, _, _)| s).collect();
    let v = baselines::variance(&then, &now, base.project_finish, now_finish, &cal);
    format::json(serde_json::json!({
        "baseline": view(&base),
        "calendar": source,
        "variance": v,
        "note": "Variance is in working days of the plan's calendar, measured on the critical-path engine's early dates. A task added since the baseline is `added` (not slip); one removed since stays listed with its baselined dates; a task unscheduled on either side has no variance (null, with the reason), never zero.",
    }))
}

/// The schedule-baseline routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/schedule-baselines", post(create))
        .add("/plans/{pid}/schedule-baselines", get(list))
        .add("/plans/{pid}/schedule-variance", get(variance))
}
