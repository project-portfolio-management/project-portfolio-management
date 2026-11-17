//! Status updates on a cadence (NC-4): the rules. A plan says how often it expects an update; each update is an
//! on-track / at-risk / off-track call with a narrative; when the next one is due, and whether it is overdue, follows
//! from the dates alone. DB-free and clock-free: every function is told the date.

use chrono::{Duration, NaiveDate};
use serde::Serialize;

/// The calls an update can make.
pub const STATUSES: &[&str] = &["on_track", "at_risk", "off_track"];

/// The shortest and longest cadence, in days.
pub const MIN_CADENCE_DAYS: i32 = 1;
/// The longest cadence, in days (about a quarter).
pub const MAX_CADENCE_DAYS: i32 = 92;

/// The longest narrative, in characters.
pub const MAX_NARRATIVE: usize = 4000;

/// Problems with a new update (empty when fine).
#[must_use]
pub fn update_problems(status: &str, narrative: &str) -> Vec<String> {
    let mut out = Vec::new();
    if !STATUSES.contains(&status) {
        out.push(format!("status is one of {}", STATUSES.join(", ")));
    }
    if narrative.trim().is_empty() || narrative.chars().count() > MAX_NARRATIVE {
        out.push(format!(
            "narrative is required (up to {MAX_NARRATIVE} characters)"
        ));
    }
    out
}

/// Whether `days` is an allowed cadence.
#[must_use]
pub fn cadence_ok(days: i32) -> bool {
    (MIN_CADENCE_DAYS..=MAX_CADENCE_DAYS).contains(&days)
}

/// Where a plan stands against its cadence on a given day.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Standing {
    /// The date the next update is due: a cadence after the last update, or after the cadence was set when there
    /// has been none.
    pub due_on: NaiveDate,
    /// Today is the due date or later.
    pub due: bool,
    /// Today is after the due date.
    pub overdue: bool,
    /// Days past the due date (0 when not overdue).
    pub days_overdue: i64,
}

/// Where a plan stands on `today`. `last_update_on` is the date of its latest update (if any) and `cadence_set_on`
/// the date its cadence was set; the clock starts from whichever is later, so setting a new cadence restarts it.
#[must_use]
pub fn standing(
    cadence_days: i32,
    cadence_set_on: NaiveDate,
    last_update_on: Option<NaiveDate>,
    today: NaiveDate,
) -> Standing {
    let from = last_update_on.map_or(cadence_set_on, |u| u.max(cadence_set_on));
    let due_on = from + Duration::days(i64::from(cadence_days));
    let late = (today - due_on).num_days();
    Standing {
        due_on,
        due: today >= due_on,
        overdue: today > due_on,
        days_overdue: late.max(0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, m, day).unwrap()
    }

    #[test]
    fn an_update_needs_a_known_status_and_a_narrative() {
        assert!(update_problems("on_track", "All well").is_empty());
        assert_eq!(update_problems("green", "x").len(), 1);
        assert_eq!(update_problems("at_risk", "  ").len(), 1);
        assert_eq!(update_problems("", "").len(), 2);
        assert_eq!(
            update_problems("at_risk", &"x".repeat(MAX_NARRATIVE + 1)).len(),
            1
        );
    }

    #[test]
    fn a_cadence_is_one_day_to_a_quarter() {
        assert!(
            !cadence_ok(0) && cadence_ok(1) && cadence_ok(7) && cadence_ok(92) && !cadence_ok(93)
        );
    }

    #[test]
    fn with_no_update_the_clock_runs_from_when_the_cadence_was_set() {
        let s = standing(7, d(10, 1), None, d(10, 7));
        assert_eq!(s.due_on, d(10, 8));
        assert!(!s.due && !s.overdue);
        let s = standing(7, d(10, 1), None, d(10, 8));
        assert!(s.due && !s.overdue && s.days_overdue == 0);
    }

    #[test]
    fn an_update_restarts_the_clock_and_lateness_is_counted_in_days() {
        let s = standing(7, d(10, 1), Some(d(10, 10)), d(10, 20));
        assert_eq!(s.due_on, d(10, 17));
        assert!(s.due && s.overdue);
        assert_eq!(s.days_overdue, 3);
    }

    #[test]
    fn setting_a_new_cadence_restarts_the_clock_even_after_an_old_update() {
        let s = standing(14, d(10, 20), Some(d(10, 1)), d(10, 25));
        assert_eq!(s.due_on, d(11, 3));
        assert!(!s.due);
    }
}
