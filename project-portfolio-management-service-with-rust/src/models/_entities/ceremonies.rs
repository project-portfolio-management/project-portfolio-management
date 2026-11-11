//! Row — `ceremonies`. Planning / daily / review /
//! retrospective against a sprint. See entity spec §5.9.3 / FR-29.

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
    pub kind: String,
    pub held_at: DateTimeWithTimeZone,
    pub facilitator_ref: Option<String>,
    pub summary: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
