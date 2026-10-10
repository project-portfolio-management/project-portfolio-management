//! Subtask roll-up (RM-5): the pure rules. DB-free, unit-tested.
//!
//! A parent task's figures are **derived on read** from its descendants
//! (Redmine's rules, adapted to what a task here carries):
//!
//! - **due** = the latest due date among descendants;
//! - **estimate** = the sum of descendants' story `points`;
//! - **progress** = the share of that estimate which is finished
//!   (estimate-weighted), reported only over descendants that *have* an
//!   estimate — those without are **named**, never counted as zero;
//! - **spent** = recorded minutes across the parent and its descendants.
//!
//! Absent evidence is `None` with a stated reason, never zero.

use std::collections::{BTreeMap, HashSet};

use chrono::NaiveDate;
use serde::Serialize;
use uuid::Uuid;

/// Cap on nesting depth, so a parent chain cannot grow without bound.
pub const MAX_DEPTH: usize = 10;

/// What the roll-up needs to know about one descendant.
#[derive(Debug, Clone)]
pub struct Fact {
    /// Task pid.
    pub pid: Uuid,
    /// Due date, if set.
    pub due: Option<NaiveDate>,
    /// Story points, if estimated.
    pub points: Option<i32>,
    /// Whether it is finished.
    pub done: bool,
    /// Recorded minutes on it.
    pub spent_minutes: i64,
}

/// A derived figure, or why there is none.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Derived<T> {
    /// The value, `None` when it cannot be derived.
    pub value: Option<T>,
    /// Why `value` is `None`.
    pub reason: Option<&'static str>,
}

impl<T> Derived<T> {
    const fn some(v: T) -> Self {
        Self {
            value: Some(v),
            reason: None,
        }
    }
    const fn none(reason: &'static str) -> Self {
        Self {
            value: None,
            reason: Some(reason),
        }
    }
}

/// The rolled-up view of a parent.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Rollup {
    /// Descendant count.
    pub descendants: usize,
    /// Finished descendants.
    pub done: usize,
    /// Latest descendant due date.
    pub due: Derived<NaiveDate>,
    /// Summed story points of estimated descendants.
    pub estimate_points: Derived<i64>,
    /// Estimate-weighted progress in basis points (0..=10000).
    pub progress_bp: Derived<i64>,
    /// Descendants with no estimate, named so they are not silently zero.
    pub excluded_unestimated: Vec<Uuid>,
    /// Recorded minutes across the parent and its descendants.
    pub spent_minutes: i64,
}

/// Roll up `descendants` (not including the parent) plus the parent's own
/// recorded minutes.
#[must_use]
pub fn rollup(descendants: &[Fact], own_spent_minutes: i64) -> Rollup {
    let spent: i64 = own_spent_minutes + descendants.iter().map(|f| f.spent_minutes).sum::<i64>();
    if descendants.is_empty() {
        return Rollup {
            descendants: 0,
            done: 0,
            due: Derived::none("no_children"),
            estimate_points: Derived::none("no_children"),
            progress_bp: Derived::none("no_children"),
            excluded_unestimated: vec![],
            spent_minutes: spent,
        };
    }
    let due = descendants.iter().filter_map(|f| f.due).max();
    let estimated: Vec<&Fact> = descendants.iter().filter(|f| f.points.is_some()).collect();
    let total: i64 = estimated
        .iter()
        .map(|f| i64::from(f.points.unwrap_or(0)))
        .sum();
    let done_points: i64 = estimated
        .iter()
        .filter(|f| f.done)
        .map(|f| i64::from(f.points.unwrap_or(0)))
        .sum();
    Rollup {
        descendants: descendants.len(),
        done: descendants.iter().filter(|f| f.done).count(),
        due: due.map_or_else(|| Derived::none("no_dated_children"), Derived::some),
        estimate_points: if estimated.is_empty() {
            Derived::none("no_estimates")
        } else {
            Derived::some(total)
        },
        progress_bp: if estimated.is_empty() || total == 0 {
            Derived::none("no_estimates")
        } else {
            Derived::some(done_points * 10_000 / total)
        },
        excluded_unestimated: descendants
            .iter()
            .filter(|f| f.points.is_none())
            .map(|f| f.pid)
            .collect(),
        spent_minutes: spent,
    }
}

/// Whether making `parent` the parent of `child` is allowed given the
/// existing `child -> parent` links: not itself, not a descendant of the
/// child (a cycle), and not deeper than [`MAX_DEPTH`].
///
/// # Errors
///
/// A message naming the rule broken.
pub fn parent_allowed(
    links: &BTreeMap<Uuid, Uuid>,
    child: Uuid,
    parent: Uuid,
) -> Result<(), String> {
    if child == parent {
        return Err("a task cannot be its own parent".to_string());
    }
    let mut cursor = Some(parent);
    let mut depth = 1usize;
    let mut seen = HashSet::new();
    while let Some(n) = cursor {
        if n == child {
            return Err("that parent is one of the task's own descendants (a cycle)".to_string());
        }
        if !seen.insert(n) {
            break;
        }
        cursor = links.get(&n).copied();
        depth += 1;
    }
    if depth > MAX_DEPTH {
        return Err(format!("subtasks nest at most {MAX_DEPTH} deep"));
    }
    Ok(())
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
    fn fact(n: u128, due: Option<u32>, points: Option<i32>, done: bool, spent: i64) -> Fact {
        Fact {
            pid: id(n),
            due: due.map(d),
            points,
            done,
            spent_minutes: spent,
        }
    }

    #[test]
    fn a_parent_with_no_children_reports_none_with_a_reason_not_zero() {
        let r = rollup(&[], 30);
        assert_eq!(r.due, Derived::none("no_children"));
        assert_eq!(r.progress_bp, Derived::none("no_children"));
        assert_eq!(r.spent_minutes, 30, "its own time still counts");
    }

    #[test]
    fn due_is_the_latest_and_progress_is_estimate_weighted() {
        let r = rollup(
            &[
                fact(1, Some(3), Some(3), true, 60),
                fact(2, Some(9), Some(1), false, 0),
                fact(3, None, Some(6), false, 15),
            ],
            0,
        );
        assert_eq!(r.due.value, Some(d(9)));
        assert_eq!(r.estimate_points.value, Some(10));
        assert_eq!(r.progress_bp.value, Some(3_000), "3 of 10 points are done");
        assert_eq!((r.descendants, r.done), (3, 1));
        assert_eq!(r.spent_minutes, 75);
    }

    #[test]
    fn unestimated_children_are_named_not_treated_as_zero() {
        let r = rollup(
            &[
                fact(1, None, Some(4), true, 0),
                fact(2, None, None, false, 0),
            ],
            0,
        );
        assert_eq!(
            r.progress_bp.value,
            Some(10_000),
            "only the estimated child is weighed"
        );
        assert_eq!(r.excluded_unestimated, [id(2)]);
    }

    #[test]
    fn nothing_estimated_or_dated_says_why() {
        let r = rollup(&[fact(1, None, None, false, 0)], 0);
        assert_eq!(r.estimate_points, Derived::none("no_estimates"));
        assert_eq!(r.progress_bp, Derived::none("no_estimates"));
        assert_eq!(r.due, Derived::none("no_dated_children"));
    }

    #[test]
    fn parents_may_not_form_cycles_or_nest_too_deep() {
        let mut links = BTreeMap::new();
        links.insert(id(2), id(1)); // 2 is a child of 1
        links.insert(id(3), id(2)); // 3 is a child of 2
        assert!(parent_allowed(&links, id(4), id(3)).is_ok());
        assert!(parent_allowed(&links, id(1), id(1)).is_err(), "self");
        assert!(
            parent_allowed(&links, id(1), id(3)).is_err(),
            "1 under its own grandchild"
        );
        assert!(
            parent_allowed(&links, id(1), id(2)).is_err(),
            "1 under its own child"
        );
        // A chain of MAX_DEPTH already: one more is too deep.
        let mut deep = BTreeMap::new();
        for n in 1..u128::try_from(MAX_DEPTH).unwrap() {
            deep.insert(id(n + 1), id(n));
        }
        assert!(parent_allowed(&deep, id(999), id(u128::try_from(MAX_DEPTH).unwrap())).is_err());
    }
}
