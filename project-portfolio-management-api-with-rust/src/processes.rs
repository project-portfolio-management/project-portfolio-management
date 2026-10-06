//! Multi-step approval processes (PL-2): the pure rules. DB-free and deterministic.
//!
//! A process is an ordered list of steps; an execution walks them **one at a time**:
//!
//! * `notify` — tell a person; it needs nothing back, so it completes at once;
//! * `approve` — ask a named person to approve the subject (a PL-1 approval request);
//!   the execution waits for their decision;
//! * `todo` — hand a named person something to do; the execution waits until they say
//!   it is done.
//!
//! A step starts only after the one before it **completes**. A rejection ends the
//! execution `rejected` and **every step after it is skipped, and logged**. A
//! cancellation does the same. [`advance`] says what happens from a given step; it
//! never looks at a clock or a database, so the rules can be tested exhaustively.

use serde::{Deserialize, Serialize};

/// The most steps a process may have.
pub const MAX_STEPS: usize = 20;

/// What a step does.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    /// Ask someone to approve; wait for the decision.
    Approve,
    /// Give someone something to do; wait until it is done.
    Todo,
    /// Tell someone; completes at once.
    Notify,
}

/// One step of a process.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Step {
    /// What it does.
    pub kind: Kind,
    /// The person it is for, as a `person:` / `worker:` URN.
    pub person_ref: String,
    /// What it says (shown to them, and in the log).
    pub title: String,
}

/// Problems with a process's steps (empty when fine).
#[must_use]
pub fn problems(steps: &[Step]) -> Vec<String> {
    let mut out = Vec::new();
    if steps.is_empty() || steps.len() > MAX_STEPS {
        out.push(format!("a process has 1 to {MAX_STEPS} steps"));
    }
    for (i, s) in steps.iter().enumerate() {
        let at = |m: &str| format!("step {}: {m}", i + 1);
        if s.title.trim().is_empty() || s.title.chars().count() > 200 {
            out.push(at("title is required (up to 200 characters)"));
        }
        let ok = s
            .person_ref
            .split_once(':')
            .is_some_and(|(k, rest)| matches!(k, "person" | "worker") && !rest.trim().is_empty());
        if !ok {
            out.push(at("person_ref must be a person:/worker: URN"));
        }
    }
    out
}

/// What running from a step does.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Advance {
    /// Steps that ran to completion on their own (`notify`), in order.
    pub completed: Vec<usize>,
    /// The step the execution now waits on, and what it is waiting for.
    pub waiting: Option<(usize, Kind)>,
    /// No step is left.
    pub finished: bool,
}

/// Start from step `from` (the next one to run): run each `notify` in turn, stop at the
/// first step that needs a person (`approve` / `todo`), or finish when none are left.
#[must_use]
pub fn advance(steps: &[Step], from: usize) -> Advance {
    let mut completed = Vec::new();
    let mut i = from;
    while let Some(step) = steps.get(i) {
        match step.kind {
            Kind::Notify => {
                completed.push(i);
                i += 1;
            }
            Kind::Approve | Kind::Todo => {
                return Advance {
                    completed,
                    waiting: Some((i, step.kind)),
                    finished: false,
                };
            }
        }
    }
    Advance {
        completed,
        waiting: None,
        finished: true,
    }
}

/// The steps after `index` that are skipped when the execution ends early.
#[must_use]
pub fn skipped_after(len: usize, index: usize) -> Vec<usize> {
    ((index + 1)..len).collect()
}

/// An execution's status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    /// Waiting on a step.
    Running,
    /// Every step done.
    Completed,
    /// An approval was rejected.
    Rejected,
    /// Stopped by a person.
    Cancelled,
    /// A step could not be started.
    Failed,
}

impl Status {
    /// The stored token.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Running => "running",
            Self::Completed => "completed",
            Self::Rejected => "rejected",
            Self::Cancelled => "cancelled",
            Self::Failed => "failed",
        }
    }

    /// Whether the execution is over.
    #[must_use]
    pub fn is_final(token: &str) -> bool {
        token != "running"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn step(kind: Kind, who: &str) -> Step {
        Step {
            kind,
            person_ref: format!("person:{who}"),
            title: format!("{kind:?} {who}"),
        }
    }

    #[test]
    fn steps_are_checked() {
        let ok = vec![step(Kind::Approve, "a")];
        assert!(problems(&ok).is_empty());
        assert_eq!(problems(&[]).len(), 1);
        assert_eq!(
            problems(&vec![step(Kind::Notify, "a"); MAX_STEPS + 1]).len(),
            1
        );
        let mut bad = step(Kind::Todo, "a");
        bad.person_ref = "alice".into();
        bad.title = " ".into();
        assert_eq!(problems(&[bad]).len(), 2);
        let mut org = step(Kind::Todo, "a");
        org.person_ref = "organization:x".into();
        assert_eq!(problems(&[org]).len(), 1, "a person, not an organisation");
    }

    #[test]
    fn notify_steps_run_on_their_own_until_a_step_needs_a_person() {
        let steps = vec![
            step(Kind::Notify, "a"),
            step(Kind::Notify, "b"),
            step(Kind::Approve, "c"),
            step(Kind::Notify, "d"),
        ];
        let a = advance(&steps, 0);
        assert_eq!(
            a,
            Advance {
                completed: vec![0, 1],
                waiting: Some((2, Kind::Approve)),
                finished: false
            }
        );
        // After the approval completes, the next start runs the trailing notify and finishes.
        let b = advance(&steps, 3);
        assert_eq!(
            b,
            Advance {
                completed: vec![3],
                waiting: None,
                finished: true
            }
        );
    }

    #[test]
    fn a_step_never_starts_before_its_predecessor_completes() {
        let steps = vec![
            step(Kind::Todo, "a"),
            step(Kind::Notify, "b"),
            step(Kind::Todo, "c"),
        ];
        let a = advance(&steps, 0);
        assert_eq!(a.waiting, Some((0, Kind::Todo)));
        assert!(
            a.completed.is_empty(),
            "the notify after the todo has not run"
        );
        assert_eq!(advance(&steps, 1).waiting, Some((2, Kind::Todo)));
    }

    #[test]
    fn a_process_of_only_notices_finishes_at_once_and_an_empty_tail_finishes() {
        let steps = vec![step(Kind::Notify, "a"), step(Kind::Notify, "b")];
        assert_eq!(
            advance(&steps, 0),
            Advance {
                completed: vec![0, 1],
                waiting: None,
                finished: true
            }
        );
        assert_eq!(
            advance(&steps, 2),
            Advance {
                completed: vec![],
                waiting: None,
                finished: true
            }
        );
        assert_eq!(
            advance(&steps, 99),
            Advance {
                completed: vec![],
                waiting: None,
                finished: true
            }
        );
    }

    #[test]
    fn what_is_skipped_when_it_ends_early() {
        assert_eq!(skipped_after(5, 1), vec![2, 3, 4]);
        assert!(skipped_after(5, 4).is_empty());
        assert!(skipped_after(0, 0).is_empty());
    }

    #[test]
    fn statuses_know_when_they_are_over() {
        assert!(!Status::is_final(Status::Running.token()));
        for s in [
            Status::Completed,
            Status::Rejected,
            Status::Cancelled,
            Status::Failed,
        ] {
            assert!(Status::is_final(s.token()));
        }
    }
}
