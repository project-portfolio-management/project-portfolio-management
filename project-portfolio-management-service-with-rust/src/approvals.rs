//! Approval requests (PL-1): the pure rules. DB-free, unit-tested.
//!
//! A request names **one approver** (an `EntityRef`) and a subject. Its life
//! is `pending → approved | rejected | withdrawn`, and every end state is
//! **final**: a decided request cannot be decided again, and a decided or
//! withdrawn one cannot be withdrawn. A rejection must say why.

use crate::notification_prefs::recipient_sub;

/// The statuses.
pub const STATUSES: &[&str] = &["pending", "approved", "rejected", "withdrawn"];

/// Cap on a request message or decision comment.
pub const MAX_TEXT: usize = 2_000;

/// A decision a approver may make.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision {
    /// Approve.
    Approved,
    /// Reject (needs a comment).
    Rejected,
}

impl Decision {
    /// Parse the wire token.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "approved" => Some(Self::Approved),
            "rejected" => Some(Self::Rejected),
            _ => None,
        }
    }

    /// The status it produces.
    #[must_use]
    pub const fn status(self) -> &'static str {
        match self {
            Self::Approved => "approved",
            Self::Rejected => "rejected",
        }
    }
}

/// Why an operation on a request is refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Refusal {
    /// The request is no longer pending (carries its status).
    NotPending(String),
    /// A rejection without a comment.
    CommentRequired,
    /// The comment is too long.
    CommentTooLong,
}

impl Refusal {
    /// A human message.
    #[must_use]
    pub fn message(&self) -> String {
        match self {
            Self::NotPending(s) => format!("the request is already {s}; a decision is final"),
            Self::CommentRequired => "a rejection must say why (comment is required)".to_string(),
            Self::CommentTooLong => format!("comment: exceeds {MAX_TEXT} characters"),
        }
    }
}

/// Apply a decision to a request currently in `status`.
///
/// # Errors
///
/// [`Refusal`] when it is not pending, a rejection lacks a comment, or the
/// comment is too long.
pub fn decide(
    status: &str,
    decision: Decision,
    comment: Option<&str>,
) -> Result<&'static str, Refusal> {
    if status != "pending" {
        return Err(Refusal::NotPending(status.to_string()));
    }
    let comment = comment.map(str::trim).filter(|c| !c.is_empty());
    if comment.is_some_and(|c| c.chars().count() > MAX_TEXT) {
        return Err(Refusal::CommentTooLong);
    }
    if decision == Decision::Rejected && comment.is_none() {
        return Err(Refusal::CommentRequired);
    }
    Ok(decision.status())
}

/// Whether a request in `status` can still be withdrawn.
///
/// # Errors
///
/// [`Refusal::NotPending`] for anything but a pending request.
pub fn withdraw(status: &str) -> Result<&'static str, Refusal> {
    if status == "pending" {
        Ok("withdrawn")
    } else {
        Err(Refusal::NotPending(status.to_string()))
    }
}

/// Whether the caller (token `sub`) is the named approver. The approver is a
/// `person:` / `worker:` reference whose id is the caller's `sub`; an
/// `organization:` approver, or a non-UUID `sub`, never matches — so a
/// decision always comes from a person, never from an anonymous caller.
#[must_use]
pub fn is_approver(approver_ref: &str, sub: Option<&str>) -> bool {
    let Some(sub) = sub else { return false };
    recipient_sub(approver_ref).is_some_and(|id| id.eq_ignore_ascii_case(sub))
}

#[cfg(test)]
mod tests {
    use super::*;

    const U1: &str = "3f2c6b7e-0000-4000-8000-000000000001";
    const U2: &str = "3f2c6b7e-0000-4000-8000-000000000002";

    #[test]
    fn approving_needs_no_comment_but_rejecting_does() {
        assert_eq!(decide("pending", Decision::Approved, None), Ok("approved"));
        assert_eq!(
            decide("pending", Decision::Rejected, None),
            Err(Refusal::CommentRequired)
        );
        assert_eq!(
            decide("pending", Decision::Rejected, Some("   ")),
            Err(Refusal::CommentRequired)
        );
        assert_eq!(
            decide("pending", Decision::Rejected, Some("over budget")),
            Ok("rejected")
        );
    }

    #[test]
    fn a_decision_is_final() {
        for s in ["approved", "rejected", "withdrawn"] {
            assert_eq!(
                decide(s, Decision::Approved, None),
                Err(Refusal::NotPending(s.to_string()))
            );
            assert_eq!(withdraw(s), Err(Refusal::NotPending(s.to_string())));
        }
        assert_eq!(withdraw("pending"), Ok("withdrawn"));
    }

    #[test]
    fn an_over_long_comment_is_refused() {
        let long = "x".repeat(MAX_TEXT + 1);
        assert_eq!(
            decide("pending", Decision::Approved, Some(&long)),
            Err(Refusal::CommentTooLong)
        );
    }

    #[test]
    fn only_the_named_person_is_the_approver() {
        assert!(is_approver(&format!("person:{U1}"), Some(U1)));
        assert!(is_approver(&format!("worker:{U1}"), Some(U1)));
        assert!(
            !is_approver(&format!("person:{U1}"), Some(U2)),
            "someone else"
        );
        assert!(!is_approver(&format!("person:{U1}"), None), "no identity");
        assert!(
            !is_approver(&format!("organization:{U1}"), Some(U1)),
            "an organization never decides"
        );
        assert!(!is_approver("person:not-a-uuid", Some("not-a-uuid")));
    }

    #[test]
    fn decisions_parse_only_the_two_tokens() {
        assert_eq!(Decision::parse("approved"), Some(Decision::Approved));
        assert_eq!(Decision::parse("rejected"), Some(Decision::Rejected));
        assert_eq!(Decision::parse("maybe"), None);
    }
}
