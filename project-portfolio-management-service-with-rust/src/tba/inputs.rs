//! Inputs.

/// The latest instant the time analysis accepts, epoch milliseconds: 2100-01-01T00:00:00Z. Every stored time is checked
/// against `0..=MAX_EPOCH_MS` where it enters ([`check_times`], RCA-IN-1), so a span is at most this long, and the
/// sums and differences of spans inside `tba` fit an `i64` for any realistic count (RCA-EXC-2: this bound times the
/// number of rows read, at most about 10⁶, is under 4.2 × 10¹⁸, below `i64::MAX`).
pub const MAX_EPOCH_MS: i64 = 4_102_444_800_000;

/// Whether every stored time of one task is inside `0..=MAX_EPOCH_MS`, and so is `as_of_ms`.
///
/// # Errors
///
/// A message naming the kind of time that is outside the range: the stored times are corrupt (RCA-IN-6).
pub fn check_times(
    transitions: &[Transition],
    clock: TaskClock,
    as_of_ms: i64,
) -> Result<(), &'static str> {
    let valid = |ms: i64| (0..=MAX_EPOCH_MS).contains(&ms);
    if !valid(clock.created_ms) || !clock.done_ms.is_none_or(valid) {
        return Err("a task's created or done time is outside 1970 to 2100");
    }
    if !transitions.iter().all(|t| valid(t.at_ms)) {
        return Err("a task transition's time is outside 1970 to 2100");
    }
    if !valid(as_of_ms) {
        return Err("the as-of time is outside 1970 to 2100");
    }
    Ok(())
}

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
