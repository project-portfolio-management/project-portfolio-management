//! Row — `criterion_evidence`. Append-only: whether a criterion was met in a quarter, with the evidence (EX-10).

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
    pub criterion_pid: Uuid,
    /// `YYYY-Qn`.
    pub quarter: String,
    pub met: bool,
    pub link: Option<String>,
    pub note: String,
    pub recorded_by: Option<String>,
    pub recorded_on: Date,
}
