//! Working calendars (SC-2): the pure rules for building a [`WorkCalendar`] from
//! stored data. DB-free, unit-tested.
//!
//! Decision 2026-10-03: calendars are **per plan and per person**. A task is
//! scheduled on its plan's calendar; an assignee's own weekly pattern, holidays
//! and **recorded absences** narrow it (their work happens only where both
//! work). Lag and float stay in the plan's business days.

use chrono::{Datelike, Duration, NaiveDate};

use crate::cpm::{CalendarError, WorkCalendar};

/// Holidays one calendar may list.
pub const MAX_HOLIDAYS: usize = 400;
/// Calendar days one recorded absence may span when expanded into holidays.
pub const MAX_ABSENCE_DAYS: i64 = 3650;

/// Parse a weekday mask: seven `0` / `1` characters, Monday first. At least one
/// day must work.
///
/// # Errors
///
/// A message when the mask is malformed or has no business day.
pub fn parse_weekdays(mask: &str) -> Result<[bool; 7], String> {
    if mask.len() != 7 || !mask.bytes().all(|b| b == b'0' || b == b'1') {
        return Err(
            "weekdays must be 7 characters of 0 or 1, Monday first (e.g. 1111100)".to_string(),
        );
    }
    let mut out = [false; 7];
    for (i, b) in mask.bytes().enumerate() {
        out[i] = b == b'1';
    }
    if out.iter().any(|w| *w) {
        Ok(out)
    } else {
        Err("at least one weekday must work".to_string())
    }
}

/// Every problem with a calendar definition (all of them).
#[must_use]
pub fn problems(weekdays: &str, holidays: &[NaiveDate]) -> Vec<String> {
    let mut out = Vec::new();
    if let Err(e) = parse_weekdays(weekdays) {
        out.push(e);
    }
    if holidays.len() > MAX_HOLIDAYS {
        out.push(format!("at most {MAX_HOLIDAYS} holidays"));
    }
    let mut sorted = holidays.to_vec();
    sorted.sort_unstable();
    sorted.dedup();
    if sorted.len() != holidays.len() {
        out.push("holidays must be distinct".to_string());
    }
    let earliest = NaiveDate::from_ymd_opt(1970, 1, 5).unwrap_or(NaiveDate::MIN);
    if holidays.iter().any(|h| *h < earliest || h.year() > 2100) {
        out.push("holidays must fall between 1970-01-05 and 2100-12-31".to_string());
    }
    out
}

/// Build a calendar from a stored mask and holidays.
///
/// # Errors
///
/// [`CalendarError`] if no weekday works; a malformed mask is also refused here.
pub fn build(weekdays: &str, holidays: &[NaiveDate]) -> Result<WorkCalendar, CalendarError> {
    let days = parse_weekdays(weekdays).map_err(|_| CalendarError::NoWorkingDays)?;
    WorkCalendar::new(days, holidays.iter().copied())
}

/// Expand recorded absences (`[from, to]`, inclusive) into individual dates,
/// each capped at [`MAX_ABSENCE_DAYS`] and reversed ranges ignored.
#[must_use]
pub fn expand_absences(periods: &[(NaiveDate, NaiveDate)]) -> Vec<NaiveDate> {
    let mut out = Vec::new();
    for (from, to) in periods {
        if to < from {
            continue;
        }
        let days = (*to - *from).num_days().min(MAX_ABSENCE_DAYS);
        out.extend((0..=days).map(|i| *from + Duration::days(i)));
    }
    out.sort_unstable();
    out.dedup();
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).unwrap()
    }

    #[test]
    fn a_weekday_mask_is_seven_binary_digits_with_a_working_day() {
        assert_eq!(
            parse_weekdays("1111100"),
            Ok([true, true, true, true, true, false, false])
        );
        for bad in ["", "111110", "11111000", "1111102", "abcdefg", "0000000"] {
            assert!(parse_weekdays(bad).is_err(), "{bad:?}");
        }
    }

    #[test]
    fn all_problems_are_reported_together() {
        assert!(problems("1111100", &[d(2026, 12, 25)]).is_empty());
        assert_eq!(
            problems("0000000", &[d(2026, 12, 25), d(2026, 12, 25)]).len(),
            2,
            "no business day and duplicate holiday"
        );
        assert_eq!(problems("1111100", &[d(1960, 1, 1)]).len(), 1);
        let many: Vec<NaiveDate> = (0..=i64::try_from(MAX_HOLIDAYS).unwrap())
            .map(|i| d(2026, 1, 1) + Duration::days(i))
            .collect();
        assert_eq!(problems("1111100", &many).len(), 1, "too many");
    }

    #[test]
    fn a_calendar_is_built_from_a_mask_and_holidays() {
        let cal = build("1111100", &[d(2026, 10, 5)]).unwrap();
        assert!(!cal.is_working(d(2026, 10, 5)), "the holiday");
        assert!(!cal.is_working(d(2026, 10, 3)), "a Saturday");
        assert!(cal.is_working(d(2026, 10, 6)));
        assert!(build("0000000", &[]).is_err());
    }

    #[test]
    fn absences_expand_inclusively_and_ignore_reversed_ranges() {
        let days = expand_absences(&[
            (d(2026, 10, 5), d(2026, 10, 7)),
            (d(2026, 10, 7), d(2026, 10, 8)),
            (d(2026, 10, 20), d(2026, 10, 1)),
        ]);
        assert_eq!(
            days,
            vec![
                d(2026, 10, 5),
                d(2026, 10, 6),
                d(2026, 10, 7),
                d(2026, 10, 8)
            ]
        );
        assert!(expand_absences(&[]).is_empty());
    }
}
