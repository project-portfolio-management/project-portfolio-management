//! Deployment-date change control (EX-5). Pure rules; the database work is in
//! [`crate::controllers::deployment_moves`].
//!
//! In a programme delivered as a **central platform with local deployment**, a partner moving the go-live of
//! its own deployment moves other deployments that depend on the platform's schedule. So the move is a
//! request: the schedule network works out what it would knock on to, a named approver decides, and until
//! then **the schedule does not change**. The approver is a person (`person:<id>`) or a role the
//! deployment names (for example `role:escalation_board`); it is never the requester.

use crate::change_requests::{identifier, is_person};

/// The programme's delivery model that puts a deployment's date under control.
pub const CONTROLLED_MODEL: &str = "central_platform_local_deployment";

/// The default approver, if the deployment sets one: `PROJECT_PORTFOLIO_MANAGEMENT_DEPLOYMENT_APPROVER`.
#[must_use]
pub fn default_approver() -> Option<String> {
    std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_DEPLOYMENT_APPROVER")
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
}

/// What is wrong with naming `approver` for a move asked for by `requester`.
#[must_use]
pub fn approver_problem(approver: &str, requester: Option<&str>) -> Option<String> {
    if let Some(role) = approver.trim().strip_prefix("role:") {
        return (role.trim().is_empty() || role.len() > 60 || role.contains(char::is_whitespace))
            .then(|| {
                "a role approver is role:<name> with no spaces, up to 60 characters".to_string()
            });
    }
    if !crate::issues::is_person_ref(approver) {
        return Some("approver must be a person: or worker: reference, or role:<name>".to_string());
    }
    requester
        .is_some_and(|r| identifier(approver).eq_ignore_ascii_case(r.trim()))
        .then(|| "the approver cannot be the person who asked for the move".to_string())
}

/// Whether the caller may decide a move whose approver is `approver`: the named person, or anyone who holds
/// the named role, and never the requester. An administrator is not the approver unless named.
#[must_use]
pub fn may_decide(
    approver: &str,
    actor: Option<&str>,
    roles: &[String],
    requester: Option<&str>,
) -> bool {
    if let (Some(a), Some(r)) = (actor, requester)
        && a.trim().eq_ignore_ascii_case(identifier(r))
    {
        return false;
    }
    match approver.trim().strip_prefix("role:") {
        Some(role) => roles.iter().any(|r| r.eq_ignore_ascii_case(role.trim())),
        None => is_person(approver, actor),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_approver_is_a_person_other_than_the_requester_or_a_role() {
        let a = "person:3f2c6b7e-0000-4000-8000-0000000000b1";
        assert!(approver_problem(a, Some("3f2c6b7e-0000-4000-8000-0000000000a1")).is_none());
        assert!(
            approver_problem(a, Some("3F2C6B7E-0000-4000-8000-0000000000B1")).is_some(),
            "the same person in another case"
        );
        assert!(approver_problem("role:escalation_board", Some("anyone")).is_none());
        assert!(approver_problem("role:", None).is_some());
        assert!(approver_problem("role:two words", None).is_some());
        assert!(approver_problem("Bob", None).is_some());
    }

    #[test]
    fn only_the_named_person_or_a_holder_of_the_role_decides_and_never_the_requester() {
        let bob = "person:3f2c6b7e-0000-4000-8000-0000000000b1";
        assert!(may_decide(
            bob,
            Some("3f2c6b7e-0000-4000-8000-0000000000b1"),
            &[],
            Some("someone-else")
        ));
        assert!(
            !may_decide(
                bob,
                Some("3f2c6b7e-0000-4000-8000-0000000000c1"),
                &[],
                Some("someone-else")
            ),
            "another writer"
        );
        let roles = vec!["escalation_board".to_string()];
        assert!(may_decide(
            "role:escalation_board",
            Some("x"),
            &roles,
            Some("y")
        ));
        assert!(
            !may_decide("role:escalation_board", Some("x"), &[], Some("y")),
            "without the role"
        );
        assert!(
            !may_decide("role:escalation_board", Some("y"), &roles, Some("y")),
            "the requester holds the role but cannot decide their own"
        );
        assert!(!may_decide(bob, None, &[], None), "nobody signed in");
    }
}
