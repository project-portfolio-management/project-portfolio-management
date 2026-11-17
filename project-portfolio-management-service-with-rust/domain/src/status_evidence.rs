//! Evidence behind a status update (BR-2): claims that cite sources, and the published definitions of the numbers.
//! DB-free and clock-free, unit-tested.
//!
//! A status update is a person's judgement; there is no computed score. A claim is a sentence with up to
//! [`MAX_SOURCES`] sources, each `kind:<pid>` (a record of the plan, [`crate::meetings::LINK_KINDS`]) or
//! `metric:<key>` (one of [`METRICS`]). Sources are stored as ids and resolved when read. A number that cannot be
//! known is `None`, never zero.

use chrono::{Days, NaiveDate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The most claims one update carries.
pub const MAX_CLAIMS: usize = 10;
/// The most sources one claim cites.
pub const MAX_SOURCES: usize = 5;
/// The longest claim text, in characters.
pub const MAX_CLAIM: usize = 500;
/// How many days ahead "due soon" looks, today included.
pub const DUE_SOON_DAYS: u64 = 7;

/// A published metric: its key, a label and how it is counted.
pub struct MetricDef {
    /// The key used in `metric:<key>`.
    pub key: &'static str,
    /// A short name.
    pub label: &'static str,
    /// Exactly how it is counted.
    pub definition: &'static str,
}

/// The metrics a claim may cite, counted over the plan's open tasks (a task whose workflow state is not "done").
pub const METRICS: &[MetricDef] = &[
    MetricDef {
        key: "overdue",
        label: "Overdue",
        definition: "Open tasks whose due date is before today. A task due today is not overdue. Unknown when there are open tasks and none has a due date.",
    },
    MetricDef {
        key: "due_soon",
        label: "Due soon",
        definition: "Open tasks due today or within the next 7 days. Unknown when there are open tasks and none has a due date.",
    },
    MetricDef {
        key: "blocked",
        label: "Blocked",
        definition: "Open tasks whose status is blocked.",
    },
    MetricDef {
        key: "no_due_date",
        label: "No due date",
        definition: "Open tasks with no due date: the tasks the other counts cannot see.",
    },
];

/// A claim as given or stored: a sentence and the sources that support it.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, schemars::JsonSchema)]
pub struct Claim {
    /// What is being asserted.
    pub text: String,
    /// `kind:<pid>` or `metric:<key>`.
    #[serde(default)]
    pub sources: Vec<String>,
}

/// A parsed source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Source {
    /// A record of the plan.
    Record {
        /// One of [`crate::meetings::LINK_KINDS`].
        kind: String,
        /// The record's `pid`.
        pid: Uuid,
    },
    /// A published metric.
    Metric(String),
}

/// Parse `kind:<pid>` or `metric:<key>`.
///
/// # Errors
///
/// A message naming what is wrong.
pub fn parse(source: &str) -> Result<Source, String> {
    let Some((kind, rest)) = source.split_once(':') else {
        return Err(format!("source {source:?} is kind:id"));
    };
    if kind == "metric" {
        return if METRICS.iter().any(|m| m.key == rest) {
            Ok(Source::Metric(rest.to_string()))
        } else {
            Err(format!("unknown metric {rest:?}"))
        };
    }
    if !crate::meetings::LINK_KINDS.contains(&kind) {
        return Err(format!("unknown source kind {kind:?}"));
    }
    Uuid::parse_str(rest)
        .map(|pid| Source::Record {
            kind: kind.to_string(),
            pid,
        })
        .map_err(|_| format!("source {source:?} has no valid id"))
}

/// Problems with the claims of a new update (empty when fine). Whether a record is live and in the plan is the
/// controller's check.
#[must_use]
pub fn problems(claims: &[Claim]) -> Vec<String> {
    let mut out = Vec::new();
    if claims.len() > MAX_CLAIMS {
        out.push(format!("at most {MAX_CLAIMS} claims"));
    }
    for (i, c) in claims.iter().enumerate() {
        if c.text.trim().is_empty() || c.text.chars().count() > MAX_CLAIM {
            out.push(format!(
                "claim {i}: text is required (up to {MAX_CLAIM} characters)"
            ));
        }
        if c.sources.len() > MAX_SOURCES {
            out.push(format!("claim {i}: at most {MAX_SOURCES} sources"));
        }
        for s in &c.sources {
            if let Err(e) = parse(s) {
                out.push(format!("claim {i}: {e}"));
            }
        }
    }
    out
}

/// What the counts need to know about one task.
pub struct TaskFact {
    /// Its due date, if it has one.
    pub due: Option<NaiveDate>,
    /// Its workflow state is not "done".
    pub open: bool,
    /// Its status is `blocked`.
    pub blocked: bool,
}

/// The plan's numbers as of `today`. `None` is unknown, not zero.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Counts {
    /// Open tasks due before today.
    pub overdue: Option<usize>,
    /// Open tasks due from today to a week ahead.
    pub due_soon: Option<usize>,
    /// Open blocked tasks.
    pub blocked: usize,
    /// Open tasks with no due date.
    pub no_due_date: usize,
}

/// Count the tasks as of `today` (a task due today is not overdue).
#[must_use]
pub fn counts(tasks: &[TaskFact], today: NaiveDate) -> Counts {
    let open: Vec<&TaskFact> = tasks.iter().filter(|t| t.open).collect();
    let dated = open.iter().filter(|t| t.due.is_some()).count();
    let soon_end = today
        .checked_add_days(Days::new(DUE_SOON_DAYS))
        .unwrap_or(NaiveDate::MAX);
    let unknown = !open.is_empty() && dated == 0;
    let n = |f: &dyn Fn(NaiveDate) -> bool| {
        (!unknown).then(|| open.iter().filter(|t| t.due.is_some_and(f)).count())
    };
    Counts {
        overdue: n(&|d| d < today),
        due_soon: n(&|d| d >= today && d <= soon_end),
        blocked: open.iter().filter(|t| t.blocked).count(),
        no_due_date: open.len() - dated,
    }
}

impl Counts {
    /// The value of one metric by key.
    #[must_use]
    pub fn value(&self, key: &str) -> Option<usize> {
        match key {
            "overdue" => self.overdue,
            "due_soon" => self.due_soon,
            "blocked" => Some(self.blocked),
            "no_due_date" => Some(self.no_due_date),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(s: &str) -> NaiveDate {
        s.parse().unwrap()
    }

    fn task(due: Option<&str>, open: bool, blocked: bool) -> TaskFact {
        TaskFact {
            due: due.map(day),
            open,
            blocked,
        }
    }

    #[test]
    fn due_today_is_due_soon_not_overdue_and_done_tasks_do_not_count() {
        let t = [
            task(Some("2026-11-02"), true, false),
            task(Some("2026-11-03"), true, true),
            task(Some("2026-11-10"), true, false),
            task(Some("2026-11-11"), true, false),
            task(Some("2026-10-01"), false, false),
            task(None, true, false),
        ];
        let c = counts(&t, day("2026-11-03"));
        assert_eq!(
            c,
            Counts {
                overdue: Some(1),
                due_soon: Some(2),
                blocked: 1,
                no_due_date: 1
            }
        );
    }

    #[test]
    fn missing_dates_make_the_date_counts_unknown_never_zero() {
        let c = counts(
            &[task(None, true, false), task(None, true, true)],
            day("2026-11-03"),
        );
        assert_eq!(
            (c.overdue, c.due_soon, c.blocked, c.no_due_date),
            (None, None, 1, 2)
        );
        let none = counts(&[], day("2026-11-03"));
        assert_eq!(
            (none.overdue, none.due_soon),
            (Some(0), Some(0)),
            "no open tasks is truly zero"
        );
    }

    #[test]
    fn sources_parse_and_bad_ones_are_named() {
        let id = "11111111-1111-1111-1111-111111111111";
        assert_eq!(
            parse(&format!("task:{id}")),
            Ok(Source::Record {
                kind: "task".into(),
                pid: id.parse().unwrap()
            })
        );
        assert_eq!(
            parse("metric:overdue"),
            Ok(Source::Metric("overdue".into()))
        );
        for bad in [
            "metric:score",
            "plan:11111111-1111-1111-1111-111111111111",
            "task:nope",
            "task",
        ] {
            assert!(parse(bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn claims_are_capped_and_each_problem_is_named() {
        let ok = Claim {
            text: "On time".into(),
            sources: vec!["metric:blocked".into()],
        };
        assert!(problems(std::slice::from_ref(&ok)).is_empty());
        let bad = Claim {
            text: " ".into(),
            sources: vec!["x".into(); 6],
        };
        assert_eq!(problems(&[bad]).len(), 2 + 6);
        assert_eq!(problems(&vec![ok; 11]).len(), 1);
    }
}
