//! Row — `delivery_confidence_assessments`. Append-only history of a plan's stated delivery confidence.

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
    /// `green`, `amber_green`, `amber`, `amber_red` or `red`.
    pub value: String,
    /// `self` or `independent`.
    pub kind: String,
    pub rationale: String,
    pub assessor: Option<String>,
    pub assessed_on: Date,
}
