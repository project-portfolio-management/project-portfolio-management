//! Earned value (SC-4): the pure rules. DB-free, unit-tested.
//!
//! **The question earned value answers** is *how much of the planned work is
//! actually done, and what did it cost?* — as three numbers in the plan's own
//! currency: **PV** (planned value: how much of the budget was supposed to be
//! earned by now), **EV** (earned value: the budget times how complete the work
//! is) and **AC** (actual cost). From them: SV = EV − PV, CV = EV − AC,
//! SPI = EV ÷ PV, CPI = EV ÷ AC, and **SPI(t)** (earned schedule), which stays
//! meaningful after a project is late, where SPI drifts to 1.0 whatever happened.
//!
//! Conventions, shared with the rest of the service:
//!
//! - **Money is minor units in `i64`**, ratios are **basis points** (10 000 = 1.0),
//!   and the arithmetic is integer (`i128` intermediates, rounded to nearest). No
//!   float touches a currency figure.
//! - **Undefined is `None` with a reason**, never zero and never a sentinel:
//!   `SPI` with no planned value yet is "no planned value yet", not infinity and
//!   not 1.0.
//! - **One currency, one block.** Nothing here adds two currencies.
//! - **Percent complete is physical, not elapsed time.** Planned value follows the
//!   baseline's calendar; earned value follows how much work is finished. They are
//!   different things, and that difference is the whole point of the method.
//!
//! **Planned value inside a period is linear** (a period planned at 3 000 over 30
//! days is 100 a day) — the standard EVM convention, applied here and nowhere
//! else; the budget-baseline forecast (T-28b) deliberately does *not* pro-rate.

use chrono::NaiveDate;
use serde::Serialize;
use uuid::Uuid;

/// How "done" a task is counted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Method {
    /// 0% until finished, then 100%.
    ZeroHundred,
    /// 0% not started, 50% started, 100% finished.
    FiftyFifty,
    /// A person-entered percent (with a mandatory note). Always flagged.
    Manual,
}

impl Method {
    /// Parse a stored or submitted method name.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "zero_hundred" => Some(Self::ZeroHundred),
            "fifty_fifty" => Some(Self::FiftyFifty),
            "manual" => Some(Self::Manual),
            _ => None,
        }
    }
}

/// A plan's default way of counting, which tasks may override.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DefaultMethod {
    /// Tasks count 0/100, equally weighted.
    ZeroHundred,
    /// Tasks count 0/50/100, equally weighted.
    FiftyFifty,
    /// Tasks count 0/100, **weighted by story points**: the physical percent of
    /// points delivered.
    Points,
}

impl DefaultMethod {
    /// Parse a stored or submitted default method.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "zero_hundred" => Some(Self::ZeroHundred),
            "fifty_fifty" => Some(Self::FiftyFifty),
            "points" => Some(Self::Points),
            _ => None,
        }
    }
}

/// Where a task is, whatever the workflow calls its states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    /// Not started.
    Todo,
    /// Started, not finished (active or waiting).
    Started,
    /// Finished.
    Done,
}

/// A task as the percent-complete rules see it.
#[derive(Debug, Clone)]
pub struct TaskInput {
    /// The task.
    pub pid: Uuid,
    /// Story points, if any.
    pub points: Option<i32>,
    /// Where it is.
    pub state: TaskState,
    /// This task's own method, overriding the plan's default.
    pub method: Option<Method>,
    /// The manual percent in basis points (0–10 000), for [`Method::Manual`].
    pub manual_bp: Option<i64>,
}

/// What weights the tasks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Weighting {
    /// Every task counts the same.
    Equal,
    /// A task counts for its story points.
    Points,
}

/// One task's contribution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TaskShare {
    /// The task.
    pub pid: Uuid,
    /// Its weight (1, or its points).
    pub weight: i64,
    /// Its percent complete in basis points.
    pub percent_bp: i64,
    /// The method used.
    pub method: Method,
}

/// The plan's percent complete.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PercentComplete {
    /// Σ(weight × percent) ÷ Σ weight, in basis points, rounded to nearest.
    pub percent_bp: i64,
    /// Σ(weight × `percent_bp`), exact (the numerator earned value uses).
    pub numerator: i128,
    /// Σ(weight × 10 000), exact (the denominator).
    pub denominator: i128,
    /// How tasks were weighted.
    pub weighting: Weighting,
    /// Each task's share.
    pub tasks: Vec<TaskShare>,
    /// Tasks that carry no weight (no story points under points weighting).
    pub unweighted: Vec<Uuid>,
    /// Tasks counted by a person-entered percent.
    pub manual: Vec<Uuid>,
}

/// The plan's percent complete, or `None` when nothing carries weight (no tasks,
/// or points weighting with no points) — there is then nothing to be a percent of.
#[must_use]
pub fn percent_complete(default: DefaultMethod, tasks: &[TaskInput]) -> Option<PercentComplete> {
    let weighting = if default == DefaultMethod::Points {
        Weighting::Points
    } else {
        Weighting::Equal
    };
    let mut shares = Vec::new();
    let mut unweighted = Vec::new();
    let mut manual = Vec::new();
    let (mut num, mut den) = (0_i128, 0_i128);
    for t in tasks {
        let weight = match weighting {
            Weighting::Equal => 1,
            Weighting::Points => i64::from(t.points.unwrap_or(0).max(0)),
        };
        if weight == 0 {
            unweighted.push(t.pid);
            continue;
        }
        let method = t.method.unwrap_or(match default {
            DefaultMethod::FiftyFifty => Method::FiftyFifty,
            DefaultMethod::ZeroHundred | DefaultMethod::Points => Method::ZeroHundred,
        });
        let percent_bp = match method {
            Method::ZeroHundred => {
                if t.state == TaskState::Done {
                    10_000
                } else {
                    0
                }
            }
            Method::FiftyFifty => match t.state {
                TaskState::Done => 10_000,
                TaskState::Started => 5_000,
                TaskState::Todo => 0,
            },
            Method::Manual => {
                manual.push(t.pid);
                t.manual_bp.unwrap_or(0).clamp(0, 10_000)
            }
        };
        num += i128::from(weight) * i128::from(percent_bp);
        den += i128::from(weight) * 10_000;
        shares.push(TaskShare {
            pid: t.pid,
            weight,
            percent_bp,
            method,
        });
    }
    if den == 0 {
        return None;
    }
    let percent_bp = i64::try_from((num * 10_000 + den / 2) / den)
        .unwrap_or(0)
        .clamp(0, 10_000);
    Some(PercentComplete {
        percent_bp,
        numerator: num,
        denominator: den,
        weighting,
        tasks: shares,
        unweighted,
        manual,
    })
}

/// One period of a budget baseline.
#[derive(Debug, Clone, Copy)]
pub struct Period {
    /// First day, inclusive.
    pub start: NaiveDate,
    /// Last day, inclusive.
    pub end: NaiveDate,
    /// Planned cost, minor units.
    pub planned_minor: i64,
}

/// Budget at completion: the sum of the baseline's periods.
#[must_use]
pub fn bac(periods: &[Period]) -> i64 {
    periods
        .iter()
        .fold(0_i64, |a, p| a.saturating_add(p.planned_minor))
}

/// The first day of the baseline, or `None` with no periods.
#[must_use]
pub fn baseline_start(periods: &[Period]) -> Option<NaiveDate> {
    periods.iter().map(|p| p.start).min()
}

/// Planned value after `t` elapsed days since `start` (t = 0 is before the first
/// day; t = 1 is the end of the first day). Whole periods count fully, the period
/// in progress counts linearly (floored to a minor unit), later ones count nothing.
#[must_use]
pub fn pv_after(periods: &[Period], start: NaiveDate, t: i64) -> i64 {
    let mut total = 0_i128;
    for p in periods {
        let offset = (p.start - start).num_days();
        let len = (p.end - p.start).num_days() + 1;
        if len <= 0 {
            continue;
        }
        if t >= offset + len {
            total += i128::from(p.planned_minor);
        } else if t > offset {
            total += i128::from(p.planned_minor) * i128::from(t - offset) / i128::from(len);
        }
    }
    i64::try_from(total).unwrap_or(i64::MAX)
}

/// Why a figure is absent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Absent {
    /// `as_of` is before the baseline starts: nothing was planned yet.
    NoPlannedValueYet,
    /// No cost has been recorded, so a cost ratio would divide by zero.
    NoActualsRecorded,
    /// The baseline is empty or plans nothing.
    NoPlannedValue,
}

/// A basis-point ratio, or why there is none.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Ratio {
    /// 10 000 = 1.0. `None` when undefined.
    pub bp: Option<i64>,
    /// Why, when `bp` is `None`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absent: Option<Absent>,
}

fn ratio(num: i64, den: i64, why: Absent) -> Ratio {
    if den == 0 {
        return Ratio {
            bp: None,
            absent: Some(why),
        };
    }
    let v = (i128::from(num) * 10_000 * 2 + i128::from(den.signum()) * i128::from(den))
        / (i128::from(den) * 2);
    Ratio {
        bp: i64::try_from(v).ok(),
        absent: None,
    }
}

/// The figures for one currency.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Block {
    /// Budget at completion.
    pub bac_minor: i64,
    /// Planned value as of the date.
    pub pv_minor: i64,
    /// Earned value: BAC × percent complete.
    pub ev_minor: i64,
    /// Actual cost, or `None` when it cannot be stated.
    pub ac_minor: Option<i64>,
    /// EV − PV.
    pub sv_minor: i64,
    /// EV − AC, or `None` without an AC.
    pub cv_minor: Option<i64>,
    /// EV ÷ PV.
    pub spi: Ratio,
    /// EV ÷ AC.
    pub cpi: Ratio,
    /// Earned schedule: the elapsed days at which the plan *should* have earned
    /// what has actually been earned.
    pub earned_schedule_days: Option<i64>,
    /// Elapsed days since the baseline started, as of the date (the "actual time").
    pub actual_days: i64,
    /// Earned-schedule SPI: earned schedule ÷ actual time.
    pub spi_t: Ratio,
}

/// Compute a currency's block.
///
/// `numerator` / `denominator` are the plan's exact percent complete
/// ([`PercentComplete`]); `ac_minor` is `None` when actual cost cannot be stated.
/// Returns `None` for a baseline with no periods.
#[must_use]
pub fn block(
    periods: &[Period],
    numerator: i128,
    denominator: i128,
    ac_minor: Option<i64>,
    as_of: NaiveDate,
) -> Option<Block> {
    let start = baseline_start(periods)?;
    let bac_minor = bac(periods);
    let ev = if denominator == 0 {
        0
    } else {
        i64::try_from((i128::from(bac_minor) * numerator * 2 + denominator) / (denominator * 2))
            .unwrap_or(i64::MAX)
            .clamp(0, bac_minor)
    };
    let actual_days = if as_of < start {
        0
    } else {
        (as_of - start).num_days() + 1
    };
    let pv = pv_after(periods, start, actual_days);
    let last = periods.iter().map(|p| p.end).max().unwrap_or(start);
    let total_days = (last - start).num_days() + 1;
    // Earned schedule: the least elapsed day count whose planned value reaches EV.
    let earned_schedule_days = if bac_minor == 0 {
        None
    } else if ev <= 0 {
        Some(0)
    } else {
        (1..=total_days).find(|t| pv_after(periods, start, *t) >= ev)
    };
    let spi_t = match earned_schedule_days {
        None => Ratio {
            bp: None,
            absent: Some(Absent::NoPlannedValue),
        },
        Some(_) if actual_days == 0 => Ratio {
            bp: None,
            absent: Some(Absent::NoPlannedValueYet),
        },
        Some(es) => ratio(es, actual_days, Absent::NoPlannedValueYet),
    };
    Some(Block {
        bac_minor,
        pv_minor: pv,
        ev_minor: ev,
        ac_minor,
        sv_minor: ev - pv,
        cv_minor: ac_minor.map(|ac| ev - ac),
        spi: if bac_minor == 0 {
            Ratio {
                bp: None,
                absent: Some(Absent::NoPlannedValue),
            }
        } else {
            ratio(ev, pv, Absent::NoPlannedValueYet)
        },
        cpi: ac_minor.map_or(
            Ratio {
                bp: None,
                absent: Some(Absent::NoActualsRecorded),
            },
            |ac| ratio(ev, ac, Absent::NoActualsRecorded),
        ),
        earned_schedule_days,
        actual_days,
        spi_t,
    })
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
    fn task(n: u128, points: Option<i32>, state: TaskState) -> TaskInput {
        TaskInput {
            pid: id(n),
            points,
            state,
            method: None,
            manual_bp: None,
        }
    }
    /// 10 days Jan 1-10 at 1 000, 10 days Jan 11-20 at 3 000: BAC 4 000.
    fn periods() -> Vec<Period> {
        vec![
            Period {
                start: d(2026, 1, 1),
                end: d(2026, 1, 10),
                planned_minor: 1_000,
            },
            Period {
                start: d(2026, 1, 11),
                end: d(2026, 1, 20),
                planned_minor: 3_000,
            },
        ]
    }

    #[test]
    fn zero_hundred_counts_only_finished_tasks_equally() {
        let t = [
            task(1, None, TaskState::Done),
            task(2, None, TaskState::Started),
            task(3, None, TaskState::Todo),
            task(4, None, TaskState::Done),
        ];
        let pc = percent_complete(DefaultMethod::ZeroHundred, &t).unwrap();
        assert_eq!(pc.percent_bp, 5_000, "2 of 4 finished");
        assert_eq!(pc.weighting, Weighting::Equal);
        assert!(pc.manual.is_empty() && pc.unweighted.is_empty());
    }

    #[test]
    fn fifty_fifty_gives_started_work_half_credit() {
        let t = [
            task(1, None, TaskState::Done),
            task(2, None, TaskState::Started),
            task(3, None, TaskState::Todo),
            task(4, None, TaskState::Todo),
        ];
        let pc = percent_complete(DefaultMethod::FiftyFifty, &t).unwrap();
        assert_eq!(pc.percent_bp, 3_750, "(100 + 50 + 0 + 0) / 4");
    }

    #[test]
    fn points_weighting_is_the_physical_percent_of_points_delivered() {
        let t = [
            task(1, Some(8), TaskState::Done),
            task(2, Some(2), TaskState::Todo),
            task(3, None, TaskState::Done),
        ];
        let pc = percent_complete(DefaultMethod::Points, &t).unwrap();
        assert_eq!(
            pc.percent_bp, 8_000,
            "8 of 10 points; the task with no points carries no weight"
        );
        assert_eq!(pc.weighting, Weighting::Points);
        assert_eq!(
            pc.unweighted,
            vec![id(3)],
            "and it is named, not silently dropped"
        );
    }

    #[test]
    fn a_manual_task_carries_its_entered_percent_and_is_flagged() {
        let mut m = task(1, None, TaskState::Todo);
        m.method = Some(Method::Manual);
        m.manual_bp = Some(6_000);
        let t = [m, task(2, None, TaskState::Done)];
        let pc = percent_complete(DefaultMethod::ZeroHundred, &t).unwrap();
        assert_eq!(pc.percent_bp, 8_000, "(60 + 100) / 2");
        assert_eq!(pc.manual, vec![id(1)]);
        let mut over = task(3, None, TaskState::Todo);
        over.method = Some(Method::Manual);
        over.manual_bp = Some(25_000);
        assert_eq!(
            percent_complete(DefaultMethod::ZeroHundred, &[over])
                .unwrap()
                .percent_bp,
            10_000,
            "clamped to 100%"
        );
    }

    #[test]
    fn nothing_to_weigh_is_no_percent_not_zero_percent() {
        assert!(percent_complete(DefaultMethod::ZeroHundred, &[]).is_none());
        assert!(
            percent_complete(DefaultMethod::Points, &[task(1, None, TaskState::Done)]).is_none(),
            "no points anywhere"
        );
    }

    #[test]
    fn completing_a_task_moves_earned_value_by_exactly_its_weight() {
        let before = [
            task(1, Some(1), TaskState::Done),
            task(2, Some(1), TaskState::Todo),
            task(3, Some(2), TaskState::Todo),
        ];
        let mut after = before.clone();
        after[2].state = TaskState::Done; // the 2-point task finishes
        let ev = |ts: &[TaskInput]| {
            let pc = percent_complete(DefaultMethod::Points, ts).unwrap();
            block(
                &periods(),
                pc.numerator,
                pc.denominator,
                None,
                d(2026, 1, 15),
            )
            .unwrap()
            .ev_minor
        };
        assert_eq!(ev(&before), 1_000, "1 of 4 points of a 4 000 budget");
        assert_eq!(
            ev(&after) - ev(&before),
            2_000,
            "the 2-point task is exactly half the budget"
        );
    }

    #[test]
    fn planned_value_is_whole_periods_plus_a_linear_share_of_the_one_in_progress() {
        let p = periods();
        let s = d(2026, 1, 1);
        assert_eq!(pv_after(&p, s, 0), 0, "before the first day");
        assert_eq!(pv_after(&p, s, 5), 500, "half of the first period");
        assert_eq!(pv_after(&p, s, 10), 1_000, "the first period done");
        assert_eq!(
            pv_after(&p, s, 15),
            1_000 + 1_500,
            "plus half of the second"
        );
        assert_eq!(pv_after(&p, s, 20), 4_000);
        assert_eq!(pv_after(&p, s, 99), 4_000, "never more than the budget");
        assert_eq!(bac(&p), 4_000);
    }

    #[test]
    fn planned_value_is_flat_across_a_gap_and_never_decreases() {
        let p = vec![
            Period {
                start: d(2026, 1, 1),
                end: d(2026, 1, 2),
                planned_minor: 200,
            },
            Period {
                start: d(2026, 1, 11),
                end: d(2026, 1, 12),
                planned_minor: 300,
            },
        ];
        let s = d(2026, 1, 1);
        assert_eq!(pv_after(&p, s, 2), 200);
        assert_eq!(pv_after(&p, s, 9), 200, "nothing is planned in the gap");
        let series: Vec<i64> = (0..=14).map(|t| pv_after(&p, s, t)).collect();
        assert!(series.windows(2).all(|w| w[0] <= w[1]), "{series:?}");
        assert_eq!(*series.last().unwrap(), 500);
    }

    #[test]
    fn the_core_figures_are_pv_ev_ac_and_their_differences() {
        // Day 15: PV 2 500. Work is 50% done: EV 2 000. Spent 2 500.
        let b = block(&periods(), 5_000, 10_000, Some(2_500), d(2026, 1, 15)).unwrap();
        assert_eq!(
            (b.bac_minor, b.pv_minor, b.ev_minor, b.ac_minor),
            (4_000, 2_500, 2_000, Some(2_500))
        );
        assert_eq!((b.sv_minor, b.cv_minor), (-500, Some(-500)));
        assert_eq!(b.spi.bp, Some(8_000), "2 000 / 2 500 = 0.80");
        assert_eq!(b.cpi.bp, Some(8_000), "2 000 / 2 500 = 0.80");
        assert_eq!(b.actual_days, 15);
    }

    #[test]
    fn earned_value_never_exceeds_the_budget() {
        let b = block(&periods(), 99_999, 10_000, None, d(2026, 1, 15)).unwrap();
        assert_eq!(
            b.ev_minor, 4_000,
            "capped at BAC even if the percent is nonsense"
        );
    }

    #[test]
    fn undefined_ratios_say_why_and_are_never_zero_or_one() {
        // Before the baseline starts there is no planned value to divide by.
        let early = block(&periods(), 0, 10_000, None, d(2025, 12, 1)).unwrap();
        assert_eq!(
            early.spi,
            Ratio {
                bp: None,
                absent: Some(Absent::NoPlannedValueYet)
            }
        );
        assert_eq!(
            early.spi_t,
            Ratio {
                bp: None,
                absent: Some(Absent::NoPlannedValueYet)
            }
        );
        // No actual cost recorded: CPI would divide by zero.
        let none = block(&periods(), 5_000, 10_000, Some(0), d(2026, 1, 15)).unwrap();
        assert_eq!(
            none.cpi,
            Ratio {
                bp: None,
                absent: Some(Absent::NoActualsRecorded)
            }
        );
        // Actual cost that cannot be stated at all.
        let unknown = block(&periods(), 5_000, 10_000, None, d(2026, 1, 15)).unwrap();
        assert_eq!((unknown.ac_minor, unknown.cv_minor), (None, None));
        assert_eq!(unknown.cpi.absent, Some(Absent::NoActualsRecorded));
        // An empty baseline is no block at all.
        assert!(block(&[], 5_000, 10_000, None, d(2026, 1, 15)).is_none());
    }

    #[test]
    fn earned_schedule_stays_meaningful_when_a_late_project_finishes() {
        // The work is 100% done on day 30 of a 20-day plan. SPI says 1.0 (EV = PV = BAC):
        // it cannot tell the project was ten days late. SPI(t) can.
        let b = block(&periods(), 10_000, 10_000, Some(4_000), d(2026, 1, 30)).unwrap();
        assert_eq!(
            b.spi.bp,
            Some(10_000),
            "the classic SPI cannot see the lateness"
        );
        assert_eq!(b.earned_schedule_days, Some(20));
        assert_eq!(b.actual_days, 30);
        assert_eq!(
            b.spi_t.bp,
            Some(6_667),
            "20 / 30: it earned in 30 days what was planned for 20"
        );
    }

    #[test]
    fn earned_schedule_is_the_day_the_plan_reached_the_earned_amount() {
        // EV 1 000 is reached exactly when the first period finishes: day 10. On day 15, SPI(t) = 10/15.
        let b = block(&periods(), 2_500, 10_000, None, d(2026, 1, 15)).unwrap();
        assert_eq!(b.ev_minor, 1_000);
        assert_eq!(b.earned_schedule_days, Some(10));
        assert_eq!(b.spi_t.bp, Some(6_667));
        // Ahead: EV 2 500 is what the plan reaches on day 15; at day 12 that is ahead of schedule.
        let ahead = block(&periods(), 6_250, 10_000, None, d(2026, 1, 12)).unwrap();
        assert_eq!(ahead.earned_schedule_days, Some(15));
        assert_eq!(ahead.spi_t.bp, Some(12_500), "15 / 12");
        // Nothing earned yet: schedule earned is zero days.
        let zero = block(&periods(), 0, 10_000, None, d(2026, 1, 12)).unwrap();
        assert_eq!(
            (zero.earned_schedule_days, zero.spi_t.bp),
            (Some(0), Some(0))
        );
    }

    #[test]
    fn on_plan_is_exactly_one() {
        let b = block(&periods(), 6_250, 10_000, Some(2_500), d(2026, 1, 15)).unwrap();
        assert_eq!(
            (b.spi.bp, b.cpi.bp, b.spi_t.bp),
            (Some(10_000), Some(10_000), Some(10_000))
        );
        assert_eq!((b.sv_minor, b.cv_minor), (0, Some(0)));
    }

    #[test]
    fn a_zero_budget_has_no_ratios() {
        let p = vec![Period {
            start: d(2026, 1, 1),
            end: d(2026, 1, 10),
            planned_minor: 0,
        }];
        let b = block(&p, 5_000, 10_000, Some(10), d(2026, 1, 5)).unwrap();
        assert_eq!(b.ev_minor, 0);
        assert_eq!(b.spi.absent, Some(Absent::NoPlannedValue));
        assert_eq!(b.spi_t.absent, Some(Absent::NoPlannedValue));
    }

    #[test]
    fn methods_parse_by_their_stored_names() {
        assert_eq!(Method::parse("manual"), Some(Method::Manual));
        assert_eq!(
            Method::parse("points"),
            None,
            "points is a plan default, not a task method"
        );
        assert_eq!(DefaultMethod::parse("points"), Some(DefaultMethod::Points));
        assert_eq!(DefaultMethod::parse("manual"), None);
    }
}
