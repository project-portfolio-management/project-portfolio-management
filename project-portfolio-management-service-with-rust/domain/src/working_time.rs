//! Working time that is not a plan's own calendar (WW-1.4): the length of a working day and the working week used
//! for a plan that has no calendar, both per deployment. DB-free, unit-tested.
//!
//! A plan's calendar (`calendars.rs`) always wins: its weekday mask and its holidays decide which days work. These
//! settings fill in only what a calendar does not say. The defaults (480 minutes, Monday to Friday) are the common
//! office pattern, **stated as defaults, not as a rule**: a deployment where the week is Sunday to Thursday, or Monday
//! to Saturday, or the day is seven and a half hours, sets them.

use crate::calendars::parse_weekdays;

/// Minutes in a working day when nothing else is set.
pub const DEFAULT_MINUTES_PER_DAY: i64 = 480;
/// The working week of a plan with no calendar, Monday first, when nothing else is set.
pub const DEFAULT_WORKING_WEEK: &str = "1111100";
/// The variable that sets the minutes in a working day (1 to 1440).
pub const MINUTES_PER_DAY_VAR: &str = "PROJECT_PORTFOLIO_MANAGEMENT_MINUTES_PER_DAY";
/// The variable that sets the working week of a plan with no calendar (seven `0`/`1`, Monday first).
pub const WORKING_WEEK_VAR: &str = "PROJECT_PORTFOLIO_MANAGEMENT_DEFAULT_WORKING_WEEK";

/// Minutes in a working day from a setting's text: a whole number from 1 to 1440, else the default.
#[must_use]
pub fn parse_minutes_per_day(raw: Option<&str>) -> i64 {
    raw.and_then(|s| s.trim().parse::<i64>().ok())
        .filter(|m| (1..=1440).contains(m))
        .unwrap_or(DEFAULT_MINUTES_PER_DAY)
}

/// A working week from a setting's text: a valid weekday mask, else the default.
#[must_use]
pub fn parse_working_week(raw: Option<&str>) -> String {
    raw.map(str::trim)
        .filter(|s| parse_weekdays(s).is_ok())
        .map_or_else(|| DEFAULT_WORKING_WEEK.to_string(), str::to_string)
}

/// The deployment's minutes per working day.
#[must_use]
pub fn minutes_per_day() -> i64 {
    parse_minutes_per_day(std::env::var(MINUTES_PER_DAY_VAR).ok().as_deref())
}

/// The deployment's working week for a plan with no calendar.
#[must_use]
pub fn working_week() -> String {
    parse_working_week(std::env::var(WORKING_WEEK_VAR).ok().as_deref())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_day_length_is_a_setting_with_a_stated_default() {
        assert_eq!(parse_minutes_per_day(None), 480);
        assert_eq!(parse_minutes_per_day(Some("450")), 450);
        assert_eq!(parse_minutes_per_day(Some(" 420 ")), 420);
        for bad in ["0", "-5", "1441", "eight", ""] {
            assert_eq!(parse_minutes_per_day(Some(bad)), 480, "{bad}");
        }
    }

    #[test]
    fn the_week_is_a_setting_and_any_valid_mask_is_taken() {
        assert_eq!(parse_working_week(None), "1111100");
        // Sunday to Thursday, Monday first: Mon-Thu work, Fri and Sat off, Sun works.
        assert_eq!(parse_working_week(Some("1111001")), "1111001");
        assert_eq!(parse_working_week(Some("1111110")), "1111110");
        for bad in ["0000000", "11111", "abcdefg"] {
            assert_eq!(parse_working_week(Some(bad)), "1111100", "{bad}");
        }
    }
}
