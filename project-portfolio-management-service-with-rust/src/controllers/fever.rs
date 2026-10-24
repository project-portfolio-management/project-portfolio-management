//! The buffer fever chart (DP-7). Rules in [`crate::fever`].
//!
//! **Two sources.** A plan with a frozen critical-chain buffer plan (CC-4) is read from that: its project
//! buffer, its chain and the forecast slip. A plan without one falls back to the stand-in: a declared buffer
//! and the baselined *critical path* (the baseline's tasks with no total float). Each entry says which
//! (`source`), and the pack says so.

use std::collections::{HashMap, HashSet};

use loco_rs::prelude::*;
use sea_orm::QueryOrder;
use serde::Deserialize;
use uuid::Uuid;

use super::db_err;
use super::unprocessable_entity as unprocessable;
use crate::auth::MaybeAuthUser;
use crate::fever::{self as rules, Absent, Lines};
use crate::models::_entities::{
    plans, programme_buffers as buffers, schedule_baseline_tasks, schedule_baselines,
};
use crate::models::audit_logs::Model as AuditModel;

/// The routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/project-buffer", get(get_buffer))
        .add("/plans/{pid}/project-buffer", put(set_buffer))
        .add("/buffer-fever", get(live))
}

const RULE: &str = "half_of_critical_duration";

/// The baselined critical work of a plan: `(baseline version, finish, critical tasks as (pid, days))`.
async fn baselined_chain(
    ctx: &AppContext,
    plan: Uuid,
) -> Result<Option<(schedule_baselines::Model, Vec<(Uuid, i64)>)>> {
    let Some(b) = schedule_baselines::Entity::find()
        .filter(schedule_baselines::Column::PlanPid.eq(plan))
        .order_by_desc(schedule_baselines::Column::Version)
        .one(&ctx.db)
        .await
        .map_err(db_err)?
    else {
        return Ok(None);
    };
    let tasks = schedule_baseline_tasks::Entity::find()
        .filter(schedule_baseline_tasks::Column::BaselinePid.eq(b.pid))
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let chain = tasks
        .into_iter()
        .filter(|t| t.total_float == Some(0))
        .filter_map(|t| {
            t.duration_days
                .filter(|d| *d > 0)
                .map(|d| (t.task_pid, i64::from(d)))
        })
        .collect();
    Ok(Some((b, chain)))
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct BufferBody {
    #[serde(default)]
    buffer_days: Option<i64>,
    #[serde(default)]
    rule: Option<String>,
    #[serde(default)]
    note: Option<String>,
}

/// `PUT /api/plans/{pid}/project-buffer` — declare a programme's project buffer in business days, or let
/// a stated rule give it (`rule: "half_of_critical_duration"`: half the baselined critical work, rounded
/// up). Append-only: the latest declaration is the buffer.
#[debug_handler]
async fn set_buffer(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<BufferBody>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    if plan.kind.as_deref() != Some("Program") {
        return Err(unprocessable(
            "only a plan of kind Program has a project buffer",
        ));
    }
    let chain = baselined_chain(&ctx, plan.pid).await?;
    let (days, source, rule) = match (b.buffer_days, b.rule.as_deref()) {
        (Some(d), None) if (1..=2000).contains(&d) => (d, "declared", None),
        (Some(_), None) => {
            return Err(unprocessable("buffer_days must be 1 to 2000 business days"));
        }
        (None, Some(RULE)) => {
            let Some((_, c)) = &chain else {
                return Err(unprocessable(
                    "the rule needs a schedule baseline to work from",
                ));
            };
            let total: i64 = c.iter().map(|(_, d)| d).sum();
            if total < 1 {
                return Err(unprocessable(
                    "the baseline has no critical work with a duration, so the rule gives no buffer",
                ));
            }
            (
                rules::half_of_critical(total),
                "rule",
                Some(RULE.to_string()),
            )
        }
        (None, Some(_)) => return Err(unprocessable("rule must be half_of_critical_duration")),
        _ => {
            return Err(unprocessable(
                "give either buffer_days or rule, not both and not neither",
            ));
        }
    };
    let row = buffers::ActiveModel {
        pid: ActiveValue::set(Uuid::new_v4()),
        plan_pid: ActiveValue::set(plan.pid),
        buffer_days: ActiveValue::set(i32::try_from(days).unwrap_or(i32::MAX)),
        source: ActiveValue::set(source.to_string()),
        rule: ActiveValue::set(rule),
        note: ActiveValue::set(
            b.note
                .map(|n| n.trim().to_string())
                .filter(|n| !n.is_empty()),
        ),
        declared_by: ActiveValue::set(caller.actor().map(str::to_string)),
        declared_on: ActiveValue::set(crate::tz::deployment_today()),
        baseline_version: ActiveValue::set(chain.as_ref().map(|(b, _)| b.version)),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(db_err)?;
    AuditModel::record(
        &ctx.db,
        plan.pid,
        "project_buffer_set",
        caller.actor(),
        Some(serde_json::json!({ "buffer_days": days, "source": source })),
    )
    .await
    .map_err(|e| Error::string(&e.to_string()))?;
    format::json(
        serde_json::json!({ "plan_pid": plan.pid, "buffer_days": row.buffer_days, "source": row.source }),
    )
}

/// `GET /api/plans/{pid}/project-buffer` — the buffer, its history, and the chart point now.
#[debug_handler]
async fn get_buffer(State(ctx): State<AppContext>, Path(pid): Path<String>) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let history = buffers::Entity::find()
        .filter(buffers::Column::PlanPid.eq(plan.pid))
        .order_by_asc(buffers::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let (lines, bad) = rules::configured_lines();
    let now = one(&ctx, &plan, &lines, None).await?;
    format::json(serde_json::json!({
        "plan_pid": plan.pid,
        "history": history.iter().map(|h| serde_json::json!({
            "buffer_days": h.buffer_days, "source": h.source, "rule": h.rule, "note": h.note,
            "declared_by": h.declared_by, "declared_on": h.declared_on,
            "baseline_version": h.baseline_version })).collect::<Vec<_>>(),
        "now": now, "lines": lines, "configured_lines_unusable": bad,
    }))
}

/// One programme's entry: the point, or why there is none.
async fn one(
    ctx: &AppContext,
    plan: &plans::Model,
    lines: &Lines,
    previous: Option<&serde_json::Value>,
) -> Result<serde_json::Value> {
    // A frozen critical-chain buffer plan supersedes the declared buffer and the baselined critical path (CC-4).
    if let Some((row, r)) = super::buffer_management::project_reading(ctx, plan.pid).await? {
        let mut entry = serde_json::json!({ "plan_pid": plan.pid, "plan": plan.name, "source": "critical_chain" });
        entry["buffer"] = serde_json::json!({
            "days": r.buffer_days, "source": "critical_chain", "rule": null, "declared_on": row.taken_on,
        });
        entry["chain"] =
            serde_json::json!({ "done_days": r.chain_done_days, "total_days": r.chain_total_days });
        if let Some(p) = r.point {
            entry["point"] = serde_json::to_value(&p).map_err(|e| Error::string(&e.to_string()))?;
            entry["previous"] = previous.cloned().unwrap_or(serde_json::Value::Null);
        } else {
            entry["point"] = serde_json::Value::Null;
            if let Some(why) = r.no_point {
                entry["no_fever_chart_point"] =
                    serde_json::json!({ "code": why, "reason": why.reason() });
            }
        }
        return Ok(entry);
    }
    let buffer = buffers::Entity::find()
        .filter(buffers::Column::PlanPid.eq(plan.pid))
        .order_by_desc(buffers::Column::Id)
        .one(&ctx.db)
        .await
        .map_err(db_err)?;
    let chain = baselined_chain(ctx, plan.pid).await?;
    let mut entry = serde_json::json!({ "plan_pid": plan.pid, "plan": plan.name });
    let absent = |why: Absent| serde_json::json!({ "code": why, "reason": why.reason() });
    let Some(buffer) = buffer else {
        entry["point"] = serde_json::Value::Null;
        entry["no_fever_chart_point"] = absent(Absent::NoBuffer);
        return Ok(entry);
    };
    let Some((baseline, critical)) = chain else {
        entry["point"] = serde_json::Value::Null;
        entry["no_fever_chart_point"] = absent(Absent::NoBaseline);
        entry["buffer_days"] = buffer.buffer_days.into();
        return Ok(entry);
    };
    let total: i64 = critical.iter().map(|(_, d)| d).sum();
    // Done work: the baselined critical tasks that are done now. A task deleted since counts as not done.
    let net = super::cpm::load_network(ctx, plan.pid).await?;
    let done_set: HashSet<Uuid> = net
        .rows
        .iter()
        .filter(|t| t.status == "done")
        .map(|t| t.pid)
        .collect();
    let done: i64 = critical
        .iter()
        .filter(|(p, _)| done_set.contains(p))
        .map(|(_, d)| d)
        .sum();
    // The forecast finish, against the baselined one, in business days of the plan's calendar.
    let forecast = super::schedule_baselines::current(ctx, plan.pid).await.ok();
    let slip = forecast.and_then(|(_, finish, cal, _)| {
        let (now, then) = (
            cal.index_of(finish?)?,
            cal.index_of(baseline.project_finish?)?,
        );
        Some(now - then)
    });
    entry["buffer"] = serde_json::json!({
        "days": buffer.buffer_days, "source": buffer.source, "rule": buffer.rule,
        "declared_on": buffer.declared_on,
    });
    entry["baseline_version"] = baseline.version.into();
    entry["chain"] = serde_json::json!({ "done_days": done, "total_days": total });
    let Some(slip) = slip else {
        entry["point"] = serde_json::Value::Null;
        entry["no_fever_chart_point"] = absent(Absent::NoForecast);
        return Ok(entry);
    };
    match rules::point(done, total, slip, i64::from(buffer.buffer_days), lines) {
        Ok(p) => {
            entry["point"] = serde_json::to_value(&p).map_err(|e| Error::string(&e.to_string()))?;
            entry["previous"] = previous.cloned().unwrap_or(serde_json::Value::Null);
        }
        Err(why) => {
            entry["point"] = serde_json::Value::Null;
            entry["no_fever_chart_point"] = absent(why);
        }
    }
    Ok(entry)
}

/// Each programme's point in an earlier pack's body, by plan.
fn previous_points(body: Option<&str>) -> HashMap<String, serde_json::Value> {
    let mut out = HashMap::new();
    let Some(v) = body.and_then(|b| serde_json::from_str::<serde_json::Value>(b).ok()) else {
        return out;
    };
    for p in v["buffer_fever"]["programmes"]
        .as_array()
        .into_iter()
        .flatten()
    {
        if let (Some(id), pt) = (p["plan_pid"].as_str(), &p["point"])
            && !pt.is_null()
        {
            out.insert(
                id.to_string(),
                serde_json::json!({
                    "chain_completed_pct": pt["chain_completed_pct"],
                    "buffer_consumed_pct": pt["buffer_consumed_pct"], "zone": pt["zone"] }),
            );
        }
    }
    out
}

/// The fever-chart section over the programmes in `include`, with the trail from `previous_body`.
///
/// # Errors
///
/// When a database read fails.
pub(crate) async fn fever_section(
    ctx: &AppContext,
    include: &HashSet<Uuid>,
    previous_body: Option<&str>,
) -> Result<serde_json::Value> {
    let (lines, bad) = rules::configured_lines();
    let prev = previous_points(previous_body);
    let mut progs: Vec<plans::Model> = plans::Entity::find()
        .filter(plans::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .filter(|p| p.kind.as_deref() == Some("Program") && include.contains(&p.pid))
        .collect();
    progs.sort_by(|a, b| a.name.cmp(&b.name).then(a.pid.cmp(&b.pid)));
    let mut entries = Vec::new();
    let mut table = Vec::new();
    for p in &progs {
        let e = one(ctx, p, &lines, prev.get(&p.pid.to_string())).await?;
        if !e["point"].is_null() {
            table.push(serde_json::json!({
                "plan": p.name,
                "chain_completed_pct": e["point"]["chain_completed_pct"],
                "buffer_consumed_pct": e["point"]["buffer_consumed_pct"],
                "zone": e["point"]["zone"], "edge": e["point"]["edge"],
                "slip_days": e["point"]["slip_days"], "buffer_days": e["point"]["buffer_days"],
                "previous_zone": e["previous"]["zone"],
            }));
        }
        entries.push(e);
    }
    Ok(serde_json::json!({
        "lines": lines,
        "configured_lines_unusable_default_used": bad,
        "programmes": entries,
        "table": table,
        "note": "x is the share of the baselined critical work that is done; y is the share of the project buffer used by the forecast slip against the baselined finish, in business days of the plan's calendar. A programme with a frozen critical-chain buffer plan (source critical_chain) is read from it; any other uses the declared buffer and the baselined critical path (tasks with no total float) as a stand-in. Below the lower line is green; a point on a line takes the kinder zone. Recovery (below 0 %) and overrun (above 100 %) are drawn at the edge with their true figure. A programme with no buffer, no baseline, no critical work or no forecast has no point, with the reason. The table lists every point; previous is the point in the last issued pack.",
    }))
}

/// `GET /api/buffer-fever` — the programmes you can read, now.
#[debug_handler]
async fn live(State(ctx): State<AppContext>, caller: MaybeAuthUser) -> Result<Response> {
    let all: Vec<Uuid> = plans::Entity::find()
        .filter(plans::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|p| p.pid)
        .collect();
    let readable = super::plan_access::readable_plans(&ctx, &caller, &all).await;
    format::json(fever_section(&ctx, &readable, None).await?)
}
