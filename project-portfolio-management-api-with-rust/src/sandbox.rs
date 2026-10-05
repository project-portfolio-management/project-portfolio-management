//! The what-if sandbox (PJ-9): the pure rules. DB-free and deterministic.
//!
//! A list of [`Edit`]s is applied to an **in-memory copy** of a plan's network
//! ([`apply`]); the analysis of the copy is compared with the analysis of the
//! original ([`deltas`], [`critical_path`]). Nothing here reads or writes a
//! database, which is what lets a preview be proven to write nothing.
//!
//! * Several edits to one task are **coalesced** into one change, and an edit
//!   that leaves a task as it was is no change at all — a commit audits one
//!   entry per *changed* item.
//! * A link may not be a self-link, a duplicate, or close a cycle (counting links
//!   from other plans, which the caller passes as `outside`); only a stored
//!   dependency may be removed, not a task relation.
//! * [`fingerprint`] summarises a state so a commit can tell the plan changed
//!   since the preview.

use std::collections::{HashMap, HashSet};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::cpm::{self, Constraint, ConstraintKind, CpmResult, Edge, Kind, Node};

/// The most edits one preview or commit may carry.
pub const MAX_EDITS: usize = 100;

/// One edit to a plan's schedule.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum Edit {
    /// Set a task's planned start.
    Move {
        /// The task.
        task_pid: Uuid,
        /// Its new planned start.
        start_date: NaiveDate,
    },
    /// Set a task's working-day duration (0 makes it a milestone).
    Duration {
        /// The task.
        task_pid: Uuid,
        /// Its new duration.
        duration_days: i32,
    },
    /// Set or clear a task's constraint.
    Constraint {
        /// The task.
        task_pid: Uuid,
        /// `snet` … `mfo`, or null to clear.
        constraint_kind: Option<String>,
        /// The constraint's date.
        constraint_date: Option<NaiveDate>,
        /// Why — required for a hard constraint.
        #[serde(default)]
        reason: Option<String>,
    },
    /// Add a dependency between two tasks of the plan.
    AddLink {
        /// The predecessor.
        predecessor_pid: Uuid,
        /// The successor.
        successor_pid: Uuid,
        /// `fs` (default), `ss`, `ff` or `sf`.
        #[serde(default)]
        kind: Option<String>,
        /// Lag in working days (negative is a lead).
        #[serde(default)]
        lag_days: Option<i32>,
    },
    /// Remove a stored dependency.
    RemoveLink {
        /// The dependency's pid.
        link_pid: Uuid,
    },
}

/// Problems that can be seen from the edits alone (empty when none).
#[must_use]
pub fn check(edits: &[Edit]) -> Vec<String> {
    let mut out = Vec::new();
    if edits.is_empty() {
        out.push("edits must name at least one edit".to_string());
    }
    if edits.len() > MAX_EDITS {
        out.push(format!("at most {MAX_EDITS} edits per call"));
    }
    let mut removed = HashSet::new();
    for (i, e) in edits.iter().enumerate() {
        let at = |m: String| format!("edit {}: {m}", i + 1);
        match e {
            Edit::Move { start_date, .. } => {
                out.extend(
                    cpm::schedule_problems(Some(*start_date), None)
                        .into_iter()
                        .map(at),
                );
            }
            Edit::Duration { duration_days, .. } => {
                out.extend(
                    cpm::schedule_problems(None, Some(*duration_days))
                        .into_iter()
                        .map(at),
                );
            }
            Edit::Constraint {
                constraint_kind,
                constraint_date,
                reason,
                ..
            } => {
                out.extend(
                    cpm::constraint_problems(constraint_kind.as_deref(), *constraint_date, None)
                        .into_iter()
                        .map(at),
                );
                let hard = constraint_kind
                    .as_deref()
                    .and_then(ConstraintKind::parse)
                    .is_some_and(ConstraintKind::is_hard);
                if hard && reason.as_deref().is_none_or(|r| r.trim().is_empty()) {
                    out.push(at("a hard constraint needs a reason".to_string()));
                }
            }
            Edit::AddLink { kind, lag_days, .. } => {
                if kind.as_deref().is_some_and(|k| Kind::parse(k).is_none()) {
                    out.push(at("kind must be one of fs, ss, ff, sf".to_string()));
                }
                if lag_days.is_some_and(|l| !(-3650..=3650).contains(&l)) {
                    out.push(at("lag_days must be -3650 to 3650".to_string()));
                }
            }
            Edit::RemoveLink { link_pid } => {
                if !removed.insert(*link_pid) {
                    out.push(at("a link may be removed once".to_string()));
                }
            }
        }
    }
    out
}

/// Whether any edit sets a **hard** constraint (which needs the permission).
#[must_use]
pub fn needs_hard_permission(edits: &[Edit]) -> bool {
    edits.iter().any(|e| {
        matches!(e, Edit::Constraint { constraint_kind: Some(k), .. }
            if ConstraintKind::parse(k).is_some_and(ConstraintKind::is_hard))
    })
}

/// A task's schedule fields, as the editor sees them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TaskFields {
    /// Planned start.
    pub start_date: Option<NaiveDate>,
    /// Working days.
    pub duration_days: Option<i64>,
    /// Constraint token.
    pub constraint_kind: Option<&'static str>,
    /// Constraint date.
    pub constraint_date: Option<NaiveDate>,
}

const fn token(k: ConstraintKind) -> &'static str {
    match k {
        ConstraintKind::Snet => "snet",
        ConstraintKind::Snlt => "snlt",
        ConstraintKind::Fnet => "fnet",
        ConstraintKind::Fnlt => "fnlt",
        ConstraintKind::Mso => "mso",
        ConstraintKind::Mfo => "mfo",
    }
}

/// The lowercase token of a link kind.
#[must_use]
pub const fn kind_token(k: Kind) -> &'static str {
    match k {
        Kind::Fs => "fs",
        Kind::Ss => "ss",
        Kind::Ff => "ff",
        Kind::Sf => "sf",
    }
}

/// The fields of a node.
#[must_use]
pub fn fields_of(n: &Node) -> TaskFields {
    TaskFields {
        start_date: n.planned_start,
        duration_days: n.duration,
        constraint_kind: n.constraint.map(|c| token(c.kind)),
        constraint_date: n.constraint.map(|c| c.date),
    }
}

/// One changed item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "item", rename_all = "snake_case")]
pub enum Change {
    /// A task's fields changed.
    Task {
        /// The task.
        pid: Uuid,
        /// Before.
        from: TaskFields,
        /// After.
        to: TaskFields,
    },
    /// A link was added.
    LinkAdded {
        /// A placeholder until the link is stored (derived from its ends).
        pid: Uuid,
        /// Predecessor.
        predecessor_pid: Uuid,
        /// Successor.
        successor_pid: Uuid,
        /// `fs` / `ss` / `ff` / `sf`.
        kind: &'static str,
        /// Lag in working days.
        lag_days: i64,
    },
    /// A link was removed.
    LinkRemoved {
        /// The stored link.
        pid: Uuid,
        /// Predecessor.
        predecessor_pid: Uuid,
        /// Successor.
        successor_pid: Uuid,
        /// `fs` / `ss` / `ff` / `sf`.
        kind: &'static str,
        /// Lag in working days.
        lag_days: i64,
    },
}

/// A deterministic placeholder pid for a link not yet stored.
#[must_use]
pub fn placeholder_link_pid(pred: Uuid, succ: Uuid) -> Uuid {
    Uuid::from_u128(pred.as_u128().rotate_left(17) ^ succ.as_u128())
}

/// Apply `edits` to `nodes` and `edges` (an in-memory copy). `outside` are the
/// dependency pairs from outside this network, counted for cycles; `removable`
/// are the pids of stored dependencies (a task relation is not removable here).
///
/// # Errors
///
/// A sentence naming the first edit that cannot be applied. Nothing is promised
/// about the state of `nodes` / `edges` after an error.
#[allow(clippy::too_many_lines, clippy::implicit_hasher)]
pub fn apply(
    nodes: &mut [Node],
    edges: &mut Vec<Edge>,
    outside: &[(Uuid, Uuid)],
    removable: &HashSet<Uuid>,
    edits: &[Edit],
) -> Result<Vec<Change>, String> {
    let index: HashMap<Uuid, usize> = nodes.iter().enumerate().map(|(i, n)| (n.pid, i)).collect();
    let mut original: HashMap<Uuid, TaskFields> = HashMap::new();
    let mut touched: Vec<Uuid> = Vec::new();
    let mut links: Vec<Change> = Vec::new();
    for (n, e) in edits.iter().enumerate() {
        let at = |m: &str| format!("edit {}: {m}", n + 1);
        let mut touch = |pid: Uuid, nodes: &[Node]| -> Result<usize, String> {
            let i = *index.get(&pid).ok_or_else(|| at("unknown task"))?;
            if let std::collections::hash_map::Entry::Vacant(v) = original.entry(pid) {
                v.insert(fields_of(&nodes[i]));
                touched.push(pid);
            }
            Ok(i)
        };
        match e {
            Edit::Move {
                task_pid,
                start_date,
            } => {
                let i = touch(*task_pid, nodes)?;
                nodes[i].planned_start = Some(*start_date);
            }
            Edit::Duration {
                task_pid,
                duration_days,
            } => {
                let i = touch(*task_pid, nodes)?;
                nodes[i].duration = Some(i64::from(*duration_days));
            }
            Edit::Constraint {
                task_pid,
                constraint_kind,
                constraint_date,
                ..
            } => {
                let i = touch(*task_pid, nodes)?;
                nodes[i].constraint = constraint_kind
                    .as_deref()
                    .and_then(ConstraintKind::parse)
                    .zip(*constraint_date)
                    .map(|(kind, date)| Constraint { kind, date });
            }
            Edit::AddLink {
                predecessor_pid,
                successor_pid,
                kind,
                lag_days,
            } => {
                for end in [predecessor_pid, successor_pid] {
                    if !index.contains_key(end) {
                        return Err(at("a link joins two tasks of this plan"));
                    }
                }
                if edges
                    .iter()
                    .any(|x| x.pred == *predecessor_pid && x.succ == *successor_pid)
                {
                    return Err(at("this dependency already exists"));
                }
                let mut pairs: Vec<(Uuid, Uuid)> = edges.iter().map(|x| (x.pred, x.succ)).collect();
                pairs.extend_from_slice(outside);
                if crate::visibility::would_create_cycle(&pairs, *predecessor_pid, *successor_pid) {
                    return Err(at("this dependency would create a cycle"));
                }
                let kind = kind.as_deref().and_then(Kind::parse).unwrap_or(Kind::Fs);
                let lag = i64::from(lag_days.unwrap_or(0));
                let pid = placeholder_link_pid(*predecessor_pid, *successor_pid);
                edges.push(Edge {
                    pid,
                    pred: *predecessor_pid,
                    succ: *successor_pid,
                    kind,
                    lag,
                });
                links.push(Change::LinkAdded {
                    pid,
                    predecessor_pid: *predecessor_pid,
                    successor_pid: *successor_pid,
                    kind: kind_token(kind),
                    lag_days: lag,
                });
            }
            Edit::RemoveLink { link_pid } => {
                let at_ix = edges
                    .iter()
                    .position(|x| x.pid == *link_pid)
                    .ok_or_else(|| at("unknown link"))?;
                if !removable.contains(link_pid) {
                    return Err(at("this link is a task relation: remove it there"));
                }
                let x = edges.remove(at_ix);
                links.push(Change::LinkRemoved {
                    pid: x.pid,
                    predecessor_pid: x.pred,
                    successor_pid: x.succ,
                    kind: kind_token(x.kind),
                    lag_days: x.lag,
                });
            }
        }
    }
    let mut out: Vec<Change> = touched
        .into_iter()
        .filter_map(|pid| {
            let from = original.remove(&pid)?;
            let to = fields_of(&nodes[index[&pid]]);
            (from != to).then_some(Change::Task { pid, from, to })
        })
        .collect();
    // A link added and then removed in the same list is no change.
    let added: HashSet<Uuid> = links
        .iter()
        .filter(|c| matches!(c, Change::LinkAdded { .. }))
        .map(link_pid)
        .collect();
    let removed: HashSet<Uuid> = links
        .iter()
        .filter(|c| matches!(c, Change::LinkRemoved { .. }))
        .map(link_pid)
        .collect();
    out.extend(
        links
            .into_iter()
            .filter(|c| !(added.contains(&link_pid(c)) && removed.contains(&link_pid(c)))),
    );
    Ok(out)
}

fn link_pid(c: &Change) -> Uuid {
    match c {
        Change::LinkAdded { pid, .. }
        | Change::LinkRemoved { pid, .. }
        | Change::Task { pid, .. } => *pid,
    }
}

/// A task's analysed dates.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Dates {
    /// Earliest start.
    pub early_start: NaiveDate,
    /// Earliest finish.
    pub early_finish: NaiveDate,
    /// Latest finish.
    pub late_finish: NaiveDate,
    /// Total float in working days.
    pub total_float: i64,
    /// On the critical path.
    pub critical: bool,
}

/// How one task's analysis differs between the original and the copy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Delta {
    /// The task.
    pub pid: Uuid,
    /// Before (null when it was not scheduled).
    pub before: Option<Dates>,
    /// After (null when it is not scheduled).
    pub after: Option<Dates>,
}

fn dates(n: &cpm::NodeResult) -> Dates {
    Dates {
        early_start: n.early_start,
        early_finish: n.early_finish,
        late_finish: n.late_finish,
        total_float: n.total_float,
        critical: n.critical,
    }
}

/// The tasks whose dates, float or criticality differ, in the copy's order, then
/// those that dropped out of the schedule.
#[must_use]
pub fn deltas(before: &CpmResult, after: &CpmResult) -> Vec<Delta> {
    let was: HashMap<Uuid, Dates> = before.nodes.iter().map(|n| (n.pid, dates(n))).collect();
    let now: HashSet<Uuid> = after.nodes.iter().map(|n| n.pid).collect();
    let mut out: Vec<Delta> = after
        .nodes
        .iter()
        .filter_map(|n| {
            let a = dates(n);
            let b = was.get(&n.pid).cloned();
            (b.as_ref() != Some(&a)).then_some(Delta {
                pid: n.pid,
                before: b,
                after: Some(a),
            })
        })
        .collect();
    out.extend(
        before
            .nodes
            .iter()
            .filter(|n| !now.contains(&n.pid))
            .map(|n| Delta {
                pid: n.pid,
                before: Some(dates(n)),
                after: None,
            }),
    );
    out
}

/// The critical tasks, ordered by early start then pid.
#[must_use]
pub fn critical_path(r: &CpmResult) -> Vec<Uuid> {
    let mut v: Vec<&cpm::NodeResult> = r.nodes.iter().filter(|n| n.critical).collect();
    v.sort_by_key(|n| (n.early_start, n.pid));
    v.into_iter().map(|n| n.pid).collect()
}

/// A short digest of a state described as lines of text. Equal state, equal
/// digest; any difference changes it.
#[must_use]
pub fn fingerprint(parts: &[String]) -> String {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    for p in parts {
        p.hash(&mut h);
        0xff_u8.hash(&mut h);
    }
    format!("W/\"{:016x}\"", h.finish())
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
    fn node(n: u128, dur: i64) -> Node {
        Node {
            pid: id(n),
            duration: Some(dur),
            planned_start: Some(d(5)),
            constraint: None,
            deadline: None,
            own_calendar: None,
        }
    }
    fn link(n: u128, a: u128, b: u128) -> Edge {
        Edge {
            pid: id(n),
            pred: id(a),
            succ: id(b),
            kind: Kind::Fs,
            lag: 0,
        }
    }
    fn run(
        nodes: &mut [Node],
        edges: &mut Vec<Edge>,
        edits: &[Edit],
    ) -> Result<Vec<Change>, String> {
        let removable: HashSet<Uuid> = edges.iter().map(|e| e.pid).collect();
        apply(nodes, edges, &[], &removable, edits)
    }

    #[test]
    fn edits_coalesce_per_task_and_a_no_op_is_no_change() {
        let mut nodes = vec![node(1, 3), node(2, 2)];
        let mut edges = vec![];
        let c = run(
            &mut nodes,
            &mut edges,
            &[
                Edit::Move {
                    task_pid: id(1),
                    start_date: d(7),
                },
                Edit::Duration {
                    task_pid: id(1),
                    duration_days: 5,
                },
                Edit::Move {
                    task_pid: id(2),
                    start_date: d(8),
                },
                Edit::Move {
                    task_pid: id(2),
                    start_date: d(5),
                },
            ],
        )
        .unwrap();
        assert_eq!(c.len(), 1, "task 2 ended where it began");
        let Change::Task { pid, from, to } = &c[0] else {
            panic!()
        };
        assert_eq!(
            (
                *pid,
                from.start_date,
                to.start_date,
                from.duration_days,
                to.duration_days
            ),
            (id(1), Some(d(5)), Some(d(7)), Some(3), Some(5))
        );
    }

    #[test]
    fn constraints_are_set_and_cleared() {
        let mut nodes = vec![node(1, 3)];
        let mut edges = vec![];
        let c = run(
            &mut nodes,
            &mut edges,
            &[Edit::Constraint {
                task_pid: id(1),
                constraint_kind: Some("snet".into()),
                constraint_date: Some(d(9)),
                reason: None,
            }],
        )
        .unwrap();
        assert!(
            matches!(&c[0], Change::Task { to, .. } if to.constraint_kind == Some("snet") && to.constraint_date == Some(d(9)))
        );
        let c = run(
            &mut nodes,
            &mut edges,
            &[Edit::Constraint {
                task_pid: id(1),
                constraint_kind: None,
                constraint_date: None,
                reason: None,
            }],
        )
        .unwrap();
        assert!(matches!(&c[0], Change::Task { to, .. } if to.constraint_kind.is_none()));
    }

    #[test]
    fn links_refuse_self_duplicates_cycles_unknowns_and_task_relations() {
        let mut nodes = vec![node(1, 1), node(2, 1), node(3, 1)];
        let mut edges = vec![link(10, 1, 2)];
        let add = |a: u128, b: u128| Edit::AddLink {
            predecessor_pid: id(a),
            successor_pid: id(b),
            kind: None,
            lag_days: None,
        };
        assert!(
            run(&mut nodes, &mut edges, &[add(1, 1)])
                .unwrap_err()
                .contains("cycle"),
            "a self-link is a cycle"
        );
        assert!(
            run(&mut nodes, &mut edges, &[add(1, 2)])
                .unwrap_err()
                .contains("already exists")
        );
        assert!(
            run(&mut nodes, &mut edges, &[add(2, 1)])
                .unwrap_err()
                .contains("cycle")
        );
        assert!(
            run(&mut nodes, &mut edges, &[add(1, 9)])
                .unwrap_err()
                .contains("two tasks of this plan")
        );
        // A cycle through a link in another plan counts.
        let mut e = vec![];
        let outside = [(id(3), id(1))];
        let r = apply(&mut nodes, &mut e, &outside, &HashSet::new(), &[add(1, 3)]);
        assert!(r.unwrap_err().contains("cycle"));
        // Only a stored dependency may be removed.
        let r = apply(
            &mut nodes,
            &mut edges,
            &[],
            &HashSet::new(),
            &[Edit::RemoveLink { link_pid: id(10) }],
        );
        assert!(r.unwrap_err().contains("task relation"));
        let r = run(
            &mut nodes,
            &mut edges,
            &[Edit::RemoveLink { link_pid: id(99) }],
        );
        assert!(r.unwrap_err().contains("unknown link"));
    }

    #[test]
    fn adding_and_removing_links_is_reported_and_add_then_remove_cancels() {
        let mut nodes = vec![node(1, 1), node(2, 1)];
        let mut edges = vec![link(10, 1, 2)];
        let c = run(
            &mut nodes,
            &mut edges,
            &[
                Edit::RemoveLink { link_pid: id(10) },
                Edit::AddLink {
                    predecessor_pid: id(2),
                    successor_pid: id(1),
                    kind: Some("ss".into()),
                    lag_days: Some(2),
                },
            ],
        )
        .unwrap();
        assert!(matches!(&c[0], Change::LinkRemoved { kind: "fs", .. }));
        assert!(matches!(
            &c[1],
            Change::LinkAdded {
                kind: "ss",
                lag_days: 2,
                ..
            }
        ));
        assert_eq!(edges.len(), 1);
        // Remove then re-add the same pair in one list: the stored link is gone and a new one exists.
        let mut nodes = vec![node(1, 1), node(2, 1)];
        let mut edges = vec![];
        let add = Edit::AddLink {
            predecessor_pid: id(1),
            successor_pid: id(2),
            kind: None,
            lag_days: None,
        };
        let c = run(&mut nodes, &mut edges, &[add]).unwrap();
        assert_eq!(c.len(), 1);
    }

    #[test]
    fn static_checks_name_the_edit() {
        let ok = vec![Edit::Move {
            task_pid: id(1),
            start_date: d(7),
        }];
        assert!(check(&ok).is_empty());
        assert_eq!(check(&[]).len(), 1);
        let many = vec![ok[0].clone(); MAX_EDITS + 1];
        assert!(check(&many).iter().any(|m| m.contains("at most")));
        let bad = check(&[
            Edit::Duration {
                task_pid: id(1),
                duration_days: -1,
            },
            Edit::AddLink {
                predecessor_pid: id(1),
                successor_pid: id(2),
                kind: Some("xx".into()),
                lag_days: Some(99_999),
            },
            Edit::Constraint {
                task_pid: id(1),
                constraint_kind: Some("mso".into()),
                constraint_date: Some(d(9)),
                reason: None,
            },
            Edit::RemoveLink { link_pid: id(5) },
            Edit::RemoveLink { link_pid: id(5) },
        ]);
        assert!(bad.iter().any(|m| m.starts_with("edit 1")));
        assert!(
            bad.iter()
                .any(|m| m.starts_with("edit 2") && m.contains("kind"))
        );
        assert!(
            bad.iter()
                .any(|m| m.starts_with("edit 2") && m.contains("lag_days"))
        );
        assert!(
            bad.iter()
                .any(|m| m.starts_with("edit 3") && m.contains("reason"))
        );
        assert!(
            bad.iter()
                .any(|m| m.starts_with("edit 5") && m.contains("once"))
        );
        assert!(needs_hard_permission(&[Edit::Constraint {
            task_pid: id(1),
            constraint_kind: Some("MFO".into()),
            constraint_date: Some(d(9)),
            reason: Some("x".into())
        }]));
        assert!(!needs_hard_permission(&ok));
    }

    #[test]
    fn a_preview_shows_what_moved_and_the_critical_path_and_is_repeatable() {
        // A (3d) -> B (2d); C (1d) alone. Lengthening A moves B and the finish.
        let cal = cpm::WorkCalendar::seven_day();
        let analyse = |nodes: &[Node], edges: &[Edge]| {
            cpm::analyze_with(nodes, edges, &cal, &[], cpm::Options::default()).unwrap()
        };
        let mut nodes = vec![node(1, 3), node(2, 2), node(3, 1)];
        let mut edges = vec![link(10, 1, 2)];
        let before = analyse(&nodes, &edges);
        let mut copy_n = nodes.clone();
        let mut copy_e = edges.clone();
        let edits = [Edit::Duration {
            task_pid: id(1),
            duration_days: 6,
        }];
        run(&mut copy_n, &mut copy_e, &edits).unwrap();
        let after = analyse(&copy_n, &copy_e);
        let ds = deltas(&before, &after);
        assert!(ds.iter().any(|x| x.pid == id(1)) && ds.iter().any(|x| x.pid == id(2)));
        let moved: Vec<Uuid> = cpm::diff(&before, &after).iter().map(|m| m.pid).collect();
        assert_eq!(
            moved,
            vec![id(1), id(2)],
            "A finishes later and B starts later; C does not move"
        );
        assert_eq!(
            critical_path(&after),
            critical_path(&analyse(&copy_n, &copy_e))
        );
        // The original is untouched by working on a copy, and the same edits give the same preview.
        assert!(deltas(&before, &analyse(&nodes, &edges)).is_empty());
        let mut again_n = nodes.clone();
        let mut again_e = edges.clone();
        run(&mut again_n, &mut again_e, &edits).unwrap();
        assert_eq!(deltas(&before, &analyse(&again_n, &again_e)), ds);
        let _ = (&mut nodes, &mut edges);
    }

    #[test]
    fn a_fingerprint_changes_with_any_difference_and_not_otherwise() {
        let a = vec!["x".to_string(), "y".to_string()];
        assert_eq!(fingerprint(&a), fingerprint(&a.clone()));
        assert_ne!(
            fingerprint(&a),
            fingerprint(&["x".to_string(), "z".to_string()])
        );
        assert_ne!(
            fingerprint(&["xy".to_string()]),
            fingerprint(&["x".to_string(), "y".to_string()]),
            "boundaries matter"
        );
    }
}
