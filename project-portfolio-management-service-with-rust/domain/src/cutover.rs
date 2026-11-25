//! Go / no-go for safety-critical cutovers (EX-6). Pure rules; the database work is in
//! `project_portfolio_management::controllers::cutover`.
//!
//! A cutover decision is made against **stop criteria published before it opens**; each is marked met or not
//! while it is open, and the roles the deployment names must co-sign. **Go** needs every criterion met and
//! every co-signature. **No-go** and **delay for safety** can be decided at any time while it is open: they
//! are decisions about safety, recorded and shown as such, and not a slipped date.

/// The decisions.
pub const OUTCOMES: [&str; 3] = ["go", "no_go", "delay_for_safety"];

/// Co-signing roles used when the deployment names none or names only invalid ones.
fn default_roles() -> Vec<String> {
    vec![
        "accountable_professional_lead".to_string(),
        "delivery_organization".to_string(),
    ]
}

/// Parse a comma-separated list of role names; names with spaces or over 60 characters are dropped, and an
/// empty result is the default.
#[must_use]
pub fn parse_roles(raw: &str) -> Vec<String> {
    let roles: Vec<String> = raw
        .split(',')
        .map(|r| r.trim().to_lowercase())
        .filter(|r| !r.is_empty() && r.len() <= 60 && !r.contains(char::is_whitespace))
        .collect();
    if roles.is_empty() {
        default_roles()
    } else {
        roles
    }
}

/// The roles that must co-sign a go: `PROJECT_PORTFOLIO_MANAGEMENT_CUTOVER_COSIGNERS`, a comma-separated list
/// of role names, by default the partner's accountable professional lead and the delivery organization.
#[must_use]
pub fn required_roles() -> Vec<String> {
    std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_CUTOVER_COSIGNERS")
        .map_or_else(|_| default_roles(), |v| parse_roles(&v))
}

/// The digest a co-signature attests to (EX-6a): SHA-256, in hex, over each criterion's position, mark and
/// evidence text, in position order. Any change to a mark or to its evidence changes it.
#[must_use]
pub fn evidence_digest(criteria: &[(i32, Option<bool>, Option<&str>)]) -> String {
    use sha2::{Digest, Sha256};
    use std::fmt::Write;
    let mut h = Sha256::new();
    for (n, met, evidence) in criteria {
        let m = match met {
            Some(true) => "met",
            Some(false) => "unmet",
            None => "unmarked",
        };
        let e = evidence.unwrap_or("");
        // Length-prefixed, so one field cannot run into the next.
        h.update(format!("{n}|{m}|{}|{e}\n", e.len()).as_bytes());
    }
    h.finalize().iter().fold(String::new(), |mut out, b| {
        let _ = write!(out, "{b:02x}");
        out
    })
}

/// The roles a decision requires: the list frozen when it opened, or the current setting for one opened before.
#[must_use]
pub fn frozen_roles(stored: Option<&str>) -> Vec<String> {
    stored.map_or_else(required_roles, |s| {
        s.split(',')
            .map(str::to_string)
            .filter(|r| !r.is_empty())
            .collect()
    })
}

/// The roles that have signed and whose signature is for the `current` evidence, and the roles whose only
/// signatures are for other evidence (`stale`). Signatures are `(role, digest)`; one with no digest never counts.
#[must_use]
pub fn current_and_stale(
    signatures: &[(String, Option<String>)],
    current: &str,
) -> (Vec<String>, Vec<String>) {
    let mut ok: Vec<String> = Vec::new();
    for (r, d) in signatures {
        if d.as_deref() == Some(current) && !ok.contains(r) {
            ok.push(r.clone());
        }
    }
    let mut stale: Vec<String> = Vec::new();
    for (r, _) in signatures {
        if !ok.contains(r) && !stale.contains(r) {
            stale.push(r.clone());
        }
    }
    (ok, stale)
}

/// What stands in the way of a go: criteria not marked or not met (by their number, from 1), and roles that
/// have not co-signed. Empty means a go may be recorded.
#[must_use]
pub fn go_problems(
    criteria: &[Option<bool>],
    required: &[String],
    signed: &[String],
) -> Vec<String> {
    let mut out = Vec::new();
    let unmarked: Vec<String> = criteria
        .iter()
        .enumerate()
        .filter(|(_, m)| m.is_none())
        .map(|(i, _)| (i + 1).to_string())
        .collect();
    if !unmarked.is_empty() {
        out.push(format!("criteria not yet marked: {}", unmarked.join(", ")));
    }
    let unmet: Vec<String> = criteria
        .iter()
        .enumerate()
        .filter(|(_, m)| **m == Some(false))
        .map(|(i, _)| (i + 1).to_string())
        .collect();
    if !unmet.is_empty() {
        out.push(format!("criteria not met: {}", unmet.join(", ")));
    }
    if criteria.is_empty() {
        out.push("there are no stop criteria".to_string());
    }
    let missing: Vec<&str> = required
        .iter()
        .filter(|r| !signed.iter().any(|s| s == *r))
        .map(String::as_str)
        .collect();
    if !missing.is_empty() {
        out.push(format!("co-signatures missing: {}", missing.join(", ")));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn roles(r: &[&str]) -> Vec<String> {
        r.iter().map(|x| (*x).to_string()).collect()
    }

    #[test]
    fn go_needs_every_criterion_met_and_every_cosignature() {
        let req = roles(&["lead", "delivery"]);
        assert!(
            go_problems(
                &[Some(true), Some(true)],
                &req,
                &roles(&["lead", "delivery"])
            )
            .is_empty()
        );
        assert_eq!(
            go_problems(&[Some(true), Some(true)], &req, &roles(&["lead"])),
            ["co-signatures missing: delivery"]
        );
        assert_eq!(
            go_problems(
                &[Some(true), Some(false), None],
                &req,
                &roles(&["lead", "delivery"])
            ),
            ["criteria not yet marked: 3", "criteria not met: 2"]
        );
        assert_eq!(go_problems(&[], &[], &[]), ["there are no stop criteria"]);
    }

    #[test]
    fn a_signature_counts_only_for_the_evidence_it_was_given_on() {
        let a = evidence_digest(&[(1, Some(true), Some("Rehearsal record"))]);
        assert_eq!(
            a,
            evidence_digest(&[(1, Some(true), Some("Rehearsal record"))])
        );
        assert_ne!(
            a,
            evidence_digest(&[(1, Some(true), Some("Rehearsal record 2"))])
        );
        assert_ne!(
            a,
            evidence_digest(&[(1, Some(false), Some("Rehearsal record"))])
        );
        assert_ne!(a, evidence_digest(&[(1, None, None)]));
        let sigs = vec![
            ("lead".to_string(), Some(a.clone())),
            ("delivery".to_string(), Some("old".to_string())),
            ("legacy".to_string(), None),
        ];
        let (ok, stale) = current_and_stale(&sigs, &a);
        assert_eq!(
            (ok, stale),
            (roles(&["lead"]), roles(&["delivery", "legacy"]))
        );
        let again = vec![
            ("delivery".to_string(), Some("old".to_string())),
            ("delivery".to_string(), Some(a.clone())),
        ];
        assert_eq!(
            current_and_stale(&again, &a),
            (roles(&["delivery"]), vec![]),
            "signing again makes it current"
        );
    }

    #[test]
    fn frozen_roles_win_over_the_setting() {
        assert_eq!(frozen_roles(Some("a,b")), ["a", "b"]);
    }

    #[test]
    fn the_cosigners_are_configurable_and_a_bad_setting_falls_back() {
        assert_eq!(
            parse_roles("Clinical Lead, ,supplier"),
            ["supplier"],
            "a role with a space in it is dropped"
        );
        assert_eq!(
            parse_roles("clinical_lead,Supplier"),
            ["clinical_lead", "supplier"]
        );
        assert_eq!(
            parse_roles(" , "),
            ["accountable_professional_lead", "delivery_organization"]
        );
    }
}
