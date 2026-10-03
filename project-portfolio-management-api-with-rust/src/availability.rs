//! Availability over time (PJ-3): the pure rules. DB-free, unit-tested.
//!
//! A person's capacity is **100% unless a range says otherwise**: a part-time
//! spell is 60%, leave is 0%, a ramp-up is 50% for a month. Capacity is read
//! **per day**, never averaged: a person on 100% in week one and 40% in week two
//! is over-allocated by a 60% demand in week two even though the fortnight's
//! average would hide it.
//!
//! Ranges for one person may not overlap; that is what makes "the range in force
//! on a day" a single answer.

use chrono::{Duration, NaiveDate};
use serde::Serialize;

/// Capacity when no range applies.
pub const DEFAULT_PERCENT: i32 = 100;
/// Most days one load computation walks.
pub const MAX_WINDOW_DAYS: i64 = 3_660;

/// One availability range, inclusive on both ends.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
    /// First day.
    pub from: NaiveDate,
    /// Last day.
    pub to: NaiveDate,
    /// The most of their time they can commit (0–100).
    pub max_percent: i32,
}

/// Every problem with a new range (all of them).
#[must_use]
pub fn problems(from: NaiveDate, to: NaiveDate, max_percent: i32) -> Vec<String> {
    let mut out = Vec::new();
    if to < from {
        out.push("to is before from".to_string());
    }
    if !(0..=100).contains(&max_percent) {
        out.push(format!("max_percent must be 0 to 100, got {max_percent}"));
    }
    if (to - from).num_days() > 3_650 {
        out.push("a range may span at most ten years".to_string());
    }
    out
}

/// Whether `[from, to]` overlaps any of `existing`.
#[must_use]
pub fn overlaps(existing: &[Range], from: NaiveDate, to: NaiveDate) -> bool {
    existing.iter().any(|r| r.from <= to && r.to >= from)
}

/// The capacity in force on `day`: the covering range's, else
/// [`DEFAULT_PERCENT`].
#[must_use]
pub fn capacity_on(ranges: &[Range], day: NaiveDate) -> i32 {
    ranges
        .iter()
        .find(|r| r.from <= day && r.to >= day)
        .map_or(DEFAULT_PERCENT, |r| r.max_percent)
}

/// An allocation as the load sees it; an absent bound is open-ended.
#[derive(Debug, Clone, Copy)]
pub struct Alloc {
    /// Percent of capacity asked for.
    pub percent: i32,
    /// First day, or open.
    pub start: Option<NaiveDate>,
    /// Last day, or open.
    pub end: Option<NaiveDate>,
}

/// A run of consecutive days with the same demand and capacity, over capacity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Over {
    /// First day of the run.
    pub from: NaiveDate,
    /// Last day of the run.
    pub to: NaiveDate,
    /// Summed percent asked for on each of those days.
    pub demand: i64,
    /// The capacity in force on each of those days.
    pub capacity: i32,
}

/// The load over a window.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Load {
    /// The highest single-day demand.
    pub peak_demand: i64,
    /// The lowest capacity in force on any day of the window.
    pub min_capacity: i32,
    /// Days where demand exceeded capacity.
    pub days_over: i64,
    /// Those days, merged into runs.
    pub over: Vec<Over>,
}

/// Walk `[from, to]` day by day: demand is the sum of the percentages of the
/// allocations covering the day, capacity the range in force; a day is over when
/// demand exceeds capacity. A window longer than [`MAX_WINDOW_DAYS`] is clamped.
#[must_use]
pub fn load(from: NaiveDate, to: NaiveDate, allocs: &[Alloc], ranges: &[Range]) -> Load {
    let to = to.min(from + Duration::days(MAX_WINDOW_DAYS));
    let mut out = Load {
        peak_demand: 0,
        min_capacity: DEFAULT_PERCENT,
        days_over: 0,
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
        out.min_capacity = out.min_capacity.min(capacity);
        if demand > i64::from(capacity) {
            out.days_over += 1;
            match out.over.last_mut() {
                Some(last)
                    if last.demand == demand
                        && last.capacity == capacity
                        && last.to + Duration::days(1) == day =>
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
        day += Duration::days(1);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).unwrap()
    }
    fn r(a: NaiveDate, b: NaiveDate, p: i32) -> Range {
        Range {
            from: a,
            to: b,
            max_percent: p,
        }
    }
    fn a(p: i32, s: Option<NaiveDate>, e: Option<NaiveDate>) -> Alloc {
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
            "a one-day, zero-percent leave"
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
    fn capacity_is_one_hundred_where_no_range_applies() {
        let ranges = [r(d(2026, 10, 5), d(2026, 10, 9), 40)];
        assert_eq!(capacity_on(&ranges, d(2026, 10, 4)), 100);
        assert_eq!(capacity_on(&ranges, d(2026, 10, 5)), 40);
        assert_eq!(capacity_on(&ranges, d(2026, 10, 9)), 40);
        assert_eq!(capacity_on(&ranges, d(2026, 10, 10)), 100);
        assert_eq!(capacity_on(&[], d(2026, 10, 10)), 100);
    }

    #[test]
    fn with_no_ranges_a_window_is_over_exactly_when_the_demand_exceeds_100() {
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
        assert_eq!(l.min_capacity, 100);
        assert_eq!(l.days_over, 3);
        assert_eq!(
            l.over,
            vec![Over {
                from: d(2026, 10, 4),
                to: d(2026, 10, 6),
                demand: 110,
                capacity: 100
            }]
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
            &ranges,
        );
        assert_eq!(l.days_over, 7, "only the 40% week is over");
        assert_eq!(
            l.over,
            vec![Over {
                from: d(2026, 10, 12),
                to: d(2026, 10, 18),
                demand: 80,
                capacity: 40
            }]
        );
        assert_eq!(l.min_capacity, 40);
        // The fortnight's average capacity (70%) would have said 80 > 70 for all fourteen days.
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
        assert_eq!(l.over[0].capacity, 0);
        // No demand on a zero-percent day is not over.
        let l = load(
            d(2026, 10, 1),
            d(2026, 10, 3),
            &[],
            &[r(d(2026, 10, 2), d(2026, 10, 2), 0)],
        );
        assert_eq!(l.days_over, 0);
        assert_eq!(l.min_capacity, 0);
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
                capacity: 100
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
