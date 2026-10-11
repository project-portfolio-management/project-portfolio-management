//! Resource leveling (SC-6): an **advisory** proposal and a separate, confirmed
//! apply. The rules are [`crate::leveling`]; this is the wiring.
//!
//! `GET /api/plans/{pid}/leveling` changes nothing. It finds the days a person's
//! unfinished tasks ask for more than their capacity and proposes start delays
//! that stay within float (see the module docs of [`crate::leveling`]).
//! `POST /api/plans/{pid}/leveling/apply` applies a proposal the caller has seen
//! and confirmed: it recomputes the proposal, refuses any move that is not part
//! of it (the plan changed since), and sets each start date through the bulk
//! edit (RM-9), writing **one audit entry per moved task**.
//!
//! A person's detail is masked for a caller who may not manage that person
//! (`access=admin`, `role=resource_manager`, or the person themselves): they see
//! `person-N` labels and the tasks, but not who, nor the percentages.

use std::collections::{BTreeSet, HashMap};

use axum::http::StatusCode;
use chrono::NaiveDate;
use loco_rs::controller::ErrorDetail;
use loco_rs::prelude::*;
use serde::Deserialize;
use serde_json::Value;
use uuid::Uuid;

use crate::auth::MaybeAuthUser;
use crate::leveling::{self, Demand, Input, Params, Proposal, Resource};
use crate::models::visibility as vis_models;
use crate::workflow::{Category, category_of};

fn unprocessable(message: &str) -> Error {
    Error::CustomError(
        StatusCode::UNPROCESSABLE_ENTITY,
        ErrorDetail::new("unprocessable_entity", message),
    )
}

fn conflict(code: &str, message: &str) -> Error {
    Error::CustomError(StatusCode::CONFLICT, ErrorDetail::new(code, message))
}

#[derive(Debug, Deserialize)]
struct LevelingParams {
    /// Let a critical task move; the finish impact is then stated.
    #[serde(default)]
    allow_critical: bool,
    /// Percent of a person's day one task asks for (default 100).
    #[serde(default)]
    load_percent: Option<i32>,
}

fn load_percent(p: Option<i32>) -> Result<i32> {
    let v = p.unwrap_or(100);
    if (1..=100).contains(&v) {
        Ok(v)
    } else {
        Err(unprocessable("load_percent must be 1 to 100"))
    }
}

pub(crate) struct Computed {
    pub(crate) net: super::cpm::Network,
    pub(crate) proposal: Proposal,
    pub(crate) titles: HashMap<Uuid, String>,
    /// Who does each assigned, unfinished task.
    pub(crate) people: HashMap<Uuid, String>,
    /// Each task's aggressive and safe figures (CC-2); empty unless the schedule was built at them.
    pub(crate) estimates: HashMap<Uuid, crate::ccpm_estimates::Resolved>,
}

pub(crate) async fn compute(
    ctx: &AppContext,
    plan: Uuid,
    allow_critical: bool,
    percent: i32,
) -> Result<Computed> {
    compute_with(ctx, plan, allow_critical, percent, false).await
}

/// As [`compute`]; with `aggressive` the tasks are scheduled at their aggressive estimates where they have
/// one (CC-3), the others keeping their likely duration.
pub(crate) async fn compute_with(
    ctx: &AppContext,
    plan: Uuid,
    allow_critical: bool,
    percent: i32,
    aggressive: bool,
) -> Result<Computed> {
    let mut net = super::cpm::load_network(ctx, plan).await?;
    let mut estimates = HashMap::new();
    if aggressive {
        estimates = super::ccpm_estimates::resolved(ctx, plan, &net.rows).await?;
        for n in &mut net.nodes {
            if let Some(a) = estimates.get(&n.pid).and_then(|r| r.aggressive) {
                n.duration = Some(a.days);
            }
        }
    }
    let workflow = super::workflow::in_force(ctx, plan, "task").await?;
    let demands: Vec<Demand> = net
        .rows
        .iter()
        .filter_map(|t| {
            let person = t.assignee_ref.clone()?;
            let category = category_of(&workflow, &t.status);
            if matches!(category, Some(Category::Done)) {
                return None;
            }
            Some(Demand {
                pid: t.pid,
                person,
                percent,
                // Only a task not yet started may be told to start later.
                movable: matches!(category, Some(Category::Todo) | None),
            })
        })
        .collect();
    let people: BTreeSet<&str> = demands.iter().map(|d| d.person.as_str()).collect();
    let mut resources: HashMap<String, Resource> = HashMap::new();
    for (person, ranges) in super::availability::all_ranges(ctx).await? {
        if people.contains(person.as_str()) {
            resources.entry(person).or_default().ranges = ranges;
        }
    }
    // Other plans' committed bookings are fixed load; this plan's own are the
    // same work its tasks describe, so they are not counted twice.
    for a in vis_models::all_allocations(&ctx.db).await? {
        if a.plan_pid != plan && people.contains(a.person_ref.as_str()) {
            resources
                .entry(a.person_ref.clone())
                .or_default()
                .background
                .push(crate::availability::Alloc {
                    percent: a.percent,
                    start: a.start_date,
                    end: a.end_date,
                });
        }
    }
    let proposal = leveling::propose(
        &Input {
            nodes: &net.nodes,
            edges: &net.edges,
            calendar: &net.plan_cal,
            own: &net.own,
            demands: &demands,
            resources: &resources,
        },
        Params { allow_critical },
    )
    .map_err(|crate::cpm::CpmError::Cycle(on)| {
        unprocessable(&format!(
            "the dependencies contain a cycle, so nothing can be leveled (on or behind it: {})",
            on.iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        ))
    })?;
    let titles = net.rows.iter().map(|t| (t.pid, t.title.clone())).collect();
    let people = demands.iter().map(|d| (d.pid, d.person.clone())).collect();
    Ok(Computed {
        net,
        proposal,
        titles,
        people,
        estimates,
    })
}

/// The proposal as JSON, with each person named or masked for this caller.
fn shape(c: &Computed, caller: &MaybeAuthUser, allow_critical: bool, percent: i32) -> Value {
    let p = &c.proposal;
    let mut people: BTreeSet<&str> = BTreeSet::new();
    people.extend(p.moves.iter().map(|m| m.person.as_str()));
    people.extend(p.over_before.iter().map(|r| r.person.as_str()));
    people.extend(p.unresolved.iter().map(|r| r.person.as_str()));
    let label: HashMap<&str, String> = people
        .iter()
        .enumerate()
        .map(|(i, p)| (*p, format!("person-{}", i + 1)))
        .collect();
    let allowed = |person: &str| super::availability::may_manage_person(caller, person);
    let titled = |pid: &Uuid| serde_json::json!({ "pid": pid, "title": c.titles.get(pid) });
    let who = |obj: &mut Value, person: &str| {
        obj["person"] = serde_json::json!(label[person]);
        if allowed(person) {
            obj["person_ref"] = serde_json::json!(person);
        } else {
            obj["person_ref"] = Value::Null;
            obj["masked"] = serde_json::json!(true);
            for k in ["demand", "capacity"] {
                if obj.get(k).is_some() {
                    obj[k] = Value::Null;
                }
            }
        }
    };
    let run = |r: &leveling::OverRun| {
        let mut v = serde_json::to_value(r).unwrap_or_default();
        who(&mut v, &r.person);
        v["tasks"] = Value::Array(r.tasks.iter().map(titled).collect());
        v
    };
    let moves: Vec<Value> = p
        .moves
        .iter()
        .map(|m| {
            let mut v = serde_json::to_value(m).unwrap_or_default();
            who(&mut v, &m.person);
            v["title"] = serde_json::json!(c.titles.get(&m.pid));
            v
        })
        .collect();
    serde_json::json!({
        "advisory": true,
        "calendar": c.net.cal_source,
        "allow_critical": allow_critical,
        "load_percent": percent,
        "moves": moves,
        "effects": p.effects.iter().map(|e| {
            let mut v = serde_json::to_value(e).unwrap_or_default();
            v["title"] = serde_json::json!(c.titles.get(&e.pid));
            v
        }).collect::<Vec<_>>(),
        "over_allocated": p.over_before.iter().map(&run).collect::<Vec<_>>(),
        "unresolved": p.unresolved.iter().map(&run).collect::<Vec<_>>(),
        "finish_before": p.finish_before,
        "finish_after": p.finish_after,
        "finish_delay_days": p.finish_delay_days,
        "critical_moved": p.critical_moved,
        "truncated": p.truncated,
        "note": "This is advice; nothing has been changed. A task is delayed only within its total float (the project finish does not move) unless allow_critical is set, in which case any critical task moved is marked and the finish impact is shown. A task already started, a task with a hard constraint, and a task whose delay would create negative float are never moved; what cannot be fixed is listed under unresolved with the reason. Demand counts each unfinished, assigned, scheduled task as load_percent of its assignee on each day it works, plus the person's committed bookings in other plans; capacity is their availability (100% when none). Apply with POST …/leveling/apply and confirm=true.",
    })
}

/// `GET /api/plans/{pid}/leveling` — propose a leveling. Changes nothing.
#[debug_handler]
async fn propose(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Query(q): Query<LevelingParams>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    let percent = load_percent(q.load_percent)?;
    let c = compute(&ctx, plan.pid, q.allow_critical, percent).await?;
    format::json(shape(&c, &caller, q.allow_critical, percent))
}

#[derive(Debug, Deserialize)]
struct ApplyMove {
    task_pid: Uuid,
    start_date: NaiveDate,
}

#[derive(Debug, Deserialize)]
struct ApplyBody {
    /// The moves to apply, as proposed.
    moves: Vec<ApplyMove>,
    /// Must be `true`: applying is a decision, not a side effect of reading.
    #[serde(default)]
    confirm: bool,
    #[serde(default)]
    allow_critical: bool,
    #[serde(default)]
    load_percent: Option<i32>,
    /// Why (audited with each move).
    #[serde(default)]
    reason: Option<String>,
}

/// `POST /api/plans/{pid}/leveling/apply` — apply moves from a proposal.
#[debug_handler]
async fn apply(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(pid): Path<String>,
    Json(b): Json<ApplyBody>,
) -> Result<Response> {
    let plan = super::governance::find_item(&ctx, &pid).await?;
    if !b.confirm {
        return Err(unprocessable(
            "leveling changes dates: send confirm=true after reading the proposal",
        ));
    }
    if b.moves.is_empty() {
        return Err(unprocessable("moves must name at least one task"));
    }
    if b.moves.len() > crate::task_bulk::MAX_IDS {
        return Err(unprocessable(&format!(
            "at most {} moves per call",
            crate::task_bulk::MAX_IDS
        )));
    }
    if b.reason.as_deref().is_some_and(|r| r.chars().count() > 500) {
        return Err(unprocessable("reason is at most 500 characters"));
    }
    let percent = load_percent(b.load_percent)?;
    let c = compute(&ctx, plan.pid, b.allow_critical, percent).await?;
    let proposed: HashMap<Uuid, &leveling::Move> =
        c.proposal.moves.iter().map(|m| (m.pid, m)).collect();
    let mut seen = std::collections::HashSet::new();
    for m in &b.moves {
        if !seen.insert(m.task_pid) {
            return Err(unprocessable("a task may appear once in moves"));
        }
        if proposed
            .get(&m.task_pid)
            .is_none_or(|p| p.to_start != m.start_date)
        {
            return Err(conflict(
                "stale_proposal",
                &format!(
                    "{} is not part of the current proposal at {}: the plan has changed since it was read, so read the proposal again",
                    m.task_pid, m.start_date
                ),
            ));
        }
    }
    // The apply goes through the what-if sandbox (PJ-9): one transaction, one
    // audit entry per moved task (kept as `task_leveled`, with the proposal's words).
    let mut edits = Vec::with_capacity(b.moves.len());
    let mut overrides = super::sandbox::AuditOverrides::new();
    for m in &b.moves {
        let p = proposed[&m.task_pid];
        edits.push(crate::sandbox::Edit::Move {
            task_pid: m.task_pid,
            start_date: m.start_date,
        });
        overrides.insert(
            m.task_pid,
            (
                "task_leveled",
                serde_json::json!({
                    "from_start": p.from_start, "to_start": p.to_start,
                    "delay_days": p.delay_days, "critical": p.critical,
                    "because_of": p.because_of, "reason": b.reason,
                    "finish_before": c.proposal.finish_before, "finish_after": c.proposal.finish_after,
                }),
            ),
        );
    }
    super::sandbox::commit(
        &ctx,
        &caller,
        plan.pid,
        &edits,
        None,
        b.reason.as_deref(),
        &overrides,
    )
    .await?;
    let results: Vec<Value> = b
        .moves
        .iter()
        .map(|m| serde_json::json!({ "pid": m.task_pid, "result": "ok" }))
        .collect();
    let ok = results.len();
    format::json(serde_json::json!({
        "requested": results.len(),
        "ok": ok,
        "refused": results.len() - ok,
        "results": results,
        "finish_before": c.proposal.finish_before,
        "finish_after": c.proposal.finish_after,
        "finish_delay_days": c.proposal.finish_delay_days,
    }))
}

/// The leveling routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/plans/{pid}/leveling", get(propose))
        .add("/plans/{pid}/leveling/apply", post(apply))
}
