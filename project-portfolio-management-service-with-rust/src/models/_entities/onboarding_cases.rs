//! Row — `onboarding_cases`. One onboarding or offboarding, of a person on a plan or of a plan itself.

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
    pub kind: String,
    pub plan_pid: Uuid,
    pub subject_ref: Option<String>,
    pub title: String,
    pub effective_date: Date,
    /// `open`, `completed` or `cancelled`.
    pub status: String,
    pub created_by: Option<String>,
    pub closed_at: Option<DateTimeWithTimeZone>,
    pub closed_reason: Option<String>,
}
