//! Cumulative flow (spec §10.2).

use super::{
    BACKLOG_STATUS, BLOCKED_STATUS, BOARD_ORDER, BTreeMap, FINISHED_STATUS, Serialize, Transition,
    is_started,
};

/// The most samples a cumulative-flow request may produce, so a wide
/// window cannot become an unbounded response (security invariant 3).
pub const MAX_FLOW_SAMPLES: usize = 400;

/// One task's history, as the cumulative-flow derivation consumes it.
#[derive(Clone, Debug)]
pub struct TaskHistory {
    /// When the task was created, epoch milliseconds.
    pub created_ms: i64,
    /// Its transitions, in any order.
    pub transitions: Vec<Transition>,
}

/// One sample of the board: how many tasks stood in each status at an
/// instant.
#[derive(Clone, Debug, Serialize)]
pub struct FlowSample {
    /// The instant sampled, epoch milliseconds.
    pub at_ms: i64,
    /// Status → count. Every board status is present, including at
    /// zero, so a consumer stacking the bands never has to decide
    /// whether a missing key means zero or means unknown.
    pub counts: BTreeMap<String, usize>,
    /// Tasks that existed by then.
    pub total: usize,
    /// Of those, finished.
    pub done: usize,
    /// Of those, started and not finished — the band a cumulative-flow
    /// diagram's vertical gap represents.
    pub work_in_progress: usize,
}

/// The status one task stood in at `at_ms`, or `None` if it did not
/// exist yet.
///
/// A task that existed but has no transition at or before the instant
/// is reported as `todo` — the same pessimistic treatment of unknown
/// history that [`intervals`] applies, so a backfilled board does not
/// show items appearing from nowhere mid-chart.
#[must_use]
pub fn status_at(history: &TaskHistory, at_ms: i64) -> Option<&str> {
    if at_ms < history.created_ms {
        return None;
    }
    Some(
        history
            .transitions
            .iter()
            .filter(|t| t.at_ms <= at_ms)
            .max_by_key(|t| t.at_ms)
            .map_or(BACKLOG_STATUS, |t| t.to_status.as_str()),
    )
}

/// Derive a cumulative flow diagram: the board's composition sampled at
/// a fixed interval across a window.
///
/// This is the one view here that cannot be assembled client-side — it
/// needs every task's whole history at once, and an API that returned
/// that would be shipping the log to the browser to re-derive what the
/// server already indexes.
///
/// Samples are inclusive of both ends and capped at
/// [`MAX_FLOW_SAMPLES`]; a `step_ms` of zero or less yields no samples
/// rather than looping.
#[must_use]
pub fn cumulative_flow(
    histories: &[TaskHistory],
    from_ms: i64,
    to_ms: i64,
    step_ms: i64,
) -> Vec<FlowSample> {
    if step_ms <= 0 || to_ms < from_ms {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut at = from_ms;
    while at <= to_ms && out.len() < MAX_FLOW_SAMPLES {
        let mut counts: BTreeMap<String, usize> = BOARD_ORDER
            .iter()
            .chain(std::iter::once(&BLOCKED_STATUS))
            .map(|status| ((*status).to_string(), 0))
            .collect();
        let (mut total, mut done, mut wip) = (0usize, 0usize, 0usize);
        for history in histories {
            let Some(status) = status_at(history, at) else {
                continue;
            };
            total += 1;
            *counts.entry(status.to_string()).or_insert(0) += 1;
            if status == FINISHED_STATUS {
                done += 1;
            } else if is_started(status) {
                wip += 1;
            }
        }
        out.push(FlowSample {
            at_ms: at,
            counts,
            total,
            done,
            work_in_progress: wip,
        });
        // The final sample lands exactly on `to_ms` even when the step
        // does not divide the window, so the chart ends at "now" rather
        // than at some point before it.
        if at == to_ms {
            break;
        }
        at = at.saturating_add(step_ms).min(to_ms);
    }
    out
}
