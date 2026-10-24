//! Persistence seam.

use super::{BTreeMap, Transition, default_classes, parse_classes};

/// Build the `task_transitions` row for one status change.
///
/// Kept here rather than in the controller so the two call sites — task
/// creation and the board move — cannot drift, and so the log's shape
/// is defined next to the analysis that reads it. The caller inserts it
/// **inside the same transaction as the change that caused it**
/// (spec §5.1 invariant 3): a committed move without its transition
/// would silently shorten the item's recorded life, and nothing
/// downstream could tell.
#[must_use]
pub fn transition_row(
    task_pid: uuid::Uuid,
    plan_pid: uuid::Uuid,
    from_status: Option<String>,
    to_status: String,
    at: chrono::DateTime<chrono::Utc>,
    actor_ref: Option<String>,
    assignee_ref: Option<String>,
) -> crate::models::_entities::task_transitions::ActiveModel {
    use sea_orm::ActiveValue;
    crate::models::_entities::task_transitions::ActiveModel {
        pid: ActiveValue::set(uuid::Uuid::new_v4()),
        task_pid: ActiveValue::set(task_pid),
        plan_pid: ActiveValue::set(plan_pid),
        from_status: ActiveValue::set(from_status),
        to_status: ActiveValue::set(to_status),
        at: ActiveValue::set(at.into()),
        actor_ref: ActiveValue::set(actor_ref),
        assignee_ref: ActiveValue::set(assignee_ref),
        backfilled: ActiveValue::set(false),
        ..Default::default()
    }
}

/// A stored transition row → the pure analysis input.
#[must_use]
pub fn to_transition(row: &crate::models::_entities::task_transitions::Model) -> Transition {
    Transition {
        from_status: row.from_status.clone(),
        to_status: row.to_status.clone(),
        at_ms: row.at.timestamp_millis(),
        assignee_ref: row.assignee_ref.clone(),
        backfilled: row.backfilled,
        blocked_reason: row.blocked_reason.clone(),
    }
}

/// The classification map in force: the deployment override where it
/// parses whole, else the disclosed default (spec §5.3).
#[must_use]
pub fn classes_in_force() -> BTreeMap<String, String> {
    parse_classes(
        std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_FLOW_CLASSES")
            .ok()
            .as_deref(),
    )
    .unwrap_or_else(default_classes)
}
