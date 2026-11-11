//! Row — `buffer_recovery_actions`. What was agreed about a buffer in the red; closed once, never deleted (CC-4).

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
    /// `project` or `feeding:<join task>:<first task>` in the buffer plan.
    pub buffer_key: String,
    pub action: String,
    pub owner_ref: Option<String>,
    pub due_date: Option<Date>,
    /// `open`, `done` or `dropped`.
    pub status: String,
    pub closed_on: Option<Date>,
}
