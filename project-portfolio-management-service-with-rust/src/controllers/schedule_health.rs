//! Schedule health self-check (SC-9): the fourteen DCMA-style checks over a
//! plan's task network. The rules are [`crate::schedule_health`]; this loads the
//! network, the status date and the latest baseline, and shapes the answer.
//!
//! Named a *self-check* everywhere: it is not a DCMA compliance assessment.

use super::db_err;
use std::collections::HashMap;

use chrono::NaiveDate;
use loco_rs::prelude::*;
use sea_orm::QueryOrder;
use serde::Deserialize;
use uuid::Uuid;

use super::unprocessable_entity as unprocessable;
use crate::models::_entities::{schedule_baseline_tasks, schedule_baselines};
use crate::schedule_health::{self as health, Input, Task};
use crate::workflow::{Category, category_of};

const ENV: &str = "PROJECT_PORTFOLIO_MANAGEMENT_SCHEDULE_HEALTH_THRESHOLDS";

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct Params {
    /// The status date (default today).
    #[serde(default)]
    as_of: Option<NaiveDate>,
}

/// `GET /api/plans/{pid}/schedule-health` — run the fourteen checks.
#[debug_handler]
async fn schedule_health(
    State(ctx): State<AppContext>,
    Path(pid): Path<String>,
    Query(q): Query<Params>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let as_of = q.as_of.unwrap_or_else(crate::tz::deployment_today);
    if as_of < chrono::NaiveDate::from_ymd_opt(1970, 1, 5).unwrap_or_default()
        || chrono::Datelike::year(&as_of) > 2100
    {
        return Err(unprocessable(
            "as_of must be between 1970-01-05 and 2100-12-31",
        ));
    }
    let net = super::cpm::load_network(&ctx, plan.pid).await?;
    let workflow = super::workflow::in_force(&ctx, plan.pid, "task").await?;

    // The latest baseline, if any: each task's baselined finish.
    let baseline = schedule_baselines::Entity::find()
        .filter(schedule_baselines::Column::PlanPid.eq(plan.pid))
        .order_by_desc(schedule_baselines::Column::Version)
        .one(&ctx.db)
        .await
        .map_err(db_err)?;
    let finishes: HashMap<Uuid, NaiveDate> = match &baseline {
        None => HashMap::new(),
        Some(b) => schedule_baseline_tasks::Entity::find()
            .filter(schedule_baseline_tasks::Column::BaselinePid.eq(b.pid))
            .all(&ctx.db)
            .await
            .map_err(db_err)?
            .into_iter()
            .filter_map(|t| t.finish_date.map(|f| (t.task_pid, f)))
            .collect(),
    };

    let tasks: Vec<Task> = net
        .rows
        .iter()
        .map(|t| Task {
            pid: t.pid,
            complete: matches!(category_of(&workflow, &t.status), Some(Category::Done)),
            has_resource: t.assignee_ref.is_some(),
            done_on: t.done_at.map(|d| d.date_naive()),
            baseline_finish: finishes.get(&t.pid).copied(),
        })
        .collect();

    let (thresholds, defaulted) = health::parse_thresholds(std::env::var(ENV).ok().as_deref());
    let report = health::assess(
        &Input {
            tasks: &tasks,
            nodes: &net.nodes,
            edges: &net.edges,
            calendar: &net.plan_cal,
            own: &net.own,
            as_of,
            baseline_exists: baseline.is_some(),
        },
        &thresholds,
    );
    let titles: HashMap<Uuid, &str> = net.rows.iter().map(|t| (t.pid, t.title.as_str())).collect();
    let checks: Vec<serde_json::Value> = report
        .checks
        .iter()
        .map(|c| {
            let mut v = serde_json::to_value(c).unwrap_or_default();
            v["offenders"] = serde_json::json!(
                c.offenders
                    .iter()
                    .map(|p| serde_json::json!({
                "pid": p, "title": titles.get(p) }))
                    .collect::<Vec<_>>()
            );
            v
        })
        .collect();
    format::json(serde_json::json!({
        "name": "Schedule health self-check",
        "disclaimer": "This is a self-check in the style of the DCMA 14-Point Assessment. It is not a DCMA assessment and does not show DCMA compliance.",
        "preset": health::PRESET,
        "thresholds": { "values": thresholds, "source": if defaulted { "default" } else { ENV } },
        "plan_pid": plan.pid,
        "as_of": as_of,
        "calendar": net.cal_source,
        "baseline": baseline.as_ref().map(|b| serde_json::json!({ "version": b.version, "taken": b.created_at })),
        "tasks": tasks.len(),
        "checks": checks,
        "passed": report.passed,
        "failed": report.failed,
        "not_computable": report.not_computable,
        "all_pass": report.failed == 0 && report.not_computable == 0,
        "note": "A check that cannot be computed (no baseline, a dependency cycle, nothing incomplete or due to measure) has pass null and a reason, and never counts as a pass; all_pass is true only when all fourteen were computed and passed. Each check names its denominator in of_what. The preset's thresholds are the commonly published ones, checked against two secondary descriptions of the assessment rather than the DCMA pamphlet itself; PROJECT_PORTFOLIO_MANAGEMENT_SCHEDULE_HEALTH_THRESHOLDS (a JSON object) overrides any of them.",
    }))
}

/// The schedule-health routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/schedule-health", get(schedule_health))
}
