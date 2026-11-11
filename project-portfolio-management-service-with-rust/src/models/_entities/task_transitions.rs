//! Row — `task_transitions`. The append-only task status
//! transition log, from which every time-based-analysis interval is
//! derived. See `spec/time-based-analysis.md` §5.1.

// The row's shape is documented by the migration.
#![expect(missing_docs)]

use super::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub id: i32,
    pub pid: Uuid,
    pub task_pid: Uuid,
    /// Denormalized so a whole board is one query.
    pub plan_pid: Uuid,
    /// `None` marks the task's creation.
    pub from_status: Option<String>,
    pub to_status: String,
    pub at: DateTimeWithTimeZone,
    pub actor_ref: Option<String>,
    pub assignee_ref: Option<String>,
    /// Synthesized by the migration rather than observed — reported by
    /// every analysis so thin evidence stays visible.
    pub backfilled: bool,
    /// Why the task was blocked, on a transition into `blocked`; one of
    /// [`crate::tba::BLOCKED_REASONS`]. `None` when not recorded.
    pub blocked_reason: Option<String>,
}
