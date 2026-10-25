//! Blueprint sync (NC-12): what changed in a template since a plan was made from it, as a diff of tasks, and the
//! rules for applying a change to one plan. DB-free, unit-tested.
//!
//! A plan made from a template keeps a **baseline**: the template's tasks for that plan as they were when the
//! plan was last brought up to date. The diff compares that baseline with the template now. A change to a task the
//! plan's own people have since edited (or removed) is **locally edited**: it is applied only when the caller says
//! so by name, never by default.
//!
//! Scope, deliberately narrow: tasks of one plan (title, description, points). Milestones, components, plan fields
//! and workflows are not synced.

use serde::{Deserialize, Serialize};

use crate::plan_templates::{Snapshot, TplTask};

/// What kind of change a template made to one task.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    /// The template has a task the baseline did not.
    Add,
    /// The template changed a task's description or points.
    Change,
    /// The template no longer has a task the baseline had.
    Remove,
}

/// One change, named so the caller can accept it: `add:3`, `change:1`, `remove:2` (the template task's key).
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Change {
    /// `kind:key`.
    pub id: String,
    /// The kind.
    pub kind: Kind,
    /// The template task key.
    pub key: usize,
    /// The task's title (the template's, or the baseline's for a removal).
    pub title: String,
    /// The baseline task, for a change or removal.
    pub from: Option<TplTask>,
    /// The template task now, for an addition or change.
    pub to: Option<TplTask>,
}

/// The id a caller uses to accept a change.
#[must_use]
pub fn change_id(kind: Kind, key: usize) -> String {
    let k = match kind {
        Kind::Add => "add",
        Kind::Change => "change",
        Kind::Remove => "remove",
    };
    format!("{k}:{key}")
}

fn same_content(a: &TplTask, b: &TplTask) -> bool {
    a.title == b.title && a.description == b.description && a.points == b.points
}

/// The changes that take `baseline` to `current`, tasks matched by key, in key order.
#[must_use]
pub fn diff(baseline: &[TplTask], current: &[TplTask]) -> Vec<Change> {
    let mut out = Vec::new();
    for to in current {
        match baseline.iter().find(|b| b.key == to.key) {
            None => out.push(Change {
                id: change_id(Kind::Add, to.key),
                kind: Kind::Add,
                key: to.key,
                title: to.title.clone(),
                from: None,
                to: Some(to.clone()),
            }),
            Some(from) if !same_content(from, to) => out.push(Change {
                id: change_id(Kind::Change, to.key),
                kind: Kind::Change,
                key: to.key,
                title: to.title.clone(),
                from: Some(from.clone()),
                to: Some(to.clone()),
            }),
            Some(_) => {}
        }
    }
    for from in baseline {
        if !current.iter().any(|c| c.key == from.key) {
            out.push(Change {
                id: change_id(Kind::Remove, from.key),
                kind: Kind::Remove,
                key: from.key,
                title: from.title.clone(),
                from: Some(from.clone()),
                to: None,
            });
        }
    }
    out.sort_by_key(|c| c.key);
    out
}

/// A plan's task as it stands now, for the local-edit check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Local {
    /// Title.
    pub title: String,
    /// Description.
    pub description: Option<String>,
    /// Points.
    pub points: Option<i32>,
}

/// Whether the plan's people have edited the task since the baseline: it differs from the baseline, or it was
/// removed (`local` is `None`). An addition has no local task and is never edited.
#[must_use]
pub fn locally_edited(change: &Change, local: Option<&Local>) -> bool {
    let Some(from) = &change.from else {
        return false;
    };
    match local {
        None => true,
        Some(l) => {
            l.title != from.title || l.description != from.description || l.points != from.points
        }
    }
}

/// Carry the keys of `old` into a re-captured `new`, so a diff stays meaningful when a template is revised.
/// A node is matched to the old node of the same plan name and parent; a task to the old task of the same title
/// (case-insensitive) in that node. A matched task keeps its old key; a new task gets a key above every old one.
/// A task renamed in the revision therefore reads as a removal and an addition, not a change.
#[must_use]
pub fn rekey(old: &Snapshot, mut new: Snapshot) -> Snapshot {
    let mut node_map: Vec<Option<usize>> = Vec::with_capacity(new.nodes.len());
    for n in &new.nodes {
        let parent_old = n.parent.and_then(|p| node_map.get(p).copied().flatten());
        let found = old.nodes.iter().position(|o| {
            o.plan.name.eq_ignore_ascii_case(&n.plan.name)
                && match (o.parent, n.parent) {
                    (None, None) => true,
                    (Some(op), Some(_)) => Some(op) == parent_old,
                    _ => false,
                }
        });
        node_map.push(found);
    }
    for (ni, node) in new.nodes.iter_mut().enumerate() {
        let old_tasks: &[TplTask] = node_map[ni].map_or(&[], |o| old.nodes[o].tasks.as_slice());
        let mut next_key = old_tasks.iter().map(|t| t.key + 1).max().unwrap_or(0);
        let mut used: Vec<usize> = Vec::new();
        let mut key_of: Vec<(usize, usize)> = Vec::new(); // (new position-key, final key)
        for t in &node.tasks {
            let hit = old_tasks
                .iter()
                .find(|o| o.title.eq_ignore_ascii_case(&t.title) && !used.contains(&o.key));
            let final_key = if let Some(o) = hit {
                used.push(o.key);
                o.key
            } else {
                let k = next_key;
                next_key += 1;
                k
            };
            key_of.push((t.key, final_key));
        }
        let remap = |k: usize| {
            key_of
                .iter()
                .find(|(from, _)| *from == k)
                .map(|(_, to)| *to)
        };
        for t in &mut node.tasks {
            t.parent = t.parent.and_then(remap);
            t.key = remap(t.key).unwrap_or(t.key);
        }
    }
    new
}

/// Whether `new` keeps the shape `old` had: every old plan is still at its place, with its name and parent. Plans may
/// be added after them. Sync finds a plan's node by position, so a revision that moves one is refused.
///
/// # Errors
///
/// Says which plan moved, was renamed or was dropped.
pub fn same_shape(old: &Snapshot, new: &Snapshot) -> Result<(), String> {
    for (i, o) in old.nodes.iter().enumerate() {
        let Some(n) = new.nodes.get(i) else {
            return Err(format!(
                "the plan \"{}\" is no longer in the tree",
                o.plan.name
            ));
        };
        if !n.plan.name.eq_ignore_ascii_case(&o.plan.name) || n.parent != o.parent {
            return Err(format!(
                "the plan \"{}\" has moved or been renamed; a revision may change tasks and add plans at the end, not reshape the tree",
                o.plan.name
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plan_templates::{Snapshot, TplNode};
    use project_portfolio_management_matcher::Plan;

    fn task(key: usize, title: &str, points: Option<i32>) -> TplTask {
        TplTask {
            key,
            title: title.into(),
            description: None,
            points,
            flow_type: None,
            due_offset_days: None,
            component: None,
            parent: None,
        }
    }

    fn snapshot(name: &str, tasks: Vec<TplTask>) -> Snapshot {
        Snapshot {
            base_date: None,
            has_offsets: false,
            nodes: vec![TplNode {
                key: 0,
                parent: None,
                plan: Plan {
                    name: name.into(),
                    ..Plan::default()
                },
                start_offset_days: None,
                target_offset_days: None,
                components: vec![],
                tasks,
                milestones: vec![],
                workflow: None,
            }],
        }
    }

    #[test]
    fn the_diff_names_additions_changes_and_removals_by_key() {
        let base = vec![
            task(0, "Plan", Some(1)),
            task(1, "Build", Some(3)),
            task(2, "Ship", None),
        ];
        let now = vec![
            task(0, "Plan", Some(1)),
            task(1, "Build", Some(5)),
            task(3, "Review", None),
        ];
        let ids: Vec<String> = diff(&base, &now).into_iter().map(|c| c.id).collect();
        assert_eq!(ids, ["change:1", "remove:2", "add:3"]);
        assert!(diff(&base, &base).is_empty());
    }

    #[test]
    fn a_change_to_an_edited_or_removed_task_is_a_local_edit_and_an_addition_never_is() {
        let base = vec![task(1, "Build", Some(3))];
        let now = vec![task(1, "Build", Some(5)), task(2, "New", None)];
        let changes = diff(&base, &now);
        let change = &changes[0];
        let add = &changes[1];
        let untouched = Local {
            title: "Build".into(),
            description: None,
            points: Some(3),
        };
        let edited = Local {
            points: Some(8),
            ..untouched.clone()
        };
        assert!(!locally_edited(change, Some(&untouched)));
        assert!(locally_edited(change, Some(&edited)));
        assert!(
            locally_edited(change, None),
            "a task the plan deleted is a local decision"
        );
        assert!(!locally_edited(add, None));
    }

    #[test]
    fn a_revision_may_add_plans_at_the_end_but_not_move_or_drop_one() {
        let old = snapshot("Launch", vec![]);
        let mut grown = snapshot("Launch", vec![]);
        let mut extra = grown.nodes[0].clone();
        extra.key = 1;
        extra.parent = Some(0);
        extra.plan.name = "Extra".into();
        grown.nodes.push(extra);
        assert!(same_shape(&old, &grown).is_ok());
        assert!(same_shape(&grown, &old).unwrap_err().contains("Extra"));
        assert!(
            same_shape(&old, &snapshot("Other", vec![]))
                .unwrap_err()
                .contains("moved or been renamed")
        );
    }

    #[test]
    fn a_revised_template_keeps_the_keys_of_the_tasks_it_still_has() {
        let old = snapshot(
            "Launch",
            vec![
                task(0, "Plan", None),
                task(1, "Build", None),
                task(2, "Ship", None),
            ],
        );
        // The revision inserted a task in the middle and dropped "Ship": positions shift, titles do not.
        let new = snapshot(
            "Launch",
            vec![
                task(0, "Plan", None),
                task(1, "Review", None),
                task(2, "build", Some(5)),
            ],
        );
        let keyed = rekey(&old, new);
        let by_title: Vec<(String, usize)> = keyed.nodes[0]
            .tasks
            .iter()
            .map(|t| (t.title.clone(), t.key))
            .collect();
        assert_eq!(
            by_title,
            [
                ("Plan".to_string(), 0),
                ("Review".to_string(), 3),
                ("build".to_string(), 1)
            ]
        );
        let ids: Vec<String> = diff(&old.nodes[0].tasks, &keyed.nodes[0].tasks)
            .into_iter()
            .map(|c| c.id)
            .collect();
        assert_eq!(ids, ["change:1", "remove:2", "add:3"]);
    }
}
