//! Gate decisions in a period, and programme status (DP-5). Pure rules; the database work is in
//! [`crate::controllers::programme_gates`].
//!
//! A gate review maps to an outcome: `approved` and `approved_with_conditions` are **passed**,
//! `rejected` is **failed**, `hold` is **deferred**. A gate that was planned for a date that has
//! passed, and was never passed or failed, is also deferred. A programme is `active` unless a
//! recorded decision made it `paused` or `stopped`; `stopped` is terminal.

#![allow(missing_docs)]

use chrono::NaiveDate;

/// The statuses a programme can have.
pub const STATUSES: &[&str] = &["active", "paused", "stopped"];

/// A reason must say something.
pub const MIN_REASON: usize = 10;

/// What a gate-review decision means for the pack.
#[must_use]
pub fn outcome(decision: &str) -> Option<&'static str> {
    match decision {
        "approved" | "approved_with_conditions" => Some("passed"),
        "rejected" => Some("failed"),
        "hold" => Some("deferred"),
        _ => None,
    }
}

/// One gate review, reduced to what the view needs. Slices are in recorded order, oldest first.
#[derive(Debug, Clone)]
pub struct Review {
    pub gate: String,
    pub decision: String,
    pub decided_on: NaiveDate,
}

/// A gate with a planned date.
#[derive(Debug, Clone)]
pub struct Planned {
    pub gate: String,
    pub planned_on: NaiveDate,
}

/// One line of the gate view.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Line {
    pub gate: String,
    pub outcome: &'static str,
    pub decided_on: Option<NaiveDate>,
    pub planned_on: Option<NaiveDate>,
    pub reason: Option<&'static str>,
    /// Days past the planned date at the cut-off, for a deferred gate that has one.
    pub days_late: Option<i64>,
}

/// The gate view for one plan in a period.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct View {
    pub passed: Vec<Line>,
    pub failed: Vec<Line>,
    pub deferred: Vec<Line>,
}

fn in_period(d: NaiveDate, period: &str) -> bool {
    d.format("%Y-%m").to_string() == period
}

/// Classify. `passed` and `failed` are the reviews decided **in the period**; `deferred` is what is
/// outstanding at the cut-off: a gate whose latest review is `hold`, or whose planned date has passed
/// with no passing or failing review. Reviews after the cut-off are ignored.
#[must_use]
pub fn classify(reviews: &[Review], planned: &[Planned], period: &str, cut_off: NaiveDate) -> View {
    let known: Vec<&Review> = reviews.iter().filter(|r| r.decided_on <= cut_off).collect();
    let planned_on = |gate: &str| {
        planned
            .iter()
            .find(|p| p.gate == gate)
            .map(|p| p.planned_on)
    };
    let mut v = View::default();
    for r in known.iter().filter(|r| in_period(r.decided_on, period)) {
        let Some(o) = outcome(&r.decision) else {
            continue;
        };
        let line = Line {
            gate: r.gate.clone(),
            outcome: o,
            decided_on: Some(r.decided_on),
            planned_on: planned_on(&r.gate),
            reason: None,
            days_late: None,
        };
        match o {
            "passed" => v.passed.push(line),
            "failed" => v.failed.push(line),
            _ => {}
        }
    }
    let mut gates: Vec<&str> = known
        .iter()
        .map(|r| r.gate.as_str())
        .chain(planned.iter().map(|p| p.gate.as_str()))
        .collect();
    gates.sort_unstable();
    gates.dedup();
    for g in gates {
        let mine: Vec<&&Review> = known.iter().filter(|r| r.gate == g).collect();
        let settled = mine
            .iter()
            .any(|r| matches!(outcome(&r.decision), Some("passed" | "failed")));
        let last = mine.last();
        let plan = planned_on(g);
        if let Some(r) = last.filter(|r| r.decision == "hold") {
            // The latest word on this gate is a hold.
            v.deferred.push(Line {
                gate: g.to_string(),
                outcome: "deferred",
                decided_on: Some(r.decided_on),
                planned_on: plan,
                reason: Some("reviewed as hold"),
                days_late: plan.map(|p| (cut_off - p).num_days()).filter(|d| *d > 0),
            });
            continue;
        }
        if !settled && let Some(p) = plan.filter(|p| *p < cut_off) {
            v.deferred.push(Line {
                gate: g.to_string(),
                outcome: "deferred",
                decided_on: None,
                planned_on: Some(p),
                reason: Some("planned date passed without a review"),
                days_late: Some((cut_off - p).num_days()),
            });
        }
    }
    v
}

/// One status decision.
#[derive(Debug, Clone)]
pub struct Decision {
    pub status: String,
    pub decided_on: NaiveDate,
}

/// The programme's status at a date: the last decision on or before it, else `active`.
#[must_use]
pub fn status_at(history: &[Decision], at: NaiveDate) -> &'static str {
    history
        .iter()
        .rfind(|d| d.decided_on <= at)
        .map_or("active", |d| match d.status.as_str() {
            "paused" => "paused",
            "stopped" => "stopped",
            _ => "active",
        })
}

/// Whether `to` may follow `from`. A refusal says why.
///
/// # Errors
///
/// A message when the change is not allowed.
pub fn check_transition(from: &str, to: &str) -> Result<(), &'static str> {
    if !STATUSES.contains(&to) {
        return Err("status must be one of active, paused, stopped");
    }
    if from == "stopped" {
        return Err("a stopped programme stays stopped; its record is kept");
    }
    if from == to {
        return Err("the programme already has that status");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(s: &str) -> NaiveDate {
        NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap()
    }
    fn rev(gate: &str, decision: &str, on: &str) -> Review {
        Review {
            gate: gate.into(),
            decision: decision.into(),
            decided_on: d(on),
        }
    }
    fn plan(gate: &str, on: &str) -> Planned {
        Planned {
            gate: gate.into(),
            planned_on: d(on),
        }
    }

    #[test]
    fn decisions_map_to_outcomes() {
        assert_eq!(outcome("approved"), Some("passed"));
        assert_eq!(outcome("approved_with_conditions"), Some("passed"));
        assert_eq!(outcome("rejected"), Some("failed"));
        assert_eq!(outcome("hold"), Some("deferred"));
        assert_eq!(outcome("maybe"), None);
    }

    #[test]
    fn passed_and_failed_are_those_decided_in_the_period() {
        let r = [
            rev("g1_feasibility", "approved", "2026-09-30"),
            rev("g2_definition", "approved_with_conditions", "2026-10-02"),
            rev("g3_delivery", "rejected", "2026-10-20"),
            rev("g4_launch", "approved", "2026-11-01"),
        ];
        let v = classify(&r, &[], "2026-10", d("2026-10-31"));
        assert_eq!(
            v.passed.iter().map(|l| l.gate.as_str()).collect::<Vec<_>>(),
            ["g2_definition"]
        );
        assert_eq!(
            v.failed.iter().map(|l| l.gate.as_str()).collect::<Vec<_>>(),
            ["g3_delivery"]
        );
        assert!(
            v.deferred.is_empty(),
            "a review after the cut-off is ignored"
        );
    }

    #[test]
    fn a_planned_date_that_passed_without_a_review_is_deferred() {
        let v = classify(
            &[],
            &[plan("g2_definition", "2026-10-10")],
            "2026-10",
            d("2026-10-31"),
        );
        assert_eq!(v.deferred.len(), 1);
        assert_eq!(v.deferred[0].days_late, Some(21));
        assert_eq!(
            v.deferred[0].reason,
            Some("planned date passed without a review")
        );
        // A future planned date is not late; the planned day itself is not late either.
        assert!(
            classify(
                &[],
                &[plan("g2_definition", "2026-11-05")],
                "2026-10",
                d("2026-10-31")
            )
            .deferred
            .is_empty()
        );
        assert!(
            classify(
                &[],
                &[plan("g2_definition", "2026-10-31")],
                "2026-10",
                d("2026-10-31")
            )
            .deferred
            .is_empty()
        );
    }

    #[test]
    fn a_hold_is_deferred_until_a_later_review_settles_the_gate() {
        let held = [rev("g2_definition", "hold", "2026-10-05")];
        let v = classify(&held, &[], "2026-10", d("2026-10-31"));
        assert_eq!(v.deferred.len(), 1);
        assert_eq!(v.deferred[0].reason, Some("reviewed as hold"));
        let then_passed = [
            rev("g2_definition", "hold", "2026-10-05"),
            rev("g2_definition", "approved", "2026-10-20"),
        ];
        let v = classify(
            &then_passed,
            &[plan("g2_definition", "2026-10-01")],
            "2026-10",
            d("2026-10-31"),
        );
        assert!(v.deferred.is_empty());
        assert_eq!(v.passed.len(), 1);
    }

    #[test]
    fn a_failed_gate_is_failed_not_deferred_even_past_its_date() {
        let r = [rev("g3_delivery", "rejected", "2026-09-01")];
        let v = classify(
            &r,
            &[plan("g3_delivery", "2026-08-15")],
            "2026-10",
            d("2026-10-31"),
        );
        assert!(
            v.deferred.is_empty() && v.failed.is_empty(),
            "failed in September, not this period"
        );
    }

    #[test]
    fn status_is_the_last_decision_on_or_before_the_date() {
        let h = [
            Decision {
                status: "paused".into(),
                decided_on: d("2026-09-01"),
            },
            Decision {
                status: "active".into(),
                decided_on: d("2026-09-20"),
            },
            Decision {
                status: "stopped".into(),
                decided_on: d("2026-10-10"),
            },
        ];
        assert_eq!(status_at(&[], d("2026-10-31")), "active");
        assert_eq!(status_at(&h, d("2026-08-31")), "active");
        assert_eq!(status_at(&h, d("2026-09-10")), "paused");
        assert_eq!(status_at(&h, d("2026-09-30")), "active");
        assert_eq!(status_at(&h, d("2026-10-10")), "stopped");
    }

    #[test]
    fn stopped_is_terminal_and_a_repeat_is_refused() {
        assert!(check_transition("active", "paused").is_ok());
        assert!(check_transition("paused", "active").is_ok());
        assert!(check_transition("paused", "stopped").is_ok());
        assert!(check_transition("active", "stopped").is_ok());
        assert!(check_transition("stopped", "active").is_err());
        assert!(check_transition("stopped", "stopped").is_err());
        assert!(check_transition("active", "active").is_err());
        assert!(check_transition("active", "frozen").is_err());
    }
}
