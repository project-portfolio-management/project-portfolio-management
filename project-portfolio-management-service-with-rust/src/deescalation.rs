//! The outcome scorecard and the de-escalation tracker (EX-10). Pure rules; the database work is in
//! [`crate::controllers::deescalation`].
//!
//! A scorecard is a **named set of KPIs** (KO-1) read by calendar quarter: a KPI is *met* in a quarter when
//! the latest reading dated in that quarter is at or better than its target. A quarter with no reading is
//! **missing**, which is not the same as not met, and it breaks a run.
//!
//! The tracker records the escalation level **as set by the sponsor** (never computed), a list of criteria
//! each with the evidence given per quarter, and counts **consecutive quarters met**: a quarter is met when
//! every scorecard KPI is met and every criterion is marked met with evidence. The number of quarters the
//! rule asks for is configuration (`PROJECT_PORTFOLIO_MANAGEMENT_DEESCALATION_QUARTERS`, default 2). The
//! count is a fact offered to the person who decides; it moves no level.

#![allow(missing_docs)]

use chrono::{Datelike, NaiveDate};

/// A calendar quarter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Quarter {
    /// The year.
    pub year: i32,
    /// 1 to 4.
    pub q: u32,
}

impl Quarter {
    /// The quarter a date falls in.
    #[must_use]
    pub fn of(d: NaiveDate) -> Self {
        Self { year: d.year(), q: (d.month() - 1) / 3 + 1 }
    }

    /// `2026-Q3`.
    #[must_use]
    pub fn label(self) -> String {
        format!("{}-Q{}", self.year, self.q)
    }

    /// Parse `2026-Q3`.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        let (y, q) = s.trim().split_once("-Q")?;
        let (year, q) = (y.parse::<i32>().ok()?, q.parse::<u32>().ok()?);
        ((1..=4).contains(&q) && (2000..=2100).contains(&year)).then_some(Self { year, q })
    }

    /// The quarter before.
    #[must_use]
    pub fn previous(self) -> Self {
        if self.q == 1 { Self { year: self.year - 1, q: 4 } } else { Self { year: self.year, q: self.q - 1 } }
    }

    /// The first day.
    #[must_use]
    pub fn start(self) -> NaiveDate {
        NaiveDate::from_ymd_opt(self.year, (self.q - 1) * 3 + 1, 1).unwrap_or_default()
    }

    /// The last day.
    #[must_use]
    pub fn end(self) -> NaiveDate {
        let next = if self.q == 4 { Self { year: self.year + 1, q: 1 } } else { Self { year: self.year, q: self.q + 1 } };
        next.start().pred_opt().unwrap_or_default()
    }
}

/// The last quarter that has finished by `today`.
#[must_use]
pub fn last_finished(today: NaiveDate) -> Quarter {
    Quarter::of(today).previous()
}

/// The quarters to read, oldest first, ending at the last finished quarter.
#[must_use]
pub fn window(today: NaiveDate, count: usize) -> Vec<Quarter> {
    let mut out = Vec::new();
    let mut q = last_finished(today);
    for _ in 0..count.clamp(1, 16) {
        out.push(q);
        q = q.previous();
    }
    out.reverse();
    out
}

/// How a quarter stands.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum QuarterState {
    /// Every input is met.
    Met,
    /// Every input is present and at least one is not met.
    NotMet { because: Vec<String> },
    /// At least one input is missing, so the quarter can be neither met nor not met.
    Missing { because: Vec<String> },
}

/// One input for a quarter: a KPI reading or a criterion, `None` when nothing was recorded.
#[derive(Debug, Clone)]
pub struct Input {
    /// What it is, in words (a KPI's name or a criterion's text).
    pub name: String,
    /// `Some(true)` met, `Some(false)` not met, `None` missing.
    pub met: Option<bool>,
}

/// A quarter's state from its inputs. No inputs at all is missing, never met.
#[must_use]
pub fn quarter_state(inputs: &[Input]) -> QuarterState {
    if inputs.is_empty() {
        return QuarterState::Missing { because: vec!["there is nothing to measure: no KPI and no criterion".to_string()] };
    }
    let missing: Vec<String> = inputs.iter().filter(|i| i.met.is_none()).map(|i| format!("{}: nothing recorded", i.name)).collect();
    if !missing.is_empty() {
        return QuarterState::Missing { because: missing };
    }
    let not_met: Vec<String> = inputs.iter().filter(|i| i.met == Some(false)).map(|i| format!("{}: not met", i.name)).collect();
    if not_met.is_empty() { QuarterState::Met } else { QuarterState::NotMet { because: not_met } }
}

/// Consecutive met quarters counting back from the last one (the slice is oldest first). A quarter that is
/// not met or is missing ends the run.
#[must_use]
pub fn consecutive_met(states: &[QuarterState]) -> usize {
    states.iter().rev().take_while(|s| **s == QuarterState::Met).count()
}

/// The quarters the rule asks for: `PROJECT_PORTFOLIO_MANAGEMENT_DEESCALATION_QUARTERS`, 1 to 12, default 2.
#[must_use]
pub fn required_quarters() -> (usize, bool) {
    match std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_DEESCALATION_QUARTERS") {
        Err(_) => (2, false),
        Ok(v) => match v.trim().parse::<usize>() {
            Ok(n) if (1..=12).contains(&n) => (n, false),
            _ => (2, true),
        },
    }
}

/// The starter set of measures: names and directions only, **with no targets**. A deployment chooses its own
/// measures and sets its own targets; this list is a convenience, not a default.
pub const TEMPLATE: &[(&str, &str, &str, &str)] = &[
    ("programmes_with_independent_assurance", "Programmes with independent assurance in the last quarter", "higher_better", "percent"),
    ("milestones_met_on_time", "Milestones met on or before their date", "higher_better", "percent"),
    ("benefits_validated", "Planned benefits validated by someone outside the programme", "higher_better", "percent"),
    ("red_without_decision", "Programmes at red with no decision recorded", "lower_better", "count"),
    ("open_critical_issues", "Open issues at critical severity", "lower_better", "count"),
    ("change_requests_waiting", "Change requests waiting more than 30 days for a decision", "lower_better", "count"),
];

#[cfg(test)]
mod tests {
    use super::*;

    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).unwrap()
    }

    fn input(name: &str, met: Option<bool>) -> Input {
        Input { name: name.into(), met }
    }

    #[test]
    fn quarters_are_calendar_quarters_with_exact_ends() {
        assert_eq!(Quarter::of(d(2026, 3, 31)).label(), "2026-Q1");
        assert_eq!(Quarter::of(d(2026, 4, 1)).label(), "2026-Q2");
        assert_eq!(Quarter { year: 2026, q: 1 }.end(), d(2026, 3, 31));
        assert_eq!(Quarter { year: 2026, q: 4 }.end(), d(2026, 12, 31));
        assert_eq!(Quarter { year: 2026, q: 1 }.previous().label(), "2025-Q4");
        assert_eq!(Quarter::parse("2026-Q3"), Some(Quarter { year: 2026, q: 3 }));
        assert_eq!(Quarter::parse("2026-Q5"), None);
    }

    #[test]
    fn only_a_finished_quarter_is_read() {
        assert_eq!(last_finished(d(2026, 10, 11)).label(), "2026-Q3", "we are in Q4");
        assert_eq!(last_finished(d(2026, 1, 1)).label(), "2025-Q4");
        let w = window(d(2026, 10, 11), 3);
        assert_eq!(w.iter().map(|q| q.label()).collect::<Vec<_>>(), ["2026-Q1", "2026-Q2", "2026-Q3"]);
    }

    #[test]
    fn a_missing_reading_is_missing_not_not_met_and_says_why() {
        assert_eq!(quarter_state(&[input("Uptime", Some(true)), input("Defects", Some(true))]), QuarterState::Met);
        assert_eq!(
            quarter_state(&[input("Uptime", Some(true)), input("Defects", None)]),
            QuarterState::Missing { because: vec!["Defects: nothing recorded".to_string()] }
        );
        assert_eq!(
            quarter_state(&[input("Uptime", Some(false)), input("Defects", Some(true))]),
            QuarterState::NotMet { because: vec!["Uptime: not met".to_string()] }
        );
        assert!(matches!(quarter_state(&[input("A", Some(false)), input("B", None)]), QuarterState::Missing { .. }), "missing outranks not met");
        assert!(matches!(quarter_state(&[]), QuarterState::Missing { .. }), "nothing to measure is never met");
    }

    #[test]
    fn the_run_is_the_consecutive_met_quarters_ending_at_the_last_one() {
        let m = QuarterState::Met;
        let miss = QuarterState::Missing { because: vec![] };
        let nm = QuarterState::NotMet { because: vec![] };
        assert_eq!(consecutive_met(&[m.clone(), m.clone(), m.clone()]), 3);
        assert_eq!(consecutive_met(&[m.clone(), miss.clone(), m.clone()]), 1, "a missing quarter breaks the run");
        assert_eq!(consecutive_met(&[m.clone(), m.clone(), nm]), 0);
        assert_eq!(consecutive_met(&[m, miss]), 0);
        assert_eq!(consecutive_met(&[]), 0);
    }

    #[test]
    fn the_template_has_no_targets_and_every_direction_is_valid() {
        assert!(TEMPLATE.iter().all(|(_, _, dir, _)| crate::kpi::Direction::parse(dir).is_some()));
        assert!(TEMPLATE.len() >= 5);
    }
}
