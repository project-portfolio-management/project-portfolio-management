//! Cost rates (PJ-2): the pure rules. DB-free, deterministic, integer money.
//!
//! * A **rate** is minor units per hour, **effective from a date**; the rate for
//!   a day is the latest one that took effect on or before it. A day before the
//!   first rate has **no rate** — that is `None`, never zero.
//! * **Labour cost** is minutes × rate ÷ 60. Over several days it is worked out
//!   cumulatively and rounded once per day boundary, so the daily costs **sum
//!   exactly to the rounded total** with no drift, whichever rates applied on
//!   which days (a rate change part-way uses each day's own rate).
//! * An **accrual rule** spreads a fixed or material cost over a task's business
//!   days: all on the first day (`start`), all on the last (`end`), or evenly
//!   (`prorated`) in whole minor units that sum to the amount exactly.
//! * Currencies are never summed here: every function works in one currency.

use chrono::NaiveDate;
use serde::Serialize;

use crate::work_profile::spread;

/// One rate in a table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rate {
    /// The first day it applies.
    pub effective_from: NaiveDate,
    /// Minor units per hour.
    pub per_hour_minor: i64,
}

/// The rate in force on `day`: the latest `effective_from` on or before it.
#[must_use]
pub fn rate_on(rates: &[Rate], day: NaiveDate) -> Option<i64> {
    rates
        .iter()
        .filter(|r| r.effective_from <= day)
        .max_by_key(|r| r.effective_from)
        .map(|r| r.per_hour_minor)
}

/// Cost of `minutes` at `per_hour_minor`, rounded half up to a minor unit.
#[must_use]
pub fn labour_cost_minor(minutes: i64, per_hour_minor: i64) -> i64 {
    let n = i128::from(minutes) * i128::from(per_hour_minor);
    i64::try_from((n + 30) / 60).unwrap_or(i64::MAX)
}

/// The cost of work over days, priced day by day.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Priced {
    /// The cost on each day that had a rate, in order; they sum to `total_minor`.
    pub days: Vec<(NaiveDate, i64)>,
    /// The cost of all rated minutes, rounded once.
    pub total_minor: i64,
    /// Minutes on days with no rate in force (their cost is unknown, not zero).
    pub unrated_minutes: i64,
}

/// Price `(day, minutes)` pairs with a per-day rate lookup.
pub fn price_days<F: Fn(NaiveDate) -> Option<i64>>(
    days: &[(NaiveDate, i64)],
    rate_for: F,
) -> Priced {
    let mut numerator: i128 = 0;
    let mut paid: i64 = 0;
    let mut out = Vec::new();
    let mut unrated = 0_i64;
    for (day, minutes) in days {
        match rate_for(*day) {
            None => unrated += minutes,
            Some(rate) => {
                numerator += i128::from(*minutes) * i128::from(rate);
                let upto = i64::try_from((numerator + 30) / 60).unwrap_or(i64::MAX);
                out.push((*day, upto - paid));
                paid = upto;
            }
        }
    }
    Priced {
        days: out,
        total_minor: paid,
        unrated_minutes: unrated,
    }
}

/// When a cost falls across a task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Accrual {
    /// All on the first business day.
    Start,
    /// Evenly across the business days.
    Prorated,
    /// All on the last business day.
    End,
}

impl Accrual {
    /// Parse `start` / `prorated` / `end`.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "start" => Some(Self::Start),
            "prorated" => Some(Self::Prorated),
            "end" => Some(Self::End),
            _ => None,
        }
    }
}

/// Spread `amount_minor` over `days` by an accrual rule. The parts sum to the
/// amount exactly. `None` when there is an amount but no day to put it on.
#[must_use]
pub fn accrue(
    amount_minor: i64,
    days: &[NaiveDate],
    accrual: Accrual,
) -> Option<Vec<(NaiveDate, i64)>> {
    if days.is_empty() {
        return (amount_minor == 0).then(Vec::new);
    }
    let parts: Vec<i64> = match accrual {
        Accrual::Start => (0..days.len())
            .map(|i| if i == 0 { amount_minor } else { 0 })
            .collect(),
        Accrual::End => (0..days.len())
            .map(|i| if i + 1 == days.len() { amount_minor } else { 0 })
            .collect(),
        Accrual::Prorated => spread(amount_minor, &vec![1; days.len()]),
    };
    Some(days.iter().copied().zip(parts).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).unwrap()
    }
    fn rates() -> Vec<Rate> {
        // Deliberately out of order: lookup must not depend on it.
        vec![
            Rate {
                effective_from: d(2026, 7, 1),
                per_hour_minor: 6_000,
            },
            Rate {
                effective_from: d(2026, 1, 1),
                per_hour_minor: 5_000,
            },
            Rate {
                effective_from: d(2027, 1, 1),
                per_hour_minor: 7_000,
            },
        ]
    }

    #[test]
    fn a_lookup_by_date_returns_the_rate_in_force_that_day() {
        let r = rates();
        assert_eq!(
            rate_on(&r, d(2026, 1, 1)),
            Some(5_000),
            "on the day it takes effect"
        );
        assert_eq!(rate_on(&r, d(2026, 6, 30)), Some(5_000));
        assert_eq!(rate_on(&r, d(2026, 7, 1)), Some(6_000));
        assert_eq!(rate_on(&r, d(2026, 12, 31)), Some(6_000));
        assert_eq!(
            rate_on(&r, d(2030, 1, 1)),
            Some(7_000),
            "the latest stays in force"
        );
    }

    #[test]
    fn a_day_before_the_first_rate_has_no_rate_never_zero() {
        assert_eq!(rate_on(&rates(), d(2025, 12, 31)), None);
        assert_eq!(rate_on(&[], d(2026, 1, 1)), None);
    }

    #[test]
    fn labour_cost_rounds_half_up_to_a_minor_unit() {
        assert_eq!(labour_cost_minor(60, 5_000), 5_000);
        assert_eq!(labour_cost_minor(480, 5_000), 40_000);
        assert_eq!(labour_cost_minor(1, 5_000), 83, "83.33");
        assert_eq!(labour_cost_minor(1, 4_500), 75);
        assert_eq!(labour_cost_minor(1, 4_530), 76, "75.5 rounds up");
        assert_eq!(labour_cost_minor(0, 5_000), 0);
    }

    #[test]
    fn daily_costs_sum_exactly_to_the_rounded_total() {
        // Seven days of 70 minutes at 3 333 an hour: each day is 3 888.5 — no drift allowed.
        let days: Vec<(NaiveDate, i64)> = (1..=7).map(|n| (d(2026, 10, n), 70)).collect();
        let p = price_days(&days, |_| Some(3_333));
        assert_eq!(p.days.len(), 7);
        assert_eq!(p.days.iter().map(|x| x.1).sum::<i64>(), p.total_minor);
        assert_eq!(p.total_minor, labour_cost_minor(7 * 70, 3_333));
        assert_eq!(p.unrated_minutes, 0);
        // And across lots of awkward numbers.
        for rate in [1, 7, 99, 3_333, 12_345, 99_999] {
            for minutes in [1, 13, 70, 479] {
                let days: Vec<(NaiveDate, i64)> =
                    (1..=11).map(|n| (d(2026, 10, n), minutes)).collect();
                let p = price_days(&days, |_| Some(rate));
                assert_eq!(p.days.iter().map(|x| x.1).sum::<i64>(), p.total_minor);
                assert_eq!(
                    p.total_minor,
                    labour_cost_minor(11 * minutes, rate),
                    "{rate} x {minutes}"
                );
            }
        }
    }

    #[test]
    fn a_rate_change_part_way_prices_each_day_at_its_own_rate() {
        let r = rates();
        let days = vec![(d(2026, 6, 30), 480), (d(2026, 7, 1), 480)];
        let p = price_days(&days, |day| rate_on(&r, day));
        assert_eq!(p.days[0].1, 40_000, "8 h at 50.00");
        assert_eq!(p.days[1].1, 48_000, "8 h at 60.00");
        assert_eq!(p.total_minor, 88_000);
    }

    #[test]
    fn minutes_with_no_rate_are_counted_and_never_priced_at_zero() {
        let r = rates();
        let days = vec![(d(2025, 12, 30), 480), (d(2026, 1, 2), 480)];
        let p = price_days(&days, |day| rate_on(&r, day));
        assert_eq!(p.unrated_minutes, 480);
        assert_eq!(
            p.days,
            vec![(d(2026, 1, 2), 40_000)],
            "only the rated day has a cost"
        );
        let none = price_days(&days, |_| None);
        assert_eq!(
            (none.total_minor, none.unrated_minutes, none.days.len()),
            (0, 960, 0)
        );
    }

    #[test]
    fn accrual_rules_put_the_cost_where_they_say_and_always_sum_to_it() {
        let days = vec![d(2026, 10, 5), d(2026, 10, 6), d(2026, 10, 7)];
        assert_eq!(
            accrue(100, &days, Accrual::Start).unwrap(),
            vec![(days[0], 100), (days[1], 0), (days[2], 0)]
        );
        assert_eq!(
            accrue(100, &days, Accrual::End).unwrap(),
            vec![(days[0], 0), (days[1], 0), (days[2], 100)]
        );
        // Prorated across a 3-period task sums exactly to the cost: 34 + 33 + 33.
        let p = accrue(100, &days, Accrual::Prorated).unwrap();
        assert_eq!(p.iter().map(|x| x.1).collect::<Vec<_>>(), vec![34, 33, 33]);
        for amount in [1, 2, 99, 100, 1_001, 123_457] {
            for n in 1..=9 {
                let ds: Vec<NaiveDate> = (1..=n).map(|i| d(2026, 10, i)).collect();
                for rule in [Accrual::Start, Accrual::Prorated, Accrual::End] {
                    let parts = accrue(amount, &ds, rule).unwrap();
                    assert_eq!(
                        parts.iter().map(|x| x.1).sum::<i64>(),
                        amount,
                        "{rule:?} {amount} over {n}"
                    );
                }
            }
        }
    }

    #[test]
    fn a_cost_with_no_day_to_fall_on_is_none_not_dropped() {
        assert_eq!(accrue(100, &[], Accrual::Prorated), None);
        assert_eq!(accrue(0, &[], Accrual::Start), Some(vec![]));
        assert_eq!(Accrual::parse("start"), Some(Accrual::Start));
        assert_eq!(Accrual::parse("weekly"), None);
    }
}
