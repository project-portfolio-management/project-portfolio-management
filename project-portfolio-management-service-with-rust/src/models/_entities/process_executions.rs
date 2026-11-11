//! Row — `process_executions`. One run of a process on a subject, pinned to a version.

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
    pub process_pid: Uuid,
    pub version: i32,
    pub subject_kind: String,
    pub subject_pid: Uuid,
    /// `running`, `completed`, `rejected`, `cancelled` or `failed`.
    pub status: String,
    /// The index of the step it is on (or the number of steps once finished).
    pub current_step: i32,
    pub started_by: String,
    pub finished_at: Option<DateTimeWithTimeZone>,
    pub note: Option<String>,
}
