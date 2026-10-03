//! Scheduled report runs (T-28e): the pure cadence rules. DB-free, unit-tested.
//!
//! A schedule runs a saved report daily, weekly or monthly. Dates are **UTC
//! calendar dates** (a scheduled report is a portfolio artifact, not a personal
//! view). A run that was missed (the server was down for a week) runs **once**
//! and then moves to the next future date: it never replays every missed day.

use chrono::{Datelike, Duration, NaiveDate};

/// Cadences.
pub const CADENCES: &[&str] = &["daily", "weekly", "monthly"];
/// Formats a scheduled run may produce.
pub const FORMATS: &[&str] = &["csv", "json"];
/// Schedules one person may hold.
pub const MAX_PER_PERSON: u64 = 20;

/// The first run date strictly after `after` for `cadence`, keeping the day of
/// the month for `monthly` (clamped: a schedule that began on the 31st runs on
/// the 28th, 29th or 30th in shorter months, and returns to the 31st).
#[must_use]
pub fn next_after(cadence: &str, anchor: NaiveDate, after: NaiveDate) -> Option<NaiveDate> {
    match cadence {
        "daily" => Some(after + Duration::days(1)),
        "weekly" => Some(after + Duration::days(7)),
        "monthly" => {
            let (mut y, mut m) = (after.year(), after.month());
            loop {
                if m == 12 {
                    y += 1;
                    m = 1;
                } else {
                    m += 1;
                }
                let last = last_day(y, m)?;
                let d = NaiveDate::from_ymd_opt(y, m, anchor.day().min(last))?;
                if d > after {
                    return Some(d);
                }
            }
        }
        _ => None,
    }
}

fn last_day(y: i32, m: u32) -> Option<u32> {
    let first_next = if m == 12 {
        NaiveDate::from_ymd_opt(y + 1, 1, 1)?
    } else {
        NaiveDate::from_ymd_opt(y, m + 1, 1)?
    };
    Some((first_next - Duration::days(1)).day())
}

/// After a run on `today`: the next date to run (always in the future, so a
/// missed run is not replayed).
#[must_use]
pub fn after_run(cadence: &str, anchor: NaiveDate, today: NaiveDate) -> Option<NaiveDate> {
    next_after(cadence, anchor, today)
}

/// Whether a schedule is due on `today`.
#[must_use]
pub fn is_due(next_run: NaiveDate, enabled: bool, today: NaiveDate) -> bool {
    enabled && next_run <= today
}

/// The row count of a rendered report body, so a run can say how many rows it
/// wrote (**including zero**).
#[must_use]
pub fn rows_in(format: &str, body: &str) -> i32 {
    let n = if format == "csv" {
        body.lines().count().saturating_sub(1)
    } else {
        serde_json::from_str::<serde_json::Value>(body)
            .ok()
            .and_then(|v| {
                v["rows"]
                    .as_u64()
                    .or_else(|| v["groups"].as_array().map(|a| a.len() as u64))
            })
            .and_then(|n| usize::try_from(n).ok())
            .unwrap_or(0)
    };
    i32::try_from(n).unwrap_or(i32::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).unwrap()
    }

    #[test]
    fn daily_and_weekly_step_forward() {
        assert_eq!(
            next_after("daily", d(2026, 10, 3), d(2026, 10, 3)),
            Some(d(2026, 10, 4))
        );
        assert_eq!(
            next_after("weekly", d(2026, 10, 3), d(2026, 10, 3)),
            Some(d(2026, 10, 10))
        );
        assert_eq!(next_after("hourly", d(2026, 10, 3), d(2026, 10, 3)), None);
    }

    #[test]
    fn monthly_keeps_the_day_and_clamps_in_short_months() {
        let anchor = d(2026, 1, 31);
        let feb = next_after("monthly", anchor, d(2026, 1, 31)).unwrap();
        assert_eq!(feb, d(2026, 2, 28), "clamped");
        let mar = next_after("monthly", anchor, feb).unwrap();
        assert_eq!(mar, d(2026, 3, 31), "back to the 31st");
        assert_eq!(
            next_after("monthly", anchor, d(2026, 12, 31)),
            Some(d(2027, 1, 31)),
            "year rolls"
        );
        assert_eq!(
            next_after("monthly", d(2028, 1, 30), d(2028, 1, 30)),
            Some(d(2028, 2, 29)),
            "leap year"
        );
    }

    #[test]
    fn a_missed_run_runs_once_then_moves_to_the_future() {
        // Due on the 1st, the server was down until the 20th.
        let today = d(2026, 10, 20);
        let next = after_run("daily", d(2026, 10, 1), today).unwrap();
        assert_eq!(
            next,
            d(2026, 10, 21),
            "not October 2nd: no replay of the missed days"
        );
        assert!(is_due(d(2026, 10, 1), true, today));
        assert!(!is_due(next, true, today));
        assert!(
            !is_due(d(2026, 10, 1), false, today),
            "disabled is never due"
        );
    }

    #[test]
    fn rows_are_counted_including_zero() {
        assert_eq!(rows_in("csv", "a,b\n1,2\n3,4\n"), 2);
        assert_eq!(rows_in("csv", "a,b\n"), 0, "header only is zero rows");
        assert_eq!(rows_in("csv", ""), 0);
        assert_eq!(rows_in("json", r#"{"rows":3,"data":[]}"#), 3);
        assert_eq!(rows_in("json", r#"{"rows":0}"#), 0);
        assert_eq!(rows_in("json", "not json"), 0);
    }
}
