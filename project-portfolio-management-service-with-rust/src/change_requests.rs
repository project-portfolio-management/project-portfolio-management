//! Change requests (IR-4): the pure rules. A change request is a controlled proposal to change an approved
//! baseline. It moves through a fixed set of states, **one named approver decides and that person is not
//! the requester**, a decision needs a reason, and impacts are stated and never guessed.
//!
//! ```text
//! draft ──submit──▶ submitted ──review──▶ under_review ──approve──▶ approved ──implement──▶ implemented
//!   │                  │  └──────approve / reject─────────┘  └─reject─▶ rejected
//!   └────withdraw──────┴────────────withdraw────────────────────────────▶ withdrawn
//! ```

use chrono::NaiveDate;

/// What kind of change.
pub const KINDS: &[&str] = &["scope", "schedule", "cost", "quality", "other"];
/// Every status.
pub const STATUSES: &[&str] = &[
    "draft",
    "submitted",
    "under_review",
    "approved",
    "rejected",
    "implemented",
    "withdrawn",
];
/// The shortest decision reason, in characters.
pub const MIN_DECISION_REASON: usize = 10;
/// The longest title, in characters.
pub const TITLE_MAX: usize = 200;
/// The longest description, reason or summary, in characters.
pub const TEXT_MAX: usize = 10_000;

/// What a person does to a change request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// Send a draft for decision.
    Submit,
    /// The approver starts looking at it.
    Review,
    /// The approver approves.
    Approve,
    /// The approver rejects.
    Reject,
    /// The approved change has been carried out.
    Implement,
    /// The requester withdraws it.
    Withdraw,
}

/// The status an action leads to from `status`, or `None` when it is not allowed there.
#[must_use]
pub fn next(status: &str, action: Action) -> Option<&'static str> {
    use Action::{Approve, Implement, Reject, Review, Submit, Withdraw};
    Some(match (status, action) {
        ("draft", Submit) => "submitted",
        ("submitted", Review) => "under_review",
        ("submitted" | "under_review", Approve) => "approved",
        ("submitted" | "under_review", Reject) => "rejected",
        ("approved", Implement) => "implemented",
        ("draft" | "submitted" | "under_review" | "approved", Withdraw) => "withdrawn",
        _ => return None,
    })
}

/// Whether a change request in this status can still be edited (only a draft).
#[must_use]
pub fn is_editable(status: &str) -> bool {
    status == "draft"
}

/// Whether it is still open (not decided against, finished or withdrawn).
#[must_use]
pub fn is_open(status: &str) -> bool {
    matches!(status, "draft" | "submitted" | "under_review" | "approved")
}

/// The content of a change request, as typed.
#[derive(Debug, Clone)]
pub struct Draft<'a> {
    /// Short title.
    pub title: &'a str,
    /// What is to change.
    pub description: &'a str,
    /// Why.
    pub reason: &'a str,
    /// One of [`KINDS`].
    pub kind: &'a str,
    /// Business days the finish moves (negative pulls it in).
    pub impact_schedule_days: Option<i64>,
    /// Cost change in minor units.
    pub impact_cost_minor: Option<i64>,
    /// Its currency (three capital letters); required with a cost, and only then.
    pub cost_currency: Option<&'a str>,
    /// A written summary of the impact.
    pub impact_summary: Option<&'a str>,
}

/// Everything wrong with a draft, in plain words; empty means it can be saved.
#[must_use]
pub fn problems(d: &Draft<'_>) -> Vec<String> {
    let mut out = Vec::new();
    if d.title.trim().is_empty() {
        out.push("the title is required".to_string());
    } else if d.title.trim().chars().count() > TITLE_MAX {
        out.push(format!("the title is longer than {TITLE_MAX} characters"));
    }
    for (name, text) in [("description", d.description), ("reason", d.reason)] {
        if text.trim().is_empty() {
            out.push(format!("the {name} is required"));
        } else if text.chars().count() > TEXT_MAX {
            out.push(format!("the {name} is longer than {TEXT_MAX} characters"));
        }
    }
    if d.impact_summary
        .is_some_and(|s| s.chars().count() > TEXT_MAX)
    {
        out.push(format!(
            "the impact summary is longer than {TEXT_MAX} characters"
        ));
    }
    if !KINDS.contains(&d.kind) {
        out.push(format!("kind must be one of {KINDS:?}"));
    }
    if d.impact_schedule_days
        .is_some_and(|n| !(-3650..=3650).contains(&n))
    {
        out.push("the schedule impact must be from -3650 to 3650 business days".to_string());
    }
    match (d.impact_cost_minor, d.cost_currency) {
        (Some(_), None) => out.push("a cost impact needs its currency".to_string()),
        (None, Some(_)) => out.push("a currency needs a cost impact".to_string()),
        (Some(_), Some(c)) if !(c.len() == 3 && c.bytes().all(|b| b.is_ascii_uppercase())) => {
            out.push("the currency must be a 3-letter code such as GBP".to_string());
        }
        _ => {}
    }
    out
}

/// What is wrong with a decision reason, if anything.
#[must_use]
pub fn decision_problem(reason: &str) -> Option<String> {
    (reason.trim().chars().count() < MIN_DECISION_REASON)
        .then(|| format!("a decision needs a reason of at least {MIN_DECISION_REASON} characters"))
}

/// The person behind a `person:` or `worker:` reference, or a bare subject: the identifier.
#[must_use]
pub fn identifier(reference: &str) -> &str {
    reference
        .trim()
        .strip_prefix("person:")
        .or_else(|| reference.trim().strip_prefix("worker:"))
        .unwrap_or_else(|| reference.trim())
        .trim()
}

/// Whether `actor` (a token subject) is the person `reference` names.
#[must_use]
pub fn is_person(reference: &str, actor: Option<&str>) -> bool {
    let id = identifier(reference);
    !id.is_empty() && actor.is_some_and(|a| a.trim().eq_ignore_ascii_case(id))
}

/// What is wrong with naming `approver` for a request made by `requester`.
#[must_use]
pub fn approver_problem(approver: &str, requester: &str) -> Option<String> {
    if !crate::issues::is_person_ref(approver) {
        return Some("approver_ref must be a person: or worker: reference".to_string());
    }
    if identifier(approver).eq_ignore_ascii_case(identifier(requester)) {
        return Some("the approver cannot be the person who asked for the change".to_string());
    }
    None
}

/// The days between the request and now, in calendar days; never negative.
#[must_use]
pub fn age_calendar_days(requested_on: NaiveDate, today: NaiveDate) -> i64 {
    (today - requested_on).num_days().max(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn draft() -> Draft<'static> {
        Draft {
            title: "Move go-live",
            description: "Two weeks later",
            reason: "Supplier slipped",
            kind: "schedule",
            impact_schedule_days: Some(10),
            impact_cost_minor: Some(150_000),
            cost_currency: Some("GBP"),
            impact_summary: None,
        }
    }

    #[test]
    fn the_whole_state_machine_is_pinned() {
        use Action::*;
        let all = [Submit, Review, Approve, Reject, Implement, Withdraw];
        let mut allowed = Vec::new();
        for s in STATUSES {
            for a in all {
                if let Some(to) = next(s, a) {
                    allowed.push(format!("{s} --{a:?}--> {to}"));
                }
            }
        }
        assert_eq!(
            allowed,
            [
                "draft --Submit--> submitted",
                "draft --Withdraw--> withdrawn",
                "submitted --Review--> under_review",
                "submitted --Approve--> approved",
                "submitted --Reject--> rejected",
                "submitted --Withdraw--> withdrawn",
                "under_review --Approve--> approved",
                "under_review --Reject--> rejected",
                "under_review --Withdraw--> withdrawn",
                "approved --Implement--> implemented",
                "approved --Withdraw--> withdrawn",
            ]
        );
    }

    #[test]
    fn nothing_leaves_a_final_status() {
        for s in ["rejected", "implemented", "withdrawn"] {
            for a in [
                Action::Submit,
                Action::Review,
                Action::Approve,
                Action::Reject,
                Action::Implement,
                Action::Withdraw,
            ] {
                assert_eq!(next(s, a), None, "{s}");
            }
            assert!(!is_open(s));
        }
        assert!(is_open("approved"));
        assert!(is_editable("draft") && !is_editable("submitted"));
    }

    #[test]
    fn a_good_draft_has_no_problems_and_each_fault_is_named() {
        assert!(problems(&draft()).is_empty());
        let mut d = draft();
        d.title = "  ";
        d.description = "";
        d.reason = " ";
        d.kind = "huge";
        d.impact_schedule_days = Some(4000);
        assert_eq!(problems(&d).len(), 5);
        let mut d = draft();
        d.cost_currency = None;
        assert_eq!(problems(&d), ["a cost impact needs its currency"]);
        let mut d = draft();
        d.impact_cost_minor = None;
        assert_eq!(problems(&d), ["a currency needs a cost impact"]);
        let mut d = draft();
        d.cost_currency = Some("gbp");
        assert_eq!(problems(&d).len(), 1);
        let mut d = draft();
        d.impact_cost_minor = None;
        d.cost_currency = None;
        assert!(
            problems(&d).is_empty(),
            "a request with no cost impact is fine"
        );
    }

    #[test]
    fn a_decision_needs_a_real_reason() {
        assert!(decision_problem("ok").is_some());
        assert!(decision_problem("          ").is_some());
        assert!(decision_problem("Within contingency; PMO agrees").is_none());
    }

    #[test]
    fn the_approver_is_a_person_other_than_the_requester_in_any_form() {
        assert!(approver_problem("person:b", "a").is_none());
        assert!(
            approver_problem("b", "a").is_some(),
            "not a person reference"
        );
        for me in ["person:A", "worker:a", "person: a "] {
            assert!(approver_problem(me, "a").is_some(), "{me}");
        }
        assert!(is_person("person:abc", Some("ABC")));
        assert!(!is_person("person:abc", Some("abd")));
        assert!(!is_person("person:abc", None));
        assert!(
            !is_person("person:", Some("")),
            "an empty reference names no one"
        );
    }

    #[test]
    fn age_is_calendar_days_and_never_negative() {
        let d = |m, day| NaiveDate::from_ymd_opt(2026, m, day).unwrap();
        assert_eq!(age_calendar_days(d(10, 1), d(10, 11)), 10);
        assert_eq!(age_calendar_days(d(10, 11), d(10, 1)), 0);
    }
}
