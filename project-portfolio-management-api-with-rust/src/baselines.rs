//! Schedule baselines (SC-3): the pure variance rules. DB-free, unit-tested.
//!
//! A baseline is a numbered, append-only snapshot of each task's early start,
//! early finish and working duration. Variance is always **against a named
//! version**, in **working days of the plan's calendar** (so a weekend inside a
//! slip is not counted).
//!
//! Three things are kept apart on purpose, because merging them is how a status
//! report lies:
//!
//! - a task that **was in the baseline and still is** has a variance (slipped,
//!   pulled in, or on baseline);
//! - a task **added since** the baseline is `added`, **not slip**;
//! - a task **removed since** is `removed`, kept in the report with its baselined
//!   dates, so deleting a late task cannot make the plan look better.
//!
//! A task that could not be scheduled on either side has **no variance**, with
//! the reason, never a zero.

use chrono::NaiveDate;
use serde::Serialize;
use uuid::Uuid;

use crate::cpm::WorkCalendar;

/// Baselines kept per plan unless `PROJECT_PORTFOLIO_MANAGEMENT_MAX_SCHEDULE_BASELINES`
/// says otherwise (the number scheduling tools offer).
pub const DEFAULT_MAX_BASELINES: i64 = 11;

/// One task at one moment: its scheduled dates, or why it had none.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Snap {
    /// The task.
    pub task_pid: Uuid,
    /// Its title then.
    pub title: String,
    /// Early start.
    pub start: Option<NaiveDate>,
    /// Early finish.
    pub finish: Option<NaiveDate>,
    /// Working duration.
    pub duration: Option<i64>,
}

/// How a task compares with the baseline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Standing {
    /// Finishes later than baselined.
    Slipped,
    /// Finishes earlier than baselined.
    PulledIn,
    /// Same finish (it may still have moved at the start).
    OnBaseline,
    /// In the plan now, not in the baseline.
    Added,
    /// In the baseline, gone from the plan.
    Removed,
    /// In both, but unscheduled on one side: no variance can be stated.
    Unmeasurable,
}

/// One task's variance row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Row {
    /// The task.
    pub task_pid: Uuid,
    /// Its title (now, or as baselined for a removed task).
    pub title: String,
    /// How it compares.
    pub standing: Standing,
    /// Baselined start.
    pub baseline_start: Option<NaiveDate>,
    /// Baselined finish.
    pub baseline_finish: Option<NaiveDate>,
    /// Current start.
    pub start: Option<NaiveDate>,
    /// Current finish.
    pub finish: Option<NaiveDate>,
    /// Working days the start moved (positive = later); `None` when unmeasurable.
    pub start_variance_days: Option<i64>,
    /// Working days the finish moved (positive = later); `None` when unmeasurable.
    pub finish_variance_days: Option<i64>,
    /// Change in working duration; `None` when unmeasurable.
    pub duration_variance_days: Option<i64>,
    /// Why there is no variance, when there is none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<&'static str>,
}

/// The counts, which reconcile with the rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Summary {
    /// Tasks in the baseline.
    pub baseline_tasks: usize,
    /// Tasks in the plan now.
    pub current_tasks: usize,
    /// In both.
    pub matched: usize,
    /// Of those: finish later.
    pub slipped: usize,
    /// Of those: finish earlier.
    pub pulled_in: usize,
    /// Of those: same finish.
    pub on_baseline: usize,
    /// Of those: no variance can be stated.
    pub unmeasurable: usize,
    /// In the plan now only.
    pub added: usize,
    /// In the baseline only.
    pub removed: usize,
    /// Working days the plan's finish moved (positive = later); `None` when either side has no finish.
    pub plan_finish_slip_days: Option<i64>,
}

/// The variance report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Variance {
    /// One row per task seen on either side.
    pub rows: Vec<Row>,
    /// The counts.
    pub summary: Summary,
}

/// Compare `now` with the baseline `then`.
///
/// `then_finish` and `now_finish` are the plans' finish dates (the latest early
/// finish), so the plan-level slip is stated even if no single task is.
#[must_use]
pub fn variance(
    then: &[Snap],
    now: &[Snap],
    then_finish: Option<NaiveDate>,
    now_finish: Option<NaiveDate>,
    calendar: &WorkCalendar,
) -> Variance {
    let idx = |d: NaiveDate| calendar.index_of(d);
    let diff = |a: Option<NaiveDate>, b: Option<NaiveDate>| match (a.and_then(idx), b.and_then(idx))
    {
        (Some(x), Some(y)) => Some(y - x),
        _ => None,
    };
    let mut rows = Vec::new();
    for n in now {
        let Some(t) = then.iter().find(|t| t.task_pid == n.task_pid) else {
            rows.push(Row {
                task_pid: n.task_pid,
                title: n.title.clone(),
                standing: Standing::Added,
                baseline_start: None,
                baseline_finish: None,
                start: n.start,
                finish: n.finish,
                start_variance_days: None,
                finish_variance_days: None,
                duration_variance_days: None,
                reason: Some("added_since_the_baseline"),
            });
            continue;
        };
        let (sv, fv) = (diff(t.start, n.start), diff(t.finish, n.finish));
        let dv = match (t.duration, n.duration) {
            (Some(a), Some(b)) => Some(b - a),
            _ => None,
        };
        let (standing, reason) = match fv {
            Some(f) if f > 0 => (Standing::Slipped, None),
            Some(f) if f < 0 => (Standing::PulledIn, None),
            Some(_) => (Standing::OnBaseline, None),
            None => (
                Standing::Unmeasurable,
                Some(if t.finish.is_none() {
                    "not_scheduled_then"
                } else {
                    "not_scheduled_now"
                }),
            ),
        };
        rows.push(Row {
            task_pid: n.task_pid,
            title: n.title.clone(),
            standing,
            baseline_start: t.start,
            baseline_finish: t.finish,
            start: n.start,
            finish: n.finish,
            start_variance_days: sv,
            finish_variance_days: fv,
            duration_variance_days: dv,
            reason,
        });
    }
    for t in then {
        if !now.iter().any(|n| n.task_pid == t.task_pid) {
            rows.push(Row {
                task_pid: t.task_pid,
                title: t.title.clone(),
                standing: Standing::Removed,
                baseline_start: t.start,
                baseline_finish: t.finish,
                start: None,
                finish: None,
                start_variance_days: None,
                finish_variance_days: None,
                duration_variance_days: None,
                reason: Some("removed_since_the_baseline"),
            });
        }
    }
    let count = |s: Standing| rows.iter().filter(|r| r.standing == s).count();
    let summary = Summary {
        baseline_tasks: then.len(),
        current_tasks: now.len(),
        matched: now
            .iter()
            .filter(|n| then.iter().any(|t| t.task_pid == n.task_pid))
            .count(),
        slipped: count(Standing::Slipped),
        pulled_in: count(Standing::PulledIn),
        on_baseline: count(Standing::OnBaseline),
        unmeasurable: count(Standing::Unmeasurable),
        added: count(Standing::Added),
        removed: count(Standing::Removed),
        plan_finish_slip_days: diff(then_finish, now_finish),
    };
    Variance { rows, summary }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(n: u128) -> Uuid {
        Uuid::from_u128(n)
    }
    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).unwrap()
    }
    fn snap(
        n: u128,
        start: Option<NaiveDate>,
        finish: Option<NaiveDate>,
        dur: Option<i64>,
    ) -> Snap {
        Snap {
            task_pid: id(n),
            title: format!("T{n}"),
            start,
            finish,
            duration: dur,
        }
    }
    fn cal() -> WorkCalendar {
        WorkCalendar::mon_fri()
    }
    fn row(v: &Variance, n: u128) -> &Row {
        v.rows.iter().find(|r| r.task_pid == id(n)).unwrap()
    }

    #[test]
    fn a_task_that_finishes_later_has_slipped_in_working_days() {
        // Baselined to finish Friday 2026-10-09; now Wednesday 2026-10-14: 3 working days.
        let then = [snap(1, Some(d(2026, 10, 5)), Some(d(2026, 10, 9)), Some(5))];
        let now = [snap(
            1,
            Some(d(2026, 10, 5)),
            Some(d(2026, 10, 14)),
            Some(8),
        )];
        let v = variance(
            &then,
            &now,
            Some(d(2026, 10, 9)),
            Some(d(2026, 10, 14)),
            &cal(),
        );
        let r = row(&v, 1);
        assert_eq!(r.standing, Standing::Slipped);
        assert_eq!(
            r.finish_variance_days,
            Some(3),
            "the weekend between is not counted"
        );
        assert_eq!(r.start_variance_days, Some(0));
        assert_eq!(r.duration_variance_days, Some(3));
        assert_eq!(v.summary.plan_finish_slip_days, Some(3));
    }

    #[test]
    fn earlier_and_unchanged_are_told_apart() {
        let then = [
            snap(1, Some(d(2026, 10, 5)), Some(d(2026, 10, 9)), Some(5)),
            snap(2, Some(d(2026, 10, 5)), Some(d(2026, 10, 9)), Some(5)),
        ];
        let now = [
            snap(1, Some(d(2026, 10, 5)), Some(d(2026, 10, 7)), Some(3)),
            snap(2, Some(d(2026, 10, 6)), Some(d(2026, 10, 9)), Some(4)),
        ];
        let v = variance(&then, &now, None, None, &cal());
        assert_eq!(row(&v, 1).standing, Standing::PulledIn);
        assert_eq!(row(&v, 1).finish_variance_days, Some(-2));
        assert_eq!(
            row(&v, 2).standing,
            Standing::OnBaseline,
            "the finish held even though the start moved"
        );
        assert_eq!(row(&v, 2).start_variance_days, Some(1));
        assert_eq!(
            v.summary.plan_finish_slip_days, None,
            "no finish on either side is no plan variance"
        );
    }

    #[test]
    fn a_task_added_later_is_added_not_slip() {
        let then = [snap(1, Some(d(2026, 10, 5)), Some(d(2026, 10, 9)), Some(5))];
        let now = [
            snap(1, Some(d(2026, 10, 5)), Some(d(2026, 10, 9)), Some(5)),
            snap(2, Some(d(2026, 10, 12)), Some(d(2026, 10, 30)), Some(15)),
        ];
        let v = variance(
            &then,
            &now,
            Some(d(2026, 10, 9)),
            Some(d(2026, 10, 30)),
            &cal(),
        );
        let r = row(&v, 2);
        assert_eq!(r.standing, Standing::Added);
        assert_eq!(
            r.finish_variance_days, None,
            "an added task has no baseline to be late against"
        );
        assert_eq!(r.reason, Some("added_since_the_baseline"));
        assert_eq!((v.summary.slipped, v.summary.added), (0, 1));
        // The plan as a whole did move, and says so.
        assert_eq!(v.summary.plan_finish_slip_days, Some(15));
    }

    #[test]
    fn a_task_deleted_after_baselining_stays_in_the_report() {
        let then = [
            snap(1, Some(d(2026, 10, 5)), Some(d(2026, 10, 9)), Some(5)),
            snap(2, Some(d(2026, 10, 5)), Some(d(2026, 10, 30)), Some(20)),
        ];
        let now = [snap(1, Some(d(2026, 10, 5)), Some(d(2026, 10, 9)), Some(5))];
        let v = variance(&then, &now, None, None, &cal());
        let r = row(&v, 2);
        assert_eq!(r.standing, Standing::Removed);
        assert_eq!(
            (r.baseline_finish, r.finish),
            (Some(d(2026, 10, 30)), None),
            "kept with its baselined dates"
        );
        assert_eq!(v.summary.removed, 1);
    }

    #[test]
    fn a_task_unscheduled_on_either_side_has_no_variance_not_zero() {
        let then = [
            snap(1, None, None, None),
            snap(2, Some(d(2026, 10, 5)), Some(d(2026, 10, 9)), Some(5)),
        ];
        let now = [
            snap(1, Some(d(2026, 10, 5)), Some(d(2026, 10, 9)), Some(5)),
            snap(2, None, None, None),
        ];
        let v = variance(&then, &now, None, None, &cal());
        assert_eq!(
            (row(&v, 1).standing, row(&v, 1).reason),
            (Standing::Unmeasurable, Some("not_scheduled_then"))
        );
        assert_eq!(
            (row(&v, 2).standing, row(&v, 2).reason),
            (Standing::Unmeasurable, Some("not_scheduled_now"))
        );
        assert_eq!(row(&v, 1).finish_variance_days, None);
        assert_eq!(v.summary.unmeasurable, 2);
    }

    #[test]
    fn the_counts_reconcile_with_the_rows_and_both_sides() {
        let then: Vec<Snap> = (1..=6)
            .map(|n| snap(n, Some(d(2026, 10, 5)), Some(d(2026, 10, 9)), Some(5)))
            .collect();
        let mut now: Vec<Snap> = (1..=4)
            .map(|n| {
                snap(
                    n,
                    Some(d(2026, 10, 5)),
                    Some(d(2026, 10, 9) + chrono::Duration::days(i64::try_from(n).unwrap() - 2)),
                    Some(5),
                )
            })
            .collect();
        now.push(snap(
            10,
            Some(d(2026, 10, 5)),
            Some(d(2026, 10, 9)),
            Some(5),
        ));
        now.push(snap(11, None, None, None));
        let v = variance(&then, &now, None, None, &cal());
        let s = &v.summary;
        assert_eq!(
            s.matched + s.removed,
            s.baseline_tasks,
            "matched + removed = baseline"
        );
        assert_eq!(
            s.matched + s.added,
            s.current_tasks,
            "matched + added = current"
        );
        assert_eq!(
            s.slipped + s.pulled_in + s.on_baseline + s.unmeasurable,
            s.matched
        );
        assert_eq!(v.rows.len(), s.matched + s.added + s.removed);
        assert_eq!((s.removed, s.added), (2, 2));
    }

    #[test]
    fn identical_schedules_have_no_variance_anywhere() {
        let both = [snap(1, Some(d(2026, 10, 5)), Some(d(2026, 10, 9)), Some(5))];
        let v = variance(
            &both,
            &both,
            Some(d(2026, 10, 9)),
            Some(d(2026, 10, 9)),
            &cal(),
        );
        assert_eq!(row(&v, 1).standing, Standing::OnBaseline);
        assert_eq!(v.summary.plan_finish_slip_days, Some(0));
        assert_eq!(
            (
                v.summary.slipped,
                v.summary.pulled_in,
                v.summary.added,
                v.summary.removed
            ),
            (0, 0, 0, 0)
        );
    }

    #[test]
    fn empty_sides_are_fine() {
        let v = variance(&[], &[], None, None, &cal());
        assert!(v.rows.is_empty());
        assert_eq!(v.summary.matched, 0);
    }
}
