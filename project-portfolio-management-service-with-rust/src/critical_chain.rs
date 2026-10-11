//! The critical chain (CC-1). Pure rules; the database work is in [`crate::controllers::critical_chain`].
//!
//! The critical **path** is the longest chain through dependencies alone ([`crate::cpm`]). The critical
//! **chain** is the longest chain through dependencies *and resource links*, taken from a schedule that has
//! already been levelled ([`crate::leveling`]): a task that waits for a person is linked to the task that
//! person was on. The two are reported side by side and never blended.
//!
//! # The rule
//!
//! On the levelled schedule, a **dependency link** joins a finish-to-start dependency whose successor starts
//! exactly when the predecessor finishes plus its lag (the dependency is what held it back). A **resource
//! link** joins a task to each task of the same person that finishes exactly when it starts (it started when
//! the person came free). The chain is the path of such links with the greatest summed duration that ends at
//! a task finishing on the project's last day; ties go to the earlier task in the input, so the answer is a
//! pure function of its input.
//!
//! Stated limits: only finish-to-start dependencies are followed (the other three kinds still shape the
//! schedule, but are not chain links); a milestone carries no work and is not a resource link; only the
//! first of several equally long chains is returned, and `tied_chains` says whether there was another.

#![allow(missing_docs)]

use std::collections::{HashMap, HashSet};

use serde::Serialize;
use uuid::Uuid;

use crate::cpm::{CpmResult, Edge, Kind, Node};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LinkKind {
    Dependency,
    Resource,
}

/// One link of the chain: `to` could not start before `from` finished.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Link {
    pub from: Uuid,
    pub to: Uuid,
    pub kind: LinkKind,
    /// The person the resource link waits for (set only for a resource link).
    pub person: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ChainTask {
    pub pid: Uuid,
    pub duration_days: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Chain {
    /// In order, first to last.
    pub tasks: Vec<ChainTask>,
    /// One fewer than the tasks: how each is held to the one before.
    pub links: Vec<Link>,
    /// Summed duration of the chain's tasks, in business days.
    pub chain_days: i64,
    /// The chain has at least one resource link, so it is not the critical path.
    pub crosses_paths: bool,
    /// Another chain of the same length ends the project.
    pub tied_chains: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum Outcome {
    Chain(Chain),
    /// No chain, and why: `no_assignments` or `no_scheduled_tasks`.
    NoCriticalChain { reason: &'static str },
}

/// Find the critical chain on a levelled schedule. `people` maps a task to the person doing it (only
/// assigned, unfinished tasks).
#[must_use]
#[allow(clippy::too_many_lines, clippy::implicit_hasher)]
pub fn find(
    nodes: &[Node],
    edges: &[Edge],
    levelled: &CpmResult,
    people: &HashMap<Uuid, String>,
) -> Outcome {
    if people.is_empty() {
        return Outcome::NoCriticalChain {
            reason: "no_assignments",
        };
    }
    let dur: HashMap<Uuid, i64> = nodes
        .iter()
        .map(|n| (n.pid, n.duration.unwrap_or(0)))
        .collect();
    let position: HashMap<Uuid, usize> = nodes.iter().enumerate().map(|(i, n)| (n.pid, i)).collect();
    let mut order: Vec<&crate::cpm::NodeResult> = levelled.nodes.iter().collect();
    order.sort_by_key(|n| (n.es_index, n.ef_index, position.get(&n.pid).copied()));
    if order.is_empty() {
        return Outcome::NoCriticalChain {
            reason: "no_scheduled_tasks",
        };
    }
    let rank: HashMap<Uuid, usize> = order.iter().enumerate().map(|(i, n)| (n.pid, i)).collect();
    let at: HashMap<Uuid, &crate::cpm::NodeResult> = order.iter().map(|n| (n.pid, *n)).collect();

    // What held each task back, as (earlier task, link).
    let mut into: HashMap<Uuid, Vec<(Uuid, LinkKind, Option<String>)>> = HashMap::new();
    let mut seen: HashSet<(Uuid, Uuid)> = HashSet::new();
    for e in edges.iter().filter(|e| e.kind == Kind::Fs) {
        let (Some(p), Some(s)) = (at.get(&e.pred), at.get(&e.succ)) else {
            continue;
        };
        if s.es_index == p.ef_index + e.lag && rank[&e.pred] < rank[&e.succ] && seen.insert((e.pred, e.succ)) {
            into.entry(e.succ)
                .or_default()
                .push((e.pred, LinkKind::Dependency, None));
        }
    }
    for b in &order {
        let Some(person) = people.get(&b.pid) else {
            continue;
        };
        if dur.get(&b.pid).copied().unwrap_or(0) == 0 {
            continue;
        }
        for a in &order {
            if a.pid != b.pid
                && people.get(&a.pid) == Some(person)
                && dur.get(&a.pid).copied().unwrap_or(0) > 0
                && a.ef_index == b.es_index
                && rank[&a.pid] < rank[&b.pid]
                && seen.insert((a.pid, b.pid))
            {
                into.entry(b.pid)
                    .or_default()
                    .push((a.pid, LinkKind::Resource, Some(person.clone())));
            }
        }
    }

    // Longest path, in schedule order (every link goes from an earlier rank to a later one).
    let mut best: HashMap<Uuid, (i64, Option<usize>)> = HashMap::new();
    for n in &order {
        let own = dur.get(&n.pid).copied().unwrap_or(0);
        let mut pick: Option<(i64, usize)> = None;
        for (i, (from, _, _)) in into.get(&n.pid).into_iter().flatten().enumerate() {
            let len = best[from].0;
            if pick.is_none_or(|(l, _)| len > l) {
                pick = Some((len, i));
            }
        }
        best.insert(n.pid, (own + pick.map_or(0, |p| p.0), pick.map(|p| p.1)));
    }

    let finish = order.iter().map(|n| n.ef_index).max().unwrap_or(0);
    let ends: Vec<&&crate::cpm::NodeResult> = order.iter().filter(|n| n.ef_index == finish).collect();
    let top = ends.iter().map(|n| best[&n.pid].0).max().unwrap_or(0);
    let at_top: Vec<&&crate::cpm::NodeResult> =
        ends.iter().copied().filter(|n| best[&n.pid].0 == top).collect();
    let Some(end) = at_top.first() else {
        return Outcome::NoCriticalChain {
            reason: "no_scheduled_tasks",
        };
    };

    let mut tasks = vec![end.pid];
    let mut links: Vec<Link> = Vec::new();
    let mut cur = end.pid;
    let mut tied = at_top.len() > 1;
    while let Some(i) = best[&cur].1 {
        let longest = best[&into[&cur][i].0].0;
        tied |= into[&cur].iter().filter(|(f, _, _)| best[f].0 == longest).count() > 1;
        let (from, kind, person) = into[&cur][i].clone();
        links.push(Link {
            from,
            to: cur,
            kind,
            person,
        });
        tasks.push(from);
        cur = from;
    }
    tasks.reverse();
    links.reverse();
    let chain_days = tasks.iter().map(|p| dur.get(p).copied().unwrap_or(0)).sum();
    Outcome::Chain(Chain {
        crosses_paths: links.iter().any(|l| l.kind == LinkKind::Resource),
        tied_chains: tied,
        tasks: tasks
            .iter()
            .map(|p| ChainTask {
                pid: *p,
                duration_days: dur.get(p).copied().unwrap_or(0),
            })
            .collect(),
        links,
        chain_days,
    })
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::*;
    use crate::cpm::{Options, WorkCalendar, analyze};

    fn id(n: u128) -> Uuid {
        Uuid::from_u128(n)
    }

    fn node(n: u128, days: i64, start: Option<NaiveDate>) -> Node {
        Node {
            pid: id(n),
            duration: Some(days),
            planned_start: start,
            ..Node::default()
        }
    }

    fn fs(n: u128, a: u128, b: u128) -> Edge {
        Edge {
            pid: id(n),
            pred: id(a),
            succ: id(b),
            kind: Kind::Fs,
            lag: 0,
        }
    }

    fn day() -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 1, 5).unwrap()
    }

    fn run(nodes: &[Node], edges: &[Edge]) -> CpmResult {
        analyze(nodes, edges, &WorkCalendar::seven_day(), Options::default()).unwrap()
    }

    fn who(pairs: &[(u128, &str)]) -> HashMap<Uuid, String> {
        pairs.iter().map(|(n, p)| (id(*n), (*p).to_string())).collect()
    }

    fn pids(c: &Chain) -> Vec<Uuid> {
        c.tasks.iter().map(|t| t.pid).collect()
    }

    #[test]
    fn a_shared_person_makes_the_chain_cross_paths() {
        // A(3) -> B(2) and C(2) -> D(2). One person does B and D. Unlevelled, B ends day 5 and D
        // ends day 4 (the critical path is A, B). Levelled, D waits for that person and starts day 5.
        let edges = [fs(1, 1, 2), fs(2, 3, 4)];
        let plain = [node(1, 3, Some(day())), node(2, 2, None), node(3, 2, Some(day())), node(4, 2, None)];
        let path: Vec<Uuid> = run(&plain, &edges).chains[0].clone();
        assert_eq!(path, [id(1), id(2)]);
        let mut levelled = plain;
        levelled[3].planned_start = Some(day() + chrono::Duration::days(5));
        let r = run(&levelled, &edges);
        let out = find(&levelled, &edges, &r, &who(&[(2, "x"), (4, "x")]));
        let Outcome::Chain(c) = out else { panic!("a chain") };
        assert_eq!(pids(&c), [id(1), id(2), id(4)], "A, B, then D: it crosses to the other path");
        assert_eq!(c.chain_days, 7);
        assert!(c.crosses_paths);
        assert_eq!(c.links[1].kind, LinkKind::Resource);
        assert_eq!(c.links[1].person.as_deref(), Some("x"));
        assert_eq!(c.links[0].kind, LinkKind::Dependency);
        assert!(!c.tied_chains);
    }

    #[test]
    fn without_contention_the_chain_is_the_critical_path() {
        let edges = [fs(1, 1, 2), fs(2, 2, 3), fs(3, 4, 3)];
        let nodes = [node(1, 3, Some(day())), node(2, 4, None), node(3, 2, None), node(4, 2, Some(day()))];
        let r = run(&nodes, &edges);
        let people = who(&[(1, "a"), (2, "b"), (3, "c"), (4, "d")]);
        let Outcome::Chain(c) = find(&nodes, &edges, &r, &people) else { panic!("a chain") };
        assert_eq!(pids(&c), r.chains[0], "the same tasks in the same order");
        assert!(!c.crosses_paths);
        assert!(c.links.iter().all(|l| l.kind == LinkKind::Dependency));
        assert_eq!(c.chain_days, 9);
    }

    #[test]
    fn no_assignments_is_no_chain_with_the_reason() {
        let nodes = [node(1, 3, Some(day()))];
        let r = run(&nodes, &[]);
        assert_eq!(
            find(&nodes, &[], &r, &HashMap::new()),
            Outcome::NoCriticalChain { reason: "no_assignments" }
        );
    }

    #[test]
    fn equally_long_chains_are_reported_as_tied() {
        let edges = [fs(1, 1, 3), fs(2, 2, 3)];
        let nodes = [node(1, 2, Some(day())), node(2, 2, Some(day())), node(3, 1, None)];
        let r = run(&nodes, &edges);
        let Outcome::Chain(c) = find(&nodes, &edges, &r, &who(&[(1, "a")])) else { panic!("a chain") };
        assert_eq!(pids(&c), [id(1), id(3)], "the earlier task in the input wins");
        assert_eq!(c.chain_days, 3);
        assert!(c.tied_chains);
    }

    #[test]
    fn a_gap_in_a_persons_day_is_not_a_wait() {
        // B starts three days after the person finished A: they were not waiting for A.
        let nodes = [node(1, 2, Some(day())), node(2, 2, Some(day() + chrono::Duration::days(5)))];
        let r = run(&nodes, &[]);
        let Outcome::Chain(c) = find(&nodes, &[], &r, &who(&[(1, "x"), (2, "x")])) else { panic!("a chain") };
        assert_eq!(pids(&c), [id(2)]);
        assert!(c.links.is_empty());
    }
}
