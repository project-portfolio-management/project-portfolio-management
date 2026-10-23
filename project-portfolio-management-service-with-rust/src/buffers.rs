//! Project, feeding and resource buffers (CC-3). Pure rules; the database work is in
//! [`crate::controllers::buffers`].
//!
//! Tasks are scheduled at their **aggressive** estimates and the safety taken out (safe minus aggressive,
//! per task) is pooled into buffers, in business days of the plan's calendar:
//!
//! * the **project buffer** sits at the end of the critical chain and is sized from the chain's tasks;
//! * a **feeding buffer** sits where a chain of non-critical tasks joins the critical chain and is sized
//!   from that feeding chain's tasks;
//! * **resource buffers** are not time but readiness alerts (see the controller).
//!
//! Two sizing rules, chosen by `PROJECT_PORTFOLIO_MANAGEMENT_BUFFER_RULE` and printed with every answer:
//! `cut_and_paste` (half of the removed safety) and `root_sum_squares` (the square root of the sum of the
//! squares of each task's safety). Both round up to a whole business day, and a pool with any safety is
//! never smaller than one day. The rules are conventions of the method, not laws.

#![expect(missing_docs)]

use std::collections::{HashMap, HashSet};

use serde::Serialize;
use uuid::Uuid;

use crate::cpm::Edge;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SizeRule {
    CutAndPaste,
    RootSumSquares,
}

impl SizeRule {
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::CutAndPaste => "cut_and_paste",
            Self::RootSumSquares => "root_sum_squares",
        }
    }
}

/// The rule in force; the flag is true when a value was set but unusable (the default is then used).
#[must_use]
pub fn configured_rule() -> (SizeRule, bool) {
    match std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_BUFFER_RULE") {
        Err(_) => (SizeRule::CutAndPaste, false),
        Ok(v) => match v.trim() {
            "cut_and_paste" => (SizeRule::CutAndPaste, false),
            "root_sum_squares" => (SizeRule::RootSumSquares, false),
            _ => (SizeRule::CutAndPaste, true),
        },
    }
}

/// A buffer's size in business days from each task's removed safety.
#[must_use]
pub fn size(rule: SizeRule, safety: &[i64]) -> i64 {
    let total: i64 = safety.iter().copied().filter(|s| *s > 0).sum();
    if total == 0 {
        return 0;
    }
    let days = match rule {
        SizeRule::CutAndPaste => (total + 1) / 2,
        SizeRule::RootSumSquares => {
            let squares: i64 = safety.iter().filter(|s| **s > 0).map(|s| s * s).sum();
            isqrt_ceil(squares)
        }
    };
    days.max(1)
}

/// The smallest `n` with `n * n >= x`.
fn isqrt_ceil(x: i64) -> i64 {
    let mut n: i64 = 0;
    while n * n < x {
        n += 1;
    }
    n
}

/// A chain of non-critical tasks that joins the critical chain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Feeding {
    /// The critical-chain task it feeds.
    pub joins: Uuid,
    /// The feeding tasks, first to last.
    pub tasks: Vec<Uuid>,
    /// Summed aggressive duration of those tasks.
    pub duration_days: i64,
}

/// Find the feeding chains: for each dependency from outside the chain into it, the longest path (by
/// `duration`) of tasks outside the chain that ends at the dependency's predecessor. A task belongs to
/// one feeding chain only (the one joining earliest on the chain); a later join whose feeder is already
/// taken is not repeated. `chain` is in order. The result is in chain order, then input order.
#[must_use]
#[expect(clippy::missing_panics_doc, clippy::implicit_hasher)]
pub fn feeding_chains(
    chain: &[Uuid],
    nodes: &[Uuid],
    edges: &[Edge],
    duration: &HashMap<Uuid, i64>,
) -> Vec<Feeding> {
    let on_chain: HashSet<Uuid> = chain.iter().copied().collect();
    let mut preds: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
    for e in edges {
        preds.entry(e.succ).or_default().push(e.pred);
    }
    // Topological order of the nodes outside the chain (Kahn), so the longest-path pass needs no recursion.
    let outside: Vec<Uuid> = nodes
        .iter()
        .copied()
        .filter(|n| !on_chain.contains(n))
        .collect();
    let outside_set: HashSet<Uuid> = outside.iter().copied().collect();
    let mut indegree: HashMap<Uuid, usize> = outside.iter().map(|n| (*n, 0)).collect();
    let mut succs: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
    for e in edges {
        if outside_set.contains(&e.pred) && outside_set.contains(&e.succ) {
            *indegree.entry(e.succ).or_default() += 1;
            succs.entry(e.pred).or_default().push(e.succ);
        }
    }
    let mut queue: Vec<Uuid> = outside
        .iter()
        .copied()
        .filter(|n| indegree[n] == 0)
        .collect();
    let mut order = Vec::new();
    let mut head = 0;
    while head < queue.len() {
        let n = queue[head];
        head += 1;
        order.push(n);
        for s in succs.get(&n).into_iter().flatten() {
            let d = indegree.get_mut(s).expect("known");
            *d -= 1;
            if *d == 0 {
                queue.push(*s);
            }
        }
    }
    // longest[n]: the greatest summed duration of a path of outside tasks ending at n; via[n]: its previous task.
    let mut longest: HashMap<Uuid, i64> = HashMap::new();
    let mut via: HashMap<Uuid, Option<Uuid>> = HashMap::new();
    for n in &order {
        let mut best: Option<(i64, Uuid)> = None;
        for p in preds.get(n).into_iter().flatten() {
            if let Some(l) = longest.get(p)
                && best.is_none_or(|(b, _)| *l > b)
            {
                best = Some((*l, *p));
            }
        }
        longest.insert(
            *n,
            duration.get(n).copied().unwrap_or(0) + best.map_or(0, |b| b.0),
        );
        via.insert(*n, best.map(|b| b.1));
    }
    let position: HashMap<Uuid, usize> = chain.iter().enumerate().map(|(i, c)| (*c, i)).collect();
    let mut joins: Vec<(usize, usize, Uuid, Uuid)> = Vec::new();
    for (i, e) in edges.iter().enumerate() {
        if let Some(pos) = position.get(&e.succ)
            && !on_chain.contains(&e.pred)
            && longest.contains_key(&e.pred)
        {
            joins.push((*pos, i, e.pred, e.succ));
        }
    }
    joins.sort_by_key(|j| (j.0, j.1));
    let mut taken: HashSet<Uuid> = HashSet::new();
    let mut out = Vec::new();
    for (_, _, pred, join) in joins {
        if taken.contains(&pred) {
            continue;
        }
        let mut tasks = Vec::new();
        let mut cur = Some(pred);
        while let Some(t) = cur {
            if taken.contains(&t) {
                break;
            }
            tasks.push(t);
            cur = via.get(&t).copied().flatten();
        }
        tasks.reverse();
        for t in &tasks {
            taken.insert(*t);
        }
        let duration_days = tasks
            .iter()
            .map(|t| duration.get(t).copied().unwrap_or(0))
            .sum();
        out.push(Feeding {
            joins: join,
            tasks,
            duration_days,
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpm::Kind;

    fn id(n: u128) -> Uuid {
        Uuid::from_u128(n)
    }

    fn edge(n: u128, a: u128, b: u128) -> Edge {
        Edge {
            pid: id(n),
            pred: id(a),
            succ: id(b),
            kind: Kind::Fs,
            lag: 0,
        }
    }

    #[test]
    fn cut_and_paste_is_half_the_removed_safety_rounded_up() {
        // safety 4, 6, 3: total 13, half 6.5, rounded up 7.
        assert_eq!(size(SizeRule::CutAndPaste, &[4, 6, 3]), 7);
        assert_eq!(size(SizeRule::CutAndPaste, &[4, 6]), 5);
        assert_eq!(
            size(SizeRule::CutAndPaste, &[1]),
            1,
            "never below a day when there is safety"
        );
        assert_eq!(
            size(SizeRule::CutAndPaste, &[0, 0]),
            0,
            "no safety, no buffer"
        );
    }

    #[test]
    fn root_sum_squares_is_the_root_of_the_squares_rounded_up() {
        // 3 and 4: root of 25 is exactly 5.
        assert_eq!(size(SizeRule::RootSumSquares, &[3, 4]), 5);
        // 2 and 2: root of 8 is 2.83, rounded up 3.
        assert_eq!(size(SizeRule::RootSumSquares, &[2, 2]), 3);
        assert_eq!(size(SizeRule::RootSumSquares, &[0]), 0);
        // It is smaller than cut and paste for many tasks, which is the point of the rule.
        let many = [2; 16];
        assert!(size(SizeRule::RootSumSquares, &many) < size(SizeRule::CutAndPaste, &many));
    }

    #[test]
    fn a_feeding_chain_is_the_longest_run_of_outside_tasks_into_the_chain() {
        // Chain: 10 -> 11. Feeders: 1 -> 2 -> 11 (durations 3, 2) and 3 -> 11 (duration 4).
        let nodes = [id(10), id(11), id(1), id(2), id(3)];
        let edges = [
            edge(1, 10, 11),
            edge(2, 1, 2),
            edge(3, 2, 11),
            edge(4, 3, 11),
        ];
        let dur: HashMap<Uuid, i64> = [(10, 5), (11, 5), (1, 3), (2, 2), (3, 4)]
            .iter()
            .map(|(n, d)| (id(*n), *d))
            .collect();
        let f = feeding_chains(&[id(10), id(11)], &nodes, &edges, &dur);
        assert_eq!(f.len(), 2);
        assert_eq!(
            (f[0].joins, &f[0].tasks[..], f[0].duration_days),
            (id(11), &[id(1), id(2)][..], 5)
        );
        assert_eq!(
            (f[1].joins, &f[1].tasks[..], f[1].duration_days),
            (id(11), &[id(3)][..], 4)
        );
    }

    #[test]
    fn a_task_feeding_two_joins_is_buffered_once() {
        // Chain 10 -> 11 -> 12. Task 1 feeds both 11 and 12: the earlier join takes it.
        let nodes = [id(10), id(11), id(12), id(1)];
        let edges = [
            edge(1, 10, 11),
            edge(2, 11, 12),
            edge(3, 1, 11),
            edge(4, 1, 12),
        ];
        let dur: HashMap<Uuid, i64> = [(1, 3)].iter().map(|(n, d)| (id(*n), *d)).collect();
        let f = feeding_chains(&[id(10), id(11), id(12)], &nodes, &edges, &dur);
        assert_eq!(f.len(), 1);
        assert_eq!(f[0].joins, id(11));
    }

    #[test]
    fn no_outside_task_means_no_feeding_chain() {
        let nodes = [id(10), id(11)];
        let f = feeding_chains(
            &[id(10), id(11)],
            &nodes,
            &[edge(1, 10, 11)],
            &HashMap::new(),
        );
        assert!(f.is_empty());
    }
}
