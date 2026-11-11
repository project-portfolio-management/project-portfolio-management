//! Row — `scheduled_actions`. The set-and-forget queue: an
//! action configured once, held until `due_at`, then fired by the
//! sweep (endpoint or the env-gated ticker) exactly once.

// The row's shape is documented by the migration.
#![expect(missing_docs)]

use super::{DateTimeWithTimeZone, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub id: i32,
    pub pid: Uuid,
    pub subject_kind: String,
    pub subject_pid: Uuid,
    pub action_kind: String,
    pub payload: Json,
    pub due_at: DateTimeWithTimeZone,
    pub status: String,
    pub source_automation_pid: Option<Uuid>,
    pub created_by: Option<String>,
    pub fired_at: Option<DateTimeWithTimeZone>,
    pub outcome: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
