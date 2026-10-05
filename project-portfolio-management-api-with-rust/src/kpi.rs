//! KPIs: the pure rules. DB-free, unit-tested.
//!
//! A KPI is a **standing measure** with a target and a red line. Its status is
//! derived from the latest reading and never stored, so a corrected reading
//! corrects every figure that rests on it:
//!
//! * **green** — at or better than the target;
//! * **red** — at or worse than the red line;
//! * **amber** — between the two;
//! * **no data** — no reading yet. That is *not* red, and not green: a measure
//!   nobody has taken is unknown.
//!
//! "Better" depends on the direction. A KPI past its cadence (no reading within
//! twice the interval) is also flagged **stale**: its status is still the last
//! one measured, but it says how old it is.

use chrono::NaiveDate;
use serde::Serialize;

/// Which way is good.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    /// A larger value is better (sales, uptime).
    HigherBetter,
    /// A smaller value is better (defects, cost per unit).
    LowerBetter,
}

impl Direction {
    /// Parse `higher_better` / `lower_better`.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "higher_better" => Some(Self::HigherBetter),
            "lower_better" => Some(Self::LowerBetter),
            _ => None,
        }
    }
}

/// The derived status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    /// At or better than the target.
    Green,
    /// Between the target and the red line.
    Amber,
    /// At or worse than the red line.
    Red,
    /// No reading yet.
    NoData,
}

/// Why a KPI definition is refused, or `None` when it is sound: the red line
/// must lie on the **worse** side of the target, strictly, or there would be no
/// amber band and the thresholds would contradict the direction.
#[must_use]
pub fn threshold_problem(direction: Direction, target: i64, red: i64) -> Option<&'static str> {
    match direction {
        Direction::HigherBetter if red >= target => {
            Some("for a higher-is-better KPI the red line must be below the target")
        }
        Direction::LowerBetter if red <= target => {
            Some("for a lower-is-better KPI the red line must be above the target")
        }
        _ => None,
    }
}

/// The status of one reading against the thresholds.
#[must_use]
pub fn status(direction: Direction, target: i64, red: i64, value: i64) -> Status {
    match direction {
        Direction::HigherBetter => {
            if value >= target {
                Status::Green
            } else if value <= red {
                Status::Red
            } else {
                Status::Amber
            }
        }
        Direction::LowerBetter => {
            if value <= target {
                Status::Green
            } else if value >= red {
                Status::Red
            } else {
                Status::Amber
            }
        }
    }
}

/// The status given the latest reading, if there is one.
#[must_use]
pub fn status_of(direction: Direction, target: i64, red: i64, latest: Option<i64>) -> Status {
    latest.map_or(Status::NoData, |v| status(direction, target, red, v))
}

/// Which way the last two readings moved, direction-aware.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Trend {
    /// Moved toward better.
    Improving,
    /// Moved toward worse.
    Worsening,
    /// Did not change.
    Flat,
}

/// The trend from the previous reading to the latest; `None` with fewer than two.
#[must_use]
pub fn trend(direction: Direction, previous: Option<i64>, latest: Option<i64>) -> Option<Trend> {
    let (p, l) = (previous?, latest?);
    Some(match (l.cmp(&p), direction) {
        (std::cmp::Ordering::Equal, _) => Trend::Flat,
        (std::cmp::Ordering::Greater, Direction::HigherBetter)
        | (std::cmp::Ordering::Less, Direction::LowerBetter) => Trend::Improving,
        _ => Trend::Worsening,
    })
}

/// Days in a cadence interval.
#[must_use]
pub fn cadence_days(cadence: &str) -> Option<i64> {
    match cadence {
        "daily" => Some(1),
        "weekly" => Some(7),
        "monthly" => Some(31),
        "quarterly" => Some(92),
        _ => None,
    }
}

/// Whether a KPI is stale: it has a cadence and its latest reading is older than
/// twice the interval (or it has never been read). `None` when it has no cadence.
#[must_use]
pub fn stale(cadence: Option<&str>, latest: Option<NaiveDate>, as_of: NaiveDate) -> Option<bool> {
    let days = cadence.and_then(cadence_days)?;
    Some(latest.is_none_or(|l| (as_of - l).num_days() > 2 * days))
}

/// Counts of KPIs by status.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
pub struct Summary {
    /// Green.
    pub green: usize,
    /// Amber.
    pub amber: usize,
    /// Red.
    pub red: usize,
    /// No reading yet.
    pub no_data: usize,
}

impl Summary {
    /// Count one status.
    pub fn add(&mut self, s: Status) {
        match s {
            Status::Green => self.green += 1,
            Status::Amber => self.amber += 1,
            Status::Red => self.red += 1,
            Status::NoData => self.no_data += 1,
        }
    }

    /// All KPIs counted.
    #[must_use]
    pub fn total(&self) -> usize {
        self.green + self.amber + self.red + self.no_data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Direction::{HigherBetter, LowerBetter};

    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).unwrap()
    }

    #[test]
    fn higher_is_better_has_green_amber_and_red_bands() {
        // Target 95, red at 80.
        assert_eq!(status(HigherBetter, 95, 80, 99), Status::Green);
        assert_eq!(
            status(HigherBetter, 95, 80, 95),
            Status::Green,
            "at the target is green"
        );
        assert_eq!(status(HigherBetter, 95, 80, 94), Status::Amber);
        assert_eq!(status(HigherBetter, 95, 80, 81), Status::Amber);
        assert_eq!(
            status(HigherBetter, 95, 80, 80),
            Status::Red,
            "at the red line is red"
        );
        assert_eq!(status(HigherBetter, 95, 80, 10), Status::Red);
    }

    #[test]
    fn lower_is_better_mirrors_it() {
        // Target 2 defects, red at 10.
        assert_eq!(status(LowerBetter, 2, 10, 0), Status::Green);
        assert_eq!(status(LowerBetter, 2, 10, 2), Status::Green);
        assert_eq!(status(LowerBetter, 2, 10, 3), Status::Amber);
        assert_eq!(status(LowerBetter, 2, 10, 9), Status::Amber);
        assert_eq!(status(LowerBetter, 2, 10, 10), Status::Red);
        assert_eq!(status(LowerBetter, 2, 10, 99), Status::Red);
    }

    #[test]
    fn no_reading_is_no_data_never_red_or_green() {
        assert_eq!(status_of(HigherBetter, 95, 80, None), Status::NoData);
        assert_eq!(status_of(LowerBetter, 2, 10, None), Status::NoData);
        assert_eq!(
            status_of(HigherBetter, 95, 80, Some(0)),
            Status::Red,
            "a real zero is a reading"
        );
    }

    #[test]
    fn the_red_line_must_be_on_the_worse_side() {
        assert!(threshold_problem(HigherBetter, 95, 80).is_none());
        assert!(
            threshold_problem(HigherBetter, 95, 95).is_some(),
            "no amber band"
        );
        assert!(threshold_problem(HigherBetter, 95, 99).is_some());
        assert!(threshold_problem(LowerBetter, 2, 10).is_none());
        assert!(threshold_problem(LowerBetter, 2, 2).is_some());
        assert!(threshold_problem(LowerBetter, 2, 1).is_some());
    }

    #[test]
    fn trend_follows_the_direction() {
        assert_eq!(
            trend(HigherBetter, Some(80), Some(90)),
            Some(Trend::Improving)
        );
        assert_eq!(
            trend(HigherBetter, Some(90), Some(80)),
            Some(Trend::Worsening)
        );
        assert_eq!(trend(LowerBetter, Some(9), Some(4)), Some(Trend::Improving));
        assert_eq!(trend(LowerBetter, Some(4), Some(9)), Some(Trend::Worsening));
        assert_eq!(trend(LowerBetter, Some(4), Some(4)), Some(Trend::Flat));
        assert_eq!(
            trend(LowerBetter, None, Some(4)),
            None,
            "one reading is no trend"
        );
        assert_eq!(trend(LowerBetter, Some(4), None), None);
    }

    #[test]
    fn stale_means_no_reading_within_twice_the_cadence() {
        let today = d(2026, 10, 5);
        assert_eq!(
            stale(None, Some(d(2020, 1, 1)), today),
            None,
            "no cadence, no staleness"
        );
        assert_eq!(
            stale(Some("weekly"), Some(d(2026, 9, 25)), today),
            Some(false),
            "10 days, within 14"
        );
        assert_eq!(
            stale(Some("weekly"), Some(d(2026, 9, 21)), today),
            Some(false),
            "exactly twice the cadence is still in time"
        );
        assert_eq!(
            stale(Some("weekly"), Some(d(2026, 9, 20)), today),
            Some(true)
        );
        assert_eq!(
            stale(Some("daily"), Some(d(2026, 10, 2)), today),
            Some(true)
        );
        assert_eq!(
            stale(Some("monthly"), None, today),
            Some(true),
            "never read is stale"
        );
        assert_eq!(
            stale(Some("fortnightly"), None, today),
            None,
            "unknown cadence"
        );
    }

    #[test]
    fn the_summary_counts_every_status() {
        let mut s = Summary::default();
        for st in [
            Status::Green,
            Status::Green,
            Status::Red,
            Status::NoData,
            Status::Amber,
        ] {
            s.add(st);
        }
        assert_eq!(
            (s.green, s.amber, s.red, s.no_data, s.total()),
            (2, 1, 1, 1, 5)
        );
    }
}
