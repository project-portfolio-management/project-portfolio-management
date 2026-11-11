//! Row — `programme_triages`. Append-only: a dated triage of a programme (EX-7).

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
    pub triaged_on: Date,
    /// `finish`, `fix`, `pause` or `stop`.
    pub outcome: String,
    pub reason: String,
    pub decided_by: Option<String>,
}
