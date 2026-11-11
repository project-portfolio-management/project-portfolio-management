//! Row — `access_records`. Access granted or revoked, append-only.

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
    pub case_pid: Option<Uuid>,
    pub person_ref: String,
    pub access: String,
    /// `granted` or `revoked`.
    pub action: String,
    pub recorded_on: Date,
    pub recorded_by: Option<String>,
    pub note: Option<String>,
}
