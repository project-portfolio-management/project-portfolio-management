//! Comments and reactions (LT-4): the pure rules. DB-free, unit-tested.
//!
//! One generic pair of tables serves every commentable subject, keyed
//! `(subject_kind, subject_pid)`; the controller resolves each kind to a
//! live row (and, for plan-owned kinds, the owning plan for the
//! record-level read check). Reactions are a **small closed set** — a
//! free-form emoji field would be a moderation and storage problem for
//! no analytic gain.

use std::collections::BTreeSet;

use uuid::Uuid;

/// What can be commented on / reacted to.
pub const SUBJECT_KINDS: &[&str] = &["plan", "task", "risk", "proposal", "idea", "canvas_item"];

/// The closed reaction set.
pub const REACTION_KINDS: &[&str] = &["thumbs_up", "heart", "celebrate", "eyes", "question"];

/// Cap on one comment body, in characters.
pub const MAX_COMMENT_LEN: usize = 4_000;

/// Whether `value` is in `set`.
#[must_use]
pub fn is_token(set: &[&str], value: &str) -> bool {
    set.contains(&value)
}

/// Every problem with a comment's shape (not just the first).
#[must_use]
pub fn comment_problems(subject_kind: &str, body: &str) -> Vec<String> {
    let mut out = Vec::new();
    if !is_token(SUBJECT_KINDS, subject_kind) {
        out.push(format!(
            "unknown subject_kind {subject_kind:?} (allowed: {SUBJECT_KINDS:?})"
        ));
    }
    if body.trim().is_empty() {
        out.push("body is required".to_string());
    } else if body.chars().count() > MAX_COMMENT_LEN {
        out.push(format!("body: exceeds {MAX_COMMENT_LEN} characters"));
    }
    out
}

/// Every problem with a reaction's shape.
#[must_use]
pub fn reaction_problems(subject_kind: &str, kind: &str) -> Vec<String> {
    let mut out = Vec::new();
    if !is_token(SUBJECT_KINDS, subject_kind) {
        out.push(format!(
            "unknown subject_kind {subject_kind:?} (allowed: {SUBJECT_KINDS:?})"
        ));
    }
    if !is_token(REACTION_KINDS, kind) {
        out.push(format!(
            "unknown reaction {kind:?} (allowed: {REACTION_KINDS:?})"
        ));
    }
    out
}

/// Whether `actor` may edit or delete a comment written by `author`.
/// Only the author does. Both absent (auth off) is the same unauthenticated
/// caller, so it matches; an authenticated actor never matches another
/// author, and never matches an anonymous comment.
#[must_use]
pub fn may_change(author: Option<&str>, actor: Option<&str>) -> bool {
    author == actor
}

/// A token `sub` as a notification recipient. A `sub` is a user pid; it is
/// addressed as `person:<sub>` only when it parses as a UUID — anything
/// else is skipped rather than guessed into a malformed reference.
#[must_use]
pub fn sub_as_recipient(sub: &str) -> Option<String> {
    Uuid::parse_str(sub).ok().map(|u| format!("person:{u}"))
}

/// Who a new comment notifies: the subject's owner (an `EntityRef` URN,
/// when the subject has one) plus every earlier commenter, **never the
/// author**, each exactly once, in a stable order.
#[must_use]
pub fn notify_targets(
    author: Option<&str>,
    owner_ref: Option<&str>,
    prior_authors: &[String],
) -> Vec<String> {
    let author_ref = author.and_then(sub_as_recipient);
    let mut out: BTreeSet<String> = BTreeSet::new();
    if let Some(owner) = owner_ref {
        out.insert(owner.to_string());
    }
    for sub in prior_authors {
        if let Some(r) = sub_as_recipient(sub) {
            out.insert(r);
        }
    }
    if let Some(a) = author_ref {
        out.remove(&a);
    }
    out.into_iter().collect()
}

/// Per-kind reaction counts. **Every kind in the closed set is reported,
/// at zero when unused**, so a client can render a stable bar.
#[must_use]
pub fn reaction_counts<'a>(kinds: impl Iterator<Item = &'a str>) -> Vec<(&'static str, usize)> {
    let mut counts: std::collections::BTreeMap<&str, usize> = std::collections::BTreeMap::new();
    for k in kinds {
        *counts.entry(k).or_default() += 1;
    }
    REACTION_KINDS
        .iter()
        .map(|k| (*k, counts.get(k).copied().unwrap_or(0)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const U1: &str = "3f2c6b7e-0000-4000-8000-000000000001";
    const U2: &str = "3f2c6b7e-0000-4000-8000-000000000002";

    #[test]
    fn shape_problems_are_all_reported() {
        assert_eq!(comment_problems("nope", " ").len(), 2);
        assert!(comment_problems("task", "fine").is_empty());
        assert!(!comment_problems("task", &"x".repeat(MAX_COMMENT_LEN + 1)).is_empty());
        assert_eq!(reaction_problems("nope", "nope").len(), 2);
        assert!(reaction_problems("plan", "heart").is_empty());
    }

    #[test]
    fn the_author_is_never_notified_and_each_target_appears_once() {
        let owner = format!("person:{U2}");
        let t = notify_targets(
            Some(U1),
            Some(&owner),
            &[U1.to_string(), U2.to_string(), U2.to_string()],
        );
        assert_eq!(t, vec![owner], "owner once, author excluded");
    }

    #[test]
    fn only_the_author_may_change_a_comment() {
        assert!(may_change(Some(U1), Some(U1)));
        assert!(!may_change(Some(U1), Some(U2)), "another user");
        assert!(!may_change(Some(U1), None), "no credential");
        assert!(
            !may_change(None, Some(U1)),
            "an authenticated user vs an anonymous comment"
        );
        assert!(
            may_change(None, None),
            "auth off: the same unauthenticated caller"
        );
    }

    #[test]
    fn a_non_uuid_sub_is_skipped_not_guessed() {
        assert_eq!(sub_as_recipient("alice"), None);
        assert!(notify_targets(None, None, &["alice".to_string()]).is_empty());
        assert_eq!(sub_as_recipient(U1), Some(format!("person:{U1}")));
    }

    #[test]
    fn an_owner_who_is_the_author_is_dropped() {
        let owner = format!("person:{U1}");
        assert!(notify_targets(Some(U1), Some(&owner), &[]).is_empty());
    }

    #[test]
    fn every_reaction_kind_is_counted_even_at_zero() {
        let c = reaction_counts(["heart", "heart", "eyes"].into_iter());
        assert_eq!(c.len(), REACTION_KINDS.len());
        assert_eq!(c.iter().find(|(k, _)| *k == "heart").unwrap().1, 2);
        assert_eq!(c.iter().find(|(k, _)| *k == "celebrate").unwrap().1, 0);
    }
}
