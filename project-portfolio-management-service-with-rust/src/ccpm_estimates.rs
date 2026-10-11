//! Aggressive and safe estimates (CC-2). Pure rules; the database work is in
//! [`crate::controllers::ccpm_estimates`].
//!
//! A task's `duration_days` is its **likely** duration. Critical-chain scheduling wants two more: an
//! **aggressive** estimate (about 50 percent confidence) and a **safe** one (about 90 percent), and pools the
//! difference into buffers. Each is taken, in this order, from: what a person recorded for it; the
//! optimistic or pessimistic range of the schedule-risk inputs (CR-4), where set; and, only under a rule
//! the deployment chose, a derivation. Whatever is used says where it came from. A task with nothing to
//! use is listed as **missing**; no figure is ever supplied silently.
//!
//! The rule is `PROJECT_PORTFOLIO_MANAGEMENT_CCPM_ESTIMATE_RULE`:
//! `explicit_only` (default) derives nothing; `half_of_safe` takes the safe estimate to be the task's
//! duration when none was given, and the aggressive estimate to be half the safe one, rounded up. It is
//! echoed with every answer.

#![allow(missing_docs)]

use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Rule {
    ExplicitOnly,
    HalfOfSafe,
}

impl Rule {
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::ExplicitOnly => "explicit_only",
            Self::HalfOfSafe => "half_of_safe",
        }
    }
}

/// The rule in force; the flag is true when a value was set but unusable (the default is then used).
#[must_use]
pub fn configured_rule() -> (Rule, bool) {
    match std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_CCPM_ESTIMATE_RULE") {
        Err(_) => (Rule::ExplicitOnly, false),
        Ok(v) => match v.trim() {
            "explicit_only" => (Rule::ExplicitOnly, false),
            "half_of_safe" => (Rule::HalfOfSafe, false),
            _ => (Rule::ExplicitOnly, true),
        },
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Source {
    /// Recorded for this purpose.
    Recorded,
    /// The optimistic or pessimistic estimate of the schedule-risk inputs (CR-4).
    RiskRange,
    /// Derived by the rule in force.
    Rule,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Figure {
    pub days: i64,
    pub source: Source,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum State {
    /// Both figures are recorded (or taken from the risk range).
    Complete,
    /// Both figures exist, at least one derived by the rule.
    Derived,
    /// At least one figure is missing, and no rule gives it.
    Missing,
    /// The figures contradict each other (aggressive above safe).
    Inconsistent,
    /// A milestone has no work to estimate.
    Milestone,
    /// The task has no duration, so there is nothing to estimate against.
    NoDuration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Resolved {
    pub aggressive: Option<Figure>,
    pub safe: Option<Figure>,
    pub state: State,
}

/// What a task's figures are.
#[must_use]
pub fn resolve(
    duration: Option<i64>,
    recorded: (Option<i64>, Option<i64>),
    risk_range: (Option<i64>, Option<i64>),
    rule: Rule,
) -> Resolved {
    let none = |state| Resolved {
        aggressive: None,
        safe: None,
        state,
    };
    match duration {
        None => return none(State::NoDuration),
        Some(0) => return none(State::Milestone),
        Some(_) => {}
    }
    let pick = |rec: Option<i64>, range: Option<i64>| {
        rec.map(|days| Figure {
            days,
            source: Source::Recorded,
        })
        .or_else(|| {
            range.filter(|d| *d >= 1).map(|days| Figure {
                days,
                source: Source::RiskRange,
            })
        })
    };
    let mut safe = pick(recorded.1, risk_range.1);
    if safe.is_none() && rule == Rule::HalfOfSafe {
        safe = duration.map(|days| Figure {
            days,
            source: Source::Rule,
        });
    }
    let mut aggressive = pick(recorded.0, risk_range.0);
    if aggressive.is_none() && rule == Rule::HalfOfSafe {
        aggressive = safe.map(|s| Figure {
            days: ((s.days + 1) / 2).max(1),
            source: Source::Rule,
        });
    }
    let state = match (aggressive, safe) {
        (Some(a), Some(s)) if a.days > s.days => State::Inconsistent,
        (Some(a), Some(s)) if a.source == Source::Rule || s.source == Source::Rule => State::Derived,
        (Some(_), Some(_)) => State::Complete,
        _ => State::Missing,
    };
    Resolved {
        aggressive,
        safe,
        state,
    }
}

/// Why a pair of recorded figures cannot be accepted.
#[must_use]
pub fn problems(duration: Option<i64>, aggressive: Option<i64>, safe: Option<i64>) -> Vec<String> {
    let mut out = Vec::new();
    for (name, v) in [("aggressive_days", aggressive), ("safe_days", safe)] {
        if v.is_some_and(|d| !(1..=3650).contains(&d)) {
            out.push(format!("{name} must be 1 to 3650"));
        }
    }
    if let (Some(a), Some(s)) = (aggressive, safe)
        && a > s
    {
        out.push("aggressive_days is above safe_days".to_string());
    }
    match duration {
        None if aggressive.is_some() || safe.is_some() => {
            out.push("set the task's duration (duration_days) first".to_string());
        }
        Some(0) if aggressive.is_some() || safe.is_some() => {
            out.push("a milestone has no work to estimate".to_string());
        }
        _ => {}
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    type Seen = (Option<(i64, Source)>, Option<(i64, Source)>, State);

    fn fig(r: &Resolved) -> Seen {
        (
            r.aggressive.map(|f| (f.days, f.source)),
            r.safe.map(|f| (f.days, f.source)),
            r.state,
        )
    }

    #[test]
    fn recorded_figures_are_complete() {
        let r = resolve(Some(10), (Some(5), Some(9)), (None, None), Rule::ExplicitOnly);
        assert_eq!(
            fig(&r),
            (Some((5, Source::Recorded)), Some((9, Source::Recorded)), State::Complete)
        );
    }

    #[test]
    fn the_risk_range_is_reused_and_says_so() {
        let r = resolve(Some(10), (None, Some(9)), (Some(4), Some(12)), Rule::ExplicitOnly);
        assert_eq!(
            fig(&r),
            (Some((4, Source::RiskRange)), Some((9, Source::Recorded)), State::Complete),
            "a recorded figure beats the range; the range fills the other"
        );
    }

    #[test]
    fn a_task_with_neither_is_missing_not_defaulted() {
        let r = resolve(Some(10), (None, None), (None, None), Rule::ExplicitOnly);
        assert_eq!(fig(&r), (None, None, State::Missing));
        let r = resolve(Some(10), (Some(5), None), (None, None), Rule::ExplicitOnly);
        assert_eq!(fig(&r), (Some((5, Source::Recorded)), None, State::Missing), "one is not two");
    }

    #[test]
    fn the_half_of_safe_rule_derives_and_marks_what_it_derived() {
        let r = resolve(Some(9), (None, None), (None, None), Rule::HalfOfSafe);
        assert_eq!(
            fig(&r),
            (Some((5, Source::Rule)), Some((9, Source::Rule)), State::Derived),
            "9 taken as safe; half rounded up is 5"
        );
        let r = resolve(Some(9), (None, Some(6)), (None, None), Rule::HalfOfSafe);
        assert_eq!(
            fig(&r),
            (Some((3, Source::Rule)), Some((6, Source::Recorded)), State::Derived)
        );
        let r = resolve(Some(1), (None, None), (None, None), Rule::HalfOfSafe);
        assert_eq!(r.aggressive.map(|f| f.days), Some(1), "never below one day");
    }

    #[test]
    fn milestones_and_undated_tasks_are_not_estimated() {
        assert_eq!(resolve(Some(0), (None, None), (None, None), Rule::HalfOfSafe).state, State::Milestone);
        assert_eq!(resolve(None, (Some(1), Some(2)), (None, None), Rule::HalfOfSafe).state, State::NoDuration);
    }

    #[test]
    fn contradictory_figures_are_flagged() {
        let r = resolve(Some(10), (Some(8), None), (None, Some(6)), Rule::ExplicitOnly);
        assert_eq!(r.state, State::Inconsistent, "recorded aggressive 8 over the range's safe 6");
    }

    #[test]
    fn recording_is_refused_in_words() {
        assert!(problems(Some(10), Some(5), Some(9)).is_empty());
        assert_eq!(problems(Some(10), Some(9), Some(5)), ["aggressive_days is above safe_days"]);
        assert_eq!(problems(Some(10), Some(0), None), ["aggressive_days must be 1 to 3650"]);
        assert_eq!(problems(None, Some(1), None), ["set the task's duration (duration_days) first"]);
        assert_eq!(problems(Some(0), None, Some(2)), ["a milestone has no work to estimate"]);
        assert!(problems(Some(0), None, None).is_empty(), "clearing is always allowed");
    }
}
