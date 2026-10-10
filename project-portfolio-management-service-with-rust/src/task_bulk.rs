//! Bulk task operations (RM-9): the pure rules. DB-free, unit-tested.
//!
//! A bulk call reports **one result per requested id** — `ok` or `refused`
//! with the reason — so one bad item never aborts the rest and the counts
//! always reconcile: `ok + refused == requested (after de-duplication)`.

use serde::Serialize;
use uuid::Uuid;

use crate::workflow::{WorkflowDef, category_of};

/// Cap on ids in one call.
pub const MAX_IDS: usize = 200;

/// The result for one requested task.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ItemResult {
    /// The requested task id.
    pub pid: Uuid,
    /// `ok` or `refused`.
    pub result: &'static str,
    /// Why it was refused, or a note on what happened (e.g. a status mapped).
    pub note: Option<String>,
}

impl ItemResult {
    /// A success, optionally with a note.
    #[must_use]
    pub fn ok(pid: Uuid, note: Option<String>) -> Self {
        Self {
            pid,
            result: "ok",
            note,
        }
    }

    /// A refusal.
    #[must_use]
    pub fn refused(pid: Uuid, why: impl Into<String>) -> Self {
        Self {
            pid,
            result: "refused",
            note: Some(why.into()),
        }
    }
}

/// What a bulk edit does to one optional field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Change<T> {
    /// The field was not mentioned: leave it.
    Keep,
    /// The request said `null`: clear it.
    Clear,
    /// Set it.
    Set(T),
}

impl<T: Clone> Change<T> {
    /// The field's new value given its current one.
    #[must_use]
    pub fn apply(&self, current: Option<T>) -> Option<T> {
        match self {
            Self::Keep => current,
            Self::Clear => None,
            Self::Set(v) => Some(v.clone()),
        }
    }
}

/// De-duplicate requested ids keeping first-seen order, refusing an empty or
/// over-long list.
///
/// # Errors
///
/// A message when the list is empty or exceeds [`MAX_IDS`] after de-dup.
pub fn dedupe_ids(ids: &[Uuid]) -> Result<Vec<Uuid>, String> {
    let mut out: Vec<Uuid> = Vec::new();
    for id in ids {
        if !out.contains(id) {
            out.push(*id);
        }
    }
    if out.is_empty() {
        return Err("ids must name at least one task".to_string());
    }
    if out.len() > MAX_IDS {
        return Err(format!("at most {MAX_IDS} tasks per call"));
    }
    Ok(out)
}

/// The status a task lands in when it moves to a plan whose workflow is
/// `target`: unchanged if the target declares it, otherwise the target's
/// **initial** state. Returns the status and whether it was **mapped** (so
/// the caller can say so rather than silently rewriting it).
#[must_use]
pub fn map_status(target: &WorkflowDef, current: &str) -> (String, bool) {
    if category_of(target, current).is_some() {
        return (current.to_string(), false);
    }
    let initial = target
        .states
        .iter()
        .find(|s| s.is_initial)
        .map_or("todo", |s| s.key.as_str());
    (initial.to_string(), true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workflow::{Category, StateDef};

    fn id(n: u128) -> Uuid {
        Uuid::from_u128(n)
    }

    fn wf(keys: &[(&str, bool)]) -> WorkflowDef {
        WorkflowDef {
            states: keys
                .iter()
                .map(|(k, initial)| StateDef {
                    key: (*k).to_string(),
                    label: (*k).to_string(),
                    category: Category::Todo,
                    wip_limit: None,
                    is_initial: *initial,
                    is_terminal: false,
                })
                .collect(),
            transitions: vec![],
        }
    }

    #[test]
    fn ids_are_deduplicated_in_order_and_bounded() {
        assert_eq!(dedupe_ids(&[id(2), id(1), id(2)]).unwrap(), [id(2), id(1)]);
        assert!(dedupe_ids(&[]).is_err());
        let many: Vec<Uuid> = (0..=MAX_IDS as u128).map(id).collect();
        assert!(dedupe_ids(&many).is_err());
    }

    #[test]
    fn a_status_the_target_declares_is_kept() {
        let target = wf(&[("backlog", true), ("doing", false)]);
        assert_eq!(map_status(&target, "doing"), ("doing".to_string(), false));
    }

    #[test]
    fn a_status_the_target_lacks_maps_to_its_initial_state_and_says_so() {
        let target = wf(&[("backlog", true), ("doing", false)]);
        assert_eq!(
            map_status(&target, "in_review"),
            ("backlog".to_string(), true)
        );
    }

    #[test]
    fn a_change_keeps_clears_or_sets() {
        assert_eq!(Change::Keep.apply(Some(1)), Some(1));
        assert_eq!(Change::Keep.apply(None::<i32>), None);
        assert_eq!(Change::<i32>::Clear.apply(Some(1)), None);
        assert_eq!(Change::Set(2).apply(Some(1)), Some(2));
        assert_eq!(Change::Set(2).apply(None), Some(2));
    }

    #[test]
    fn results_carry_their_note() {
        assert_eq!(ItemResult::ok(id(1), None).result, "ok");
        let r = ItemResult::refused(id(2), "no");
        assert_eq!((r.result, r.note.as_deref()), ("refused", Some("no")));
    }
}
