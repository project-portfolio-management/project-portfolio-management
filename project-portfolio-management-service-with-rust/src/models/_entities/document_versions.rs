//! Row — `document_versions`. One version of a controlled document.

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
    pub document_pid: Uuid,
    pub version: i32,
    pub body: String,
    pub change_summary: Option<String>,
    /// `draft`, `in_review`, `approved`, `effective` or `superseded`.
    pub status: String,
    pub approval_pid: Option<Uuid>,
    pub effective_from: Option<Date>,
    pub superseded_on: Option<Date>,
    pub superseded_by: Option<Uuid>,
    pub created_by: Option<String>,
}
