//! Timesheets and time reports (LT-11, RM-10): the pure rules. DB-free,
//! unit-tested.
//!
//! Everything is **integer minutes** (no rounding drift), and **billable and
//! non-billable are never merged**: every row and total carries both, so a
//! single combined number can never hide the split. A figure with no
//! evidence is `None` with a reason, never zero.
//!
//! A person-week moves `open → submitted → approved | rejected`, a rejected
//! week can be resubmitted, and **approved is final** (and locks the week
//! against new entries).

use chrono::{Datelike, NaiveDate, Weekday};
use serde::Serialize;
use uuid::Uuid;

/// Parse an ISO week (`2026-W40`) to its Monday.
#[must_use]
pub fn week_start(iso: &str) -> Option<NaiveDate> {
    let (year, week) = iso.trim().split_once("-W")?;
    NaiveDate::from_isoywd_opt(year.parse().ok()?, week.parse().ok()?, Weekday::Mon)
}

/// The Monday of the ISO week containing `date`.
#[must_use]
pub fn monday_of(date: NaiveDate) -> NaiveDate {
    date - chrono::Duration::days(i64::from(date.weekday().num_days_from_monday()))
}

/// The seven dates of the week starting `monday`.
#[must_use]
pub fn days_of(monday: NaiveDate) -> [NaiveDate; 7] {
    std::array::from_fn(|i| monday + chrono::Duration::days(i64::try_from(i).unwrap_or(0)))
}

/// One time entry as the roll-ups see it.
#[derive(Debug, Clone)]
pub struct Entry {
    /// The plan.
    pub plan_pid: Uuid,
    /// Who spent it (an `EntityRef`).
    pub actor_ref: String,
    /// The day.
    pub date: NaiveDate,
    /// Whole minutes.
    pub minutes: i64,
    /// Effort category token.
    pub category: String,
    /// Whether billable.
    pub billable: bool,
}

/// Minutes split by billable-ness. The two are always carried together.
#[derive(Debug, Clone, Copy, Default, Serialize, PartialEq, Eq)]
pub struct Split {
    /// Billable minutes.
    pub billable: i64,
    /// Non-billable minutes.
    pub non_billable: i64,
}

impl Split {
    /// Billable + non-billable (reported beside, never instead of, the split).
    #[must_use]
    pub const fn total(self) -> i64 {
        self.billable + self.non_billable
    }

    fn add(&mut self, e: &Entry) {
        if e.billable {
            self.billable += e.minutes;
        } else {
            self.non_billable += e.minutes;
        }
    }
}

/// One plan's row of a week grid.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct GridRow {
    /// The plan.
    pub plan_pid: Uuid,
    /// Minutes per day, Monday first.
    pub by_day: [i64; 7],
    /// The row's billable / non-billable split.
    pub split: Split,
    /// `split.billable + split.non_billable`.
    pub total: i64,
}

/// A person's week.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Grid {
    /// The seven dates, Monday first.
    pub days: [NaiveDate; 7],
    /// One row per plan with entries, ordered by plan pid.
    pub rows: Vec<GridRow>,
    /// Minutes per day across plans.
    pub day_totals: [i64; 7],
    /// The week's split.
    pub split: Split,
    /// Total minutes, or `None` with a reason when nothing was recorded.
    pub total: Option<i64>,
    /// Why `total` is `None`.
    pub reason: Option<&'static str>,
}

/// Build the grid for entries falling in the week starting `monday`.
/// Entries outside the week are ignored.
#[must_use]
pub fn grid(entries: &[Entry], monday: NaiveDate) -> Grid {
    let days = days_of(monday);
    let mut rows: Vec<GridRow> = Vec::new();
    let mut day_totals = [0i64; 7];
    let mut split = Split::default();
    for e in entries {
        let Some(i) = days.iter().position(|d| *d == e.date) else {
            continue;
        };
        let idx = rows
            .iter()
            .position(|r| r.plan_pid == e.plan_pid)
            .unwrap_or_else(|| {
                rows.push(GridRow {
                    plan_pid: e.plan_pid,
                    by_day: [0; 7],
                    split: Split::default(),
                    total: 0,
                });
                rows.len() - 1
            });
        let row = &mut rows[idx];
        row.by_day[i] += e.minutes;
        row.split.add(e);
        row.total += e.minutes;
        day_totals[i] += e.minutes;
        split.add(e);
    }
    rows.sort_by_key(|r| r.plan_pid);
    let any = !rows.is_empty();
    Grid {
        days,
        rows,
        day_totals,
        split,
        total: any.then(|| split.total()),
        reason: (!any).then_some("no_entries"),
    }
}

/// How a time report is grouped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroupBy {
    /// By person (`actor_ref`).
    Person,
    /// By plan.
    Plan,
    /// By effort category.
    Category,
    /// By billable-ness (two groups at most).
    Billable,
    /// By calendar month (`YYYY-MM`).
    Month,
}

impl GroupBy {
    /// Parse the wire token.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "person" => Some(Self::Person),
            "plan" => Some(Self::Plan),
            "category" => Some(Self::Category),
            "billable" => Some(Self::Billable),
            "month" => Some(Self::Month),
            _ => None,
        }
    }

    fn key(self, e: &Entry) -> String {
        match self {
            Self::Person => e.actor_ref.clone(),
            Self::Plan => e.plan_pid.to_string(),
            Self::Category => e.category.clone(),
            Self::Billable => if e.billable {
                "billable"
            } else {
                "non_billable"
            }
            .to_string(),
            Self::Month => format!("{:04}-{:02}", e.date.year(), e.date.month()),
        }
    }
}

/// One group of a time report.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Group {
    /// The group key.
    pub key: String,
    /// Entries in it.
    pub entries: usize,
    /// Its split, always both halves.
    pub split: Split,
    /// `split.billable + split.non_billable`.
    pub minutes: i64,
}

/// Group `entries`; groups are ordered by key. Only groups with entries
/// exist, so an empty result is reported by the caller as `no_entries`, not
/// as a zero row.
#[must_use]
pub fn report(entries: &[Entry], by: GroupBy) -> Vec<Group> {
    let mut groups: Vec<Group> = Vec::new();
    for e in entries {
        let key = by.key(e);
        let idx = groups.iter().position(|g| g.key == key).unwrap_or_else(|| {
            groups.push(Group {
                key,
                entries: 0,
                split: Split::default(),
                minutes: 0,
            });
            groups.len() - 1
        });
        let g = &mut groups[idx];
        g.entries += 1;
        g.split.add(e);
        g.minutes += e.minutes;
    }
    groups.sort_by(|a, b| a.key.cmp(&b.key));
    groups
}

/// Why a week-status transition is refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Refusal {
    /// Nothing to submit.
    Empty,
    /// The week is already in a state that forbids this (carries it).
    Not(&'static str, String),
    /// A rejection without a comment.
    CommentRequired,
}

impl Refusal {
    /// A human message.
    #[must_use]
    pub fn message(&self) -> String {
        match self {
            Self::Empty => "the week has no time entries to submit".to_string(),
            Self::Not(what, status) => format!("cannot {what}: the week is {status}"),
            Self::CommentRequired => "a rejection must say why (comment is required)".to_string(),
        }
    }
}

/// Submit a week currently in `status` (`None` = open) holding `entries`.
///
/// # Errors
///
/// [`Refusal`] when it is empty or already submitted/approved.
pub fn submit(status: Option<&str>, entries: usize) -> Result<&'static str, Refusal> {
    match status {
        None | Some("rejected") => {
            if entries == 0 {
                Err(Refusal::Empty)
            } else {
                Ok("submitted")
            }
        }
        Some(s) => Err(Refusal::Not("submit", s.to_string())),
    }
}

/// Approve or reject a submitted week.
///
/// # Errors
///
/// [`Refusal`] when the week is not submitted, or a rejection lacks a comment.
pub fn decide(
    status: Option<&str>,
    approve: bool,
    comment: Option<&str>,
) -> Result<&'static str, Refusal> {
    if status != Some("submitted") {
        return Err(Refusal::Not("decide", status.unwrap_or("open").to_string()));
    }
    if approve {
        return Ok("approved");
    }
    if comment.map(str::trim).is_none_or(str::is_empty) {
        return Err(Refusal::CommentRequired);
    }
    Ok("rejected")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 10, day).unwrap()
    }
    fn p(n: u128) -> Uuid {
        Uuid::from_u128(n)
    }
    fn e(plan: u128, day: u32, minutes: i64, billable: bool) -> Entry {
        Entry {
            plan_pid: p(plan),
            actor_ref: "person:x".into(),
            date: d(day),
            minutes,
            category: "delivery".into(),
            billable,
        }
    }

    #[test]
    fn iso_weeks_parse_to_their_monday() {
        assert_eq!(week_start("2026-W40"), NaiveDate::from_ymd_opt(2026, 9, 28));
        assert_eq!(week_start("2026-W41"), Some(d(5)));
        assert_eq!(week_start("2026-W54"), None);
        assert_eq!(week_start("nope"), None);
        assert_eq!(monday_of(d(8)), d(5), "a Thursday");
        assert_eq!(monday_of(d(5)), d(5), "a Monday is its own");
    }

    #[test]
    fn the_grid_sums_by_day_and_plan_and_never_merges_billable() {
        let g = grid(
            &[
                e(1, 5, 60, true),
                e(1, 5, 30, false),
                e(2, 7, 45, false),
                e(1, 12, 999, true), /* next week */
            ],
            d(5),
        );
        assert_eq!(g.rows.len(), 2);
        assert_eq!(g.day_totals[0], 90);
        assert_eq!(g.day_totals[2], 45);
        assert_eq!(
            g.split,
            Split {
                billable: 60,
                non_billable: 75
            }
        );
        assert_eq!(g.total, Some(135), "the next week's entry is ignored");
        assert_eq!(g.rows[0].split.total(), g.rows[0].total);
    }

    #[test]
    fn an_empty_week_is_none_with_a_reason_not_zero() {
        let g = grid(&[], d(5));
        assert_eq!((g.total, g.reason), (None, Some("no_entries")));
        assert!(g.rows.is_empty());
    }

    #[test]
    fn a_report_groups_and_always_carries_both_halves() {
        let es = [e(1, 5, 60, true), e(2, 6, 30, false), e(1, 20, 10, true)];
        let by_plan = report(&es, GroupBy::Plan);
        assert_eq!(by_plan.len(), 2);
        assert_eq!(
            by_plan[0].split,
            Split {
                billable: 70,
                non_billable: 0
            }
        );
        let by_b = report(&es, GroupBy::Billable);
        assert_eq!(by_b.iter().map(|g| g.minutes).sum::<i64>(), 100);
        let by_month = report(&es, GroupBy::Month);
        assert_eq!(by_month[0].key, "2026-10");
        assert!(report(&[], GroupBy::Person).is_empty());
        assert_eq!(GroupBy::parse("category"), Some(GroupBy::Category));
        assert_eq!(GroupBy::parse("nope"), None);
    }

    #[test]
    fn the_week_status_machine() {
        assert_eq!(submit(None, 3), Ok("submitted"));
        assert_eq!(submit(None, 0), Err(Refusal::Empty));
        assert_eq!(
            submit(Some("rejected"), 1),
            Ok("submitted"),
            "a rejected week can be resubmitted"
        );
        assert!(submit(Some("approved"), 1).is_err());
        assert!(submit(Some("submitted"), 1).is_err());
        assert_eq!(decide(Some("submitted"), true, None), Ok("approved"));
        assert_eq!(
            decide(Some("submitted"), false, None),
            Err(Refusal::CommentRequired)
        );
        assert_eq!(
            decide(Some("submitted"), false, Some("fix day 3")),
            Ok("rejected")
        );
        assert!(
            decide(None, true, None).is_err(),
            "an open week cannot be decided"
        );
        assert!(
            decide(Some("approved"), true, None).is_err(),
            "approved is final"
        );
    }
}
