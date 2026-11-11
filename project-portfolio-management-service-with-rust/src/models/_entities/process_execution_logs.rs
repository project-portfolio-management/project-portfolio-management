//! Row — `process_execution_logs`. The append-only log of an execution.

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
    pub execution_pid: Uuid,
    pub step_index: Option<i32>,
    pub event: String,
    pub actor: Option<String>,
    pub note: Option<String>,
}
