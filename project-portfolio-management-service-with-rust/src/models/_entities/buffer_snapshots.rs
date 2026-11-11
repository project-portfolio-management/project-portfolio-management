//! Row — `buffer_snapshots`. Append-only: every buffer's reading at a moment (CC-4).

#![expect(missing_docs)]

use super::{Date, DateTimeWithTimeZone};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub id: i64,
    pub pid: Uuid,
    pub plan_pid: Uuid,
    pub buffer_plan_pid: Uuid,
    pub taken_on: Date,
    /// The readings as JSON text.
    pub document: String,
}
