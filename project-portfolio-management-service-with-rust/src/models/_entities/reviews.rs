//! Row — `reviews`. A collaborative-review invitation: one
//! subject (idea / proposal / plan) delegated to one internal or
//! external expert, with their verdict once submitted.

// The row's shape is documented by the migration.
#![expect(missing_docs)]

use super::{Date, DateTimeWithTimeZone};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub id: i32,
    pub pid: Uuid,
    pub subject_kind: String,
    pub subject_pid: Uuid,
    pub reviewer_ref: String,
    pub reviewer_scope: String,
    pub expertise: Option<String>,
    pub status: String,
    pub due_on: Option<Date>,
    pub score: Option<i32>,
    pub recommendation: Option<String>,
    pub comment: Option<String>,
    pub invited_by: Option<String>,
    pub responded_at: Option<DateTimeWithTimeZone>,
    pub submitted_at: Option<DateTimeWithTimeZone>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
