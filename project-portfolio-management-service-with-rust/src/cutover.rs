//! Go / no-go for safety-critical cutovers (EX-6). Pure rules; the database work is in
//! [`crate::controllers::cutover`].
//!
//! A cutover decision is made against **stop criteria published before it opens**; each is marked met or not
//! while it is open, and the roles the deployment names must co-sign. **Go** needs every criterion met and
//! every co-signature. **No-go** and **delay for safety** can be decided at any time while it is open: they
//! are decisions about safety, recorded and shown as such, and not a slipped date.

/// The decisions.
pub const OUTCOMES: [&str; 3] = ["go", "no_go", "delay_for_safety"];

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
