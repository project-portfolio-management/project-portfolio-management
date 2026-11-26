//! Availability over time (PJ-3): the pure rules. DB-free, unit-tested.
//!
//! A person's capacity on a day is **what a range in force says, and nothing else** (CF-7): a part-time spell is
//! 60%, leave is 0%, a ramp-up is 50% for a month. A day no range covers has **no declared capacity**: it is not
//! assumed to be 100%. Such a day can still be **over** when its demand exceeds [`MAX_PERCENT`] (no range can
//! allow more), and is never over when nothing is asked of it; otherwise it is **unknown**, counted apart and never
//! called within capacity. Capacity is read
//! **per day**, never averaged: a person on 100% in week one and 40% in week two
//! is over-allocated by a 60% demand in week two even though the fortnight's
//! average would hide it.
//!
//! Ranges for one person may not overlap; that is what makes "the range in force
//! on a day" a single answer.

use crate::datetime::Date;
use serde::Serialize;

/// The most capacity any range may declare. A day asked for more than this is over whatever is declared.
pub const MAX_PERCENT: i32 = 100;
/// Most calendar days one load computation walks.
pub const MAX_WINDOW_DAYS: i64 = 3_660;

/// One availability range, inclusive on both ends.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
    /// First day.
    pub from: Date,
    /// Last day.
    pub to: Date,
    /// The most of their time they can commit (0–100).
    pub max_percent: i32,
}

/// Every problem with a new range (all of them).
#[must_use]
pub fn problems(from: Date, to: Date, max_percent: i32) -> Vec<String> {
    let mut out = Vec::new();
    if to < from {
        out.push("to is before from".to_string());
    }
    if !(0..=MAX_PERCENT).contains(&max_percent) {
        out.push(format!("max_percent must be 0 to 100, got {max_percent}"));
    }
    if (to - from).num_days() > 3_650 {
        out.push("a range may span at most ten years".to_string());
    }
    out
}

/// Whether `[from, to]` overlaps any of `existing`.
#[must_use]
pub fn overlaps(existing: &[Range], from: Date, to: Date) -> bool {
    existing.iter().any(|r| r.from <= to && r.to >= from)
}

/// The capacity in force on `day`: the covering range's, or `None` when no range covers it (not declared, CF-7).
#[must_use]
pub fn capacity_on(ranges: &[Range], day: Date) -> Option<i32> {
    ranges
        .iter()
        .find(|r| r.from <= day && r.to >= day)
        .map(|r| r.max_percent)
}

/// What can be said of one day's `demand` against a capacity that may not be declared.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DayVerdict {
    /// Demand exceeds the declared capacity, or exceeds [`MAX_PERCENT`] when none is declared.
    Over,
    /// Demand is within the declared capacity, or nothing is asked.
    Within,
    /// Something is asked, no more than [`MAX_PERCENT`], and no capacity is declared: neither over nor within.
    Unknown,
}

/// Judge one day (see [`DayVerdict`]).
#[must_use]
pub fn day_verdict(demand: i64, capacity: Option<i32>) -> DayVerdict {
    match capacity {
        Some(c) if demand > i64::from(c) => DayVerdict::Over,
        Some(_) => DayVerdict::Within,
        None if demand > i64::from(MAX_PERCENT) => DayVerdict::Over,
        None if demand <= 0 => DayVerdict::Within,
        None => DayVerdict::Unknown,
    }
}

/// An allocation as the load sees it; an absent bound is open-ended.
#[derive(Debug, Clone, Copy)]
pub struct Alloc {
    /// Percent of capacity asked for.
    pub percent: i32,
    /// First day, or open.
    pub start: Option<Date>,
    /// Last day, or open.
    pub end: Option<Date>,
}

/// A run of consecutive days with the same demand and capacity, over capacity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Over {
    /// First day of the run.
    pub from: Date,
    /// Last day of the run.
    pub to: Date,
    /// Summed percent asked for on each of those days.
    pub demand: i64,
    /// The capacity in force on each of those days; `None` where none is declared (the demand is then over
    /// [`MAX_PERCENT`]).
    pub capacity: Option<i32>,
}

/// The load over a window.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Load {
    /// The highest single-day demand.
    pub peak_demand: i64,
    /// The lowest capacity declared on any day of the window; `None` when no day of it has one.
    pub min_capacity: Option<i32>,
    /// Calendar days where demand exceeded capacity (see [`DayVerdict::Over`]).
    pub days_over: i64,
    /// Calendar days with demand, no more than [`MAX_PERCENT`], and no declared capacity: neither over nor within.
    pub days_unknown: i64,
    /// Calendar days of the window no range covers, whatever their demand.
    pub days_undeclared: i64,
    /// Those days, merged into runs.
    pub over: Vec<Over>,
}

impl Load {
    /// How much of the window has declared capacity: `availability` (every day), `partly_declared` or
    /// `undeclared` (no day).
    #[must_use]
    pub const fn source(&self) -> &'static str {
        if self.days_undeclared == 0 {
            "availability"
        } else if self.min_capacity.is_none() {
            "undeclared"
        } else {
            "partly_declared"
        }
    }

    /// How a refusal names the capacity: "a capacity of 40%", or that none was declared.
    #[must_use]
    pub fn capacity_phrase(&self) -> String {
        self.min_capacity.map_or_else(
            || "no declared capacity (more than the 100% any range allows)".to_string(),
            |c| format!("a capacity of {c}%"),
        )
    }

    /// Over on some day: `Some(true)`. Within on every day: `Some(false)`. Otherwise (no day over, some unknown)
    /// `None`: the load cannot be called within capacity because some capacity was never declared.
    #[must_use]
    pub const fn over(&self) -> Option<bool> {
        if self.days_over > 0 {
            Some(true)
        } else if self.days_unknown > 0 {
            None
        } else {
            Some(false)
        }
    }
}

/// Walk `[from, to]` day by day: demand is the sum of the percentages of the
/// allocations covering the day, capacity the range in force (or none); each day
/// is judged by [`day_verdict`]. A window longer than [`MAX_WINDOW_DAYS`] is clamped.
#[must_use]
pub fn load(from: Date, to: Date, allocs: &[Alloc], ranges: &[Range]) -> Load {
    let to = from
        .plus_days(MAX_WINDOW_DAYS)
        .map_or(to, |cap| to.min(cap));
    let mut out = Load {
        peak_demand: 0,
        min_capacity: None,
        days_over: 0,
        days_unknown: 0,
        days_undeclared: 0,
        over: Vec::new(),
    };
    let mut day = from;
    while day <= to {
        let demand: i64 = allocs
            .iter()
            .filter(|a| a.start.is_none_or(|s| s <= day) && a.end.is_none_or(|e| e >= day))
            .map(|a| i64::from(a.percent))
            .sum();
        let capacity = capacity_on(ranges, day);
        out.peak_demand = out.peak_demand.max(demand);
        match capacity {
            Some(c) => out.min_capacity = Some(out.min_capacity.map_or(c, |m| m.min(c))),
            None => out.days_undeclared += 1,
        }
        let verdict = day_verdict(demand, capacity);
        if verdict == DayVerdict::Unknown {
            out.days_unknown += 1;
        }
        if verdict == DayVerdict::Over {
            out.days_over += 1;
            match out.over.last_mut() {
                Some(last)
                    if last.demand == demand
                        && last.capacity == capacity
                        && last.to.succ_opt() == Some(day) =>
                {
                    last.to = day;
                }
                _ => out.over.push(Over {
                    from: day,
                    to: day,
                    demand,
                    capacity,
                }),
            }
        }
        day = match day.succ_opt() {
            Some(next) => next,
            None => break,
        };
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(y: i32, m: u32, day: u32) -> Date {
        Date::from_ymd_opt(y, m, day).unwrap()
    }
    fn r(a: Date, b: Date, p: i32) -> Range {
        Range {
            from: a,
            to: b,
            max_percent: p,
        }
    }
    fn a(p: i32, s: Option<Date>, e: Option<Date>) -> Alloc {
        Alloc {
            percent: p,
            start: s,
            end: e,
        }
    }

    #[test]
    fn a_range_is_validated_all_at_once() {
        assert!(problems(d(2026, 10, 1), d(2026, 10, 31), 60).is_empty());
        assert!(
            problems(d(2026, 10, 1), d(2026, 10, 1), 0).is_empty(),
            "a one-calendar-day, zero-percent leave"
        );
        assert_eq!(problems(d(2026, 10, 5), d(2026, 10, 1), 120).len(), 2);
        assert_eq!(problems(d(2026, 1, 1), d(2037, 1, 1), 50).len(), 1);
    }

    #[test]
    fn overlap_includes_the_boundary_days() {
        let ex = [r(d(2026, 10, 1), d(2026, 10, 10), 50)];
        assert!(
            overlaps(&ex, d(2026, 10, 10), d(2026, 10, 20)),
            "touching on the last day"
        );
        assert!(overlaps(&ex, d(2026, 9, 1), d(2026, 10, 1)));
        assert!(!overlaps(&ex, d(2026, 10, 11), d(2026, 10, 20)));
        assert!(!overlaps(&[], d(2026, 10, 1), d(2026, 10, 2)));
    }

    #[test]
    fn a_day_no_range_covers_has_no_declared_capacity() {
        let ranges = [r(d(2026, 10, 5), d(2026, 10, 9), 40)];
        assert_eq!(capacity_on(&ranges, d(2026, 10, 4)), None);
        assert_eq!(capacity_on(&ranges, d(2026, 10, 5)), Some(40));
        assert_eq!(capacity_on(&ranges, d(2026, 10, 9)), Some(40));
        assert_eq!(capacity_on(&ranges, d(2026, 10, 10)), None);
        assert_eq!(capacity_on(&[], d(2026, 10, 10)), None);
    }

    #[test]
    fn an_undeclared_day_is_over_only_past_the_most_any_range_allows_and_otherwise_unknown() {
        assert_eq!(day_verdict(101, None), DayVerdict::Over);
        assert_eq!(day_verdict(100, None), DayVerdict::Unknown);
        assert_eq!(day_verdict(1, None), DayVerdict::Unknown);
        assert_eq!(
            day_verdict(0, None),
            DayVerdict::Within,
            "nothing asked is never over"
        );
        assert_eq!(day_verdict(41, Some(40)), DayVerdict::Over);
        assert_eq!(day_verdict(40, Some(40)), DayVerdict::Within);
        assert_eq!(day_verdict(0, Some(0)), DayVerdict::Within);
    }

    #[test]
    fn with_no_ranges_a_window_is_over_past_100_and_unknown_below_never_within() {
        let l = load(
            d(2026, 10, 1),
            d(2026, 10, 10),
            &[
                a(60, None, None),
                a(50, Some(d(2026, 10, 4)), Some(d(2026, 10, 6))),
            ],
            &[],
        );
        assert_eq!(l.peak_demand, 110);
        assert_eq!(l.min_capacity, None, "nothing declared, nothing assumed");
        assert_eq!((l.days_over, l.days_unknown, l.days_undeclared), (3, 7, 10));
        assert_eq!(
            l.over,
            vec![Over {
                from: d(2026, 10, 4),
                to: d(2026, 10, 6),
                demand: 110,
                capacity: None
            }]
        );
        assert_eq!(l.over(), Some(true));
        // 60% every day and nothing declared: not over, and not within either.
        let l = load(d(2026, 10, 1), d(2026, 10, 3), &[a(60, None, None)], &[]);
        assert_eq!((l.days_over, l.days_unknown, l.over()), (0, 3, None));
        // Nothing asked: within, though nothing is declared.
        let l = load(d(2026, 10, 1), d(2026, 10, 3), &[], &[]);
        assert_eq!(
            (l.days_unknown, l.days_undeclared, l.over()),
            (0, 3, Some(false))
        );
        // Declared for the whole window and within it: within.
        let l = load(
            d(2026, 10, 1),
            d(2026, 10, 3),
            &[a(60, None, None)],
            &[r(d(2026, 9, 1), d(2026, 10, 31), 80)],
        );
        assert_eq!(
            (l.min_capacity, l.over(), l.source()),
            (Some(80), Some(false), "availability")
        );
        let partly = load(
            d(2026, 10, 1),
            d(2026, 10, 3),
            &[a(60, None, None)],
            &[r(d(2026, 10, 1), d(2026, 10, 1), 80)],
        );
        assert_eq!(partly.source(), "partly_declared");
        assert_eq!(
            load(d(2026, 10, 1), d(2026, 10, 3), &[], &[]).source(),
            "undeclared"
        );
    }

    #[test]
    fn capacity_is_read_per_day_not_averaged() {
        // 80% demand all fortnight; the second week the person is at 40%.
        let ranges = [r(d(2026, 10, 12), d(2026, 10, 18), 40)];
        let l = load(
            d(2026, 10, 5),
            d(2026, 10, 18),
            &[a(80, None, None)],
            &[r(d(2026, 10, 5), d(2026, 10, 11), 100), ranges[0]],
        );
        assert_eq!(l.days_over, 7, "only the 40% week is over");
        assert_eq!(
            l.over,
            vec![Over {
                from: d(2026, 10, 12),
                to: d(2026, 10, 18),
                demand: 80,
                capacity: Some(40)
            }]
        );
        assert_eq!(l.min_capacity, Some(40));
        assert_eq!(l.days_unknown, 0, "both weeks are declared");
        // The fortnight's average capacity (70%) would have said 80 > 70 for all
        // fourteen calendar days.
    }

    #[test]
    fn a_zero_percent_spell_makes_any_demand_over_capacity() {
        let l = load(
            d(2026, 10, 1),
            d(2026, 10, 3),
            &[a(10, None, None)],
            &[r(d(2026, 10, 2), d(2026, 10, 2), 0)],
        );
        assert_eq!(l.days_over, 1);
        assert_eq!(l.over[0].capacity, Some(0));
        // No demand on a zero-percent day is not over.
        let l = load(
            d(2026, 10, 1),
            d(2026, 10, 3),
            &[],
            &[r(d(2026, 10, 2), d(2026, 10, 2), 0)],
        );
        assert_eq!(l.days_over, 0);
        assert_eq!(l.min_capacity, Some(0));
    }

    #[test]
    fn runs_split_when_demand_or_capacity_changes() {
        let l = load(
            d(2026, 10, 1),
            d(2026, 10, 6),
            &[a(70, None, None), a(40, Some(d(2026, 10, 4)), None)],
            &[],
        );
        assert_eq!(
            l.over,
            vec![Over {
                from: d(2026, 10, 4),
                to: d(2026, 10, 6),
                demand: 110,
                capacity: None
            }]
        );
        let l = load(
            d(2026, 10, 1),
            d(2026, 10, 4),
            &[
                a(120, Some(d(2026, 10, 1)), Some(d(2026, 10, 2))),
                a(130, Some(d(2026, 10, 3)), Some(d(2026, 10, 4))),
            ],
            &[],
        );
        assert_eq!(l.over.len(), 2, "different demand ⇒ a new run");
    }

    #[test]
    fn a_huge_window_is_clamped_not_walked_forever() {
        let l = load(d(2000, 1, 1), d(2100, 1, 1), &[a(150, None, None)], &[]);
        assert!(l.days_over <= MAX_WINDOW_DAYS + 1);
    }
}
