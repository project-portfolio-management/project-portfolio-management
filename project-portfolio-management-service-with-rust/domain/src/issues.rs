//! The issues register: the pure rules. An issue is something that **has
//! happened** and needs managing (a risk is something that might). It has a
//! kind, a severity and a status; **closing** one needs a resolution; and its
//! age is counted in **calendar days** since it was raised.

use chrono::{DateTime, NaiveDate, Utc};

/// How serious it is, lowest first.
pub const SEVERITIES: &[&str] = &["low", "med", "high", "critical"];

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
    /// One of [`SEVERITIES`].
    pub severity: &'a str,
    /// Whether its status needs a resolution: a terminal state of the workflow in force.
    pub needs_resolution: bool,
    /// A `person:` or `worker:` reference.
    pub owner_ref: Option<&'a str>,
    /// When it was raised.
    pub raised_on: NaiveDate,
    /// When it should be dealt with.
    pub due_date: Option<NaiveDate>,
    /// How it was resolved; needed to close it.
    pub resolution: Option<&'a str>,
    /// When it really began, as it will stand after this save.
    pub occurred_at: Option<DateTime<Utc>>,
    /// When it was noticed, as it will stand after this save.
    pub detected_at: Option<DateTime<Utc>>,
    /// Now, which neither of those may be after.
    pub now: DateTime<Utc>,
}

/// True when the value is absent or only whitespace; both count as not supplied.
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
    if !SEVERITIES.contains(&d.severity) {
        out.push(format!("severity must be one of {SEVERITIES:?}"));
    }
    if d.owner_ref.is_some_and(|o| !is_person_ref(o)) {
        out.push("the owner must be a person: or worker: reference".to_string());
    }
    if d.due_date.is_some_and(|due| due < d.raised_on) {
        out.push("the due date cannot be before the day it was raised".to_string());
    }
    if d.needs_resolution && blank(d.resolution) {
        out.push("a closed issue needs a resolution".to_string());
    }
    for (name, at) in [("occurred", d.occurred_at), ("detected", d.detected_at)] {
        if at.is_some_and(|t| t > d.now) {
            out.push(format!("the time it {name} cannot be in the future"));
        }
    }
    if d.occurred_at.zip(d.detected_at).is_some_and(|(o, f)| o > f) {
        out.push("it cannot have occurred after it was detected".to_string());
    }
    out
}

/// Whether the status means the issue is finished with.
#[must_use]
pub fn is_done(status_category: &str) -> bool {
    status_category == "done"
}

/// Most languages one label can be translated into.
pub const LABELS_MAX: usize = 50;
/// Longest label, in characters.
pub const LABEL_MAX: usize = 100;

/// A language code as the interface uses it: lowercase, such as `cy`, `zh-cn` or `en-gb-oxendict`.
#[must_use]
pub fn is_language_code(s: &str) -> bool {
    let mut parts = s.split('-');
    let first = parts.next().unwrap_or_default();
    (2..=3).contains(&first.len())
        && first.chars().all(|c| c.is_ascii_lowercase())
        && s.len() <= 35
        && parts.all(|p| {
            (2..=8).contains(&p.len())
                && p.chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
        })
}

/// What is wrong with a set of translated labels, in plain words; empty means it can be saved.
#[must_use]
pub fn label_problems(labels: &std::collections::BTreeMap<String, String>) -> Vec<String> {
    let mut out = Vec::new();
    if labels.len() > LABELS_MAX {
        out.push(format!("at most {LABELS_MAX} translations of a label"));
    }
    for (code, text) in labels {
        if !is_language_code(code) {
            out.push(format!("`{code}` is not a language code"));
        }
        if text.trim().is_empty() || text.trim().chars().count() > LABEL_MAX {
            out.push(format!(
                "the `{code}` label is required, at most {LABEL_MAX} characters"
            ));
        }
    }
    out
}

/// Whether a key can name an issue type: a lowercase letter, then up to 39 lowercase letters, digits or `_`.
#[must_use]
pub fn is_type_key(s: &str) -> bool {
    is_key(s, 40, false)
}

/// Whether a key can name a role or a status: as a type's, up to 64 characters, and `-` is allowed.
#[must_use]
pub fn is_role_key(s: &str) -> bool {
    is_key(s, 64, true)
}

/// A lowercase letter, then lowercase letters, digits, `_` and (if allowed) `-`, up to `max` characters.
fn is_key(s: &str, max: usize, hyphen: bool) -> bool {
    let mut chars = s.chars();
    s.len() <= max
        && chars.next().is_some_and(|c| c.is_ascii_lowercase())
        && chars.all(|c| {
            c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || (hyphen && c == '-')
        })
}

/// The day an issue was resolved: kept if it already was, today if it just became
/// resolved or closed, and none while it is open.
#[must_use]
pub fn resolved_on(
    status_category: &str,
    previously: Option<NaiveDate>,
    today: NaiveDate,
) -> Option<NaiveDate> {
    if is_done(status_category) {
        Some(previously.unwrap_or(today))
    } else {
        None
    }
}

/// Where an issue stands in its workflow, for [`times_after`].
#[derive(Debug, Clone, Copy)]
pub struct Standing<'a> {
    /// `todo`, `active`, `waiting` or `done`.
    pub category: &'a str,
    /// Whether the state is terminal.
    pub terminal: bool,
}

/// The times the service keeps on an issue. None of them is typed by a person.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Times {
    /// The first move out of `todo`.
    pub acknowledged_at: Option<DateTime<Utc>>,
    /// The move into `done`.
    pub resolved_at: Option<DateTime<Utc>>,
    /// The move into a terminal state.
    pub closed_at: Option<DateTime<Utc>>,
}

/// The times after a move, or at raising when `from` is none. A time is written by the crossing it records, so an issue
/// already past that point from before times were kept stays without one: **nothing is guessed.** A reopening clears
/// the resolved and closed times, and the acknowledgement stands.
#[must_use]
pub fn times_after(
    before: Times,
    from: Option<Standing<'_>>,
    to: Standing<'_>,
    now: DateTime<Utc>,
) -> Times {
    let crossed_ack = to.category != "todo" && from.is_none_or(|f| f.category == "todo");
    let crossed_done = is_done(to.category) && !from.is_some_and(|f| is_done(f.category));
    let crossed_terminal = to.terminal && !from.is_some_and(|f| f.terminal);
    Times {
        acknowledged_at: before.acknowledged_at.or(crossed_ack.then_some(now)),
        resolved_at: if is_done(to.category) {
            before.resolved_at.or(crossed_done.then_some(now))
        } else {
            None
        },
        closed_at: if to.terminal {
            before.closed_at.or(crossed_terminal.then_some(now))
        } else {
            None
        },
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
pub fn is_overdue(status_category: &str, due: Option<NaiveDate>, today: NaiveDate) -> bool {
    !is_done(status_category) && due.is_some_and(|d| d < today)
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

    #[test]
    fn translated_labels_need_a_language_and_some_words() {
        for good in ["cy", "zh-cn", "en-gb-oxendict", "en-001"] {
            assert!(is_language_code(good), "{good}");
        }
        for bad in ["", "CY", "c", "toolong", "zh_cn", "en-", "en-x"] {
            assert!(!is_language_code(bad), "{bad}");
        }
        let good = std::collections::BTreeMap::from([("cy".to_string(), "Risg".to_string())]);
        assert!(label_problems(&good).is_empty());
        let bad = std::collections::BTreeMap::from([("CY".to_string(), " ".to_string())]);
        assert_eq!(label_problems(&bad).len(), 2);
    }

    fn day(d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 10, d).unwrap()
    }

    fn draft() -> Draft<'static> {
        Draft {
            title: "Build server is down",
            description: None,
            severity: "high",
            needs_resolution: false,
            owner_ref: Some("person:ada"),
            raised_on: day(1),
            due_date: Some(day(5)),
            resolution: None,
            occurred_at: None,
            detected_at: None,
            now: DateTime::from_timestamp(1_800_000_000, 0).unwrap_or_default(),
        }
    }

    fn at(seconds: i64) -> DateTime<Utc> {
        DateTime::from_timestamp(1_800_000_000 + seconds, 0).unwrap_or_default()
    }

    const TODO: Standing<'static> = Standing {
        category: "todo",
        terminal: false,
    };
    const ACTIVE: Standing<'static> = Standing {
        category: "active",
        terminal: false,
    };
    const RESOLVED: Standing<'static> = Standing {
        category: "done",
        terminal: false,
    };
    const CLOSED: Standing<'static> = Standing {
        category: "done",
        terminal: true,
    };

    #[test]
    fn a_time_is_written_by_the_crossing_it_records() {
        let raised = times_after(Times::default(), None, TODO, at(0));
        assert_eq!(raised, Times::default());
        let worked = times_after(raised, Some(TODO), ACTIVE, at(1));
        assert_eq!(worked.acknowledged_at, Some(at(1)));
        let fixed = times_after(worked, Some(ACTIVE), RESOLVED, at(2));
        assert_eq!(fixed.acknowledged_at, Some(at(1)));
        assert_eq!(fixed.resolved_at, Some(at(2)));
        assert_eq!(fixed.closed_at, None);
        let closed = times_after(fixed, Some(RESOLVED), CLOSED, at(3));
        assert_eq!(closed.resolved_at, Some(at(2)));
        assert_eq!(closed.closed_at, Some(at(3)));
    }

    #[test]
    fn reopening_clears_the_end_and_keeps_the_acknowledgement() {
        let closed = Times {
            acknowledged_at: Some(at(1)),
            resolved_at: Some(at(2)),
            closed_at: Some(at(3)),
        };
        let again = times_after(closed, Some(CLOSED), ACTIVE, at(4));
        assert_eq!(
            again,
            Times {
                acknowledged_at: Some(at(1)),
                resolved_at: None,
                closed_at: None,
            }
        );
    }

    #[test]
    fn an_issue_from_before_times_is_never_given_one_by_a_guess() {
        // Resolved long ago, then closed now: the closing is real, the resolving is not known.
        let closed = times_after(Times::default(), Some(RESOLVED), CLOSED, at(1));
        assert_eq!(closed.closed_at, Some(at(1)));
        assert_eq!(closed.resolved_at, None);
        // Already being worked on: finishing it says nothing of when it was first picked up.
        let fixed = times_after(Times::default(), Some(ACTIVE), RESOLVED, at(1));
        assert_eq!(fixed.acknowledged_at, None);
    }

    #[test]
    fn an_issue_raised_past_todo_is_acknowledged_when_raised() {
        let raised = times_after(Times::default(), None, ACTIVE, at(0));
        assert_eq!(raised.acknowledged_at, Some(at(0)));
    }

    #[test]
    fn occurred_comes_before_detected_and_neither_is_in_the_future() {
        let d = Draft {
            occurred_at: Some(at(-10)),
            detected_at: Some(at(-20)),
            ..draft()
        };
        assert_eq!(problems(&d).len(), 1, "{:?}", problems(&d));
        let d = Draft {
            occurred_at: Some(at(1)),
            detected_at: Some(at(2)),
            ..draft()
        };
        assert_eq!(problems(&d).len(), 2, "{:?}", problems(&d));
        let d = Draft {
            occurred_at: Some(at(-20)),
            detected_at: Some(at(-20)),
            ..draft()
        };
        assert!(problems(&d).is_empty(), "{:?}", problems(&d));
    }

    #[test]
    fn a_good_draft_has_no_problems() {
        assert!(problems(&draft()).is_empty());
    }

    #[test]
    fn closing_needs_a_resolution() {
        let mut d = draft();
        d.needs_resolution = true;
        assert_eq!(problems(&d), vec!["a closed issue needs a resolution"]);
        d.resolution = Some("   ");
        assert_eq!(problems(&d).len(), 1);
        d.resolution = Some("Replaced the disk");
        assert!(problems(&d).is_empty());
    }

    #[test]
    fn resolved_does_not_need_one_but_gets_a_date() {
        let mut d = draft();
        d.needs_resolution = false;
        assert!(problems(&d).is_empty());
        assert_eq!(resolved_on("done", None, day(9)), Some(day(9)));
        assert_eq!(resolved_on("done", Some(day(7)), day(9)), Some(day(7)));
        assert_eq!(resolved_on("todo", Some(day(7)), day(9)), None);
    }

    #[test]
    fn unknown_words_and_bad_references_are_named() {
        let mut d = draft();
        d.severity = "huge";
        d.owner_ref = Some("ada");
        let p = problems(&d);
        assert_eq!(p.len(), 2, "{p:?}");
    }

    #[test]
    fn a_type_key_and_a_role_key_are_narrow_words() {
        assert!(is_type_key("change_freeze"));
        assert!(!is_type_key("Change"));
        assert!(!is_type_key("9lives"));
        assert!(!is_type_key("two-words"));
        assert!(!is_type_key(&"a".repeat(41)));
        assert!(is_role_key("change-manager"));
        assert!(!is_role_key("admin role"));
        assert!(!is_role_key(""));
    }

    #[test]
    fn done_is_the_category_and_never_a_name() {
        assert!(is_done("done"));
        assert!(!is_done("closed"), "a status's name means nothing here");
        assert!(!is_overdue("done", Some(day(3)), day(4)));
        assert!(is_overdue("waiting", Some(day(3)), day(4)));
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
        assert!(is_overdue("todo", Some(day(3)), day(4)));
        assert!(!is_overdue("todo", Some(day(4)), day(4)));
        assert!(!is_overdue("done", Some(day(3)), day(4)));
        assert!(!is_overdue("todo", None, day(4)));
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
