//! Row — `programme_status_decisions`. Append-only: a programme is active until a
//! recorded decision pauses or stops it.

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
    /// `active`, `paused` or `stopped`.
    pub status: String,
    pub reason: String,
    pub decided_by: Option<String>,
    pub decided_on: Date,
    /// The gate review this decision follows from, if any.
    pub gate_review_pid: Option<Uuid>,
    /// The delivery exception this decision follows from, if any.
    pub exception_pid: Option<Uuid>,
}
