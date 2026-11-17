//! Milestones: the pure rules. DB-free, unit-tested.
//!
//! A milestone is a dated promise. Three things are derived from it, none stored:
//!
//! * **achievement** — a `manual` milestone is achieved when someone marks it
//!   done; a `linked` one is achieved when **every** task it waits for is done
//!   (and it waits for at least one), or when it was marked done by hand with a
//!   reason;
//! * **status** against the calendar — achieved, late (past due, not achieved),
//!   due soon, or upcoming;
//! * **schedule state** from the critical-path forecast — on track, at risk
//!   (forecast inside the risk window of the due date) or forecast late — and
//!   `no_forecast` with a reason when there is nothing to forecast from. A
//!   milestone with no forecast is never called "on track".
//!
//! Slip is read from the append-only reschedule history: the original due date
//! is the first move's *from*, and `net_days` is how far it has moved in all.

use chrono::NaiveDate;
use serde::Serialize;

/// How a milestone is completed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    /// A person marks it done.
    Manual,
    /// Achieved when its linked tasks are all done.
    Linked,
}

impl Mode {
    /// Parse `manual` / `linked`.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "manual" => Some(Self::Manual),
            "linked" => Some(Self::Linked),
            _ => None,
        }
    }
}

/// How a milestone came to be achieved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Achieved {
    /// Not achieved.
    No,
    /// Marked done by a person.
    Manual,
    /// Every linked task is done.
    Tasks,
}

/// Whether and how a milestone is achieved. `linked_complete` has one entry per
/// linked task (true = done).
#[must_use]
pub fn achieved(mode: Mode, marked_done: bool, linked_complete: &[bool]) -> Achieved {
    if mode == Mode::Linked && !linked_complete.is_empty() && linked_complete.iter().all(|c| *c) {
        return Achieved::Tasks;
    }
    if marked_done {
        Achieved::Manual
    } else {
        Achieved::No
    }
}

/// Status against the calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    /// Achieved.
    Achieved,
    /// Past its due date and not achieved.
    Late,
    /// Due within the window and not achieved.
    DueSoon,
    /// Due later.
    Upcoming,
}

/// The status given today and a "due soon" window in calendar days.
#[must_use]
pub fn status(is_achieved: bool, due: NaiveDate, today: NaiveDate, due_soon_days: i64) -> Status {
    if is_achieved {
        Status::Achieved
    } else if due < today {
        Status::Late
    } else if (due - today).num_days() <= due_soon_days {
        Status::DueSoon
    } else {
        Status::Upcoming
    }
}

/// What the critical-path forecast says.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ScheduleState {
    /// Forecast comfortably before the due date.
    OnTrack,
    /// Forecast before the due date but inside the risk window.
    AtRisk,
    /// Forecast after the due date.
    ForecastLate,
    /// Nothing to forecast from.
    NoForecast,
}

/// The state from the forecast finish and the **business days** of slack between
/// it and the due date (`slack_days`, negative when the forecast is after it).
#[must_use]
pub fn schedule_state(slack_days: i64, at_risk_days: i64) -> ScheduleState {
    if slack_days < 0 {
        ScheduleState::ForecastLate
    } else if slack_days <= at_risk_days {
        ScheduleState::AtRisk
    } else {
        ScheduleState::OnTrack
    }
}

/// One reschedule, as the summary needs it.
#[derive(Debug, Clone, Copy)]
pub struct Slip {
    /// The due date before.
    pub from: NaiveDate,
    /// The due date after.
    pub to: NaiveDate,
}

/// How far a milestone has moved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SlipSummary {
    /// The due date it started with.
    pub original_due: NaiveDate,
    /// How many times it was rescheduled.
    pub reschedules: usize,
    /// How many of those moved it later.
    pub later_moves: usize,
    /// Calendar days from the original due date to now (negative if pulled in).
    pub net_days: i64,
}

/// The slip summary from the history (oldest first) and the current due date.
#[must_use]
pub fn slip_summary(history: &[Slip], current_due: NaiveDate) -> SlipSummary {
    let original = history.first().map_or(current_due, |s| s.from);
    SlipSummary {
        original_due: original,
        reschedules: history.len(),
        later_moves: history.iter().filter(|s| s.to > s.from).count(),
        net_days: (current_due - original).num_days(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).unwrap()
    }

    #[test]
    fn a_manual_milestone_is_achieved_when_marked() {
        assert_eq!(achieved(Mode::Manual, false, &[]), Achieved::No);
        assert_eq!(achieved(Mode::Manual, true, &[]), Achieved::Manual);
        // Linked tasks mean nothing to a manual milestone.
        assert_eq!(achieved(Mode::Manual, false, &[true, true]), Achieved::No);
    }

    #[test]
    fn a_linked_milestone_is_achieved_when_every_task_is_done() {
        assert_eq!(
            achieved(Mode::Linked, false, &[true, true]),
            Achieved::Tasks
        );
        assert_eq!(achieved(Mode::Linked, false, &[true, false]), Achieved::No);
        assert_eq!(
            achieved(Mode::Linked, false, &[]),
            Achieved::No,
            "waiting for no tasks is not achievement"
        );
        // Marked done by hand with tasks still open: achieved, and says by whom.
        assert_eq!(
            achieved(Mode::Linked, true, &[true, false]),
            Achieved::Manual
        );
        // Tasks done wins the label over a stale manual flag.
        assert_eq!(achieved(Mode::Linked, true, &[true]), Achieved::Tasks);
    }

    #[test]
    fn status_follows_the_calendar() {
        let today = d(2026, 10, 5);
        assert_eq!(
            status(true, d(2026, 1, 1), today, 14),
            Status::Achieved,
            "achieved beats late"
        );
        assert_eq!(status(false, d(2026, 10, 4), today, 14), Status::Late);
        assert_eq!(
            status(false, d(2026, 10, 5), today, 14),
            Status::DueSoon,
            "due today is not late"
        );
        assert_eq!(status(false, d(2026, 10, 19), today, 14), Status::DueSoon);
        assert_eq!(status(false, d(2026, 10, 20), today, 14), Status::Upcoming);
    }

    #[test]
    fn schedule_state_is_forecast_late_inside_the_window_or_on_track() {
        assert_eq!(schedule_state(-1, 5), ScheduleState::ForecastLate);
        assert_eq!(
            schedule_state(0, 5),
            ScheduleState::AtRisk,
            "finishing on the due date is at risk"
        );
        assert_eq!(schedule_state(5, 5), ScheduleState::AtRisk);
        assert_eq!(schedule_state(6, 5), ScheduleState::OnTrack);
        assert_eq!(schedule_state(0, 0), ScheduleState::AtRisk);
        assert_eq!(schedule_state(1, 0), ScheduleState::OnTrack);
    }

    #[test]
    fn slip_reads_the_history_from_the_original_date() {
        let now = d(2026, 11, 20);
        let none = slip_summary(&[], now);
        assert_eq!(
            (
                none.original_due,
                none.reschedules,
                none.later_moves,
                none.net_days
            ),
            (now, 0, 0, 0)
        );
        let history = [
            Slip {
                from: d(2026, 11, 1),
                to: d(2026, 11, 10),
            },
            Slip {
                from: d(2026, 11, 10),
                to: d(2026, 11, 5),
            },
            Slip {
                from: d(2026, 11, 5),
                to: d(2026, 11, 20),
            },
        ];
        let s = slip_summary(&history, now);
        assert_eq!(s.original_due, d(2026, 11, 1));
        assert_eq!((s.reschedules, s.later_moves), (3, 2));
        assert_eq!(s.net_days, 19);
        // Pulled in overall.
        let pulled = slip_summary(
            &[Slip {
                from: d(2026, 11, 20),
                to: d(2026, 11, 10),
            }],
            d(2026, 11, 10),
        );
        assert_eq!(pulled.net_days, -10);
        assert_eq!(pulled.later_moves, 0);
    }

    #[test]
    fn modes_parse() {
        assert_eq!(Mode::parse("manual"), Some(Mode::Manual));
        assert_eq!(Mode::parse("linked"), Some(Mode::Linked));
        assert_eq!(Mode::parse("auto"), None);
    }
}
