//! Row — `planned_slots`. A person's own time-blocking of a task (PL-4). Instants are UTC (`timestamptz`) and are
//! rendered in the caller's zone on read. A slot never changes the task's due date, and overlapping slots are allowed —
//! reported as clashes, not refused.

#![expect(missing_docs)]

use super::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub id: i64,
    pub pid: Uuid,
    pub sub: String,
    pub task_pid: Uuid,
    pub starts_at: DateTimeWithTimeZone,
    pub ends_at: DateTimeWithTimeZone,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
