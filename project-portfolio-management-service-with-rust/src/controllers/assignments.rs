//! Assignments, scheduling rules and time-phased work (PJ-5, PJ-6). The maths is
//! [`crate::scheduling_rules`] and [`crate::work_profile`]; this is the wiring.
//!
//! * `PUT …/tasks/{pid}/assignment` — set a task's units / work / duration under a
//!   rule (which of the three stays fixed), the third being recomputed;
//! * `PUT …/tasks/{pid}/contour` — how the task's work is spread over its business days;
//! * `GET …/work-profile` and `GET /api/work-profile` — the work as buckets of days,
//!   weeks or months, task → plan → portfolio, **derived on read**.
//!
//! An assignment needs an assignee: units are a share of *their* day.

use super::db_err;
use std::collections::BTreeMap;

use chrono::NaiveDate;
use loco_rs::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

use super::unprocessable_entity as unprocessable;
use crate::auth::MaybeAuthUser;
use crate::models::_entities::{plans, tasks};
use crate::models::audit_logs::Model as AuditModel;
pub(crate) use crate::scheduling_rules::Recomputed;
use crate::scheduling_rules::{self as rules, Field, Rule, Triple};
use crate::work_profile::{self as wp, Contour, Grain, Unplaced};

fn field_name(f: Field) -> &'static str {
    match f {
        Field::Duration => "duration_days",
        Field::Units => "units_percent",
        Field::Work => "work_minutes",
    }
}

fn rule_of(t: &tasks::Model) -> Rule {
    t.scheduling_rule
        .as_deref()
        .and_then(Rule::parse)
        .unwrap_or(Rule::FixedUnits)
}

/// What recomputing a task's assignment for a new duration would change, for the
/// schedule editor: `Ok(None)` when the task carries no assignment.
///
/// # Errors
///
/// A sentence when the new duration cannot be reconciled with the task's work.
pub(crate) fn recompute_for_duration(
    task: &tasks::Model,
    new_duration: Option<i32>,
) -> std::result::Result<Option<rules::Recomputed>, String> {
    let (Some(units), Some(work), Some(new)) =
        (task.units_percent, task.work_minutes, new_duration)
    else {
        return Ok(None);
    };
    if Some(new) == task.duration_days {
        return Ok(None);
    }
    if new == 0 && work > 0 {
        return Err(
            "this task carries work, so it cannot become a milestone: clear its assignment first"
                .to_string(),
        );
    }
    rules::recompute(
        rule_of(task),
        Field::Duration,
        Triple {
            duration_days: i64::from(new),
            units_percent: i64::from(units),
            work_minutes: work,
        },
    )
    .map(Some)
    .map_err(|p| p.message().to_string())
}

#[derive(Debug, Deserialize)]
struct AssignmentBody {
    /// `fixed_units`, `fixed_duration` or `fixed_work`.
    #[serde(default)]
    rule: Option<String>,
    #[serde(default)]
    duration_days: Option<i64>,
    #[serde(default)]
    units_percent: Option<i64>,
    #[serde(default)]
    work_minutes: Option<i64>,
}

/// `PUT /api/plans/{pid}/tasks/{t_pid}/assignment` — set the task's units, work
/// and duration under a rule.
///
/// Give **one** quantity to change it and the rule works out which of the other
/// two moves (see [`crate::scheduling_rules`]); the very first time, give two and
/// the third is derived. A result whose units exceed the assignee's availability
/// on any day of the task is **flagged, not clamped**.
#[debug_handler]
#[expect(clippy::too_many_lines)]
async fn set_assignment(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, t_pid)): Path<(String, String)>,
    Json(b): Json<AssignmentBody>,
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
    if task.is_hammock {
        return Err(unprocessable(
            "a hammock is a derived span, not work: it takes no assignment",
        ));
    }
    let Some(person) = task.assignee_ref.clone() else {
        return Err(unprocessable(
            "assign the task to someone first: units are a share of their day",
        ));
    };
    let rule = match b.rule.as_deref() {
        None => rule_of(&task),
        Some(r) => Rule::parse(r).ok_or_else(|| {
            unprocessable("rule must be fixed_units, fixed_duration or fixed_work")
        })?,
    };
    let provided: Vec<(Field, i64)> = [
        (Field::Duration, b.duration_days),
        (Field::Units, b.units_percent),
        (Field::Work, b.work_minutes),
    ]
    .into_iter()
    .filter_map(|(f, v)| v.map(|v| (f, v)))
    .collect();
    let was = serde_json::json!({ "rule": task.scheduling_rule, "duration_days": task.duration_days,
        "units_percent": task.units_percent, "work_minutes": task.work_minutes });

    let outcome: Option<rules::Recomputed> = if provided.is_empty() {
        if b.rule.is_none() {
            return Err(unprocessable(
                "give a rule, or one of duration_days, units_percent and work_minutes",
            ));
        }
        None
    } else {
        let merged = (
            provided
                .iter()
                .find(|(f, _)| *f == Field::Duration)
                .map(|x| x.1)
                .or(task.duration_days.map(i64::from)),
            provided
                .iter()
                .find(|(f, _)| *f == Field::Units)
                .map(|x| x.1)
                .or(task.units_percent.map(i64::from)),
            provided
                .iter()
                .find(|(f, _)| *f == Field::Work)
                .map(|x| x.1)
                .or(task.work_minutes),
        );
        let known = [merged.0, merged.1, merged.2]
            .iter()
            .filter(|v| v.is_some())
            .count();
        let r = if let (3, Some(d), Some(u), Some(w)) = (known, merged.0, merged.1, merged.2) {
            if provided.len() != 1 {
                return Err(unprocessable(
                    "change one of duration_days, units_percent and work_minutes at a time; the rule works out the rest",
                ));
            }
            rules::recompute(
                rule,
                provided[0].0,
                Triple {
                    duration_days: d,
                    units_percent: u,
                    work_minutes: w,
                },
            )
        } else if known == 2 {
            rules::complete(merged.0, merged.1, merged.2)
        } else {
            return Err(unprocessable(
                "give two of duration_days, units_percent and work_minutes the first time (the third is derived)",
            ));
        };
        Some(r.map_err(|p| unprocessable(p.message()))?)
    };

    let mut a: tasks::ActiveModel = task.clone().into();
    a.scheduling_rule = ActiveValue::set(Some(rule.token().to_string()));
    if let Some(r) = &outcome {
        a.duration_days = ActiveValue::set(Some(
            i32::try_from(r.triple.duration_days)
                .map_err(|_| unprocessable("duration out of range"))?,
        ));
        a.units_percent = ActiveValue::set(Some(
            i32::try_from(r.triple.units_percent)
                .map_err(|_| unprocessable("units out of range"))?,
        ));
        a.work_minutes = ActiveValue::set(Some(r.triple.work_minutes));
    }
    let txn = ctx.db.begin().await.map_err(super::db_err)?;
    let row = a.update(&txn).await.map_err(db_err)?;
    AuditModel::record(
        &txn,
        row.pid,
        "task_assignment_set",
        caller.actor(),
        Some(serde_json::json!({ "from": was,
            "to": { "rule": row.scheduling_rule, "duration_days": row.duration_days,
                    "units_percent": row.units_percent, "work_minutes": row.work_minutes },
            "derived": outcome.map(|r| field_name(r.derived)), "exact": outcome.map(|r| r.exact) })),
    ).await
    .map_err(super::audit_failed)?;
    txn.commit().await.map_err(super::db_err)?;

    // Does the result ask more of the assignee than they can give, on any day?
    let capacity = capacity_check(&ctx, plan.pid, &row, &person).await?;
    format::json(serde_json::json!({
        "task_pid": row.pid,
        "rule": row.scheduling_rule,
        "duration_days": row.duration_days,
        "units_percent": row.units_percent,
        "work_minutes": row.work_minutes,
        "derived": outcome.map(|r| r.derived),
        "exact": outcome.map(|r| r.exact),
        "capacity": capacity,
        "note": "Work = duration x units x 480 / 100 minutes. A duration derived from work rounds up to whole business days, units derived from work round to the nearest percent, and the quantity you edited is never changed by the rounding; exact says whether the relation then holds to the minute. Units above the assignee's availability on a day are flagged in capacity, never clamped.",
    }))
}

/// Whether the task's units exceed its assignee's capacity on any day it runs.
async fn capacity_check(
    ctx: &AppContext,
    plan: Uuid,
    task: &tasks::Model,
    person: &str,
) -> Result<serde_json::Value> {
    let Some(units) = task.units_percent else {
        return Ok(serde_json::json!({ "checked": false, "reason": "no_units" }));
    };
    let net = super::cpm::load_network(ctx, plan).await?;
    let span = net
        .analyze(0)
        .ok()
        .and_then(|r| r.nodes.into_iter().find(|n| n.pid == task.pid))
        .map(|n| (n.early_start, n.early_finish));
    let Some((from, to)) = span else {
        return Ok(serde_json::json!({ "checked": false, "reason": "unscheduled" }));
    };
    let ranges = super::availability::ranges_for(ctx, person).await?;
    let load = crate::availability::load(
        from,
        to,
        &[crate::availability::Alloc {
            percent: units,
            start: None,
            end: None,
        }],
        &ranges,
    );
    Ok(serde_json::json!({
        "checked": true,
        "from": from, "to": to,
        "over_capacity": load.days_over > 0,
        "days_over": load.days_over,
        "min_capacity_percent": load.min_capacity,
        "over": load.over,
    }))
}

#[derive(Debug, Deserialize)]
struct ContourBody {
    contour: String,
    /// Percentages summing to 100, for `custom`.
    #[serde(default)]
    custom: Option<Vec<i64>>,
}

/// `PUT /api/plans/{pid}/tasks/{t_pid}/contour` — how the task's work is spread
/// over its business days. The total work is unchanged; audited.
#[debug_handler]
async fn set_contour(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path((pid, t_pid)): Path<(String, String)>,
    Json(b): Json<ContourBody>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let t_pid = super::parse_pid(&t_pid)?;
    let contour = wp::parse(&b.contour, b.custom.as_deref()).map_err(|m| unprocessable(&m))?;
    let task = tasks::Entity::find()
        .filter(tasks::Column::Pid.eq(t_pid))
        .filter(tasks::Column::PlanPid.eq(plan.pid))
        .filter(tasks::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    let was = serde_json::json!({ "contour": task.contour, "custom": task.contour_custom });
    let work = task.work_minutes;
    let mut a: tasks::ActiveModel = task.into();
    a.contour = ActiveValue::set(contour.token().to_string());
    a.contour_custom = ActiveValue::set(match &contour {
        Contour::Custom(w) => Some(serde_json::json!(w)),
        _ => None,
    });
    let txn = ctx.db.begin().await.map_err(super::db_err)?;
    let row = a.update(&txn).await.map_err(db_err)?;
    AuditModel::record(
        &txn,
        row.pid,
        "task_contour_set",
        caller.actor(),
        Some(serde_json::json!({ "from": was,
            "to": { "contour": row.contour, "custom": row.contour_custom }, "work_minutes": work })),
    ).await
    .map_err(super::audit_failed)?;
    txn.commit().await.map_err(super::db_err)?;
    format::json(
        serde_json::json!({ "task_pid": row.pid, "contour": row.contour,
        "custom": row.contour_custom, "work_minutes": row.work_minutes,
        "note": "The contour changes when the work falls, never how much: the total is unchanged." }),
    )
}

fn contour_of(t: &tasks::Model) -> Contour {
    let custom: Option<Vec<i64>> = t
        .contour_custom
        .as_ref()
        .and_then(|v| serde_json::from_value(v.clone()).ok());
    wp::parse(&t.contour, custom.as_deref()).unwrap_or(Contour::Flat)
}

/// One task's work placed on days.
pub(crate) struct TaskWork {
    pub pid: Uuid,
    pub title: String,
    pub assignee: Option<String>,
    pub contour: &'static str,
    pub work_minutes: i64,
    /// Minutes on each business day (empty when it could not be placed).
    pub per_day: Vec<(NaiveDate, i64)>,
    /// Why it could not be placed.
    pub unplaced: Option<String>,
}

/// The business days of task `i` of the network, between its earliest start and
/// finish on **its own calendar**; `None` when the engine could not place it.
pub(crate) fn working_days_of(
    net: &super::cpm::Network,
    analysis: Option<&crate::cpm::CpmResult>,
    i: usize,
) -> Option<Vec<NaiveDate>> {
    let pid = net.rows.get(i)?.pid;
    let node = analysis?.nodes.iter().find(|n| n.pid == pid)?;
    let cal = net.nodes[i]
        .own_calendar
        .and_then(|ix| net.own.get(ix))
        .unwrap_or(&net.plan_cal);
    let mut days = Vec::new();
    let mut d = node.early_start;
    while d <= node.early_finish {
        if cal.is_working(d) {
            days.push(d);
        }
        d += chrono::Duration::days(1);
    }
    Some(days)
}

/// Place every task's work with a stated amount of it, on the business days of
/// its own calendar between its earliest start and finish.
pub(crate) async fn plan_work(ctx: &AppContext, plan: Uuid) -> Result<Vec<TaskWork>> {
    let net = super::cpm::load_network(ctx, plan).await?;
    let analysis = net.analyze(0).ok();
    Ok(work_of(&net, analysis.as_ref()))
}

/// [`plan_work`] for a network already in hand (the what-if sandbox runs it on a
/// copy with edits applied).
pub(crate) fn work_of(
    net: &super::cpm::Network,
    analysis: Option<&crate::cpm::CpmResult>,
) -> Vec<TaskWork> {
    let mut out = Vec::new();
    for (i, t) in net.rows.iter().enumerate() {
        let Some(work) = t.work_minutes.filter(|w| *w > 0) else {
            continue;
        };
        let contour = contour_of(t);
        let (per_day, unplaced) = match working_days_of(net, analysis, i) {
            None => (Vec::new(), Some("unscheduled".to_string())),
            Some(days) => match wp::distribute(work, &days, &contour) {
                Ok(p) => (p, None),
                Err(Unplaced::NoWorkingDays) => (Vec::new(), Some("no_working_days".to_string())),
                Err(Unplaced::NoWeight) => (Vec::new(), Some("no_weight".to_string())),
            },
        };
        out.push(TaskWork {
            pid: t.pid,
            title: t.title.clone(),
            assignee: t.assignee_ref.clone(),
            contour: contour.token(),
            work_minutes: work,
            per_day,
            unplaced,
        });
    }
    out
}

#[derive(Debug, Deserialize)]
pub(crate) struct ProfileParams {
    #[serde(default)]
    pub(crate) from: Option<NaiveDate>,
    #[serde(default)]
    pub(crate) to: Option<NaiveDate>,
    /// `day`, `week` (default) or `month`.
    #[serde(default)]
    pub(crate) period: Option<String>,
}

/// The window and periods for a set of placed days.
pub(crate) fn window(
    q: &ProfileParams,
    placed: &[&(NaiveDate, i64)],
) -> Result<(
    NaiveDate,
    NaiveDate,
    Grain,
    Vec<crate::portfolio_resources::Period>,
)> {
    let grain = match q.period.as_deref() {
        None => Grain::Week,
        Some(p) => {
            Grain::parse(p).ok_or_else(|| unprocessable("period must be day, week or month"))?
        }
    };
    let first = placed.iter().map(|(d, _)| *d).min();
    let last = placed.iter().map(|(d, _)| *d).max();
    let (Some(from), Some(to)) = (q.from.or(first), q.to.or(last)) else {
        return Err(unprocessable(
            "there is no placed work to draw a window from; give from and to",
        ));
    };
    let periods = wp::periods(from, to, grain).map_err(|m| unprocessable(&m))?;
    Ok((from, to, grain, periods))
}

fn buckets_json(b: &[wp::Bucket]) -> serde_json::Value {
    serde_json::json!(b)
}

fn sum_buckets(
    parts: &[Vec<wp::Bucket>],
    periods: &[crate::portfolio_resources::Period],
) -> Vec<wp::Bucket> {
    periods
        .iter()
        .enumerate()
        .map(|(i, p)| wp::Bucket {
            from: p.from,
            to: p.to,
            minutes: parts.iter().map(|b| b[i].minutes).sum(),
        })
        .collect()
}

/// `GET /api/plans/{pid}/work-profile` — a plan's work as buckets, per task and in total.
#[debug_handler]
async fn plan_profile(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Query(q): Query<ProfileParams>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let work = plan_work(&ctx, plan.pid).await?;
    if work.is_empty() {
        return format::json(serde_json::json!({
            "plan_pid": plan.pid,
            "tasks": [], "totals": { "buckets": null, "work_minutes": 0 },
            "unplaced": [],
            "reason": "no_assignments",
            "note": "No task carries work, so there is nothing to spread. Set a task's assignment first.",
        }));
    }
    let placed: Vec<&(NaiveDate, i64)> = work.iter().flat_map(|t| t.per_day.iter()).collect();
    let (from, to, grain, periods) = window(&q, &placed)?;
    let see = super::portfolio_resources::may_manage_people(&caller);
    let mut tasks_json = Vec::new();
    let mut task_buckets: Vec<Vec<wp::Bucket>> = Vec::new();
    let mut by_person: BTreeMap<String, Vec<Vec<wp::Bucket>>> = BTreeMap::new();
    let mut unplaced = Vec::new();
    let mut outside_total = 0_i64;
    for t in &work {
        let (b, outside) = wp::bucket(&t.per_day, &periods);
        outside_total += outside;
        if let Some(why) = &t.unplaced {
            unplaced.push(serde_json::json!({ "pid": t.pid, "title": t.title, "work_minutes": t.work_minutes, "reason": why }));
        }
        let mut v = serde_json::json!({ "pid": t.pid, "title": t.title, "contour": t.contour,
            "work_minutes": t.work_minutes, "buckets": buckets_json(&b), "outside_window_minutes": outside,
            "placed": t.unplaced.is_none() });
        if see {
            v["assignee"] = serde_json::json!(t.assignee);
        }
        tasks_json.push(v);
        if let Some(who) = &t.assignee {
            by_person.entry(who.clone()).or_default().push(b.clone());
        }
        task_buckets.push(b);
    }
    let total = sum_buckets(&task_buckets, &periods);
    let placed_work: i64 = work
        .iter()
        .filter(|t| t.unplaced.is_none())
        .map(|t| t.work_minutes)
        .sum();
    let unplaced_work: i64 = work
        .iter()
        .filter(|t| t.unplaced.is_some())
        .map(|t| t.work_minutes)
        .sum();
    format::json(serde_json::json!({
        "plan_pid": plan.pid,
        "from": from, "to": to,
        "period": match grain { Grain::Day => "day", Grain::Week => "week", Grain::Month => "month" },
        "tasks": tasks_json,
        "totals": { "buckets": buckets_json(&total), "work_minutes": placed_work + unplaced_work,
                    "placed_work_minutes": placed_work, "unplaced_work_minutes": unplaced_work,
                    "outside_window_minutes": outside_total },
        "unplaced": unplaced,
        "people": if see { serde_json::json!(by_person.iter().map(|(p, parts)| (p.clone(), buckets_json(&sum_buckets(parts, &periods)))).collect::<BTreeMap<_, _>>()) } else { serde_json::Value::Null },
        "masked": !see,
        "note": "Work is spread over each task's business days (its own calendar) between its earliest start and finish, by its contour, in whole minutes that sum to its work exactly. Work that cannot be placed (the task is unscheduled, or has no business day) is listed under unplaced and counted, never dropped; work outside the window is counted in outside_window_minutes. Derived on read.",
    }))
}

/// `GET /api/work-profile` — every plan the caller can read, rolled up.
#[debug_handler]
async fn portfolio_profile(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(q): Query<ProfileParams>,
) -> Result<Response> {
    let with_work: Vec<Uuid> = {
        let mut v: Vec<Uuid> = tasks::Entity::find()
            .filter(tasks::Column::DeletedAt.is_null())
            .filter(tasks::Column::WorkMinutes.gt(0))
            .all(&ctx.db)
            .await
            .map_err(db_err)?
            .into_iter()
            .map(|t| t.plan_pid)
            .collect();
        v.sort();
        v.dedup();
        v
    };
    let readable = super::plan_access::readable_plans(&ctx, &caller, &with_work).await;
    let plan_pids: Vec<Uuid> = with_work
        .into_iter()
        .filter(|p| readable.contains(p))
        .collect();
    let mut per_plan: Vec<(Uuid, Vec<TaskWork>)> = Vec::new();
    for p in &plan_pids {
        per_plan.push((*p, plan_work(&ctx, *p).await?));
    }
    if per_plan.is_empty() {
        return format::json(serde_json::json!({
            "plans": [], "totals": { "buckets": null, "work_minutes": 0 },
            "reason": "no_assignments",
            "note": "No plan you can read has a task carrying work.",
        }));
    }
    let placed: Vec<&(NaiveDate, i64)> = per_plan
        .iter()
        .flat_map(|(_, w)| w.iter().flat_map(|t| t.per_day.iter()))
        .collect();
    let (from, to, grain, periods) = window(&q, &placed)?;
    let names: std::collections::HashMap<Uuid, String> = plans::Entity::find()
        .filter(plans::Column::Pid.is_in(plan_pids.clone()))
        .all(&ctx.db)
        .await
        .map_err(db_err)?
        .into_iter()
        .map(|p| (p.pid, p.name))
        .collect();
    let mut plans_json = Vec::new();
    let mut plan_buckets: Vec<Vec<wp::Bucket>> = Vec::new();
    let mut unplaced_work = 0_i64;
    let mut total_work = 0_i64;
    for (pid, work) in &per_plan {
        let parts: Vec<Vec<wp::Bucket>> = work
            .iter()
            .map(|t| wp::bucket(&t.per_day, &periods).0)
            .collect();
        let sum = sum_buckets(&parts, &periods);
        let plan_work_total: i64 = work.iter().map(|t| t.work_minutes).sum();
        let plan_unplaced: i64 = work
            .iter()
            .filter(|t| t.unplaced.is_some())
            .map(|t| t.work_minutes)
            .sum();
        unplaced_work += plan_unplaced;
        total_work += plan_work_total;
        plans_json.push(serde_json::json!({ "plan_pid": pid, "name": names.get(pid),
            "work_minutes": plan_work_total, "unplaced_work_minutes": plan_unplaced,
            "buckets": buckets_json(&sum) }));
        plan_buckets.push(sum);
    }
    let total = sum_buckets(&plan_buckets, &periods);
    format::json(serde_json::json!({
        "from": from, "to": to,
        "period": match grain { Grain::Day => "day", Grain::Week => "week", Grain::Month => "month" },
        "plans": plans_json,
        "totals": { "buckets": buckets_json(&total), "work_minutes": total_work, "unplaced_work_minutes": unplaced_work },
        "note": "Each plan's buckets are the sum of its tasks', and the portfolio's the sum of its plans', at every level. Plans you cannot read are left out. Derived on read.",
    }))
}

/// The assignment routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/tasks/{t_pid}/assignment", put(set_assignment))
        .add("/plans/{pid}/tasks/{t_pid}/contour", put(set_contour))
        .add("/plans/{pid}/work-profile", get(plan_profile))
        .add("/work-profile", get(portfolio_profile))
}
