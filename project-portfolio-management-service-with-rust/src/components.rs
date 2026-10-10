//! Task components (PL-11): the pure rules. DB-free, unit-tested.
//!
//! A **component** is a named grouping of a plan's tasks (an area, a
//! workstream, a Redmine-style category). Grouping must never lose work:
//! a task with no component is reported in an explicit `unassigned`
//! bucket, so the buckets always add up to the plan's task total.

use serde::Serialize;
use uuid::Uuid;

/// Cap on a component name.
pub const MAX_NAME_LEN: usize = 120;
/// Cap on components per plan.
pub const MAX_PER_PLAN: usize = 200;

/// Every problem with a component's shape (not just the first).
#[must_use]
pub fn problems(name: &str, lead_ref: Option<&str>, lead_ok: impl Fn(&str) -> bool) -> Vec<String> {
    let mut out = Vec::new();
    if name.trim().is_empty() {
        out.push("name is required".to_string());
    } else if name.chars().count() > MAX_NAME_LEN {
        out.push(format!("name: exceeds {MAX_NAME_LEN} characters"));
    }
    if let Some(lead) = lead_ref
        && !lead_ok(lead)
    {
        out.push("lead_ref must be a worker:/person: URN".to_string());
    }
    out
}

/// One bucket of the per-component summary.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Bucket {
    /// The component, or `None` for the `unassigned` bucket.
    pub component_pid: Option<Uuid>,
    /// Display name (`unassigned` for the catch-all).
    pub name: String,
    /// Open (not done) tasks.
    pub open: usize,
    /// Done tasks.
    pub done: usize,
    /// `open + done`.
    pub total: usize,
}

/// Summarize `(component_pid, done)` task facts over a plan's live
/// components. **Every component is reported, empty ones at zero**, plus an
/// `unassigned` bucket last; a task naming a component that no longer exists
/// is counted as `unassigned` rather than dropped. The bucket totals always
/// sum to `tasks.len()`.
#[must_use]
pub fn summarize(components: &[(Uuid, String)], tasks: &[(Option<Uuid>, bool)]) -> Vec<Bucket> {
    let mut out: Vec<Bucket> = components
        .iter()
        .map(|(pid, name)| Bucket {
            component_pid: Some(*pid),
            name: name.clone(),
            open: 0,
            done: 0,
            total: 0,
        })
        .collect();
    let mut unassigned = Bucket {
        component_pid: None,
        name: "unassigned".to_string(),
        open: 0,
        done: 0,
        total: 0,
    };
    for (component, done) in tasks {
        let bucket = component
            .and_then(|c| out.iter_mut().find(|b| b.component_pid == Some(c)))
            .unwrap_or(&mut unassigned);
        if *done {
            bucket.done += 1;
        } else {
            bucket.open += 1;
        }
        bucket.total += 1;
    }
    out.push(unassigned);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(n: u128) -> Uuid {
        Uuid::from_u128(n)
    }

    #[test]
    fn shape_problems_are_all_reported() {
        assert_eq!(problems("  ", Some("x"), |_| false).len(), 2);
        assert!(problems("Backend", None, |_| false).is_empty());
        assert!(!problems(&"x".repeat(MAX_NAME_LEN + 1), None, |_| true).is_empty());
    }

    #[test]
    fn every_component_is_reported_even_empty_and_unassigned_is_last() {
        let comps = vec![(id(1), "API".to_string()), (id(2), "UI".to_string())];
        let b = summarize(&comps, &[(Some(id(1)), false)]);
        assert_eq!(b.len(), 3);
        assert_eq!(b[1].total, 0, "UI reported empty, not omitted");
        assert_eq!(b[2].name, "unassigned");
    }

    #[test]
    fn the_buckets_always_sum_to_the_task_total() {
        let comps = vec![(id(1), "API".to_string())];
        let tasks = vec![
            (Some(id(1)), true),
            (Some(id(1)), false),
            (None, false),
            (Some(id(99)), true), // a component that no longer exists
        ];
        let b = summarize(&comps, &tasks);
        assert_eq!(b.iter().map(|x| x.total).sum::<usize>(), tasks.len());
        let un = b.last().unwrap();
        assert_eq!(
            (un.open, un.done),
            (1, 1),
            "orphan counted as unassigned, not dropped"
        );
        assert_eq!((b[0].open, b[0].done), (1, 1));
    }
}
