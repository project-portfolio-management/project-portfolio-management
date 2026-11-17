//! Time-phased work (PJ-5): spread a task's work over its business days by a
//! **contour**, and bucket the result into days, weeks or months. DB-free,
//! deterministic, integer minutes.
//!
//! * The spread is **exact**: the minutes of every day sum to the task's work
//!   with no rounding drift (largest remainder; ties go to the earlier day).
//! * It is **calendar-aware**: work lands only on the days the caller says are
//!   business days (the task's own calendar), never on a weekend or holiday.
//! * Buckets are derived, not stored, so a changed contour, calendar or date
//!   corrects every figure that rests on it.
//!
//! The contour shapes are the usual scheduling-tool set. **The shape names were
//! inferred from the reference tool's class names and the definitions below are
//! this project's own — confirm them before treating them as a standard.**
//! With `d` business days (day index `i` from 0):
//!
//! * `flat` — every day the same;
//! * `front_loaded` — weights `d, d-1, … 1` (most work first);
//! * `back_loaded` — weights `1, 2, … d` (most work last);
//! * `bell` — weights `min(i+1, d-i)`: rising to the middle and falling again;
//! * `turtle` — weights `min(i+1, q, d-i)` with `q = max(1, d/4)`: a ramp up over
//!   about a quarter, a plateau, a ramp down over about a quarter;
//! * `custom` — percentages summing to 100, stretched evenly over the business days.

use chrono::{Duration, NaiveDate};
use serde::Serialize;

use crate::portfolio_resources::{self as pr, Period};

/// Most custom weights.
pub const MAX_CUSTOM: usize = 40;

/// A contour.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Contour {
    /// Even.
    Flat,
    /// Most first.
    FrontLoaded,
    /// Most last.
    BackLoaded,
    /// Peak in the middle.
    Bell,
    /// Ramp, plateau, ramp.
    Turtle,
    /// Percentages summing to 100.
    Custom(Vec<i64>),
}

impl Contour {
    /// The stored token.
    #[must_use]
    pub fn token(&self) -> &'static str {
        match self {
            Self::Flat => "flat",
            Self::FrontLoaded => "front_loaded",
            Self::BackLoaded => "back_loaded",
            Self::Bell => "bell",
            Self::Turtle => "turtle",
            Self::Custom(_) => "custom",
        }
    }
}

/// Parse a contour token (and the custom weights when it is `custom`).
///
/// # Errors
///
/// A sentence when the token is unknown or the custom weights are not 1–40
/// whole percentages of 0–100 that sum to exactly 100.
pub fn parse(token: &str, custom: Option<&[i64]>) -> Result<Contour, String> {
    match token {
        "flat" => Ok(Contour::Flat),
        "front_loaded" => Ok(Contour::FrontLoaded),
        "back_loaded" => Ok(Contour::BackLoaded),
        "bell" => Ok(Contour::Bell),
        "turtle" => Ok(Contour::Turtle),
        "custom" => {
            let w = custom.ok_or("a custom contour needs its weights")?;
            if w.is_empty() || w.len() > MAX_CUSTOM {
                return Err(format!("a custom contour has 1 to {MAX_CUSTOM} weights"));
            }
            if w.iter().any(|x| !(0..=100).contains(x)) {
                return Err("each custom weight is a percentage from 0 to 100".to_string());
            }
            let sum: i64 = w.iter().sum();
            if sum != 100 {
                return Err(format!(
                    "custom weights must sum to 100%, they sum to {sum}%"
                ));
            }
            Ok(Contour::Custom(w.to_vec()))
        }
        _ => Err(
            "contour must be flat, front_loaded, back_loaded, bell, turtle or custom".to_string(),
        ),
    }
}

/// The relative weight of each of `days` business days.
#[must_use]
pub fn weights(contour: &Contour, days: usize) -> Vec<i64> {
    let d = i64::try_from(days).unwrap_or(0);
    (0..d)
        .map(|i| match contour {
            Contour::Flat => 1,
            Contour::FrontLoaded => d - i,
            Contour::BackLoaded => i + 1,
            Contour::Bell => (i + 1).min(d - i),
            Contour::Turtle => (i + 1).min((d / 4).max(1)).min(d - i),
            Contour::Custom(w) => {
                let k = i64::try_from(w.len()).unwrap_or(1);
                let ix = usize::try_from(i * k / d).unwrap_or(0).min(w.len() - 1);
                w[ix]
            }
        })
        .collect()
}

/// Split `total` by `weights` so the parts sum to `total` exactly (largest
/// remainder; ties go to the earlier part). All-zero weights give all zeros.
#[must_use]
pub fn spread(total: i64, weights: &[i64]) -> Vec<i64> {
    let sum: i64 = weights.iter().sum();
    if sum <= 0 || total <= 0 {
        return vec![0; weights.len()];
    }
    let mut parts: Vec<i64> = weights.iter().map(|w| total * w / sum).collect();
    let mut rem: Vec<(i64, usize)> = weights
        .iter()
        .enumerate()
        .map(|(i, w)| (total * w % sum, i))
        .collect();
    let mut left = total - parts.iter().sum::<i64>();
    // Largest remainder first; the earlier day wins a tie.
    rem.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
    for (_, i) in rem {
        if left == 0 {
            break;
        }
        parts[i] += 1;
        left -= 1;
    }
    parts
}

/// Why work could not be placed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Unplaced {
    /// There is work but no business day to put it on.
    NoWorkingDays,
    /// The custom weights over these days are all zero.
    NoWeight,
}

/// Put `total` minutes on `working_days` (in order) by `contour`.
///
/// # Errors
///
/// [`Unplaced`] when there is work but nowhere (or no weight) to put it.
pub fn distribute(
    total: i64,
    working_days: &[NaiveDate],
    contour: &Contour,
) -> Result<Vec<(NaiveDate, i64)>, Unplaced> {
    if total == 0 {
        return Ok(working_days.iter().map(|d| (*d, 0)).collect());
    }
    if working_days.is_empty() {
        return Err(Unplaced::NoWorkingDays);
    }
    let w = weights(contour, working_days.len());
    if w.iter().sum::<i64>() <= 0 {
        return Err(Unplaced::NoWeight);
    }
    Ok(working_days
        .iter()
        .copied()
        .zip(spread(total, &w))
        .collect())
}

/// Bucket size.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grain {
    /// One day.
    Day,
    /// A Monday-to-Sunday week.
    Week,
    /// A calendar month.
    Month,
}

impl Grain {
    /// Parse `day`, `week` or `month`.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "day" => Some(Self::Day),
            "week" => Some(Self::Week),
            "month" => Some(Self::Month),
            _ => None,
        }
    }
}

/// The periods covering `[from, to]` at a grain.
///
/// # Errors
///
/// A sentence when the window is reversed, too long, or has too many periods.
pub fn periods(from: NaiveDate, to: NaiveDate, grain: Grain) -> Result<Vec<Period>, String> {
    match grain {
        Grain::Week => pr::periods(from, to, pr::Grain::Week),
        Grain::Month => pr::periods(from, to, pr::Grain::Month),
        Grain::Day => {
            if to < from {
                return Err("to is before from".to_string());
            }
            if (to - from).num_days() >= i64::try_from(pr::MAX_PERIODS).unwrap_or(120) {
                return Err(format!(
                    "a daily view spans at most {} calendar days",
                    pr::MAX_PERIODS
                ));
            }
            let mut out = Vec::new();
            let mut d = from;
            while d <= to {
                out.push(Period { from: d, to: d });
                d += Duration::days(1);
            }
            Ok(out)
        }
    }
}

/// One bucket of minutes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Bucket {
    /// First day.
    pub from: NaiveDate,
    /// Last day.
    pub to: NaiveDate,
    /// Minutes of work.
    pub minutes: i64,
}

/// Sum per-day minutes into `periods`. Minutes outside every period are not lost
/// silently: the second value is how many fell outside the window.
#[must_use]
pub fn bucket(per_day: &[(NaiveDate, i64)], periods: &[Period]) -> (Vec<Bucket>, i64) {
    let mut out: Vec<Bucket> = periods
        .iter()
        .map(|p| Bucket {
            from: p.from,
            to: p.to,
            minutes: 0,
        })
        .collect();
    let mut outside = 0;
    for (day, minutes) in per_day {
        match out.iter_mut().find(|b| b.from <= *day && *day <= b.to) {
            Some(b) => b.minutes += minutes,
            None => outside += minutes,
        }
    }
    (out, outside)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).unwrap()
    }
    fn week_days() -> Vec<NaiveDate> {
        (5..=9).map(|n| d(2026, 10, n)).collect()
    }

    #[test]
    fn the_shapes_have_the_documented_weights() {
        assert_eq!(weights(&Contour::Flat, 4), vec![1, 1, 1, 1]);
        assert_eq!(weights(&Contour::FrontLoaded, 4), vec![4, 3, 2, 1]);
        assert_eq!(weights(&Contour::BackLoaded, 4), vec![1, 2, 3, 4]);
        assert_eq!(weights(&Contour::Bell, 5), vec![1, 2, 3, 2, 1]);
        assert_eq!(weights(&Contour::Bell, 4), vec![1, 2, 2, 1]);
        assert_eq!(weights(&Contour::Bell, 1), vec![1]);
        assert_eq!(weights(&Contour::Turtle, 8), vec![1, 2, 2, 2, 2, 2, 2, 1]);
        assert_eq!(weights(&Contour::Turtle, 3), vec![1, 1, 1]);
        assert_eq!(weights(&Contour::Turtle, 1), vec![1]);
        assert!(weights(&Contour::Flat, 0).is_empty());
    }

    #[test]
    fn a_custom_contour_is_stretched_over_the_days() {
        let c = Contour::Custom(vec![50, 50]);
        assert_eq!(weights(&c, 4), vec![50, 50, 50, 50]);
        let c = Contour::Custom(vec![10, 20, 70]);
        assert_eq!(weights(&c, 6), vec![10, 10, 20, 20, 70, 70]);
        assert_eq!(weights(&c, 3), vec![10, 20, 70]);
        assert_eq!(
            weights(&c, 2),
            vec![10, 20],
            "fewer business days than weights: the first of each stretch"
        );
    }

    #[test]
    fn custom_weights_must_sum_to_exactly_one_hundred() {
        assert!(parse("custom", Some(&[40, 60])).is_ok());
        assert!(
            parse("custom", Some(&[40, 50]))
                .unwrap_err()
                .contains("sum to 100%, they sum to 90%")
        );
        assert!(
            parse("custom", Some(&[101, -1]))
                .unwrap_err()
                .contains("0 to 100")
        );
        assert!(parse("custom", Some(&[])).is_err());
        assert!(
            parse("custom", None)
                .unwrap_err()
                .contains("needs its weights")
        );
        assert!(parse("custom", Some(&vec![1; 41])).is_err());
        assert!(parse("wavy", None).is_err());
        for t in ["flat", "front_loaded", "back_loaded", "bell", "turtle"] {
            assert_eq!(parse(t, None).unwrap().token(), t);
        }
    }

    #[test]
    fn the_spread_sums_to_the_work_exactly_with_no_drift() {
        for total in [0, 1, 7, 100, 999, 2_400, 12_345, 1_000_003] {
            for c in [
                Contour::Flat,
                Contour::FrontLoaded,
                Contour::BackLoaded,
                Contour::Bell,
                Contour::Turtle,
                Contour::Custom(vec![10, 20, 70]),
            ] {
                for days in 1..=23 {
                    let parts = spread(total, &weights(&c, days));
                    assert_eq!(
                        parts.iter().sum::<i64>(),
                        total,
                        "{total} over {days} business days by {c:?}"
                    );
                    assert!(parts.iter().all(|p| *p >= 0));
                }
            }
        }
    }

    #[test]
    fn largest_remainder_gives_the_extra_minute_to_the_biggest_remainder_then_the_earlier_day() {
        // 10 over three equal business days: 3, 3, 3 and one left over goes to the earliest.
        assert_eq!(spread(10, &[1, 1, 1]), vec![4, 3, 3]);
        // 100 over 1:2:3 is 16.67, 33.33, 50: the biggest remainder (the first, .67) gets the spare.
        assert_eq!(spread(100, &[1, 2, 3]), vec![17, 33, 50]);
        assert_eq!(spread(5, &[0, 0]), vec![0, 0]);
        assert_eq!(spread(0, &[1, 1]), vec![0, 0]);
    }

    #[test]
    fn work_lands_only_on_the_working_days_given() {
        // A task across a weekend: only the five weekdays are passed in.
        let days = week_days();
        let placed = distribute(2_400, &days, &Contour::Flat).unwrap();
        assert_eq!(placed.len(), 5);
        assert!(placed.iter().all(|(_, m)| *m == 480));
        assert!(placed.iter().all(|(day, _)| !matches!(
            chrono::Datelike::weekday(day),
            chrono::Weekday::Sat | chrono::Weekday::Sun
        )));
        let front = distribute(2_400, &days, &Contour::FrontLoaded).unwrap();
        assert!(front[0].1 > front[4].1);
        assert_eq!(front.iter().map(|(_, m)| m).sum::<i64>(), 2_400);
    }

    #[test]
    fn work_with_nowhere_to_go_is_said_not_dropped() {
        assert_eq!(
            distribute(100, &[], &Contour::Flat),
            Err(Unplaced::NoWorkingDays)
        );
        assert_eq!(
            distribute(0, &[], &Contour::Flat),
            Ok(vec![]),
            "no work needs no day"
        );
        let days = week_days();
        let zero = Contour::Custom(vec![0, 100]);
        // Two business days, weights over the first stretch only: [0, 100] stretched
        // over 1 business day is [0].
        assert_eq!(distribute(50, &days[..1], &zero), Err(Unplaced::NoWeight));
    }

    #[test]
    fn buckets_sum_to_the_work_and_account_for_what_falls_outside() {
        let days = week_days();
        let placed = distribute(2_400, &days, &Contour::BackLoaded).unwrap();
        let weeks = periods(d(2026, 10, 5), d(2026, 10, 18), Grain::Week).unwrap();
        let (b, outside) = bucket(&placed, &weeks);
        assert_eq!(b.len(), 2);
        assert_eq!(b[0].minutes, 2_400);
        assert_eq!(b[1].minutes, 0);
        assert_eq!(outside, 0);
        // A window that misses part of the work reports what it missed.
        let short = periods(d(2026, 10, 5), d(2026, 10, 7), Grain::Day).unwrap();
        let (b, outside) = bucket(&placed, &short);
        assert_eq!(b.iter().map(|x| x.minutes).sum::<i64>() + outside, 2_400);
        assert!(outside > 0);
    }

    #[test]
    fn periods_by_day_week_and_month() {
        assert_eq!(
            periods(d(2026, 10, 5), d(2026, 10, 7), Grain::Day)
                .unwrap()
                .len(),
            3
        );
        assert_eq!(
            periods(d(2026, 10, 7), d(2026, 10, 20), Grain::Week)
                .unwrap()
                .len(),
            3
        );
        assert_eq!(
            periods(d(2026, 10, 1), d(2026, 12, 31), Grain::Month)
                .unwrap()
                .len(),
            3
        );
        assert!(periods(d(2026, 10, 1), d(2027, 6, 1), Grain::Day).is_err());
        assert!(periods(d(2026, 10, 2), d(2026, 10, 1), Grain::Day).is_err());
        assert_eq!(Grain::parse("week"), Some(Grain::Week));
        assert_eq!(Grain::parse("year"), None);
    }
}
