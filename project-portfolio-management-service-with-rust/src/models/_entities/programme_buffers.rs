//! Row — `programme_buffers`. Append-only: a programme's project buffer, in business days.

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
    pub buffer_days: i32,
    /// `declared` (a person chose the number) or `rule` (a stated rule gave it).
    pub source: String,
    pub rule: Option<String>,
    pub note: Option<String>,
    pub declared_by: Option<String>,
    pub declared_on: Date,
    /// The schedule baseline version in force when it was set.
    pub baseline_version: Option<i32>,
}
