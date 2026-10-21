//! Tolerances: the pure rules. DB-free and deterministic.
//!
//! A **tolerance** is how far a project may drift on one dimension before it must be referred upward. This module
//! knows the four dimensions, how a measurement is held against a limit, and what colour that is. It measures
//! nothing itself: the controller gathers the figures from the schedule, the budget, the risk register and the
//! bookings, and says plainly when it could not.
//!
//! * **time** — calendar days the forecast finish (critical path) is after the plan's `target_date`;
//! * **cost** — basis points the estimate at completion is over the approved budget baseline;
//! * **risk** — the highest score (probability × impact, 1 to 25) among the plan's open risks;
//! * **resources** — percentage points above 100 that the plan's named people are booked, at worst, in the next 90
//!   days across the plans the reader can see.
//!
//! The colour is about **how much of the limit is used**: green below `amber_from_percent` of it, amber from there
//! up to the limit, red beyond it. A measurement that is zero or favourable is green. One that could not be made
//! is `unknown`, never green. Absent evidence is not good news.

use chrono::NaiveDate;
use serde::Serialize;

/// The four dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Dimension {
    /// Slip against the plan's target date.
    Time,
    /// Slip against the latest schedule baseline.
    Schedule,
    /// Budget overrun.
    Cost,
    /// The worst open risk.
    Risk,
    /// Over-booking of the people on the plan.
    Resources,
}

impl Dimension {
    /// Every dimension, in the order they are shown.
    pub const ALL: [Self; 5] = [
        Self::Time,
        Self::Schedule,
        Self::Cost,
        Self::Risk,
        Self::Resources,
    ];

    /// The stored and URL token.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Time => "time",
            Self::Schedule => "schedule",
            Self::Cost => "cost",
            Self::Risk => "risk",
            Self::Resources => "resources",
        }
    }

    /// Parse a token.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|d| d.token() == s)
    }

    /// What the limit and the measurement are counted in.
    #[must_use]
    pub const fn unit(self) -> &'static str {
        match self {
            Self::Time | Self::Schedule => "calendar_days",
            Self::Cost => "basis_points",
            Self::Risk => "risk_score",
            Self::Resources => "percentage_points",
        }
    }

    /// The most a limit may be (and the least is [`Self::min_limit`]).
    #[must_use]
    pub const fn max_limit(self) -> i64 {
        match self {
            Self::Time | Self::Schedule => 3_650,
            Self::Cost => 100_000,
            Self::Risk => 25,
            Self::Resources => 500,
        }
    }

    /// The least a limit may be: zero tolerance is allowed for everything except risk, where a score starts at 1.
    #[must_use]
    pub const fn min_limit(self) -> i64 {
        match self {
            Self::Risk => 1,
            _ => 0,
        }
    }

    /// One sentence on what is measured and how.
    #[must_use]
    pub const fn basis(self) -> &'static str {
        match self {
            Self::Time => {
                "calendar days the critical-path forecast finish is after the plan's target_date (zero when on or ahead)"
            }
            Self::Schedule => {
                "calendar days the critical-path forecast finish is after the finish recorded in the plan's latest schedule baseline (zero when on or ahead)"
            }
            Self::Cost => {
                "basis points the estimate at completion is over the latest approved budget baseline (zero when under)"
            }
            Self::Risk => {
                "the highest probability x impact among the plan's open and mitigating risks"
            }
            Self::Resources => {
                "percentage points above 100 that the plan's named, committed people are booked at worst in the next 90 days, across the plans you can read"
            }
        }
    }
}

/// Default for how much of the limit may be used before green turns amber.
pub const DEFAULT_AMBER_FROM_PERCENT: i64 = 75;

/// The colour of one dimension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    /// Well inside the limit.
    Green,
    /// Close to the limit.
    Amber,
    /// Beyond the limit.
    Red,
    /// A limit is set but the measurement could not be made.
    Unknown,
    /// No limit is set (here or above).
    NotSet,
}

/// Every problem with a tolerance as submitted.
#[must_use]
pub fn problems(
    dimension: Dimension,
    limit: i64,
    amber_from_percent: i64,
    note: Option<&str>,
) -> Vec<String> {
    let mut out = Vec::new();
    let (lo, hi) = (dimension.min_limit(), dimension.max_limit());
    if !(lo..=hi).contains(&limit) {
        out.push(format!(
            "limit for {} is {lo} to {hi} ({})",
            dimension.token(),
            dimension.unit()
        ));
    }
    if !(1..=99).contains(&amber_from_percent) {
        out.push("amber_from_percent is 1 to 99".to_string());
    }
    if note.is_some_and(|n| n.chars().count() > 1_000) {
        out.push("note is longer than 1000 characters".to_string());
    }
    out
}

/// Hold a measurement against a limit. Returns the colour and how much of the limit is used (percent, rounded
/// down), which is `None` when there is no measurement or the limit is zero.
///
/// A measurement of zero or less is green. With a limit of zero any positive measurement is red.
#[must_use]
pub fn classify(
    measured: Option<i64>,
    limit: i64,
    amber_from_percent: i64,
) -> (Status, Option<i64>) {
    let Some(m) = measured else {
        return (Status::Unknown, None);
    };
    if m <= 0 {
        return (Status::Green, Some(0));
    }
    if limit <= 0 {
        return (Status::Red, None);
    }
    let used = i64::try_from(i128::from(m) * 100 / i128::from(limit)).unwrap_or(i64::MAX);
    let status = if m > limit {
        Status::Red
    } else if used >= amber_from_percent {
        Status::Amber
    } else {
        Status::Green
    };
    (status, Some(used))
}

/// The colour for the plan as a whole: red if any dimension is red, else amber, else unknown (a limit with no
/// measurement is never hidden behind greens), else green; no limits anywhere is `NotSet`.
#[must_use]
pub fn overall(statuses: &[Status]) -> Status {
    let set: Vec<Status> = statuses
        .iter()
        .copied()
        .filter(|s| *s != Status::NotSet)
        .collect();
    if set.is_empty() {
        return Status::NotSet;
    }
    for worst in [Status::Red, Status::Amber, Status::Unknown] {
        if set.contains(&worst) {
            return worst;
        }
    }
    Status::Green
}

/// The risk score of one risk (probability and impact are each 1 to 5).
#[must_use]
pub const fn risk_score(probability: i32, impact: i32) -> i32 {
    probability * impact
}

/// Calendar days `finish` is after `target`, or zero when on or ahead.
#[must_use]
pub fn days_late(finish: NaiveDate, target: NaiveDate) -> i64 {
    (finish - target).num_days().max(0)
}

/// Basis points `forecast` is over `baseline`, or zero when under; `None` for a baseline of zero or less, which has no
/// meaningful percentage.
#[must_use]
pub fn overrun_bps(forecast: i64, baseline: i64) -> Option<i64> {
    if baseline <= 0 {
        return None;
    }
    let over = i128::from(forecast) - i128::from(baseline);
    if over <= 0 {
        return Some(0);
    }
    Some(i64::try_from(over * 10_000 / i128::from(baseline)).unwrap_or(i64::MAX))
}

/// One booking: a percentage of a person over a span, each end open when absent.
pub type Booking = (Option<NaiveDate>, Option<NaiveDate>, i32);

/// The worst over-booking, in percentage points above 100, on any day from `from` to `to` (inclusive) for one
/// person's `bookings`; zero when never over 100.
#[must_use]
pub fn peak_overbooking(bookings: &[Booking], from: NaiveDate, to: NaiveDate) -> i64 {
    if to < from {
        return 0;
    }
    // Sweep the days on which the total can change: the window start and each booking's first day, and the day
    // after each booking's last day.
    let mut edges: Vec<NaiveDate> = vec![from];
    for (s, e, _) in bookings {
        if let Some(s) = s.filter(|s| *s > from && *s <= to) {
            edges.push(s);
        }
        if let Some(day_after) = e
            .and_then(|e| e.succ_opt())
            .filter(|d| *d > from && *d <= to)
        {
            edges.push(day_after);
        }
    }
    edges.sort_unstable();
    edges.dedup();
    edges
        .into_iter()
        .map(|day| {
            let total: i64 = bookings
                .iter()
                .filter(|(s, e, _)| s.is_none_or(|s| s <= day) && e.is_none_or(|e| e >= day))
                .map(|(_, _, p)| i64::from(*p))
                .sum();
            (total - 100).max(0)
        })
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).expect("date")
    }

    #[test]
    fn a_measurement_is_green_below_the_amber_line_amber_up_to_the_limit_and_red_beyond() {
        assert_eq!(classify(Some(0), 10, 75), (Status::Green, Some(0)));
        assert_eq!(
            classify(Some(-5), 10, 75),
            (Status::Green, Some(0)),
            "favourable"
        );
        assert_eq!(classify(Some(7), 10, 75), (Status::Green, Some(70)));
        assert_eq!(classify(Some(8), 10, 75), (Status::Amber, Some(80)));
        assert_eq!(
            classify(Some(10), 10, 75),
            (Status::Amber, Some(100)),
            "exactly at the limit is still inside it"
        );
        assert_eq!(classify(Some(11), 10, 75), (Status::Red, Some(110)));
        assert_eq!(
            classify(None, 10, 75),
            (Status::Unknown, None),
            "never green without evidence"
        );
        assert_eq!(
            classify(Some(1), 0, 75),
            (Status::Red, None),
            "zero tolerance, any slip is red"
        );
        assert_eq!(classify(Some(0), 0, 75), (Status::Green, Some(0)));
        assert_eq!(
            classify(Some(i64::MAX), 1, 75).0,
            Status::Red,
            "no overflow"
        );
    }

    #[test]
    fn the_plan_takes_its_worst_colour_and_an_unknown_is_not_hidden_by_greens() {
        use Status::{Amber, Green, NotSet, Red, Unknown};
        assert_eq!(overall(&[Green, Amber, Red]), Red);
        assert_eq!(overall(&[Green, Amber, Unknown]), Amber);
        assert_eq!(overall(&[Green, Unknown, NotSet]), Unknown);
        assert_eq!(overall(&[Green, Green, NotSet]), Green);
        assert_eq!(overall(&[NotSet, NotSet]), NotSet);
        assert_eq!(overall(&[]), NotSet);
    }

    #[test]
    fn limits_are_bounded_per_dimension() {
        assert!(
            problems(Dimension::Time, 0, 75, None).is_empty(),
            "zero tolerance on time is allowed"
        );
        assert!(!problems(Dimension::Time, 3651, 75, None).is_empty());
        assert!(
            !problems(Dimension::Risk, 0, 75, None).is_empty(),
            "a risk score starts at 1"
        );
        assert!(problems(Dimension::Risk, 25, 75, None).is_empty());
        assert!(!problems(Dimension::Cost, -1, 75, None).is_empty());
        assert_eq!(
            problems(Dimension::Resources, 501, 0, Some(&"n".repeat(1001))).len(),
            3
        );
        for d in Dimension::ALL {
            assert_eq!(Dimension::parse(d.token()), Some(d));
        }
        assert_eq!(Dimension::parse("scope"), None);
    }

    #[test]
    fn slip_and_overrun_are_never_negative() {
        assert_eq!(days_late(d(2026, 11, 10), d(2026, 11, 1)), 9);
        assert_eq!(days_late(d(2026, 10, 1), d(2026, 11, 1)), 0);
        assert_eq!(overrun_bps(110, 100), Some(1_000));
        assert_eq!(overrun_bps(90, 100), Some(0));
        assert_eq!(overrun_bps(5, 0), None, "no percentage of nothing");
        assert_eq!(risk_score(4, 5), 20);
    }

    #[test]
    fn overbooking_is_the_worst_day_not_the_average() {
        let (from, to) = (d(2026, 11, 1), d(2026, 11, 30));
        assert_eq!(
            peak_overbooking(&[(None, None, 60), (None, None, 40)], from, to),
            0,
            "exactly full"
        );
        assert_eq!(
            peak_overbooking(&[(None, None, 80), (None, None, 50)], from, to),
            30
        );
        // The two bookings overlap only on 10 and 11 November.
        let overlap = [
            (Some(d(2026, 11, 1)), Some(d(2026, 11, 11)), 70),
            (Some(d(2026, 11, 10)), Some(d(2026, 11, 20)), 70),
        ];
        assert_eq!(peak_overbooking(&overlap, from, to), 40);
        let apart = [
            (Some(d(2026, 11, 1)), Some(d(2026, 11, 9)), 70),
            (Some(d(2026, 11, 10)), Some(d(2026, 11, 20)), 70),
        ];
        assert_eq!(
            peak_overbooking(&apart, from, to),
            0,
            "back to back is not overlap"
        );
        // A booking that ends before the window, or starts after it, does not count.
        let outside = [
            (Some(d(2026, 10, 1)), Some(d(2026, 10, 31)), 100),
            (Some(d(2026, 12, 1)), None, 100),
            (None, None, 90),
        ];
        assert_eq!(peak_overbooking(&outside, from, to), 0);
        assert_eq!(peak_overbooking(&[], from, to), 0);
        assert_eq!(
            peak_overbooking(&[(None, None, 200)], to, from),
            0,
            "an empty window"
        );
    }
}
