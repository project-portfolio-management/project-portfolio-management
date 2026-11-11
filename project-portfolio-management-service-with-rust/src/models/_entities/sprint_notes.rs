//! Row — `sprint_notes`. Retro / feedback log entries per
//! sprint (`went_well` / `improve` / `action` / `feedback`);
//! convertible to tasks (`task_pid` records the conversion).

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
    pub sprint_pid: Uuid,
    pub category: String,
    pub body: String,
    pub task_pid: Option<Uuid>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
