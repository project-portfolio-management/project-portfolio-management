//! **Time-based analysis** (TBA) — pure computation over a task's
//! status-transition log. No I/O, no clock read (`as_of` is always a
//! parameter), fully deterministic, and therefore unit-testable without
//! a database.
//!
//! See `spec/time-based-analysis.md` for the contract. The rules this
//! module exists to enforce, and which a later "simplification" would
//! quietly destroy:
//!
//! 1. **Cycle time and lead time are different numbers** (§6.1). An
//!    item that sat in `todo` for three weeks and was built in two calendar days
//!    has a cycle time of 2 calendar days and a lead time of 23. Reporting the
//!    first as "our delivery time" is a tenfold flattering misreport,
//!    and it is the commonest error in the field — so both are always
//!    returned, and never one labelled as the other.
//! 2. **The denominator is elapsed calendar time** (§6.3), never the
//!    sum of recorded activity. Here that is structural: the intervals
//!    come from one ordered log and partition the elapsed time exactly,
//!    so there is no unrecorded remainder to omit.
//! 3. **No status is excluded and no clock pauses** (§12.3). Every
//!    millisecond from creation to finish lands in exactly one status
//!    and one category, and they sum to the lead time by construction —
//!    a property test, not a promise. Lead time rather than cycle time
//!    is the partitioned span, because the backlog dwell is real time
//!    the requester waited and must land somewhere; flow efficiency is
//!    still measured against cycle time, since the team cannot be held
//!    to the backlog's dwell.
//! 4. **Nothing here is per-person** (§12.4). Handoff counts describe
//!    the item's journey; there is deliberately no per-assignee cycle
//!    time, and adding one would destroy the data quality the free-at-
//!    collection design depends on.
//!
//! Vocabulary is the value-stream-mapping one (VA / NNVA / UNVA, LT /
//! PT / VT / %A / #HO / RFPY) and the queueing-theory one (λ / μ / ρ /
//! τ / κ); see the spec's §2 for provenance.

use std::collections::BTreeMap;

use serde::Serialize;

/// Milliseconds in one day.
pub const DAY_MS: i64 = 86_400_000;

/// The value-adding category: the product is being built.
pub const CATEGORY_VALUE_ADDING: &str = "value_adding";

/// Necessary non-value-adding: required, but not building.
pub const CATEGORY_NECESSARY: &str = "necessary_non_value_adding";

/// Unnecessary non-value-adding: queueing, blocking, rework.
pub const CATEGORY_UNNECESSARY: &str = "unnecessary_non_value_adding";

/// The VSM categories. Closed vocabulary.
pub const CATEGORIES: &[&str] = &[
    CATEGORY_VALUE_ADDING,
    CATEGORY_NECESSARY,
    CATEGORY_UNNECESSARY,
];

/// The board status that means finished. Time stops here.
pub const FINISHED_STATUS: &str = "done";

/// The board status that means not yet started — the backlog.
pub const BACKLOG_STATUS: &str = "todo";

/// The board status that means externally halted.
pub const BLOCKED_STATUS: &str = "blocked";

/// Why a task was blocked, recorded on the move into [`BLOCKED_STATUS`]
/// (spec §17). Fixed, so blocked time can be totalled by reason. The same
/// list is a `CHECK` on `task_transitions.blocked_reason`.
pub const BLOCKED_REASONS: [&str; 5] = [
    "waiting_on_person",
    "external_dependency",
    "awaiting_decision",
    "awaiting_resource",
    "other",
];

/// The key under which blocked time with no recorded reason is reported.
pub const UNRECORDED_REASON: &str = "unrecorded";

/// Check a move's blocked reason: only a move into `blocked` may carry
/// one, and it must be in [`BLOCKED_REASONS`].
///
/// # Errors
///
/// A sentence naming the problem, for a `422`.
pub fn check_blocked_reason(to_status: &str, reason: Option<&str>) -> Result<(), String> {
    let Some(reason) = reason else {
        return Ok(());
    };
    if to_status != BLOCKED_STATUS {
        return Err(format!(
            "`blocked_reason` belongs on a move into `{BLOCKED_STATUS}`, not `{to_status}`"
        ));
    }
    if BLOCKED_REASONS.contains(&reason) {
        Ok(())
    } else {
        Err(format!(
            "`blocked_reason` must be one of {BLOCKED_REASONS:?}, not `{reason}`"
        ))
    }
}

/// Blocked time by reason: each `blocked` interval takes the reason of the
/// latest move into `blocked` at or before it, else [`UNRECORDED_REASON`].
/// Sums to the `blocked` interval total.
#[must_use]
pub fn blocked_by_reason(transitions: &[Transition], spans: &[Interval]) -> BTreeMap<String, i64> {
    let mut entries: Vec<&Transition> = transitions
        .iter()
        .filter(|t| t.to_status == BLOCKED_STATUS)
        .collect();
    entries.sort_by_key(|t| t.at_ms);
    let mut out: BTreeMap<String, i64> = BTreeMap::new();
    for span in spans.iter().filter(|s| s.status == BLOCKED_STATUS) {
        let reason = entries
            .iter()
            .rev()
            .find(|t| t.at_ms <= span.from_ms)
            .and_then(|t| t.blocked_reason.clone())
            .unwrap_or_else(|| UNRECORDED_REASON.to_string());
        let slot = out.entry(reason).or_insert(0);
        *slot = slot.saturating_add(span.duration_ms());
    }
    out
}

/// The board order used to decide whether a move was **backwards**
/// (§6.5). `blocked` is deliberately absent: it is orthogonal to
/// progress, so moving into or out of it is never rework.
pub const BOARD_ORDER: &[&str] = &["todo", "in_progress", "in_review", "done"];

/// The **disclosed default** status classification (§5.3).
///
/// `todo` is `inventory` waste rather than merely "not started": work
/// bought and not yet used, aging while it waits. `in_review` is
/// necessary — review is not waste — even though review *waiting* is,
/// and the board cannot tell the two apart (spec §17).
pub const DEFAULT_CLASSES: &[(&str, &str)] = &[
    (BACKLOG_STATUS, CATEGORY_UNNECESSARY),
    ("in_progress", CATEGORY_VALUE_ADDING),
    ("in_review", CATEGORY_NECESSARY),
    (BLOCKED_STATUS, CATEGORY_UNNECESSARY),
];

/// The VSM waste type a non-value-adding status represents.
#[must_use]
pub fn waste_for(status: &str) -> Option<&'static str> {
    match status {
        BACKLOG_STATUS => Some("inventory"),
        BLOCKED_STATUS => Some("waiting"),
        _ => None,
    }
}

/// The default classification as a map.
#[must_use]
pub fn default_classes() -> BTreeMap<String, String> {
    DEFAULT_CLASSES
        .iter()
        .map(|(status, category)| ((*status).to_string(), (*category).to_string()))
        .collect()
}

/// Parse the `PROJECT_PORTFOLIO_MANAGEMENT_FLOW_CLASSES` override — a
/// map of status → VSM category.
///
/// Absent / blank / unparsable / an unknown category ⇒ `None`, and the
/// caller falls back to [`default_classes`] **whole**. Half-applying an
/// override would produce a figure that matches no stated
/// classification at all, which is worse than ignoring it — the same
/// posture as the existing WIP-limit parser.
#[must_use]
pub fn parse_classes(raw: Option<&str>) -> Option<BTreeMap<String, String>> {
    let raw = raw?.trim();
    if raw.is_empty() {
        return None;
    }
    let parsed: BTreeMap<String, String> = serde_json::from_str(raw).ok()?;
    if parsed.is_empty() {
        return None;
    }
    for category in parsed.values() {
        if !CATEGORIES.contains(&category.as_str()) {
            return None;
        }
    }
    Some(parsed)
}

/// Classify a status, falling back to `unnecessary_non_value_adding`
/// for anything the map does not name.
///
/// The fallback is deliberately the *pessimistic* one: an unclassified
/// status counts against you, so adding a board column cannot silently
/// improve the flow efficiency.
#[must_use]
pub fn classify<'a>(classes: &'a BTreeMap<String, String>, status: &str) -> &'a str {
    classes
        .get(status)
        .map_or(CATEGORY_UNNECESSARY, String::as_str)
}

/// Whether a status counts as started (anything past the backlog).
#[must_use]
pub fn is_started(status: &str) -> bool {
    status != BACKLOG_STATUS
}

/// Whether a move went backwards on the board (§6.5). Moves into or out
/// of `blocked` are never rework.
#[must_use]
pub fn is_backwards(from: &str, to: &str) -> bool {
    if from == BLOCKED_STATUS || to == BLOCKED_STATUS {
        return false;
    }
    match (
        BOARD_ORDER.iter().position(|s| *s == from),
        BOARD_ORDER.iter().position(|s| *s == to),
    ) {
        (Some(a), Some(b)) => b < a,
        _ => false,
    }
}

mod attrition;
mod cohort;
mod constraints;
mod cumulative;
mod flow;
mod forecast;
mod gauges;
mod inputs;
mod outputs;
mod persistence;
mod rollup;

pub use attrition::*;
pub use cohort::*;
pub use constraints::*;
pub use cumulative::*;
pub use flow::*;
pub use forecast::*;
pub use gauges::*;
pub use inputs::*;
pub use outputs::*;
pub use persistence::*;
pub use rollup::*;

#[cfg(test)]
mod tests;
