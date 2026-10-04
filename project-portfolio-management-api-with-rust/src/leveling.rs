//! Resource leveling (SC-6): the pure rules. DB-free, unit-tested.
//!
//! **Advisory.** From the critical-path network ([`crate::cpm`]) and who is
//! doing what, find the days a person is asked for more than their capacity and
//! *propose* a set of start delays that remove the over-allocation without
//! breaking the schedule. Nothing is changed here; the caller applies a proposal
//! through a separate, confirmed call.
//!
//! # The rule
//!
//! Demand on a day is the summed `percent` of the person's **unfinished** tasks
//! whose earliest dates cover it (on a day that task works) plus any fixed
//! background load. Capacity is the person's availability range in force that
//! day ([`crate::availability`]), 100% when none applies.
//!
//! The first over-allocated day (earliest date, then person) is resolved by
//! **keeping** tasks in priority order until the capacity is spent and
//! **delaying** the lowest-priority task that is left over. Priority is:
//! tasks that cannot move first (started, hard-constrained, critical unless
//! `allow_critical`), then **least total float**, then earliest start, then
//! input order. A delay is a new planned start (a lower bound), the network is
//! re-run, and the delay is **accepted only if** it creates no negative float
//! and no hard-constraint conflict, leaves no task unscheduled that was
//! scheduled, and — unless `allow_critical` — leaves the project finish where it
//! was (that is "within total float"). A delay that fails is not forced: the task
//! is frozen and the conflict is reported.
//!
//! The procedure is a pure function of its input: ties are broken by input
//! order, never by hash order or time, so the same input gives the same proposal.

use std::collections::{BTreeMap, HashMap, HashSet};

use chrono::{Duration, NaiveDate};
use serde::Serialize;
use uuid::Uuid;

use crate::availability::{Alloc, Range, capacity_on};
use crate::cpm::{
    CpmError, CpmResult, Edge, Moved, Node, NodeResult, Options, WorkCalendar, analyze_with, diff,
};

/// Most re-analyses one proposal performs before it stops and says so.
pub const MAX_ITERATIONS: usize = 3_000;
/// Most days one person's load is walked.
pub const MAX_SPAN_DAYS: i64 = 3_660;

/// What one task asks of one person.
#[derive(Debug, Clone)]
pub struct Demand {
    /// The task.
    pub pid: Uuid,
    /// Who is doing it.
    pub person: String,
    /// Percent of their capacity it takes on each working day (1–100).
    pub percent: i32,
    /// `false` for a task already started: it keeps its date.
    pub movable: bool,
}

/// What one person brings to the window.
#[derive(Debug, Clone, Default)]
pub struct Resource {
    /// Availability ranges (capacity is 100% where none applies).
    pub ranges: Vec<Range>,
    /// Fixed load from outside this network (other plans' bookings).
    pub background: Vec<Alloc>,
}

/// The whole input.
pub struct Input<'a> {
    /// The task network.
    pub nodes: &'a [Node],
    /// Dependencies.
    pub edges: &'a [Edge],
    /// The plan's calendar.
    pub calendar: &'a WorkCalendar,
    /// Per-assignee calendars (see [`analyze_with`]).
    pub own: &'a [WorkCalendar],
    /// Who is asked for what.
    pub demands: &'a [Demand],
    /// Each person's availability and background load.
    pub resources: &'a HashMap<String, Resource>,
}

/// Tuning.
#[derive(Debug, Clone, Copy, Default)]
pub struct Params {
    /// Allow a critical task (no float) to move, which delays the finish.
    pub allow_critical: bool,
}

/// A run of consecutive over-allocated days for one person.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OverRun {
    /// Who.
    pub person: String,
    /// First day.
    pub from: NaiveDate,
    /// Last day.
    pub to: NaiveDate,
    /// Summed percent asked for on each day (tasks and background).
    pub demand: i64,
    /// The capacity in force on each day.
    pub capacity: i32,
    /// The tasks active on those days.
    pub tasks: Vec<Uuid>,
    /// Why it remains (set on `unresolved` runs): `fixed_tasks`, `critical_task`,
    /// `no_float`, `hard_constraint`, `did_not_move` or `limit`.
    pub reason: Option<&'static str>,
}

/// One proposed delay.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Move {
    /// The task.
    pub pid: Uuid,
    /// Who it is for.
    pub person: String,
    /// Its earliest start today.
    pub from_start: NaiveDate,
    /// The planned start to set (a lower bound; logic may place it later).
    pub to_start: NaiveDate,
    /// Where the network then puts it.
    pub scheduled_start: NaiveDate,
    /// Its earliest finish today.
    pub from_finish: NaiveDate,
    /// Its earliest finish afterwards.
    pub to_finish: NaiveDate,
    /// Working days later it starts.
    pub delay_days: i64,
    /// It had no float: moving it moves the project finish.
    pub critical: bool,
    /// Total float before.
    pub float_before: i64,
    /// Total float after.
    pub float_after: i64,
    /// The first over-allocated day that caused it.
    pub because_of: NaiveDate,
}

/// The advice.
#[derive(Debug, Clone, Serialize)]
pub struct Proposal {
    /// The delays, in input order.
    pub moves: Vec<Move>,
    /// Other tasks whose dates change as a consequence (successors pushed).
    pub effects: Vec<Moved>,
    /// Over-allocation as the schedule stands.
    pub over_before: Vec<OverRun>,
    /// What the proposal could not remove, with why.
    pub unresolved: Vec<OverRun>,
    /// Project finish today.
    pub finish_before: Option<NaiveDate>,
    /// Project finish with the proposal applied.
    pub finish_after: Option<NaiveDate>,
    /// Working days the finish moves (0 unless a critical task was allowed to move).
    pub finish_delay_days: i64,
    /// A critical task is among the moves.
    pub critical_moved: bool,
    /// The iteration limit stopped the search early.
    pub truncated: bool,
}

struct OverDay {
    person: String,
    day: NaiveDate,
    demand: i64,
    capacity: i32,
    /// Indices into the analysis' `nodes`.
    active: Vec<usize>,
}

fn run(input: &Input, overrides: &HashMap<Uuid, NaiveDate>) -> Result<CpmResult, CpmError> {
    let nodes: Vec<Node> = input
        .nodes
        .iter()
        .map(|n| {
            let mut n = *n;
            if let Some(d) = overrides.get(&n.pid) {
                n.planned_start = Some(*d);
            }
            n
        })
        .collect();
    analyze_with(
        &nodes,
        input.edges,
        input.calendar,
        input.own,
        Options {
            near_critical_days: 0,
            ..Options::default()
        },
    )
}

/// Every over-allocated day with at least one task on it, earliest first.
fn over_days(input: &Input, res: &CpmResult) -> Vec<OverDay> {
    let demand_of: HashMap<Uuid, &Demand> = input.demands.iter().map(|d| (d.pid, d)).collect();
    let calendar_of: HashMap<Uuid, &WorkCalendar> = input
        .nodes
        .iter()
        .map(|n| {
            (
                n.pid,
                n.own_calendar
                    .and_then(|i| input.own.get(i))
                    .unwrap_or(input.calendar),
            )
        })
        .collect();
    let mut by_person: BTreeMap<&str, Vec<usize>> = BTreeMap::new();
    for (i, n) in res.nodes.iter().enumerate() {
        if let Some(d) = demand_of.get(&n.pid) {
            by_person.entry(d.person.as_str()).or_default().push(i);
        }
    }
    let empty = Resource::default();
    let mut out = Vec::new();
    for (person, idxs) in by_person {
        let res_of = input.resources.get(person).unwrap_or(&empty);
        let first = idxs.iter().map(|i| res.nodes[*i].early_start).min();
        let last = idxs.iter().map(|i| res.nodes[*i].early_finish).max();
        let (Some(first), Some(last)) = (first, last) else {
            continue;
        };
        let last = last.min(first + Duration::days(MAX_SPAN_DAYS));
        let mut day = first;
        while day <= last {
            let active: Vec<usize> = idxs
                .iter()
                .copied()
                .filter(|i| {
                    let n = &res.nodes[*i];
                    n.early_start <= day
                        && day <= n.early_finish
                        && calendar_of.get(&n.pid).is_some_and(|c| c.is_working(day))
                })
                .collect();
            if !active.is_empty() {
                let tasks: i64 = active
                    .iter()
                    .map(|i| i64::from(demand_of[&res.nodes[*i].pid].percent))
                    .sum();
                let background: i64 = res_of
                    .background
                    .iter()
                    .filter(|a| a.start.is_none_or(|s| s <= day) && a.end.is_none_or(|e| e >= day))
                    .map(|a| i64::from(a.percent))
                    .sum();
                let capacity = capacity_on(&res_of.ranges, day);
                if tasks + background > i64::from(capacity) {
                    out.push(OverDay {
                        person: person.to_string(),
                        day,
                        demand: tasks + background,
                        capacity,
                        active,
                    });
                }
            }
            day += Duration::days(1);
        }
    }
    out.sort_by(|a, b| a.day.cmp(&b.day).then_with(|| a.person.cmp(&b.person)));
    out
}

fn runs(
    days: &[OverDay],
    res: &CpmResult,
    reason: impl Fn(&[Uuid]) -> &'static str,
) -> Vec<OverRun> {
    let mut by_person: BTreeMap<&str, Vec<&OverDay>> = BTreeMap::new();
    for d in days {
        by_person.entry(d.person.as_str()).or_default().push(d);
    }
    let mut out: Vec<OverRun> = Vec::new();
    for (person, ds) in by_person {
        let mut current: Option<OverRun> = None;
        for d in ds {
            let tasks: Vec<Uuid> = d.active.iter().map(|i| res.nodes[*i].pid).collect();
            match current.as_mut() {
                Some(r)
                    if r.demand == d.demand
                        && r.capacity == d.capacity
                        && r.to + Duration::days(1) == d.day =>
                {
                    r.to = d.day;
                    for t in tasks {
                        if !r.tasks.contains(&t) {
                            r.tasks.push(t);
                        }
                    }
                }
                _ => {
                    if let Some(r) = current.take() {
                        out.push(r);
                    }
                    current = Some(OverRun {
                        person: person.to_string(),
                        from: d.day,
                        to: d.day,
                        demand: d.demand,
                        capacity: d.capacity,
                        tasks,
                        reason: None,
                    });
                }
            }
        }
        out.extend(current);
    }
    for r in &mut out {
        r.reason = Some(reason(&r.tasks)).filter(|s| !s.is_empty());
    }
    out.sort_by(|a, b| a.from.cmp(&b.from).then_with(|| a.person.cmp(&b.person)));
    out
}

fn negative(r: &CpmResult) -> HashSet<Uuid> {
    r.nodes
        .iter()
        .filter(|n| n.total_float < 0)
        .map(|n| n.pid)
        .collect()
}

fn conflicted(r: &CpmResult) -> HashSet<Uuid> {
    r.nodes
        .iter()
        .filter(|n| n.hard_conflict)
        .map(|n| n.pid)
        .collect()
}

/// Propose a leveling.
///
/// # Errors
///
/// [`CpmError::Cycle`] when the dependencies contain a cycle.
#[allow(clippy::too_many_lines)]
pub fn propose(input: &Input, params: Params) -> Result<Proposal, CpmError> {
    let base = run(input, &HashMap::new())?;
    let demand_of: HashMap<Uuid, &Demand> = input.demands.iter().map(|d| (d.pid, d)).collect();
    let node_of: HashMap<Uuid, &Node> = input.nodes.iter().map(|n| (n.pid, n)).collect();
    let order: HashMap<Uuid, usize> = input
        .nodes
        .iter()
        .enumerate()
        .map(|(i, n)| (n.pid, i))
        .collect();

    let over_before = runs(&over_days(input, &base), &base, |_| "");

    let mut overrides: HashMap<Uuid, NaiveDate> = HashMap::new();
    let mut because: HashMap<Uuid, NaiveDate> = HashMap::new();
    let mut frozen: HashMap<Uuid, &'static str> = HashMap::new();
    let mut skipped: HashSet<(String, NaiveDate)> = HashSet::new();
    let mut current = base.clone();
    let mut truncated = false;

    // Why a task cannot be moved, if it cannot (before any attempt).
    let blocked = |pid: Uuid, r: &NodeResult| -> Option<&'static str> {
        if !demand_of.get(&pid).is_some_and(|d| d.movable) {
            return Some("fixed_tasks");
        }
        if node_of
            .get(&pid)
            .and_then(|n| n.constraint)
            .is_some_and(|c| c.kind.is_hard())
        {
            return Some("hard_constraint");
        }
        if !params.allow_critical && r.total_float <= 0 {
            return Some("critical_task");
        }
        None
    };

    let mut iterations = 0;
    'search: loop {
        let days = over_days(input, &current);
        let Some(d) = days
            .into_iter()
            .find(|d| !skipped.contains(&(d.person.clone(), d.day)))
        else {
            break;
        };
        iterations += 1;
        if iterations > MAX_ITERATIONS {
            truncated = true;
            break;
        }
        let empty = Resource::default();
        let res_of = input.resources.get(&d.person).unwrap_or(&empty);
        let background: i64 = res_of
            .background
            .iter()
            .filter(|a| a.start.is_none_or(|s| s <= d.day) && a.end.is_none_or(|e| e >= d.day))
            .map(|a| i64::from(a.percent))
            .sum();

        // Keep in priority order; what does not fit is displaced.
        let mut ranked: Vec<usize> = d.active.clone();
        ranked.sort_by_key(|&i| {
            let n = &current.nodes[i];
            let fixed = blocked(n.pid, n).is_some() || frozen.contains_key(&n.pid);
            // Fixed first, then tasks not yet delayed, then tasks already
            // delayed: a delayed task keeps sliding rather than displacing
            // another (its float shrinks as it moves, which must not promote it).
            let class = if fixed {
                0
            } else if overrides.contains_key(&n.pid) {
                2
            } else {
                1
            };
            (
                class,
                n.total_float,
                n.es_index,
                order.get(&n.pid).copied().unwrap_or(usize::MAX),
            )
        });
        let mut used = background;
        let mut displaced: Vec<usize> = Vec::new();
        for &i in &ranked {
            let n = &current.nodes[i];
            let pct = i64::from(demand_of[&n.pid].percent);
            let fixed = blocked(n.pid, n).is_some() || frozen.contains_key(&n.pid);
            if fixed || used + pct <= i64::from(d.capacity) {
                used += pct;
            } else {
                displaced.push(i);
            }
        }
        // The lowest-priority displaced task that is allowed to move.
        let candidate = displaced.iter().rev().copied().find(|&i| {
            let n = &current.nodes[i];
            blocked(n.pid, n).is_none() && !frozen.contains_key(&n.pid)
        });
        let Some(c) = candidate else {
            skipped.insert((d.person.clone(), d.day));
            continue 'search;
        };
        let before = current.nodes[c].clone();
        let Some(idx) = input.calendar.index_of(d.day) else {
            frozen.insert(before.pid, "did_not_move");
            continue 'search;
        };
        let target = input.calendar.date_of(idx + 1);
        let mut trial = overrides.clone();
        trial.insert(before.pid, target);
        let Ok(after) = run(input, &trial) else {
            frozen.insert(before.pid, "no_float");
            continue 'search;
        };
        let moved = after
            .nodes
            .iter()
            .find(|n| n.pid == before.pid)
            .is_some_and(|n| n.es_index > before.es_index);
        let valid = negative(&after).is_subset(&negative(&current))
            && conflicted(&after).is_subset(&conflicted(&current))
            && after.unscheduled.len() <= current.unscheduled.len()
            && (params.allow_critical || after.project_finish == current.project_finish);
        if !moved {
            frozen.insert(before.pid, "did_not_move");
        } else if !valid {
            frozen.insert(before.pid, "no_float");
        } else {
            overrides.insert(before.pid, target);
            because.entry(before.pid).or_insert(d.day);
            current = after;
        }
    }

    // What is left, with the reason a person can act on.
    let left = over_days(input, &current);
    let unresolved = runs(&left, &current, |tasks| {
        let reasons: Vec<&'static str> = tasks
            .iter()
            .filter_map(|t| {
                let n = current.nodes.iter().find(|n| n.pid == *t)?;
                frozen.get(t).copied().or_else(|| blocked(*t, n))
            })
            .collect();
        if truncated {
            "limit"
        } else if reasons.contains(&"no_float") {
            "no_float"
        } else if reasons.contains(&"critical_task") {
            "critical_task"
        } else if reasons.contains(&"hard_constraint") {
            "hard_constraint"
        } else if reasons.contains(&"did_not_move") {
            "did_not_move"
        } else {
            "fixed_tasks"
        }
    });

    let was: HashMap<Uuid, &NodeResult> = base.nodes.iter().map(|n| (n.pid, n)).collect();
    let now: HashMap<Uuid, &NodeResult> = current.nodes.iter().map(|n| (n.pid, n)).collect();
    let mut moves: Vec<Move> = overrides
        .iter()
        .filter_map(|(pid, to_start)| {
            let (b, a) = (was.get(pid)?, now.get(pid)?);
            Some(Move {
                pid: *pid,
                person: demand_of.get(pid)?.person.clone(),
                from_start: b.early_start,
                to_start: *to_start,
                scheduled_start: a.early_start,
                from_finish: b.early_finish,
                to_finish: a.early_finish,
                delay_days: a.es_index - b.es_index,
                critical: b.total_float <= 0,
                float_before: b.total_float,
                float_after: a.total_float,
                because_of: *because.get(pid)?,
            })
        })
        .collect();
    moves.sort_by_key(|m| order.get(&m.pid).copied().unwrap_or(usize::MAX));
    let effects: Vec<Moved> = diff(&base, &current)
        .into_iter()
        .filter(|m| !overrides.contains_key(&m.pid))
        .collect();
    let finish_delay_days = match (base.project_finish, current.project_finish) {
        (Some(b), Some(a)) => {
            input.calendar.index_of(a).unwrap_or(0) - input.calendar.index_of(b).unwrap_or(0)
        }
        _ => 0,
    };
    Ok(Proposal {
        critical_moved: moves.iter().any(|m| m.critical),
        moves,
        effects,
        over_before,
        unresolved,
        finish_before: base.project_finish,
        finish_after: current.project_finish,
        finish_delay_days,
        truncated,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpm::Kind;

    fn id(n: u128) -> Uuid {
        Uuid::from_u128(n)
    }
    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).unwrap()
    }
    /// Monday 2026-10-05.
    fn monday() -> NaiveDate {
        d(2026, 10, 5)
    }
    fn node(n: u128, dur: i64) -> Node {
        Node {
            pid: id(n),
            duration: Some(dur),
            planned_start: Some(monday()),
            ..Node::default()
        }
    }
    fn demand(n: u128, who: &str, movable: bool) -> Demand {
        Demand {
            pid: id(n),
            person: who.to_string(),
            percent: 100,
            movable,
        }
    }
    fn go(
        nodes: &[Node],
        edges: &[Edge],
        demands: &[Demand],
        resources: &HashMap<String, Resource>,
        params: Params,
    ) -> Proposal {
        let cal = WorkCalendar::mon_fri();
        propose(
            &Input {
                nodes,
                edges,
                calendar: &cal,
                own: &[],
                demands,
                resources,
            },
            params,
        )
        .unwrap()
    }

    #[test]
    fn delays_the_task_with_float_and_keeps_the_finish() {
        // A is the 5-day critical task; B and C (2 days, float 3) share a person.
        let nodes = [node(1, 5), node(2, 2), node(3, 2)];
        let demands = [demand(2, "x", true), demand(3, "x", true)];
        let p = go(&nodes, &[], &demands, &HashMap::new(), Params::default());
        assert_eq!(p.over_before.len(), 1);
        assert_eq!(p.moves.len(), 1);
        let m = &p.moves[0];
        assert_eq!(m.pid, id(3), "input order breaks the tie: B keeps its date");
        assert_eq!(
            (m.from_start, m.scheduled_start),
            (monday(), d(2026, 10, 7))
        );
        assert_eq!(m.delay_days, 2);
        assert!(!m.critical);
        assert_eq!(p.finish_before, p.finish_after);
        assert_eq!(p.finish_delay_days, 0);
        assert!(p.unresolved.is_empty());
    }

    #[test]
    fn nothing_over_allocated_gives_an_empty_proposal() {
        let nodes = [node(1, 5), node(2, 2)];
        let demands = [demand(1, "x", true), demand(2, "y", true)];
        let p = go(&nodes, &[], &demands, &HashMap::new(), Params::default());
        assert!(p.moves.is_empty() && p.effects.is_empty());
        assert!(p.over_before.is_empty() && p.unresolved.is_empty());
        assert!(!p.truncated);
    }

    #[test]
    fn a_critical_task_is_not_moved_unless_allowed() {
        let nodes = [node(1, 5), node(2, 5)];
        let demands = [demand(1, "x", true), demand(2, "x", true)];
        let p = go(&nodes, &[], &demands, &HashMap::new(), Params::default());
        assert!(p.moves.is_empty(), "both are critical");
        assert_eq!(p.unresolved.len(), 1);
        assert_eq!(p.unresolved[0].reason, Some("critical_task"));

        let p = go(
            &nodes,
            &[],
            &demands,
            &HashMap::new(),
            Params {
                allow_critical: true,
            },
        );
        assert_eq!(p.moves.len(), 1);
        assert!(p.moves[0].critical && p.critical_moved);
        assert_eq!(p.finish_delay_days, 5, "the finish impact is stated");
        assert_eq!(p.finish_after, Some(d(2026, 10, 16)));
        assert!(p.unresolved.is_empty());
    }

    #[test]
    fn a_started_task_keeps_its_date() {
        let nodes = [node(1, 5), node(2, 2), node(3, 2)];
        // B is started (fixed): C must give way even though input order favours B.
        let demands = [demand(2, "x", false), demand(3, "x", true)];
        let p = go(&nodes, &[], &demands, &HashMap::new(), Params::default());
        assert_eq!(p.moves.len(), 1);
        assert_eq!(p.moves[0].pid, id(3));

        // Both started: nothing may move, and the reason says so.
        let demands = [demand(2, "x", false), demand(3, "x", false)];
        let p = go(&nodes, &[], &demands, &HashMap::new(), Params::default());
        assert!(p.moves.is_empty());
        assert_eq!(p.unresolved[0].reason, Some("fixed_tasks"));
    }

    #[test]
    fn leave_is_capacity_zero_and_the_task_moves_past_it() {
        // A is 5 days, B is 1 day (float 4); the person is on leave on Monday.
        let nodes = [node(1, 5), node(2, 1)];
        let demands = [demand(2, "x", true)];
        let mut r = HashMap::new();
        r.insert(
            "x".to_string(),
            Resource {
                ranges: vec![Range {
                    from: monday(),
                    to: monday(),
                    max_percent: 0,
                }],
                background: vec![],
            },
        );
        let p = go(&nodes, &[], &demands, &r, Params::default());
        assert_eq!(p.moves.len(), 1);
        assert_eq!(p.moves[0].scheduled_start, d(2026, 10, 6));
        assert!(p.unresolved.is_empty());
    }

    #[test]
    fn background_load_counts() {
        // 60% booked elsewhere: a single 100% task is over on every day.
        let nodes = [node(1, 5), node(2, 1)];
        let demands = [demand(2, "x", true)];
        let mut r = HashMap::new();
        r.insert(
            "x".to_string(),
            Resource {
                ranges: vec![],
                background: vec![Alloc {
                    percent: 60,
                    start: Some(monday()),
                    end: Some(monday()),
                }],
            },
        );
        let p = go(&nodes, &[], &demands, &r, Params::default());
        assert_eq!(p.over_before.len(), 1);
        assert_eq!(p.over_before[0].demand, 160);
        assert_eq!(p.moves[0].scheduled_start, d(2026, 10, 6));
    }

    #[test]
    fn float_exhausted_is_reported_not_forced() {
        // B has one day of float and must be delayed two to clear the conflict.
        let nodes = [node(1, 3), node(2, 2), node(3, 2)];
        let edges = [Edge {
            pid: id(100),
            pred: id(2),
            succ: id(1),
            kind: Kind::Fs,
            lag: 0,
        }];
        // Network: B(2d) -> A(3d) is the critical chain (5 days); C is 2 days, float 3.
        // Make C's person also do B, so B/C overlap; B is critical so C moves.
        let demands = [demand(2, "x", true), demand(3, "x", true)];
        let p = go(&nodes, &edges, &demands, &HashMap::new(), Params::default());
        assert_eq!(p.finish_before, p.finish_after);
        assert!(p.moves.iter().all(|m| !m.critical));
        // Now give C no float (a 5-day task): the conflict cannot be removed.
        let nodes = [node(1, 3), node(2, 2), node(3, 5)];
        let p = go(&nodes, &edges, &demands, &HashMap::new(), Params::default());
        assert!(p.moves.is_empty());
        assert!(!p.unresolved.is_empty());
        assert_eq!(p.finish_before, p.finish_after);
    }

    #[test]
    fn the_same_input_gives_the_same_proposal() {
        let nodes = [node(1, 6), node(2, 2), node(3, 2), node(4, 3)];
        let demands = [
            demand(2, "x", true),
            demand(3, "x", true),
            demand(4, "x", true),
        ];
        let a = go(&nodes, &[], &demands, &HashMap::new(), Params::default());
        let b = go(&nodes, &[], &demands, &HashMap::new(), Params::default());
        assert_eq!(
            serde_json::to_string(&a).unwrap(),
            serde_json::to_string(&b).unwrap()
        );
    }

    #[test]
    fn a_dependent_is_pushed_and_the_effect_is_listed() {
        // A is 8 days (critical). B is 3 days (float 5); C (2 days) -> D (1 day)
        // also has float 5. B and C share a person; the tie goes to input order,
        // so C slides past B and D, which follows C, is pushed with it.
        let mut nodes = vec![node(1, 8), node(2, 3), node(3, 2)];
        nodes.push(Node {
            planned_start: None,
            ..node(4, 1)
        });
        let edges = [Edge {
            pid: id(100),
            pred: id(3),
            succ: id(4),
            kind: Kind::Fs,
            lag: 0,
        }];
        let demands = [demand(2, "x", true), demand(3, "x", true)];
        let p = go(&nodes, &edges, &demands, &HashMap::new(), Params::default());
        assert_eq!(p.moves.len(), 1);
        assert_eq!(p.moves[0].pid, id(3));
        assert!(p.effects.iter().any(|e| e.pid == id(4)), "D is listed");
        assert_eq!(p.finish_before, p.finish_after);
    }

    #[test]
    fn less_float_keeps_its_date() {
        // C -> D has less float than B, so B is the one that gives way.
        let mut nodes = vec![node(1, 8), node(2, 2), node(3, 2)];
        nodes.push(Node {
            planned_start: None,
            ..node(4, 2)
        });
        let edges = [Edge {
            pid: id(100),
            pred: id(3),
            succ: id(4),
            kind: Kind::Fs,
            lag: 0,
        }];
        let demands = [demand(2, "x", true), demand(3, "x", true)];
        let p = go(&nodes, &edges, &demands, &HashMap::new(), Params::default());
        assert_eq!(p.moves.len(), 1);
        assert_eq!(p.moves[0].pid, id(2));
    }

    /// A small deterministic generator for the property test.
    struct Lcg(u64);
    impl Lcg {
        fn next(&mut self, n: u64) -> u64 {
            self.0 = self
                .0
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            (self.0 >> 33) % n
        }
        fn int(&mut self, n: u32) -> i64 {
            i64::from(u32::try_from(self.next(u64::from(n))).unwrap())
        }
        fn idx(&mut self, n: usize) -> usize {
            usize::try_from(self.next(u64::try_from(n).unwrap())).unwrap()
        }
    }

    #[test]
    fn a_proposal_never_breaks_a_dependency_or_the_finish() {
        let cal = WorkCalendar::mon_fri();
        let mut rng = Lcg(7);
        for _ in 0..150 {
            let n = 3 + rng.idx(8);
            let nodes: Vec<Node> = (0..n)
                .map(|i| Node {
                    pid: id(i as u128 + 1),
                    duration: Some(1 + rng.int(5)),
                    planned_start: Some(monday() + Duration::days(rng.int(10))),
                    ..Node::default()
                })
                .collect();
            let mut edges = Vec::new();
            for j in 1..n {
                if rng.next(3) == 0 {
                    let i = rng.idx(j);
                    edges.push(Edge {
                        pid: id(1000 + j as u128),
                        pred: nodes[i].pid,
                        succ: nodes[j].pid,
                        kind: Kind::Fs,
                        lag: rng.int(2),
                    });
                }
            }
            let demands: Vec<Demand> = nodes
                .iter()
                .map(|nd| Demand {
                    pid: nd.pid,
                    person: format!("p{}", rng.next(2)),
                    percent: 100,
                    movable: rng.next(5) != 0,
                })
                .collect();
            let resources = HashMap::new();
            let input = Input {
                nodes: &nodes,
                edges: &edges,
                calendar: &cal,
                own: &[],
                demands: &demands,
                resources: &resources,
            };
            let p = propose(&input, Params::default()).unwrap();
            assert!(!p.critical_moved);
            assert_eq!(p.finish_before, p.finish_after, "finish never moves");
            // Apply the moves and check the logic still holds.
            let mut applied = nodes.clone();
            for m in &p.moves {
                let nd = applied.iter_mut().find(|n| n.pid == m.pid).unwrap();
                nd.planned_start = Some(m.to_start);
                assert!(m.delay_days > 0);
                assert!(demands.iter().find(|x| x.pid == m.pid).unwrap().movable);
            }
            let res = analyze_with(&applied, &edges, &cal, &[], Options::default()).unwrap();
            let at: HashMap<Uuid, &NodeResult> = res.nodes.iter().map(|n| (n.pid, n)).collect();
            for e in &edges {
                let (a, b) = (at[&e.pred], at[&e.succ]);
                assert!(b.es_index >= a.ef_index + e.lag, "dependency respected");
            }
            assert!(res.nodes.iter().all(|n| n.total_float >= 0));
        }
    }
}
