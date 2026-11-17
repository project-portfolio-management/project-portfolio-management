//! Role permissions on a plan (OC-3): the pure rules. DB-free, unit-tested.
//!
//! A role is a named set of permissions, and a person can hold several on a plan, which combine, as in Redmine. **The
//! token stays the ceiling.** A write has already passed the token's policy before any of this runs, so a role can only
//! narrow what a caller may do. It cannot make a read-only token write, open a private plan, or make anyone an
//! administrator (`crate::controllers::plan_access::is_admin` reads the token alone).
//!
//! Enforcement is opt-in per plan: a plan with no members is **not enforced**, so it behaves as it did before roles
//! carried permissions. Once it has a member, adding, editing, moving or deleting an issue there needs the permission,
//! from a role in the catalogue that is not retired, held on the plan or carried by the token. An administrator passes.

use std::collections::BTreeSet;

/// Raise an issue, by hand or from a risk or a blocked task.
pub const ISSUE_ADD: &str = "issue_add";
/// Change an issue's fields or type.
pub const ISSUE_EDIT: &str = "issue_edit";
/// Change an issue's status, with the resolution that goes with it. A workflow's gates still apply.
pub const ISSUE_MOVE: &str = "issue_move";
/// Remove an issue.
pub const ISSUE_DELETE: &str = "issue_delete";

/// Every permission a role can carry.
pub const PERMISSIONS: &[&str] = &[ISSUE_ADD, ISSUE_EDIT, ISSUE_MOVE, ISSUE_DELETE];

/// One membership: `(person, role key)`.
pub type Member = (String, String);

/// One catalogue entry, as this module needs it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Role {
    /// The role's key.
    pub key: String,
    /// The permissions it carries.
    pub permissions: Vec<String>,
    /// A retired role carries nothing and adds nothing to a membership.
    pub retired: bool,
}

/// The entries in `requested` that are not permissions.
#[must_use]
pub fn unknown_permissions(requested: &[String]) -> Vec<String> {
    requested
        .iter()
        .filter(|p| !PERMISSIONS.contains(&p.as_str()))
        .cloned()
        .collect()
}

/// What a caller holds on one plan.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Standing {
    /// Whether the plan has any members, so permissions are required on it.
    pub enforced: bool,
    /// Whether the token says administrator.
    pub admin: bool,
    /// Roles for a workflow's gates: the token's, and those held on this plan and not retired.
    pub roles: Vec<String>,
    /// The permissions those roles carry.
    pub permissions: BTreeSet<String>,
}

impl Standing {
    /// Whether the caller may do what `permission` names on this plan.
    #[must_use]
    pub fn may(&self, permission: &str) -> bool {
        !self.enforced || self.admin || self.permissions.contains(permission)
    }
}

/// Work out a caller's standing from what the token says, the plan's members and the catalogue. A role the catalogue
/// does not list keeps its place in `roles`, because a workflow's gate can name it and an empty catalogue checks nothing,
/// but it carries no permissions.
#[must_use]
pub fn standing(
    admin: bool,
    token_roles: &[String],
    actor: Option<&str>,
    members: &[Member],
    catalogue: &[Role],
) -> Standing {
    let held = members
        .iter()
        .filter(|(person, _)| actor.is_some_and(|a| person.eq_ignore_ascii_case(a)))
        .map(|(_, role)| role)
        .filter(|role| catalogue.iter().any(|r| r.key == **role && !r.retired));
    let mut roles: Vec<String> = token_roles.iter().chain(held).cloned().collect();
    roles.sort();
    roles.dedup();
    let permissions = catalogue
        .iter()
        .filter(|r| !r.retired && roles.contains(&r.key))
        .flat_map(|r| r.permissions.iter().cloned())
        .filter(|p| PERMISSIONS.contains(&p.as_str()))
        .collect();
    Standing {
        enforced: !members.is_empty(),
        admin,
        roles,
        permissions,
    }
}

/// The permissions a save of an existing issue needs. A save that moves it and changes nothing else needs only
/// [`ISSUE_MOVE`]; any other save needs [`ISSUE_EDIT`], so a save that does both needs both.
#[must_use]
pub fn needed_for_save(moved: bool, edited: bool) -> Vec<&'static str> {
    match (moved, edited) {
        (true, true) => vec![ISSUE_MOVE, ISSUE_EDIT],
        (true, false) => vec![ISSUE_MOVE],
        (false, _) => vec![ISSUE_EDIT],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ME: &str = "3f2c6b7e-0000-4000-8000-0000000001a1";

    fn role(key: &str, permissions: &[&str], retired: bool) -> Role {
        Role {
            key: key.to_string(),
            permissions: permissions.iter().map(ToString::to_string).collect(),
            retired,
        }
    }

    fn member(person: &str, role: &str) -> Member {
        (person.to_string(), role.to_string())
    }

    #[test]
    fn a_plan_with_no_members_is_not_enforced() {
        let s = standing(false, &[], Some(ME), &[], &[]);
        assert!(!s.enforced);
        assert!(s.may(ISSUE_DELETE));
    }

    #[test]
    fn someone_elses_membership_gives_nothing() {
        let s = standing(
            false,
            &[],
            Some(ME),
            &[member("someone-else", "lead")],
            &[role("lead", &[ISSUE_EDIT], false)],
        );
        assert!(s.enforced);
        assert!(!s.may(ISSUE_EDIT));
        assert!(s.roles.is_empty());
    }

    #[test]
    fn several_roles_combine_and_a_token_role_counts() {
        let s = standing(
            false,
            &["qa".to_string()],
            Some(ME),
            &[member(ME, "lead")],
            &[
                role("lead", &[ISSUE_EDIT], false),
                role("qa", &[ISSUE_MOVE], false),
            ],
        );
        assert!(s.may(ISSUE_EDIT) && s.may(ISSUE_MOVE));
        assert!(!s.may(ISSUE_DELETE));
        assert_eq!(s.roles, ["lead", "qa"]);
    }

    #[test]
    fn a_retired_role_carries_nothing_and_opens_no_gate_through_membership() {
        let s = standing(
            false,
            &[],
            Some(ME),
            &[member(ME, "lead")],
            &[role("lead", &[ISSUE_EDIT], true)],
        );
        assert!(s.enforced, "the plan stays enforced");
        assert!(!s.may(ISSUE_EDIT));
        assert!(s.roles.is_empty());
    }

    #[test]
    fn an_anonymous_caller_matches_no_member() {
        let s = standing(
            false,
            &[],
            None,
            &[member(ME, "lead")],
            &[role("lead", &[ISSUE_EDIT], false)],
        );
        assert!(!s.may(ISSUE_EDIT));
    }

    #[test]
    fn a_stored_word_outside_the_vocabulary_grants_nothing() {
        let s = standing(
            false,
            &[],
            Some(ME),
            &[member(ME, "lead")],
            &[role("lead", &["access_admin"], false)],
        );
        assert!(s.permissions.is_empty());
    }

    #[test]
    fn an_administrator_passes_and_a_role_called_admin_does_not() {
        let catalogue = [role("admin", &[], false)];
        let members = [member("x", "admin")];
        assert!(standing(true, &[], Some(ME), &members, &catalogue).may(ISSUE_DELETE));
        let held = standing(
            false,
            &["admin".to_string()],
            Some(ME),
            &members,
            &catalogue,
        );
        assert!(!held.may(ISSUE_DELETE));
    }

    #[test]
    fn a_save_needs_what_it_does() {
        assert_eq!(needed_for_save(true, false), [ISSUE_MOVE]);
        assert_eq!(needed_for_save(true, true), [ISSUE_MOVE, ISSUE_EDIT]);
        assert_eq!(needed_for_save(false, true), [ISSUE_EDIT]);
        assert_eq!(needed_for_save(false, false), [ISSUE_EDIT]);
    }

    #[test]
    fn the_vocabulary_is_checked() {
        let asked = ["issue_add".to_string(), "fly".to_string()];
        assert_eq!(unknown_permissions(&asked), ["fly"]);
    }
}
