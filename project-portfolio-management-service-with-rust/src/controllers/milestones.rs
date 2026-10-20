//! Milestones as a concept: a dated promise that can be tied to the schedule.
//! The rules are [`crate::milestones`]; this is the wiring.
//!
//! * `GET /api/milestones` — every milestone across the plans the caller can
//!   read, with derived status, achievement, schedule forecast and slip;
//! * `GET /api/milestones/{pid}` — one, with its linked tasks and slip history;
//! * `PUT …/milestones/{m_pid}/tasks` — the tasks a milestone waits for, and
//!   whether it completes from them (`linked`) or by hand (`manual`);
//! * `POST …/milestones/{m_pid}/reopen` — reopen a milestone marked done, with a
//!   reason. (Completing and rescheduling stay where they were, in
//!   `visibility.rs`, and now write the completion reason and the slip history.)
//!
//! Everything shown here is **derived on read** from the milestone, its linked
//! tasks and the critical-path analysis; the only stored facts are the links, the
//! manual completion and the append-only reschedule history.

use super::db_err;
use std::collections::{BTreeMap, HashMap, HashSet};

use axum::http::StatusCode;
use chrono::NaiveDate;
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use sea_orm::{QueryOrder, TransactionTrait};
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::MaybeAuthUser;
use crate::milestones::{self as rules, Achieved, Mode, ScheduleState, Slip};
use crate::models::_entities::{milestone_slips, milestone_tasks, milestones, tasks};
use crate::models::audit_logs::Model as AuditModel;
use crate::models::visibility as vis_models;
use crate::workflow::{Category, category_of};

/// Most tasks one milestone may wait for.
const MAX_LINKS: usize = 100;
/// Default "due soon" window, in calendar days.
const DUE_SOON_DAYS: i64 = 14;
/// Default risk window, in business days between the forecast and the due date.
const AT_RISK_DAYS: i64 = 5;

fn unprocessable(message: &str) -> Error {
    Error::CustomError(
        StatusCode::UNPROCESSABLE_ENTITY,
        ErrorDetail::new("unprocessable_entity", message),
    )
}

fn conflict(message: &str) -> Error {
    Error::CustomError(StatusCode::CONFLICT, ErrorDetail::new("conflict", message))
}

/// The tasks each milestone waits for, as pids.
async fn links_of(ctx: &AppContext, milestone_pids: &[Uuid]) -> Result<HashMap<Uuid, Vec<Uuid>>> {
    let mut out: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
    if milestone_pids.is_empty() {
        return Ok(out);
    }
    for l in milestone_tasks::Entity::find()
        .filter(milestone_tasks::Column::MilestonePid.is_in(milestone_pids.to_vec()))
        .order_by_asc(milestone_tasks::Column::Id)
        .all(&ctx.db)
        .await
        .map_err(db_err)?
    {
        out.entry(l.milestone_pid).or_default().push(l.task_pid);
    }
    Ok(out)
}

/// Whether each linked task of a milestone is done (a deleted task no longer
/// counts as waited for).
pub(crate) async fn linked_progress(ctx: &AppContext, m: &milestones::Model) -> Result<Vec<bool>> {
    let links = links_of(ctx, &[m.pid]).await?;
    let pids = links.get(&m.pid).cloned().unwrap_or_default();
    if pids.is_empty() {
        return Ok(Vec::new());
    }
    let workflow = super::workflow::in_force(ctx, m.plan_pid, "task").await?;
    Ok(tasks::Entity::find()
        .filter(tasks::Column::Pid.is_in(pids))
        .filter(tasks::Column::DeletedAt.is_null())
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .iter()
        .map(|t| matches!(category_of(&workflow, &t.status), Some(Category::Done)))
        .collect())
}

/// The subset of `rows` (linked milestones) whose every linked task is done:
/// used by the due-date sweep so it does not chase an achieved milestone.
pub(crate) async fn achieved_by_tasks(
    ctx: &AppContext,
    rows: &[milestones::Model],
) -> Result<HashSet<Uuid>> {
    let mut out = HashSet::new();
    for m in rows.iter().filter(|m| m.completion_mode == "linked") {
        let progress = linked_progress(ctx, m).await?;
        if rules::achieved(Mode::Linked, false, &progress) == Achieved::Tasks {
            out.insert(m.pid);
        }
    }
    Ok(out)
}

/// Everything derived for a set of milestones.
pub(crate) struct Derived {
    pub(crate) json: Vec<(NaiveDate, i64, serde_json::Value)>,
}

#[allow(clippy::too_many_lines)] // one linear pipeline: load, classify, forecast, shape
pub(crate) async fn derive(
    ctx: &AppContext,
    rows: &[milestones::Model],
    today: NaiveDate,
    due_soon: i64,
    at_risk: i64,
    with_history: bool,
) -> Result<Derived> {
    let pids: Vec<Uuid> = rows.iter().map(|m| m.pid).collect();
    let links = links_of(ctx, &pids).await?;
    let mut all_tasks: Vec<Uuid> = links.values().flatten().copied().collect();
    all_tasks.sort();
    all_tasks.dedup();
    let task_rows: HashMap<Uuid, tasks::Model> = if all_tasks.is_empty() {
        HashMap::new()
    } else {
        tasks::Entity::find()
            .filter(tasks::Column::Pid.is_in(all_tasks))
            .filter(tasks::Column::DeletedAt.is_null())
            .all(&ctx.db)
            .await
            .map_err(db_err)?
            .into_iter()
            .map(|t| (t.pid, t))
            .collect()
    };
    let slips: Vec<milestone_slips::Model> = if pids.is_empty() {
        Vec::new()
    } else {
        milestone_slips::Entity::find()
            .filter(milestone_slips::Column::MilestonePid.is_in(pids.clone()))
            .order_by_asc(milestone_slips::Column::Id)
            .all(&ctx.db)
            .await
            .map_err(db_err)?
    };

    // One workflow and one critical-path analysis per plan that needs them.
    let mut workflows: HashMap<Uuid, crate::workflow::WorkflowDef> = HashMap::new();
    let mut networks: HashMap<Uuid, Option<(super::cpm::Network, crate::cpm::CpmResult)>> =
        HashMap::new();
    let plan_pids: HashSet<Uuid> = rows.iter().map(|m| m.plan_pid).collect();
    for plan in &plan_pids {
        if rows
            .iter()
            .any(|m| m.plan_pid == *plan && links.contains_key(&m.pid))
        {
            workflows.insert(*plan, super::workflow::in_force(ctx, *plan, "task").await?);
        }
    }
    let complete = |plan: Uuid, t: &tasks::Model| {
        workflows
            .get(&plan)
            .is_some_and(|w| matches!(category_of(w, &t.status), Some(Category::Done)))
    };

    let mut out = Vec::with_capacity(rows.len());
    for m in rows {
        let mode = Mode::parse(&m.completion_mode).unwrap_or(Mode::Manual);
        let linked: Vec<&tasks::Model> = links
            .get(&m.pid)
            .map(|p| p.iter().filter_map(|t| task_rows.get(t)).collect())
            .unwrap_or_default();
        let progress: Vec<bool> = linked.iter().map(|t| complete(m.plan_pid, t)).collect();
        let how = rules::achieved(mode, m.done, &progress);
        let is_achieved = how != Achieved::No;
        let achieved_on = match how {
            Achieved::Tasks => linked
                .iter()
                .filter_map(|t| t.done_at.map(|d| d.date_naive()))
                .max(),
            Achieved::Manual => m.done_at.map(|d| d.date_naive()),
            Achieved::No => None,
        };
        let status = rules::status(is_achieved, m.due, today, due_soon);

        // Forecast from the critical-path analysis of the milestone's plan.
        let open: Vec<&&tasks::Model> =
            linked.iter().filter(|t| !complete(m.plan_pid, t)).collect();
        let mut schedule = serde_json::json!({ "state": null, "forecast": null, "slack_days": null,
            "reason": null, "partial": false, "unforecast_tasks": [] });
        let mut link_view: BTreeMap<Uuid, serde_json::Value> = BTreeMap::new();
        for t in &linked {
            link_view.insert(
                t.pid,
                serde_json::json!({ "task_pid": t.pid, "title": t.title,
                "complete": complete(m.plan_pid, t), "early_finish": null, "total_float": null }),
            );
        }
        if is_achieved {
            schedule["reason"] = serde_json::json!("achieved");
        } else if open.is_empty() {
            schedule["state"] = serde_json::json!(ScheduleState::NoForecast);
            schedule["reason"] = serde_json::json!("no_linked_tasks");
        } else {
            if let std::collections::hash_map::Entry::Vacant(slot) = networks.entry(m.plan_pid) {
                let loaded = match super::cpm::load_network(ctx, m.plan_pid).await {
                    Ok(net) => net.analyze(0).ok().map(|r| (net, r)),
                    Err(_) => None,
                };
                slot.insert(loaded);
            }
            match networks.get(&m.plan_pid).and_then(Option::as_ref) {
                None => {
                    schedule["state"] = serde_json::json!(ScheduleState::NoForecast);
                    schedule["reason"] = serde_json::json!("cycle");
                }
                Some((net, result)) => {
                    let by_pid: HashMap<Uuid, &crate::cpm::NodeResult> =
                        result.nodes.iter().map(|n| (n.pid, n)).collect();
                    let mut finish: Option<NaiveDate> = None;
                    let mut unforecast = Vec::new();
                    for t in &open {
                        if let Some(n) = by_pid.get(&t.pid) {
                            finish = finish.max(Some(n.early_finish));
                            if let Some(v) = link_view.get_mut(&t.pid) {
                                v["early_finish"] = serde_json::json!(n.early_finish);
                                v["total_float"] = serde_json::json!(n.total_float);
                            }
                        } else {
                            unforecast.push(t.pid);
                        }
                    }
                    match finish {
                        None => {
                            schedule["state"] = serde_json::json!(ScheduleState::NoForecast);
                            schedule["reason"] = serde_json::json!("no_scheduled_tasks");
                        }
                        Some(f) => {
                            let slack = net.plan_cal.index_of(m.due).unwrap_or(0)
                                - net.plan_cal.index_of(f).unwrap_or(0);
                            schedule["state"] =
                                serde_json::json!(rules::schedule_state(slack, at_risk));
                            schedule["forecast"] = serde_json::json!(f);
                            schedule["slack_days"] = serde_json::json!(slack);
                            schedule["partial"] = serde_json::json!(!unforecast.is_empty());
                        }
                    }
                    schedule["unforecast_tasks"] = serde_json::json!(unforecast);
                }
            }
        }

        let history: Vec<&milestone_slips::Model> =
            slips.iter().filter(|s| s.milestone_pid == m.pid).collect();
        let summary = rules::slip_summary(
            &history
                .iter()
                .map(|s| Slip {
                    from: s.from_due,
                    to: s.to_due,
                })
                .collect::<Vec<_>>(),
            m.due,
        );
        let mut v = serde_json::json!({
            "pid": m.pid,
            "plan_pid": m.plan_pid,
            "name": m.name,
            "kind": m.kind.as_deref().unwrap_or("milestone"),
            "due": m.due,
            "completion_mode": mode,
            "achieved": is_achieved,
            "achieved_by": match how { Achieved::No => None, Achieved::Manual => Some("manual"), Achieved::Tasks => Some("tasks") },
            "achieved_on": achieved_on,
            "done_reason": m.done_reason,
            "status": status,
            "schedule": schedule,
            "linked_tasks": linked.len(),
            "open_tasks": open.len(),
            "links": link_view.into_values().collect::<Vec<_>>(),
            "slip": summary,
        });
        if with_history {
            v["history"] = serde_json::json!(
                history
                    .iter()
                    .map(|s| serde_json::json!({
                "from": s.from_due, "to": s.to_due, "reason": s.reason, "actor": s.actor,
                "at": s.created_at }))
                    .collect::<Vec<_>>()
            );
        }
        out.push((m.due, m.id, v));
    }
    // Partner sign-off (EX-4): delivered keeps its meaning; met is separate. Only a milestone that requires
    // sign-off carries a `signoff` object.
    let delivered: HashMap<Uuid, Option<NaiveDate>> = out
        .iter()
        .filter_map(|(_, _, v)| {
            let pid = Uuid::parse_str(v["pid"].as_str()?).ok()?;
            let on = v["achieved_on"]
                .as_str()
                .and_then(|s| s.parse().ok())
                .or_else(|| {
                    v["achieved"]
                        .as_bool()
                        .filter(|a| *a)
                        .and(v["due"].as_str().and_then(|s| s.parse().ok()))
                });
            Some((pid, on))
        })
        .collect();
    let assessed = super::milestone_signoffs::assessments(ctx, &delivered).await?;
    for (_, _, v) in &mut out {
        if let Some(a) = v["pid"]
            .as_str()
            .and_then(|s| Uuid::parse_str(s).ok())
            .and_then(|p| assessed.get(&p))
        {
            v["signoff"] = super::milestone_signoffs::assessment_json(a);
        }
    }
    out.sort_by_key(|(due, id, _)| (*due, *id));
    Ok(Derived { json: out })
}

#[derive(Debug, Deserialize)]
struct ListParams {
    /// Only this plan's milestones.
    #[serde(default)]
    plan: Option<Uuid>,
    /// `achieved`, `late`, `due_soon` or `upcoming`.
    #[serde(default)]
    status: Option<String>,
    /// A milestone kind.
    #[serde(default)]
    kind: Option<String>,
    /// Due on or after.
    #[serde(default)]
    from: Option<NaiveDate>,
    /// Due on or before.
    #[serde(default)]
    to: Option<NaiveDate>,
    /// "Due soon" window in calendar days (default 14).
    #[serde(default)]
    due_soon_days: Option<i64>,
    /// Business days of slack under which a forecast is at risk (default 5).
    #[serde(default)]
    at_risk_days: Option<i64>,
    /// The date "today" means (default today).
    #[serde(default)]
    as_of: Option<NaiveDate>,
}

fn window(v: Option<i64>, default: i64, name: &str) -> Result<i64> {
    let v = v.unwrap_or(default);
    if (0..=3650).contains(&v) {
        Ok(v)
    } else {
        Err(unprocessable(&format!("{name} must be 0 to 3650")))
    }
}

/// `GET /api/milestones` — the portfolio view.
#[debug_handler]
async fn list(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(q): Query<ListParams>,
) -> Result<Response> {
    let due_soon = window(q.due_soon_days, DUE_SOON_DAYS, "due_soon_days")?;
    let at_risk = window(q.at_risk_days, AT_RISK_DAYS, "at_risk_days")?;
    let wanted = match q.status.as_deref() {
        None => None,
        Some(s @ ("achieved" | "late" | "due_soon" | "upcoming")) => Some(s),
        Some(_) => {
            return Err(unprocessable(
                "status must be achieved, late, due_soon or upcoming",
            ));
        }
    };
    if let (Some(a), Some(b)) = (q.from, q.to)
        && b < a
    {
        return Err(unprocessable("to is before from"));
    }
    let today = q.as_of.unwrap_or_else(|| chrono::Utc::now().date_naive());
    let mut query = milestones::Entity::find().filter(milestones::Column::DeletedAt.is_null());
    if let Some(p) = q.plan {
        query = query.filter(milestones::Column::PlanPid.eq(p));
    }
    if let Some(k) = &q.kind {
        query = query.filter(milestones::Column::Kind.eq(k.as_str()));
    }
    if let Some(f) = q.from {
        query = query.filter(milestones::Column::Due.gte(f));
    }
    if let Some(t) = q.to {
        query = query.filter(milestones::Column::Due.lte(t));
    }
    let mut rows = query
        .order_by_asc(milestones::Column::Due)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    // Plans the caller cannot read are left out without a trace.
    let mut plans: Vec<Uuid> = rows.iter().map(|m| m.plan_pid).collect();
    plans.sort();
    plans.dedup();
    let readable = super::plan_access::readable_plans(&ctx, &caller, &plans).await;
    rows.retain(|m| readable.contains(&m.plan_pid));

    let derived = derive(&ctx, &rows, today, due_soon, at_risk, false).await?;
    let mut by_status: BTreeMap<&str, usize> = [
        ("achieved", 0),
        ("late", 0),
        ("due_soon", 0),
        ("upcoming", 0),
    ]
    .into_iter()
    .collect();
    let mut by_state: BTreeMap<String, usize> = BTreeMap::new();
    let mut shown = Vec::new();
    for (_, _, v) in derived.json {
        let s = v["status"].as_str().unwrap_or("upcoming").to_string();
        if let Some(c) = by_status.get_mut(s.as_str()) {
            *c += 1;
        }
        if let Some(state) = v["schedule"]["state"].as_str() {
            *by_state.entry(state.to_string()).or_default() += 1;
        }
        if wanted.is_none_or(|w| w == s) {
            shown.push(v);
        }
    }
    format::json(serde_json::json!({
        "as_of": today,
        "due_soon_days": due_soon,
        "at_risk_days": at_risk,
        "milestones": shown,
        "summary": { "by_status": by_status, "by_schedule_state": by_state },
        "note": "Status is against the calendar: achieved, late (past due and not achieved), due_soon (within due_soon_days) or upcoming. A linked milestone is achieved when every task it waits for is done; a manual one when marked. schedule.state comes from the critical-path forecast of the tasks still open: forecast_late when they finish after the due date, at_risk when they finish within at_risk_days business days before it, on_track otherwise; a milestone with nothing to forecast from is no_forecast with a reason, never on_track. slip reads the append-only reschedule history. The summary counts every milestone visible to the caller, whatever the status filter.",
    }))
}

/// `GET /api/milestones/{pid}` — one milestone, its linked tasks and its slip history.
#[debug_handler]
async fn get_one(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
) -> Result<Response> {
    let pid = Uuid::parse_str(&pid).map_err(|_| Error::NotFound)?;
    let m = vis_models::find_milestone(&ctx.db, pid).await?;
    let readable = super::plan_access::readable_plans(&ctx, &caller, &[m.plan_pid]).await;
    if !readable.contains(&m.plan_pid) {
        return Err(Error::NotFound);
    }
    let today = chrono::Utc::now().date_naive();
    let derived = derive(&ctx, &[m], today, DUE_SOON_DAYS, AT_RISK_DAYS, true).await?;
    let (_, _, v) = derived.json.into_iter().next().ok_or(Error::NotFound)?;
    format::json(v)
}

/// `PUT …/milestones/{m_pid}/tasks` body.
#[derive(Debug, Deserialize)]
struct LinksBody {
    /// The tasks the milestone waits for (replaces the set).
    task_pids: Vec<Uuid>,
    /// `manual` or `linked`; by default `linked` when there are tasks, else `manual`.
    #[serde(default)]
    completion_mode: Option<String>,
}

/// `PUT /api/plans/{pid}/milestones/{m_pid}/tasks` — set the tasks a milestone
/// waits for, and whether it is completed from them.
#[debug_handler]
async fn set_links(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, m_pid)): Path<(String, String)>,
    Json(b): Json<LinksBody>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let m_pid = Uuid::parse_str(&m_pid).map_err(|_| Error::NotFound)?;
    let m = vis_models::find_milestone(&ctx.db, m_pid).await?;
    if m.plan_pid != plan.pid {
        return Err(Error::NotFound);
    }
    let mut seen = HashSet::new();
    if !b.task_pids.iter().all(|t| seen.insert(*t)) {
        return Err(unprocessable("a task may appear once"));
    }
    if b.task_pids.len() > MAX_LINKS {
        return Err(unprocessable(&format!("at most {MAX_LINKS} tasks")));
    }
    let mode = match b.completion_mode.as_deref() {
        None if b.task_pids.is_empty() => Mode::Manual,
        None => Mode::Linked,
        Some(s) => Mode::parse(s)
            .ok_or_else(|| unprocessable("completion_mode must be manual or linked"))?,
    };
    if mode == Mode::Linked && b.task_pids.is_empty() {
        return Err(unprocessable(
            "a linked milestone must wait for at least one task",
        ));
    }
    if !b.task_pids.is_empty() {
        let found = tasks::Entity::find()
            .filter(tasks::Column::Pid.is_in(b.task_pids.clone()))
            .filter(tasks::Column::PlanPid.eq(plan.pid))
            .filter(tasks::Column::DeletedAt.is_null())
            .all(&ctx.db)
            .await
            .map_err(db_err)?
            .len();
        if found != b.task_pids.len() {
            return Err(unprocessable(
                "every task must be a live task of this milestone's plan",
            ));
        }
    }
    let was = links_of(&ctx, &[m.pid])
        .await?
        .remove(&m.pid)
        .unwrap_or_default();
    let was_mode = m.completion_mode.clone();
    let txn = ctx.db.begin().await.map_err(db_err)?;
    milestone_tasks::Entity::delete_many()
        .filter(milestone_tasks::Column::MilestonePid.eq(m.pid))
        .exec(&txn)
        .await
        .map_err(db_err)?;
    for t in &b.task_pids {
        milestone_tasks::ActiveModel {
            pid: ActiveValue::set(Uuid::new_v4()),
            milestone_pid: ActiveValue::set(m.pid),
            task_pid: ActiveValue::set(*t),
            ..Default::default()
        }
        .insert(&txn)
        .await
        .map_err(db_err)?;
    }
    let mut a: milestones::ActiveModel = m.clone().into();
    a.completion_mode = ActiveValue::set(match mode {
        Mode::Manual => "manual".to_string(),
        Mode::Linked => "linked".to_string(),
    });
    a.update(&txn).await.map_err(db_err)?;
    txn.commit().await.map_err(db_err)?;
    AuditModel::record(
        &ctx.db,
        m.pid,
        "milestone_links_set",
        caller.actor(),
        Some(
            serde_json::json!({ "from": { "tasks": was, "mode": was_mode },
            "to": { "tasks": b.task_pids, "mode": mode } }),
        ),
    )
    .await
    .ok();
    format::json(
        serde_json::json!({ "pid": m.pid, "completion_mode": mode, "task_pids": b.task_pids }),
    )
}

/// `POST …/milestones/{m_pid}/reopen` body.
#[derive(Debug, Deserialize)]
struct ReopenBody {
    /// Why it is being reopened (required).
    reason: String,
}

/// `POST /api/plans/{pid}/milestones/{m_pid}/reopen` — undo a manual completion.
/// A reason is required and audited. A milestone achieved only because its linked
/// tasks are done cannot be reopened here: reopen a task.
#[debug_handler]
async fn reopen(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, m_pid)): Path<(String, String)>,
    Json(b): Json<ReopenBody>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let m_pid = Uuid::parse_str(&m_pid).map_err(|_| Error::NotFound)?;
    let m = vis_models::find_milestone(&ctx.db, m_pid).await?;
    if m.plan_pid != plan.pid {
        return Err(Error::NotFound);
    }
    let reason = b.reason.trim();
    if reason.is_empty() || reason.chars().count() > 500 {
        return Err(unprocessable("a reason is required (up to 500 characters)"));
    }
    let progress = linked_progress(&ctx, &m).await?;
    let mode = Mode::parse(&m.completion_mode).unwrap_or(Mode::Manual);
    if !m.done {
        return Err(conflict(
            if rules::achieved(mode, false, &progress) == Achieved::Tasks {
                "this milestone is achieved because its linked tasks are done; reopen a task instead"
            } else {
                "this milestone is not marked done"
            },
        ));
    }
    let was = (m.done_at, m.done_reason.clone());
    let row_pid = m.pid;
    let mut a: milestones::ActiveModel = m.into();
    a.done = ActiveValue::set(false);
    a.done_at = ActiveValue::set(None);
    a.done_reason = ActiveValue::set(None);
    let row = a.update(&ctx.db).await.map_err(db_err)?;
    AuditModel::record(
        &ctx.db,
        row_pid,
        "milestone_reopened",
        caller.actor(),
        Some(serde_json::json!({ "reason": reason, "was_done_at": was.0, "was_reason": was.1 })),
    )
    .await
    .ok();
    let still = rules::achieved(mode, false, &progress) == Achieved::Tasks;
    format::json(serde_json::json!({ "pid": row.pid, "done": row.done, "achieved": still }))
}

/// The milestone routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/milestones", get(list))
        .add("/milestones/{pid}", get(get_one))
        .add("/plans/{pid}/milestones/{m_pid}/tasks", put(set_links))
        .add("/plans/{pid}/milestones/{m_pid}/reopen", post(reopen))
}
