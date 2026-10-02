//! "My work" (LT-3): bucket a person's open tasks by due date, in the
//! caller's own zone. Pure and unit-tested; the zone arithmetic lives in
//! [`crate::tz`].
//!
//! Buckets, relative to the caller's local `today`:
//!
//! - `overdue`  — due before today;
//! - `today`    — due today;
//! - `this_week` — due in the next six days (today + 1 ..= today + 6),
//!   so `today` + `this_week` together span one rolling week;
//! - `later`    — due after that;
//! - `undated`  — no due date. **Never** treated as due today or as "later":
//!   an undated task is a finding, not a deadline.
//!
//! Every bucket is always present in the output, empty ones included, so a
//! client renders a stable layout and an empty `overdue` reads as good news
//! rather than a missing field.

use chrono::NaiveDate;
use serde::Serialize;

/// One due-date bucket.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Bucket {
    /// Due before today.
    Overdue,
    /// Due today.
    Today,
    /// Due in the next six days.
    ThisWeek,
    /// Due later than that.
    Later,
    /// No due date.
    Undated,
}

impl Bucket {
    /// Every bucket, in display order.
    pub const ALL: [Self; 5] = [
        Self::Overdue,
        Self::Today,
        Self::ThisWeek,
        Self::Later,
        Self::Undated,
    ];

    /// The wire key.
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::Overdue => "overdue",
            Self::Today => "today",
            Self::ThisWeek => "this_week",
            Self::Later => "later",
            Self::Undated => "undated",
        }
    }
}

/// Which bucket a task falls in on `today`.
#[must_use]
pub fn bucket(due: Option<NaiveDate>, today: NaiveDate) -> Bucket {
    let Some(due) = due else {
        return Bucket::Undated;
    };
    match (due - today).num_days() {
        d if d < 0 => Bucket::Overdue,
        0 => Bucket::Today,
        1..=6 => Bucket::ThisWeek,
        _ => Bucket::Later,
    }
}

/// Items awaiting grouping, each with its due date.
type Dated<T> = Vec<(Option<NaiveDate>, T)>;

/// Group `(due, item)` pairs into all five buckets, each ordered by due
/// date ascending (undated last within their bucket) and then by the order
/// given. The input order is the tiebreak, so callers pass stable data.
#[must_use]
pub fn group<T>(items: Vec<(Option<NaiveDate>, T)>, today: NaiveDate) -> Vec<(Bucket, Vec<T>)> {
    let mut out: Vec<(Bucket, Dated<T>)> = Bucket::ALL.iter().map(|b| (*b, Vec::new())).collect();
    for (due, item) in items {
        let b = bucket(due, today);
        if let Some((_, v)) = out.iter_mut().find(|(k, _)| *k == b) {
            v.push((due, item));
        }
    }
    out.into_iter()
        .map(|(b, mut v)| {
            v.sort_by_key(|(due, _)| *due);
            (b, v.into_iter().map(|(_, t)| t).collect())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).unwrap()
    }

    #[test]
    fn the_boundaries_fall_where_documented() {
        let today = d(2026, 10, 2);
        assert_eq!(bucket(Some(d(2026, 10, 1)), today), Bucket::Overdue);
        assert_eq!(bucket(Some(d(2026, 10, 2)), today), Bucket::Today);
        assert_eq!(bucket(Some(d(2026, 10, 3)), today), Bucket::ThisWeek);
        assert_eq!(
            bucket(Some(d(2026, 10, 8)), today),
            Bucket::ThisWeek,
            "+6 days"
        );
        assert_eq!(
            bucket(Some(d(2026, 10, 9)), today),
            Bucket::Later,
            "+7 days"
        );
        assert_eq!(bucket(None, today), Bucket::Undated);
    }

    #[test]
    fn the_same_due_date_moves_bucket_with_the_callers_today() {
        // Due 2026-10-02: today for one caller, overdue for a caller whose
        // local date has already rolled over.
        let due = Some(d(2026, 10, 2));
        assert_eq!(bucket(due, d(2026, 10, 2)), Bucket::Today);
        assert_eq!(bucket(due, d(2026, 10, 3)), Bucket::Overdue);
    }

    #[test]
    fn every_bucket_is_present_even_when_empty() {
        let g = group::<&str>(vec![], d(2026, 10, 2));
        assert_eq!(g.len(), 5);
        assert!(g.iter().all(|(_, v)| v.is_empty()));
        assert_eq!(
            g.iter().map(|(b, _)| b.key()).collect::<Vec<_>>(),
            ["overdue", "today", "this_week", "later", "undated"]
        );
    }

    #[test]
    fn items_sort_by_due_date_then_input_order() {
        let today = d(2026, 10, 2);
        let g = group(
            vec![
                (Some(d(2026, 10, 5)), "c"),
                (Some(d(2026, 10, 3)), "a"),
                (Some(d(2026, 10, 3)), "b"),
                (None, "u"),
            ],
            today,
        );
        let week = &g.iter().find(|(b, _)| *b == Bucket::ThisWeek).unwrap().1;
        assert_eq!(week, &["a", "b", "c"], "stable within a date");
        let undated = &g.iter().find(|(b, _)| *b == Bucket::Undated).unwrap().1;
        assert_eq!(undated, &["u"]);
    }
}
