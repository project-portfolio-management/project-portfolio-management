//! Row — `onboarding_items`. A checklist item of a case.

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
    pub case_pid: Uuid,
    pub position: i32,
    pub title: String,
    pub owner_ref: Option<String>,
    pub due_date: Option<Date>,
    /// `done` or `skipped`; null while open.
    pub resolution: Option<String>,
    pub note: Option<String>,
    pub resolved_at: Option<DateTimeWithTimeZone>,
    pub resolved_by: Option<String>,
}
