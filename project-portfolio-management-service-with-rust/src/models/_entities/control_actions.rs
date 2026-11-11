//! Row — `control_actions`. The fourth step of the
//! Controlling process: what a failing reading provoked. Converts into
//! the task and issue stores that already exist rather than becoming a
//! fifth one. See entity spec §5.9.8 / FR-38.

// The row's shape is documented by the migration.
#![expect(missing_docs)]

use super::{Date, DateTimeWithTimeZone};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub id: i32,
    pub pid: Uuid,
    pub reading_pid: Uuid,
    pub kind: String,
    pub description: String,
    pub owner_ref: Option<String>,
    pub due_date: Option<Date>,
    pub converted_task_pid: Option<Uuid>,
    pub converted_issue_pid: Option<Uuid>,
    pub closed_at: Option<DateTimeWithTimeZone>,
    pub outcome: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
