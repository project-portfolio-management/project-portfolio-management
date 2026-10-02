//! Private plans (RM-8): the pure rules. DB-free, unit-tested.
//!
//! A plan is **public** (today's behaviour, and the default for every plan
//! with no access row) or **private**, in which case only its **members** and
//! callers holding the `access=admin` attribute may see it. A non-member gets
//! **`404`, never `403`**, so the plan's existence is not disclosed.
//!
//! Fail closed: a private plan seen with **no identity at all** is hidden —
//! "private" must not quietly mean "public when authentication is off".

use uuid::Uuid;

/// Cap on members of one plan.
pub const MAX_MEMBERS: usize = 200;

/// Whether a caller may see a plan.
///
/// - public ⇒ everyone;
/// - private ⇒ a member (their `sub` is listed) or an admin; an anonymous
///   caller (`sub` is `None`) and a non-member are refused.
#[must_use]
pub fn can_see(private: bool, members: &[String], sub: Option<&str>, is_admin: bool) -> bool {
    if !private {
        return true;
    }
    is_admin || sub.is_some_and(|s| members.iter().any(|m| m.eq_ignore_ascii_case(s)))
}

/// Normalise a requested member list (lowercase UUIDs, deduplicated, in
/// order) and report every problem.
///
/// # Errors
///
/// A message naming each invalid entry, or an over-long list.
pub fn normalise_members(requested: &[String]) -> Result<Vec<String>, String> {
    let mut out: Vec<String> = Vec::new();
    let mut bad: Vec<&str> = Vec::new();
    for m in requested {
        match Uuid::parse_str(m.trim()) {
            Ok(u) => {
                let u = u.to_string();
                if !out.contains(&u) {
                    out.push(u);
                }
            }
            Err(_) => bad.push(m),
        }
    }
    if !bad.is_empty() {
        return Err(format!(
            "members must be user ids (UUIDs); invalid: {bad:?}"
        ));
    }
    if out.len() > MAX_MEMBERS {
        return Err(format!("a plan has at most {MAX_MEMBERS} members"));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const U1: &str = "3f2c6b7e-0000-4000-8000-000000000001";
    const U2: &str = "3f2c6b7e-0000-4000-8000-000000000002";

    #[test]
    fn a_public_plan_is_visible_to_everyone_including_anonymous() {
        assert!(can_see(false, &[], None, false));
        assert!(can_see(false, &[U1.to_string()], Some(U2), false));
    }

    #[test]
    fn a_private_plan_admits_members_and_admins_only() {
        let members = vec![U1.to_string()];
        assert!(can_see(true, &members, Some(U1), false), "a member");
        assert!(
            can_see(true, &members, Some(&U1.to_uppercase()), false),
            "case-insensitive"
        );
        assert!(
            can_see(true, &members, Some(U2), true),
            "an admin who is not a member"
        );
        assert!(!can_see(true, &members, Some(U2), false), "a non-member");
    }

    #[test]
    fn a_private_plan_fails_closed_for_an_anonymous_caller() {
        assert!(!can_see(true, &[U1.to_string()], None, false));
        assert!(!can_see(true, &[], None, false), "even with no members");
    }

    #[test]
    fn members_are_normalised_deduplicated_and_validated() {
        let m = normalise_members(&[U1.to_uppercase(), U1.to_string(), U2.to_string()]).unwrap();
        assert_eq!(m, [U1, U2]);
        let err = normalise_members(&["alice".to_string(), U1.to_string()]).unwrap_err();
        assert!(err.contains("alice"));
        let many: Vec<String> = (0..=MAX_MEMBERS as u128)
            .map(|n| Uuid::from_u128(n).to_string())
            .collect();
        assert!(normalise_members(&many).is_err());
    }
}
