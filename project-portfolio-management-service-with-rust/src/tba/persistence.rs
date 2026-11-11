//! Persistence seam.

use super::{BTreeMap, Transition, default_classes, parse_classes};

/// One status change for [`insert_transition_on`]: the task, its plan, the statuses, when, who, and the reason a move
/// into `blocked` gives.
#[derive(Debug, Clone, Copy)]
pub struct NewTransition<'a> {
    /// The task that moved.
    pub task_pid: uuid::Uuid,
    /// Its plan (denormalized so a whole board is one query).
    pub plan_pid: uuid::Uuid,
    /// The status it left; `None` marks the task's creation.
    pub from_status: Option<&'a str>,
    /// The status it entered.
    pub to_status: &'a str,
    /// When.
    pub at: chrono::DateTime<chrono::Utc>,
    /// Who moved it.
    pub actor_ref: Option<&'a str>,
    /// Its assignee at the time.
    pub assignee_ref: Option<&'a str>,
    /// Why it is blocked, on a move into `blocked`.
    pub blocked_reason: Option<&'a str>,
}

/// Inserts the `task_transitions` row for one status change, in the caller's transaction ([`crate::db::Tx`]) so it
/// commits with the change that caused it (spec §5.1 invariant 3): a committed move without its transition would
/// silently shorten the item's recorded life, and nothing downstream could tell. Kept here rather than in a controller
/// so the callers (task creation, the board move, the imports, templates) cannot drift, and so the log's shape is
/// defined next to the analysis that reads it. The row: a new pid, `backfilled` false, and `blocked_reason` (a column
/// with no default, so an absent reason is the `NULL` an omitted column gets).
///
/// # Errors
///
/// When the insert fails.
pub async fn insert_transition_on(
    db: &mut impl crate::db::Session,
    t: NewTransition<'_>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO task_transitions
             (pid, task_pid, plan_pid, from_status, to_status, at, actor_ref, assignee_ref, backfilled,
              blocked_reason)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, false, $9)",
    )
    .bind(uuid::Uuid::new_v4())
    .bind(t.task_pid)
    .bind(t.plan_pid)
    .bind(t.from_status)
    .bind(t.to_status)
    .bind(chrono::DateTime::<chrono::FixedOffset>::from(t.at))
    .bind(t.actor_ref)
    .bind(t.assignee_ref)
    .bind(t.blocked_reason)
    .execute(db.ex())
    .await
    .map(|_| ())
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
