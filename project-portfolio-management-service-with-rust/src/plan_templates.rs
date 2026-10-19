//! Plan templates (LT-2): the snapshot model and its pure rules. DB-free,
//! unit-tested.
//!
//! A template is a **self-contained snapshot** of a plan tree: the plans,
//! their tasks, components and milestones, and each plan's own task
//! workflow. Three rules make it safe to reuse:
//!
//! 1. **No people, no identity.** Leads, assignees, external identifiers,
//!    `same_as`, owner-scoped codes and alternate names are **dropped** — an
//!    instantiated plan must not pose as the plan it came from (an identifier
//!    would make it a duplicate of its source) or inherit its people.
//! 2. **Dates are offsets.** Every date is stored as whole calendar days from a base
//!    date and re-derived from the start date given at instantiation.
//! 3. **Shape, not history.** Tasks come back at the workflow's initial
//!    state, with no transitions, comments or time.

use crate::matcher::Plan;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::workflow::WorkflowDef;

/// Cap on plans in one template.
pub const MAX_PLANS: usize = 200;
/// Cap on tasks in one template.
pub const MAX_TASKS: usize = 2_000;
/// Cap on tree depth.
pub const MAX_DEPTH: usize = 6;

/// One template task.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TplTask {
    /// Index within its node (parents reference this).
    pub key: usize,
    /// Title.
    pub title: String,
    /// Description.
    pub description: Option<String>,
    /// Story points.
    pub points: Option<i32>,
    /// Flow type.
    pub flow_type: Option<String>,
    /// Due date as calendar days from the base date.
    pub due_offset_days: Option<i64>,
    /// Component name (within the node).
    pub component: Option<String>,
    /// Parent task key (within the node).
    pub parent: Option<usize>,
}

/// One template milestone.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TplMilestone {
    /// Name.
    pub name: String,
    /// Kind label.
    pub kind: Option<String>,
    /// Due date as calendar days from the base date.
    pub offset_days: i64,
}

/// One plan of the tree.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TplNode {
    /// Index in the template (children name their parent by it).
    pub key: usize,
    /// Parent node key; `None` for the root.
    pub parent: Option<usize>,
    /// The scrubbed plan record.
    pub plan: Plan,
    /// Plan start as calendar days from the base date.
    pub start_offset_days: Option<i64>,
    /// Plan target as calendar days from the base date.
    pub target_offset_days: Option<i64>,
    /// Component names.
    pub components: Vec<String>,
    /// Tasks.
    pub tasks: Vec<TplTask>,
    /// Milestones.
    pub milestones: Vec<TplMilestone>,
    /// This plan's own task workflow, if it had one.
    pub workflow: Option<WorkflowDef>,
}

/// The whole template.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    /// The base date the offsets are measured from (informational).
    pub base_date: Option<NaiveDate>,
    /// Whether any date was captured, i.e. whether instantiation needs a
    /// start date.
    pub has_offsets: bool,
    /// Plans in parent-before-child order.
    pub nodes: Vec<TplNode>,
}

/// Parse a plan date that is a **full** `YYYY-MM-DD`; `YYYY` / `YYYY-MM`
/// (coarse dates) are not days and cannot be offset.
#[must_use]
pub fn full_date(s: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(s.trim(), "%Y-%m-%d").ok()
}

/// The base date: the root's start date when it is a full date, else the
/// earliest known date (a milestone or task due date), else none.
#[must_use]
pub fn choose_base(root_start: Option<&str>, other_dates: &[NaiveDate]) -> Option<NaiveDate> {
    root_start
        .and_then(full_date)
        .or_else(|| other_dates.iter().copied().min())
}

/// Calendar days from `base` to `date` (negative if before).
#[must_use]
pub fn offset(base: NaiveDate, date: NaiveDate) -> i64 {
    (date - base).num_days()
}

/// `base` shifted by `days`.
#[must_use]
pub fn shift(base: NaiveDate, days: i64) -> NaiveDate {
    base + chrono::Duration::days(days)
}

/// Strip everything that identifies a plan or names a person, keeping its
/// descriptive content.
#[must_use]
pub fn scrub(plan: &Plan) -> Plan {
    let mut p = plan.clone();
    p.lead_ref = None;
    p.identifiers.clear();
    p.same_as.clear();
    p.code = None;
    p.alternate_names.clear();
    p.relationships.clear();
    p.parent_ref = None;
    p.status = None;
    p.phase = None;
    p.start_date = None;
    p.target_date = None;
    p
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, m, day).unwrap()
    }

    #[test]
    fn only_full_dates_are_days() {
        assert_eq!(full_date("2026-10-05"), Some(d(10, 5)));
        assert_eq!(full_date("2026-10"), None, "a month is not a day");
        assert_eq!(full_date("2026"), None);
        assert_eq!(full_date("junk"), None);
    }

    #[test]
    fn the_base_is_the_root_start_else_the_earliest_known_date() {
        assert_eq!(choose_base(Some("2026-10-05"), &[d(9, 1)]), Some(d(10, 5)));
        assert_eq!(
            choose_base(Some("2026-10"), &[d(10, 9), d(10, 3)]),
            Some(d(10, 3))
        );
        assert_eq!(choose_base(None, &[]), None);
    }

    #[test]
    fn offsets_round_trip_including_negative() {
        let base = d(10, 5);
        for date in [d(10, 5), d(10, 20), d(9, 28)] {
            assert_eq!(shift(base, offset(base, date)), date);
        }
        assert_eq!(offset(base, d(9, 28)), -7);
    }

    #[test]
    fn scrubbing_removes_people_and_identity_but_keeps_description() {
        let mut p = Plan::new("Apollo");
        p.lead_ref = Some("person:3f2c6b7e-0000-4000-8000-000000000001".into());
        p.code = Some("PROJ-1".into());
        p.alternate_names = vec!["Moon shot".into()];
        p.keywords = vec!["space".into()];
        p.start_date = Some("2026-10-05".into());
        p.parent_ref = Some("3f2c6b7e-0000-4000-8000-000000000002".into());
        let s = scrub(&p);
        assert_eq!(s.name, "Apollo");
        assert_eq!(s.keywords, ["space"], "descriptive content survives");
        assert!(s.lead_ref.is_none() && s.code.is_none() && s.alternate_names.is_empty());
        assert!(s.identifiers.is_empty() && s.same_as.is_empty() && s.parent_ref.is_none());
        assert!(
            s.start_date.is_none(),
            "dates are re-derived from offsets, not copied"
        );
    }
}
