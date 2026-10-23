//! Buffer management (CC-4). Pure rules; the database work is in [`crate::controllers::buffer_management`].
//!
//! A frozen buffer plan ([`crate::buffers`]) names, for each buffer, the tasks it protects, their planned
//! durations, the buffer's size in business days and the planned finish of the chain it protects. A
//! **reading** compares that plan with the live schedule:
//!
//! * *chain completed* is the share of the chain's planned work (by planned duration) whose tasks are done;
//! * *buffer consumed* is the **forecast slip** of the chain's end over the buffer: the forecast finish less
//!   the planned finish, in business days of the plan's calendar, as a percentage of the buffer. Below zero
//!   it is recovery; above 100 it is overrun, and the true figure is kept (see [`crate::fever::point`]);
//! * the **zone** is read off the same sloped lines as the fever chart, which stay configurable and printed.
//!
//! A buffer in the red needs a recovery action; the service records one and never invents one.

#![expect(missing_docs)]

use std::collections::HashSet;

use chrono::NaiveDate;
use serde::Serialize;
use uuid::Uuid;

use crate::cpm::WorkCalendar;
use crate::fever::{self, Absent, Lines, Point};

/// One buffer as frozen.
#[derive(Debug, Clone)]
pub struct Spec {
    pub key: String,
    /// `None` for a feeding chain that could not be sized.
    pub days: Option<i64>,
    /// The tasks it protects and their planned durations.
    pub tasks: Vec<(Uuid, i64)>,
    pub planned_finish: NaiveDate,
}

#[derive(Debug, Clone, Serialize)]
pub struct Reading {
    pub key: String,
    pub buffer_days: Option<i64>,
    pub chain_done_days: i64,
    pub chain_total_days: i64,
    pub slip_days: Option<i64>,
    pub point: Option<Point>,
    /// Why there is no point.
    pub no_point: Option<Absent>,
}

impl Reading {
    /// The zone, if there is a point.
    #[must_use]
    pub fn zone(&self) -> Option<&'static str> {
        self.point.as_ref().map(|p| p.zone)
    }
}

/// Read one buffer. `forecast_finish` is the live schedule's finish of the chain's last task, if it has one.
#[must_use]
#[expect(clippy::implicit_hasher)]
pub fn read(
    spec: &Spec,
    done: &HashSet<Uuid>,
    forecast_finish: Option<NaiveDate>,
    calendar: &WorkCalendar,
    lines: &Lines,
) -> Reading {
    let total: i64 = spec.tasks.iter().map(|(_, d)| d).sum();
    let finished: i64 = spec
        .tasks
        .iter()
        .filter(|(t, _)| done.contains(t))
        .map(|(_, d)| d)
        .sum();
    let slip = forecast_finish
        .and_then(|f| Some(calendar.index_of(f)? - calendar.index_of(spec.planned_finish)?));
    let base = Reading {
        key: spec.key.clone(),
        buffer_days: spec.days,
        chain_done_days: finished,
        chain_total_days: total,
        slip_days: slip,
        point: None,
        no_point: None,
    };
    let Some(days) = spec.days else {
        return Reading {
            no_point: Some(Absent::NoBuffer),
            ..base
        };
    };
    let Some(slip) = slip else {
        return Reading {
            no_point: Some(Absent::NoForecast),
            ..base
        };
    };
    match fever::point(finished, total, slip, days, lines) {
        Ok(p) => Reading {
            point: Some(p),
            ..base
        },
        Err(why) => Reading {
            no_point: Some(why),
            ..base
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fever::DEFAULT_LINES;

    fn id(n: u128) -> Uuid {
        Uuid::from_u128(n)
    }

    fn day(d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 1, d).unwrap()
    }

    fn spec(days: Option<i64>) -> Spec {
        Spec {
            key: "project".into(),
            days,
            tasks: vec![(id(1), 6), (id(2), 4)],
            planned_finish: day(12), // Monday 12 January 2026
        }
    }

    fn at(forecast: NaiveDate, done: &[u128], days: Option<i64>) -> Reading {
        let done: HashSet<Uuid> = done.iter().map(|n| id(*n)).collect();
        read(
            &spec(days),
            &done,
            Some(forecast),
            &WorkCalendar::mon_fri(),
            &DEFAULT_LINES,
        )
    }

    #[test]
    fn consumption_is_the_forecast_slip_over_the_buffer() {
        // Planned finish Monday 12th; forecast Thursday 15th is 3 business days late; buffer 6 days.
        let r = at(day(15), &[1], Some(6));
        assert_eq!(r.slip_days, Some(3));
        let p = r.point.unwrap();
        assert!((p.buffer_consumed_pct - 50.0).abs() < 1e-9);
        assert!(
            (p.chain_completed_pct - 60.0).abs() < 1e-9,
            "6 of 10 planned days are done"
        );
    }

    #[test]
    fn early_is_recovery_and_late_beyond_the_buffer_is_overrun_in_the_red() {
        let early = at(day(8), &[1, 2], Some(6)); // Thursday 8th: 2 business days early
        assert_eq!(early.slip_days, Some(-2));
        let p = early.point.clone().unwrap();
        assert!(p.buffer_consumed_pct < 0.0 && p.edge == Some("below"));
        assert_eq!(p.zone, "green");
        let late = at(day(26), &[], Some(6)); // 10 business days late on a 6-day buffer
        let p = late.point.clone().unwrap();
        assert!(p.buffer_consumed_pct > 100.0 && p.edge == Some("above"));
        assert_eq!(late.zone(), Some("red"));
    }

    #[test]
    fn the_zone_boundary_is_the_fever_charts_and_a_point_on_the_line_is_the_kinder_zone() {
        // Nothing done (x = 0): the green/amber line is at 30 percent of a 10-day buffer = 3 days.
        let on = at(day(15), &[], Some(10));
        assert_eq!(on.slip_days, Some(3));
        assert_eq!(on.zone(), Some("green"), "on the line is green");
        let over = at(day(16), &[], Some(10));
        assert_eq!(over.zone(), Some("amber"), "one day over is amber");
    }

    #[test]
    fn an_unsized_buffer_has_no_point_and_says_why() {
        let r = at(day(15), &[], None);
        assert!(r.point.is_none());
        assert_eq!(r.no_point, Some(Absent::NoBuffer));
        let done = HashSet::new();
        let r = read(
            &spec(Some(5)),
            &done,
            None,
            &WorkCalendar::mon_fri(),
            &DEFAULT_LINES,
        );
        assert_eq!(r.no_point, Some(Absent::NoForecast));
    }
}
