//! Guest access (PL-7): the pure rules. DB-free, unit-tested.
//!
//! A **guest** is a signed-in caller whose token carries `role=guest`: an
//! outside collaborator (a vendor, a client). A guest sees **only** the
//! subjects granted to them, and every other request is `404` — a guest cannot
//! enumerate plans, search, or read a report. A grant is `read` or `comment`,
//! can expire, and is revocable; both take effect on the very next request.
//!
//! The path rules below decide what a guest may *attempt*; the grant decides
//! whether it is granted. Anything not named here is denied by default.

use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Grant levels.
pub const ACCESS: &[&str] = &["read", "comment"];

/// Whether a token's `role` attribute makes the caller a guest.
#[must_use]
pub fn is_guest(roles: &[String]) -> bool {
    roles.iter().any(|r| r == "guest")
}

/// Whether a grant is live at `now`.
#[must_use]
pub fn grant_live(expires_at: Option<DateTime<Utc>>, revoked: bool, now: DateTime<Utc>) -> bool {
    !revoked && expires_at.is_none_or(|e| e > now)
}

/// What a guest request is, once its path and method are read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Attempt {
    /// Reading the granted plan (`/api/plans/{pid}` or beneath it).
    ReadPlan(Uuid),
    /// Reading a subject's comments (`GET /api/comments?...`) — the subject is
    /// in the query string, so the handler checks it.
    ReadComments,
    /// Adding a comment (`POST /api/comments`).
    PostComment,
    /// Reading attachments of a subject.
    ReadAttachments,
    /// The guest's own harmless endpoints (`/api/whoami`, `/api/me/…` GETs).
    Own,
    /// Anything else: denied.
    Denied,
}

/// Classify a request by method and path.
#[must_use]
pub fn classify(method: &str, path: &str) -> Attempt {
    let get = method == "GET" || method == "HEAD";
    if let Some(rest) = path.strip_prefix("/api/plans/") {
        let mut parts = rest.split('/');
        let first = parts.next().unwrap_or_default();
        let sub = parts.next();
        // Only the (masked) plan record and the sub-resources that carry no
        // people references. Tasks, risks and allocations name assignees and
        // owners, so a guest does not get them.
        let safe =
            matches!(sub, None | Some("milestones" | "components")) && parts.next().is_none();
        return match (Uuid::parse_str(first), get && safe) {
            (Ok(pid), true) => Attempt::ReadPlan(pid),
            _ => Attempt::Denied,
        };
    }
    match (method, path) {
        ("GET" | "HEAD", "/api/comments") => Attempt::ReadComments,
        ("POST", "/api/comments") => Attempt::PostComment,
        ("GET" | "HEAD", "/api/attachments") => Attempt::ReadAttachments,
        ("GET" | "HEAD", "/api/whoami" | "/api/plans/whoami") => Attempt::Own,
        _ if get && path.starts_with("/api/me/") => Attempt::Own,
        _ => Attempt::Denied,
    }
}

/// A grant, as the decision sees it.
#[derive(Debug, Clone)]
pub struct Grant {
    /// The subject kind (`plan`, `task`, …).
    pub subject_kind: String,
    /// The subject.
    pub subject_pid: Uuid,
    /// `read` or `comment`.
    pub access: String,
    /// Still live.
    pub live: bool,
}

/// Whether a live grant covers `subject` for an action needing `need`
/// (`comment` implies `read`; `read` does not imply `comment`).
#[must_use]
pub fn covers(grants: &[Grant], kind: &str, pid: Uuid, need: &str) -> bool {
    grants.iter().any(|g| {
        g.live
            && g.subject_kind == kind
            && g.subject_pid == pid
            && (g.access == "comment" || (g.access == "read" && need == "read"))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    fn id(n: u128) -> Uuid {
        Uuid::from_u128(n)
    }

    #[test]
    fn only_the_guest_role_makes_a_guest() {
        assert!(is_guest(&["dev".into(), "guest".into()]));
        assert!(!is_guest(&["Guest".into()]), "case matters");
        assert!(!is_guest(&[]));
    }

    #[test]
    fn a_grant_dies_on_expiry_or_revocation() {
        let now = Utc::now();
        assert!(grant_live(None, false, now));
        assert!(grant_live(Some(now + Duration::hours(1)), false, now));
        assert!(
            !grant_live(Some(now - Duration::seconds(1)), false, now),
            "expired"
        );
        assert!(!grant_live(None, true, now), "revoked");
    }

    #[test]
    fn a_guest_may_attempt_only_a_few_narrow_things() {
        let p = Uuid::from_u128(5);
        assert_eq!(
            classify("GET", &format!("/api/plans/{p}")),
            Attempt::ReadPlan(p)
        );
        assert_eq!(
            classify("GET", &format!("/api/plans/{p}/milestones")),
            Attempt::ReadPlan(p)
        );
        // Tasks, risks and allocations carry people references: not for guests.
        for sub in ["tasks", "risks", "allocations", "milestones/x/y"] {
            assert_eq!(
                classify("GET", &format!("/api/plans/{p}/{sub}")),
                Attempt::Denied,
                "{sub}"
            );
        }
        // Mutating a plan, listing plans, searching and reports are all denied.
        assert_eq!(classify("PUT", &format!("/api/plans/{p}")), Attempt::Denied);
        assert_eq!(
            classify("DELETE", &format!("/api/plans/{p}")),
            Attempt::Denied
        );
        assert_eq!(
            classify("GET", "/api/plans"),
            Attempt::Denied,
            "no enumerating"
        );
        assert_eq!(classify("GET", "/api/plans/search"), Attempt::Denied);
        assert_eq!(classify("POST", "/api/plans"), Attempt::Denied);
        assert_eq!(classify("GET", "/api/reports"), Attempt::Denied);
        assert_eq!(classify("GET", "/api/at-a-glance"), Attempt::Denied);
        assert_eq!(classify("GET", "/api/comments"), Attempt::ReadComments);
        assert_eq!(classify("POST", "/api/comments"), Attempt::PostComment);
        assert_eq!(
            classify("POST", "/api/attachments"),
            Attempt::Denied,
            "no uploads"
        );
        assert_eq!(classify("GET", "/api/me/tasks"), Attempt::Own);
        assert_eq!(classify("POST", "/api/me/feed-token"), Attempt::Denied);
    }

    #[test]
    fn comment_implies_read_but_read_does_not_imply_comment() {
        let g = |access: &str, live: bool| Grant {
            subject_kind: "plan".into(),
            subject_pid: id(1),
            access: access.into(),
            live,
        };
        assert!(covers(&[g("read", true)], "plan", id(1), "read"));
        assert!(!covers(&[g("read", true)], "plan", id(1), "comment"));
        assert!(covers(&[g("comment", true)], "plan", id(1), "read"));
        assert!(covers(&[g("comment", true)], "plan", id(1), "comment"));
        assert!(
            !covers(&[g("comment", false)], "plan", id(1), "read"),
            "a dead grant covers nothing"
        );
        assert!(
            !covers(&[g("read", true)], "plan", id(2), "read"),
            "another subject"
        );
        assert!(
            !covers(&[g("read", true)], "task", id(1), "read"),
            "another kind"
        );
    }
}
