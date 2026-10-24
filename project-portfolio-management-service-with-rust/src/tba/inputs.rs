//! Inputs.

/// One recorded status change, as the analysis consumes it.
#[derive(Clone, Debug)]
pub struct Transition {
    /// The status left; `None` marks the task's creation.
    pub from_status: Option<String>,
    /// The status entered.
    pub to_status: String,
    /// When, epoch milliseconds.
    pub at_ms: i64,
    /// Who it was assigned to at that moment.
    pub assignee_ref: Option<String>,
    /// Synthesized by the migration rather than observed.
    pub backfilled: bool,
    /// Why the task was blocked, on a move into `blocked`.
    pub blocked_reason: Option<String>,
}

/// The task facts the analysis needs beyond its transitions.
#[derive(Clone, Copy, Debug)]
pub struct TaskClock {
    /// When the task was created, epoch milliseconds.
    pub created_ms: i64,
    /// When it first reached `done`, if it has.
    pub done_ms: Option<i64>,
}
