//! Row — `time_entries`. Recorded effort: **an assertion
//! typed by a person**, not an observation. See entity spec §5.9.3.

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
    pub plan_pid: Uuid,
    pub task_pid: Option<Uuid>,
    pub actor_ref: String,
    pub spent_on: Date,
    pub minutes: i64,
    pub category: String,
    pub billable: bool,
    pub note: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
