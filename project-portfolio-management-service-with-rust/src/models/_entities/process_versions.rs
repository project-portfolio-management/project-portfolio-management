//! Row — `process_versions`. One immutable version of a process: its ordered steps.

#![expect(missing_docs)]

use super::{DateTimeWithTimeZone, Json};
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
    pub steps: Json,
    pub created_by: Option<String>,
}
