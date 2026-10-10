//! Task relations (LT-10): the pure rules. DB-free, unit-tested.
//!
//! Kinds and what each *does* (the behaviours are Redmine's, proven in
//! daily use, and each refusal names the relation that caused it):
//!
//! - `related` — informational and symmetric.
//! - `blocks` — `from` blocks `to`: `to` cannot move to a finished state
//!   while `from` is still open. Acyclic.
//! - `precedes` — `from` precedes `to`: when both are dated, `to` must be due
//!   **after** `from`. Acyclic. (Tasks carry a due date, not a start date, so
//!   the rule is on due dates.)
//! - `duplicates` — `from` is a duplicate **of** `to`: closing `to` closes
//!   `from`.
//! - `copied_from` — `from` was copied from `to`. Informational.

use std::collections::{HashMap, HashSet};

use chrono::NaiveDate;
use uuid::Uuid;

/// The relation kinds.
pub const KINDS: &[&str] = &["related", "blocks", "precedes", "duplicates", "copied_from"];

/// Kinds that must stay acyclic.
#[must_use]
pub fn is_directed_acyclic(kind: &str) -> bool {
    matches!(kind, "blocks" | "precedes")
}

/// Kinds where `(a, b)` and `(b, a)` are the same relation.
#[must_use]
pub fn is_symmetric(kind: &str) -> bool {
    kind == "related"
}

/// Every problem with a relation's shape.
#[must_use]
pub fn problems(kind: &str, from: Uuid, to: Uuid) -> Vec<String> {
    let mut out = Vec::new();
    if !KINDS.contains(&kind) {
        out.push(format!("unknown kind {kind:?} (allowed: {KINDS:?})"));
    }
    if from == to {
        out.push("a task cannot be related to itself".to_string());
    }
    out
}

/// Whether adding `from -> to` to `edges` (the existing edges of the same
/// kind) would close a cycle: i.e. `from` is already reachable from `to`.
#[must_use]
pub fn creates_cycle(edges: &[(Uuid, Uuid)], from: Uuid, to: Uuid) -> bool {
    let mut next: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
    for (a, b) in edges {
        next.entry(*a).or_default().push(*b);
    }
    let mut stack = vec![to];
    let mut seen: HashSet<Uuid> = HashSet::new();
    while let Some(n) = stack.pop() {
        if n == from {
            return true;
        }
        if seen.insert(n) {
            stack.extend(next.get(&n).into_iter().flatten().copied());
        }
    }
    false
}

/// Whether a `precedes` relation is violated by the two due dates: both set
/// and the successor not strictly after the predecessor. An undated task
/// constrains nothing.
#[must_use]
pub fn precedes_violated(pred_due: Option<NaiveDate>, succ_due: Option<NaiveDate>) -> bool {
    matches!((pred_due, succ_due), (Some(p), Some(s)) if s <= p)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(n: u128) -> Uuid {
        Uuid::from_u128(n)
    }

    fn d(day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 10, day).unwrap()
    }

    #[test]
    fn shape_problems_are_all_reported() {
        assert_eq!(problems("nope", id(1), id(1)).len(), 2);
        assert!(problems("blocks", id(1), id(2)).is_empty());
    }

    #[test]
    fn a_cycle_is_detected_directly_and_transitively() {
        let edges = [(id(1), id(2)), (id(2), id(3))];
        assert!(
            creates_cycle(&edges, id(3), id(1)),
            "3 -> 1 closes 1->2->3->1"
        );
        assert!(creates_cycle(&edges, id(2), id(1)), "2 -> 1 closes 1->2->1");
        assert!(
            !creates_cycle(&edges, id(1), id(3)),
            "a shortcut is not a cycle"
        );
        assert!(!creates_cycle(&edges, id(4), id(1)), "an unrelated node");
        assert!(!creates_cycle(&[], id(1), id(2)));
    }

    #[test]
    fn precedes_needs_the_successor_strictly_later() {
        assert!(precedes_violated(Some(d(5)), Some(d(5))), "same day");
        assert!(precedes_violated(Some(d(5)), Some(d(4))), "earlier");
        assert!(!precedes_violated(Some(d(5)), Some(d(6))));
        assert!(
            !precedes_violated(None, Some(d(1))),
            "an undated predecessor constrains nothing"
        );
        assert!(!precedes_violated(Some(d(1)), None));
    }

    #[test]
    fn only_blocks_and_precedes_are_acyclic_and_only_related_is_symmetric() {
        for k in KINDS {
            assert_eq!(
                is_directed_acyclic(k),
                matches!(*k, "blocks" | "precedes"),
                "{k}"
            );
            assert_eq!(is_symmetric(k), *k == "related", "{k}");
        }
    }
}
