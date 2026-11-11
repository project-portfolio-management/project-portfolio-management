//! Row — `threshold_decisions`. Append-only: what was decided when a threshold was crossed (EX-7).

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
    pub exception_pid: Uuid,
    pub plan_pid: Uuid,
    pub threshold_kind: String,
    /// `pause`, `reset`, `stop` or `override`.
    pub action: String,
    /// An override ends on this date.
    pub expiry: Option<Date>,
    pub reason: String,
    pub decided_by: Option<String>,
    pub decided_on: Date,
}
