//! Delivery confidence, the exceptions it opens, and the portfolio map (DP-2, DP-3, DP-4): pure rules.
//!
//! The doctrine from the pack's brief: a stored assessment sits **beside** the derived RAG and is never
//! blended with it; no assessment is `not_assessed`, never green; the trend compares with the previous
//! **issued** pack; a worsening to `amber_red` or `red` opens an exception whose clock runs from the
//! moment it opened.

use chrono::{DateTime, Duration, NaiveDate, Utc};

/// The five-point scale, best first.
pub const SCALE: [&str; 5] = ["green", "amber_green", "amber", "amber_red", "red"];

/// A rationale needs at least this many characters.
pub const MIN_RATIONALE: usize = 10;

/// How many elapsed hours an exception has to be completed (the §37 default).
pub const DEFAULT_WINDOW_HOURS: i64 = 48;

/// Position on the scale, 0 = best, 4 = worst.
#[must_use]
pub fn rank(value: &str) -> Option<usize> {
    SCALE.iter().position(|v| *v == value)
}

/// The derived RAG mapped onto the scale, for comparison only (it is never stored as an assessment).
#[must_use]
pub fn rank_of_rag(rag: &str) -> Option<usize> {
    match rag {
        "green" => Some(0),
        "amber" => Some(2),
        "red" => Some(4),
        _ => None,
    }
}

/// Why an assessment is not acceptable, if it is not.
#[must_use]
pub fn assessment_problems(value: &str, kind: &str, rationale: &str) -> Vec<String> {
    let mut p = Vec::new();
    if rank(value).is_none() {
        p.push(format!("value must be one of {SCALE:?}"));
    }
    if !matches!(kind, "self" | "independent") {
        p.push("kind must be self or independent".to_string());
    }
    if rationale.trim().chars().count() < MIN_RATIONALE {
        p.push(format!(
            "a rationale of at least {MIN_RATIONALE} characters is required"
        ));
    }
    p
}

/// What the map shows for a plan, from its assessments (oldest first): the latest **independent** one,
/// else the latest self one, marked as such.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shown {
    /// The value on the scale.
    pub value: String,
    /// `independent` or `self`.
    pub kind: String,
    /// When it was assessed.
    pub assessed_on: NaiveDate,
}

/// A single stored assessment, as the rules need it.
#[derive(Debug, Clone)]
pub struct Assessment {
    /// Value on the scale.
    pub value: String,
    /// `self` or `independent`.
    pub kind: String,
    /// When assessed.
    pub assessed_on: NaiveDate,
}

/// The assessment the map shows.
#[must_use]
pub fn shown(history: &[Assessment]) -> Option<Shown> {
    let pick = |kind: &str| history.iter().rev().find(|a| a.kind == kind);
    pick("independent").or_else(|| pick("self")).map(|a| Shown {
        value: a.value.clone(),
        kind: a.kind.clone(),
        assessed_on: a.assessed_on,
    })
}

/// The first day of a `YYYY-MM` period.
#[must_use]
pub fn period_start(period: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(&format!("{period}-01"), "%Y-%m-%d").ok()
}

/// An assessment older than the pack's period is stale.
#[must_use]
pub fn is_stale(assessed_on: NaiveDate, period: &str) -> bool {
    period_start(period).is_some_and(|s| assessed_on < s)
}

/// The direction against the previous issued pack: `up` is closer to green.
#[must_use]
pub fn trend(previous: Option<&str>, now: Option<&str>) -> &'static str {
    match (previous.and_then(rank), now.and_then(rank)) {
        (None, Some(_)) => "new",
        (Some(p), Some(n)) if n < p => "up",
        (Some(p), Some(n)) if n > p => "down",
        (Some(_), Some(_)) => "same",
        // Not assessed now: there is no direction to claim.
        (_, None) => "not_assessed",
    }
}

/// Whether two views are two or more steps apart on the scale.
#[must_use]
pub fn disagree(a: &str, b: usize) -> bool {
    rank(a).is_some_and(|x| x.abs_diff(b) >= 2)
}

/// Whether a value opens an exception.
#[must_use]
pub fn opens_exception(value: &str) -> bool {
    matches!(value, "amber_red" | "red")
}

/// Whether moving from `before` to `after` is a worsening.
#[must_use]
pub fn worsens(before: &str, after: &str) -> bool {
    matches!((rank(before), rank(after)), (Some(b), Some(a)) if a > b)
}

/// The exception window in hours (`PROJECT_PORTFOLIO_MANAGEMENT_EXCEPTION_WINDOW_HOURS`, 1 to 720).
#[must_use]
pub fn window_hours() -> i64 {
    std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_EXCEPTION_WINDOW_HOURS")
        .ok()
        .and_then(|v| v.trim().parse::<i64>().ok())
        .filter(|h| (1..=720).contains(h))
        .unwrap_or(DEFAULT_WINDOW_HOURS)
}

/// The four things an exception needs to be complete.
#[derive(Debug, Clone, Default)]
pub struct Parts<'a> {
    /// The cause.
    pub cause: Option<&'a str>,
    /// The decision needed.
    pub decision_needed: Option<&'a str>,
    /// A named owner (a reference).
    pub owner_ref: Option<&'a str>,
    /// The date the decision is needed by.
    pub decide_by: Option<NaiveDate>,
}

/// What is still missing, by name.
#[must_use]
pub fn missing(p: &Parts<'_>) -> Vec<&'static str> {
    let blank = |s: Option<&str>| s.is_none_or(|v| v.trim().chars().count() < 3);
    let mut m = Vec::new();
    if blank(p.cause) {
        m.push("cause");
    }
    if blank(p.decision_needed) {
        m.push("decision_needed");
    }
    if blank(p.owner_ref) {
        m.push("owner");
    }
    if p.decide_by.is_none() {
        m.push("decide_by");
    }
    m
}

/// Whether an exception is in breach: it was not completed within the window, counted from when it
/// opened. Exactly the window is still in time. A completed exception is judged by when it was
/// completed, so a late completion stays a breach on the record.
#[must_use]
pub fn breached(
    opened_at: DateTime<Utc>,
    completed_at: Option<DateTime<Utc>>,
    now: DateTime<Utc>,
    window_hours: i64,
) -> bool {
    completed_at.unwrap_or(now) - opened_at > Duration::hours(window_hours)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).unwrap()
    }
    fn a(value: &str, kind: &str, on: NaiveDate) -> Assessment {
        Assessment {
            value: value.into(),
            kind: kind.into(),
            assessed_on: on,
        }
    }

    #[test]
    fn an_assessment_names_every_fault() {
        let p = assessment_problems("purple", "boss", "x");
        assert_eq!(p.len(), 3);
        assert!(assessment_problems("amber", "self", "Two suppliers are late").is_empty());
        assert_eq!(
            assessment_problems("red", "self", "   short   ").len(),
            1,
            "padding does not count"
        );
    }

    #[test]
    fn the_map_shows_the_latest_independent_else_the_latest_self_and_says_which() {
        assert_eq!(shown(&[]), None, "no assessment is not green");
        let h = [
            a("green", "self", d(2026, 8, 1)),
            a("amber", "self", d(2026, 9, 1)),
        ];
        let s = shown(&h).unwrap();
        assert_eq!((s.value.as_str(), s.kind.as_str()), ("amber", "self"));
        let h = [
            a("red", "independent", d(2026, 7, 1)),
            a("green", "self", d(2026, 9, 1)),
        ];
        let s = shown(&h).unwrap();
        assert_eq!(
            (s.value.as_str(), s.kind.as_str()),
            ("red", "independent"),
            "never blended"
        );
    }

    #[test]
    fn stale_means_before_the_first_day_of_the_period() {
        assert!(is_stale(d(2026, 9, 30), "2026-10"));
        assert!(!is_stale(d(2026, 10, 1), "2026-10"));
        assert!(!is_stale(d(2026, 10, 31), "2026-10"));
        assert!(
            !is_stale(d(2020, 1, 1), "garbage"),
            "an unreadable period claims nothing"
        );
    }

    #[test]
    fn trend_compares_with_the_previous_issued_pack() {
        assert_eq!(trend(Some("amber"), Some("green")), "up");
        assert_eq!(trend(Some("amber"), Some("red")), "down");
        assert_eq!(trend(Some("amber"), Some("amber")), "same");
        assert_eq!(trend(None, Some("amber")), "new");
        assert_eq!(trend(Some("amber"), None), "not_assessed");
        assert_eq!(trend(None, None), "not_assessed");
    }

    #[test]
    fn a_two_step_gap_is_a_disagreement_and_one_step_is_not() {
        assert!(disagree("green", 2));
        assert!(!disagree("green", 1));
        assert!(disagree("red", rank_of_rag("amber").unwrap()));
        assert!(!disagree("amber_red", rank_of_rag("red").unwrap()));
        assert_eq!(rank_of_rag("blue"), None);
    }

    #[test]
    fn only_amber_red_and_red_open_an_exception_and_worsening_is_strict() {
        assert!(opens_exception("amber_red") && opens_exception("red"));
        assert!(!opens_exception("amber") && !opens_exception("green"));
        assert!(
            worsens("amber_red", "red") && !worsens("red", "amber_red") && !worsens("red", "red")
        );
        assert!(!worsens("nonsense", "red"));
    }

    #[test]
    fn an_exception_is_complete_only_with_all_four_parts() {
        assert_eq!(
            missing(&Parts::default()),
            ["cause", "decision_needed", "owner", "decide_by"]
        );
        let full = Parts {
            cause: Some("supplier"),
            decision_needed: Some("extend"),
            owner_ref: Some("person:x"),
            decide_by: Some(d(2026, 11, 1)),
        };
        assert!(missing(&full).is_empty());
        let blank = Parts {
            cause: Some("  "),
            ..full.clone()
        };
        assert_eq!(missing(&blank), ["cause"]);
    }

    #[test]
    fn the_clock_is_pinned_at_47h59_48h00_and_48h01() {
        let opened = Utc.with_ymd_and_hms(2026, 10, 1, 9, 0, 0).unwrap();
        let at = |mins: i64| opened + Duration::minutes(mins);
        assert!(!breached(opened, None, at(47 * 60 + 59), 48), "47h59");
        assert!(
            !breached(opened, None, at(48 * 60), 48),
            "48h00 is still in time"
        );
        assert!(breached(opened, None, at(48 * 60 + 1), 48), "48h01");
        // Completed in time stays fine however long it is looked at afterwards.
        assert!(!breached(opened, Some(at(60)), at(10_000), 48));
        // Completed late stays a breach even though it is complete.
        assert!(breached(opened, Some(at(49 * 60)), at(50 * 60), 48));
        // A shorter window is honoured.
        assert!(breached(opened, None, at(121), 2));
    }

    #[test]
    fn the_window_is_bounded() {
        assert_eq!(window_hours(), 48);
    }
}
