//! Burn-up and flow for a product programme (EX-14). Pure.
//!
//! A programme that delivers a product has no budget baseline to earn value against, so the pack's
//! earned-value view shows what it can honestly show instead: how much work has been created and how much
//! finished, week by week (**burn-up**), and how fast work finishes (**throughput**). Only real creation
//! and completion stamps count; there is no ideal line and nothing is interpolated. Counts are of tasks.

#![allow(missing_docs)]

use chrono::{Duration, NaiveDate};
use serde::Serialize;

/// One task's stamps.
#[derive(Debug, Clone, Copy)]
pub struct Item {
    pub created: NaiveDate,
    pub done: Option<NaiveDate>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Point {
    /// The last day of the week the point is for.
    pub date: NaiveDate,
    /// Tasks created on or before the date.
    pub scope: usize,
    /// Tasks finished on or before the date.
    pub done: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Flow {
    pub open: usize,
    pub done: usize,
    pub scope: usize,
    /// Weekly points, oldest first, ending at `as_of`.
    pub burn_up: Vec<Point>,
    /// Tasks finished per week over the last four weeks.
    pub throughput_per_week: f64,
}

/// Burn-up over the last `weeks` weeks (at most 26) and throughput over the last four. `None` when the
/// programme has no tasks: that is "no flow data", never a flat line.
#[must_use]
#[allow(clippy::cast_precision_loss)] // task counts are tiny
pub fn flow(items: &[Item], as_of: NaiveDate, weeks: u32) -> Option<Flow> {
    if items.is_empty() {
        return None;
    }
    let weeks = weeks.clamp(1, 26);
    let at = |d: NaiveDate| Point {
        date: d,
        scope: items.iter().filter(|i| i.created <= d).count(),
        done: items
            .iter()
            .filter(|i| i.done.is_some_and(|x| x <= d))
            .count(),
    };
    let burn_up: Vec<Point> = (0..weeks)
        .rev()
        .map(|w| at(as_of - Duration::days(i64::from(w) * 7)))
        .collect();
    let recent = items
        .iter()
        .filter(|i| {
            i.done
                .is_some_and(|x| x <= as_of && x > as_of - Duration::days(28))
        })
        .count();
    let now = at(as_of);
    Some(Flow {
        open: now.scope.saturating_sub(now.done),
        done: now.done,
        scope: now.scope,
        burn_up,
        throughput_per_week: recent as f64 / 4.0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, m, day).unwrap()
    }

    #[test]
    fn no_tasks_is_no_flow_data_not_a_flat_line() {
        assert!(flow(&[], d(10, 31), 8).is_none());
    }

    #[test]
    fn burn_up_counts_only_real_stamps_week_by_week() {
        let items = [
            Item {
                created: d(10, 1),
                done: Some(d(10, 10)),
            },
            Item {
                created: d(10, 1),
                done: Some(d(10, 25)),
            },
            Item {
                created: d(10, 20),
                done: None,
            },
        ];
        let f = flow(&items, d(10, 31), 4).unwrap();
        assert_eq!((f.scope, f.done, f.open), (3, 2, 1));
        assert_eq!(f.burn_up.len(), 4);
        assert_eq!(f.burn_up.last().unwrap().date, d(10, 31));
        // The week ending 10 October holds the first completion and the two tasks created by then.
        let early = f.burn_up.iter().find(|p| p.date == d(10, 10)).unwrap();
        assert_eq!((early.scope, early.done), (2, 1));
        assert!(
            f.burn_up.windows(2).all(|w| w[0].done <= w[1].done),
            "completions never go backwards"
        );
    }

    #[test]
    fn throughput_is_completions_in_the_last_four_weeks_per_week() {
        let items: Vec<Item> = (0..8)
            .map(|i| Item {
                created: d(9, 1),
                done: Some(d(10, 20 + i)),
            })
            .collect();
        let f = flow(&items, d(10, 31), 8).unwrap();
        assert!((f.throughput_per_week - 2.0).abs() < 1e-9);
        let old = [Item {
            created: d(1, 1),
            done: Some(d(2, 1)),
        }];
        assert!(
            flow(&old, d(10, 31), 8).unwrap().throughput_per_week.abs() < 1e-9,
            "old completions do not count"
        );
    }
}
