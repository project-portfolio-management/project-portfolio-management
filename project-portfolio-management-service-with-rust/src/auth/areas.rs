//! The service-owned ABAC areas (`finance`, `comms`, `break_glass`, `subjects`, `access_review`, escalations, shared frameworks), placed in front of every policy so a policy file cannot open them.

#[allow(
    clippy::wildcard_imports,
    reason = "the parent module holds the shared imports and the other parts"
)]
use super::*;

/// A rule over one `resource.area` and the subject attribute that opens it.
pub(super) fn area_rule(
    effect: Effect,
    actions: &[ActionPattern],
    area: &str,
    subject: Option<(&str, &str)>,
) -> Rule {
    let mut when = std::collections::BTreeMap::new();
    when.insert("resource.area".to_string(), vec![area.to_string()]);
    if let Some((key, value)) = subject {
        when.insert(key.to_string(), vec![value.to_string()]);
    }
    Rule {
        effect,
        actions: actions.to_vec(),
        when,
        obligations: Vec::new(),
    }
}

/// The rules this service **always** puts in front of the deployment's policy, so
/// that an operator's policy file (or the built-in default) cannot leave them open:
///
/// * `resource.area = finance` (cost rates, task costs, labour cost, and the
///   earned-value figures that rest on them): every action needs
///   `access=admin` or `finance=cost_rates`;
/// * `resource.area = comms` (announcements): **writes** need `access=admin` or
///   `comms=publish`; reads fall through to the policy (the default allows them).
///
/// * `resource.area = subjects` (data-subject export and pseudonymisation) and
///   `resource.area = access_review` (the recertification report): every action needs
///   `access=admin`, with no other holder.
///
/// * `resource.area = escalation_acknowledgement` and `resource.area = escalation_resolution`
///   (CF-4: saying an escalation has been seen, and answering it): every action needs
///   `access=admin` or `escalation=acknowledge` / `escalation=resolve` respectively. Holding
///   one does not give the other, and neither replaces the right to write to the plan.
///
/// * `resource.area = shared_framework` (CF-4): creating, editing or deleting an **organisation-wide**
///   ways-of-working or governance framework needs `access=admin` or `framework=edit`. A framework a plan
///   owns follows that plan's write access, as before.
///
/// A handler asks with [`area_allows`]. A deployment that wants other holders can
/// widen who carries those attributes in its identity service; it cannot widen
/// these rules from the policy file, by design.
#[must_use]
pub fn service_rules() -> Vec<Rule> {
    use ActionPattern::{Any, Delete, Destructive, Write};
    let mut rules = Vec::new();
    for (area, who, actions) in [
        ("finance", "finance", vec![Any]),
        ("comms", "comms", vec![Write, Delete, Destructive]),
    ] {
        let key = if who == "finance" {
            ("finance", "cost_rates")
        } else {
            ("comms", "publish")
        };
        rules.push(area_rule(
            Effect::Allow,
            &actions,
            area,
            Some(("access", "admin")),
        ));
        rules.push(area_rule(Effect::Allow, &actions, area, Some(key)));
        rules.push(area_rule(Effect::Deny, &actions, area, None));
    }
    // Break-glass access (RG-10): an administrator, or a person the identity provider has marked
    // `breakglass=eligible`; nobody else, whatever the policy file says.
    rules.push(area_rule(
        Effect::Allow,
        &[ActionPattern::Any],
        "break_glass",
        Some(("access", "admin")),
    ));
    rules.push(area_rule(
        Effect::Allow,
        &[ActionPattern::Any],
        "break_glass",
        Some(("breakglass", "eligible")),
    ));
    rules.push(area_rule(
        Effect::Deny,
        &[ActionPattern::Any],
        "break_glass",
        None,
    ));
    // Escalations (CF-4): acknowledging and resolving are separate rights, each held by an
    // administrator or by the attribute `escalation=<verb>` the identity service assigns.
    for (area, verb) in [
        (ESCALATION_ACKNOWLEDGEMENT, "acknowledge"),
        (ESCALATION_RESOLUTION, "resolve"),
    ] {
        rules.push(area_rule(
            Effect::Allow,
            &[ActionPattern::Any],
            area,
            Some(("access", "admin")),
        ));
        rules.push(area_rule(
            Effect::Allow,
            &[ActionPattern::Any],
            area,
            Some(("escalation", verb)),
        ));
        rules.push(area_rule(Effect::Deny, &[ActionPattern::Any], area, None));
    }
    // Organisation-wide frameworks (CF-4): an administrator, or `framework=edit`.
    rules.push(area_rule(
        Effect::Allow,
        &[ActionPattern::Any],
        SHARED_FRAMEWORK,
        Some(("access", "admin")),
    ));
    rules.push(area_rule(
        Effect::Allow,
        &[ActionPattern::Any],
        SHARED_FRAMEWORK,
        Some(("framework", "edit")),
    ));
    rules.push(area_rule(
        Effect::Deny,
        &[ActionPattern::Any],
        SHARED_FRAMEWORK,
        None,
    ));
    // Data-subject requests (RG-6) and the access review (RG-9): administrators only, whatever the
    // policy file says.
    for area in ["subjects", "access_review"] {
        rules.push(area_rule(
            Effect::Allow,
            &[ActionPattern::Any],
            area,
            Some(("access", "admin")),
        ));
        rules.push(area_rule(Effect::Deny, &[ActionPattern::Any], area, None));
    }
    rules
}

/// The area of the right to acknowledge an escalation (CF-4).
pub const ESCALATION_ACKNOWLEDGEMENT: &str = "escalation_acknowledgement";

/// The area of the right to resolve an escalation (CF-4).
pub const ESCALATION_RESOLUTION: &str = "escalation_resolution";

/// The area of the right to change an organisation-wide framework (CF-4).
pub const SHARED_FRAMEWORK: &str = "shared_framework";

/// `policy` with [`service_rules`] first (first match wins).
#[must_use]
pub fn with_service_rules(policy: Policy) -> Policy {
    let mut rules = service_rules();
    rules.extend(policy.rules);
    Policy { rules }
}

/// Whether the caller may do `action` in a service `area` (`finance`, `comms`),
/// decided by the policy. Allowed when enforcement is off (the deployment's posture
/// for every write); denied when it is on and there are no verified claims.
#[must_use]
pub fn area_allows(caller: &MaybeAuthUser, area: &str, action: Action) -> bool {
    if !require_auth() {
        return true;
    }
    let Some(claims) = caller.claims() else {
        return false;
    };
    let mut resource = std::collections::BTreeMap::new();
    resource.insert("area".to_string(), vec![area.to_string()]);
    policy()
        .current()
        .evaluate_with_resource(claims, action, ENTITY, &resource)
        .allowed
}
