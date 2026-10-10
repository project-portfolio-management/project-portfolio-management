//! The issues register: the pure rules. An issue is something that **has
//! happened** and needs managing (a risk is something that might). It has a
//! kind, a severity and a status; **closing** one needs a resolution; and its
//! age is counted in **calendar days** since it was raised.

use chrono::NaiveDate;

/// What sort of issue it is.
pub const KINDS: &[&str] = &[
    "bug",
    "risk",
    "blocker",
    "question",
    "improvement",
    "concern",
];
/// How serious it is, lowest first.
pub const SEVERITIES: &[&str] = &["low", "med", "high", "critical"];
/// Where it is in its life.
pub const STATUSES: &[&str] = &["open", "in_progress", "resolved", "closed"];

/// Longest title, in characters.
pub const TITLE_MAX: usize = 200;
/// Longest description or resolution, in characters.
pub const TEXT_MAX: usize = 10_000;

/// What a person sends to raise or replace an issue.
#[derive(Debug, Clone)]
pub struct Draft<'a> {
    /// Short title.
    pub title: &'a str,
    /// Longer description.
    pub description: Option<&'a str>,
    /// One of [`KINDS`].
    pub kind: &'a str,
    /// One of [`SEVERITIES`].
    pub severity: &'a str,
    /// One of [`STATUSES`].
    pub status: &'a str,
    /// A `person:` or `worker:` reference.
    pub owner_ref: Option<&'a str>,
    /// When it was raised.
    pub raised_on: NaiveDate,
    /// When it should be dealt with.
    pub due_date: Option<NaiveDate>,
    /// How it was resolved; needed to close it.
    pub resolution: Option<&'a str>,
}

fn blank(s: Option<&str>) -> bool {
    s.is_none_or(|v| v.trim().is_empty())
}

/// A `person:` or `worker:` reference with something after the colon.
#[must_use]
pub fn is_person_ref(s: &str) -> bool {
    ["person:", "worker:"].iter().any(|p| {
        s.strip_prefix(p)
            .is_some_and(|rest| !rest.trim().is_empty())
    })
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
    for (name, text) in [("description", d.description), ("resolution", d.resolution)] {
        if text.is_some_and(|t| t.chars().count() > TEXT_MAX) {
            out.push(format!("the {name} is longer than {TEXT_MAX} characters"));
        }
    }
    if !KINDS.contains(&d.kind) {
        out.push(format!("kind must be one of {KINDS:?}"));
    }
    if !SEVERITIES.contains(&d.severity) {
        out.push(format!("severity must be one of {SEVERITIES:?}"));
    }
    if !STATUSES.contains(&d.status) {
        out.push(format!("status must be one of {STATUSES:?}"));
    }
    if d.owner_ref.is_some_and(|o| !is_person_ref(o)) {
        out.push("the owner must be a person: or worker: reference".to_string());
    }
    if d.due_date.is_some_and(|due| due < d.raised_on) {
        out.push("the due date cannot be before the day it was raised".to_string());
    }
    if d.status == "closed" && blank(d.resolution) {
        out.push("a closed issue needs a resolution".to_string());
    }
    out
}

/// Whether the status means the issue is finished with.
#[must_use]
pub fn is_done(status: &str) -> bool {
    matches!(status, "resolved" | "closed")
}

/// The day an issue was resolved: kept if it already was, today if it just became
/// resolved or closed, and none while it is open.
#[must_use]
pub fn resolved_on(
    status: &str,
    previously: Option<NaiveDate>,
    today: NaiveDate,
) -> Option<NaiveDate> {
    if is_done(status) {
        Some(previously.unwrap_or(today))
    } else {
        None
    }
}

/// Calendar days since the issue was raised, until it was resolved (or until
/// `today` while it is still open). Never negative.
#[must_use]
pub fn age_calendar_days(
    raised_on: NaiveDate,
    resolved_on: Option<NaiveDate>,
    today: NaiveDate,
) -> i64 {
    (resolved_on.unwrap_or(today) - raised_on).num_days().max(0)
}

/// Whether an open issue is past its due date.
#[must_use]
pub fn is_overdue(status: &str, due: Option<NaiveDate>, today: NaiveDate) -> bool {
    !is_done(status) && due.is_some_and(|d| d < today)
}

/// Rank for sorting: a higher severity first.
#[must_use]
pub fn severity_rank(severity: &str) -> usize {
    SEVERITIES.iter().position(|s| *s == severity).unwrap_or(0)
}

/// The severity of an issue raised from a risk, from its exposure (probability times impact,
/// each 1 to 5, so 1 to 25): 20 or more critical, 12 or more high, 6 or more med, else low.
#[must_use]
pub fn severity_from_exposure(exposure: i32) -> &'static str {
    match exposure {
        20.. => "critical",
        12..=19 => "high",
        6..=11 => "med",
        _ => "low",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 10, d).unwrap()
    }

    fn draft() -> Draft<'static> {
        Draft {
            title: "Build server is down",
            description: None,
            kind: "blocker",
            severity: "high",
            status: "open",
            owner_ref: Some("person:ada"),
            raised_on: day(1),
            due_date: Some(day(5)),
            resolution: None,
        }
    }

    #[test]
    fn a_good_draft_has_no_problems() {
        assert!(problems(&draft()).is_empty());
    }

    #[test]
    fn closing_needs_a_resolution() {
        let mut d = draft();
        d.status = "closed";
        assert_eq!(problems(&d), vec!["a closed issue needs a resolution"]);
        d.resolution = Some("   ");
        assert_eq!(problems(&d).len(), 1);
        d.resolution = Some("Replaced the disk");
        assert!(problems(&d).is_empty());
    }

    #[test]
    fn resolved_does_not_need_one_but_gets_a_date() {
        let mut d = draft();
        d.status = "resolved";
        assert!(problems(&d).is_empty());
        assert_eq!(resolved_on("resolved", None, day(9)), Some(day(9)));
        assert_eq!(resolved_on("closed", Some(day(7)), day(9)), Some(day(7)));
        assert_eq!(resolved_on("open", Some(day(7)), day(9)), None);
    }

    #[test]
    fn unknown_words_and_bad_references_are_named() {
        let mut d = draft();
        d.kind = "typo";
        d.severity = "huge";
        d.status = "later";
        d.owner_ref = Some("ada");
        let p = problems(&d);
        assert_eq!(p.len(), 4, "{p:?}");
    }

    #[test]
    fn the_due_date_cannot_precede_the_raised_date() {
        let mut d = draft();
        d.due_date = Some(day(1) - chrono::Days::new(1));
        assert_eq!(problems(&d).len(), 1);
    }

    #[test]
    fn age_is_calendar_days_and_stops_when_resolved() {
        assert_eq!(age_calendar_days(day(1), None, day(11)), 10);
        assert_eq!(age_calendar_days(day(1), Some(day(4)), day(11)), 3);
        assert_eq!(age_calendar_days(day(5), None, day(1)), 0);
    }

    #[test]
    fn overdue_only_while_not_done() {
        assert!(is_overdue("open", Some(day(3)), day(4)));
        assert!(!is_overdue("open", Some(day(4)), day(4)));
        assert!(!is_overdue("closed", Some(day(3)), day(4)));
        assert!(!is_overdue("open", None, day(4)));
    }

    #[test]
    fn exposure_maps_to_a_severity_at_each_boundary() {
        for (exposure, expected) in [
            (1, "low"),
            (5, "low"),
            (6, "med"),
            (11, "med"),
            (12, "high"),
            (19, "high"),
            (20, "critical"),
            (25, "critical"),
        ] {
            assert_eq!(severity_from_exposure(exposure), expected, "{exposure}");
        }
    }

    #[test]
    fn severity_sorts_critical_highest() {
        assert!(severity_rank("critical") > severity_rank("high"));
        assert!(severity_rank("low") < severity_rank("med"));
    }
}
