//! Row — `satisfaction_responses`. NPS / CSAT sentiment about a plan. Stores a **role, never
//! an identity**.
//! See entity spec §5.9.6.

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
    pub plan_pid: Uuid,
    pub surveyed_at: DateTimeWithTimeZone,
    pub instrument: String,
    pub score: i16,
    pub respondent_role: String,
    pub comment: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
