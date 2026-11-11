//! Row — `director_packs`. One version of the monthly pack for one audience.

#![expect(missing_docs)]

use super::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub id: i64,
    pub pid: Uuid,
    /// `YYYY-MM`.
    pub period: String,
    pub audience: String,
    pub version: i32,
    /// `draft` or `issued`.
    pub status: String,
    pub cut_off: DateTimeWithTimeZone,
    /// The exact JSON text computed at drafting.
    pub body: String,
    pub change_note: Option<String>,
    pub supersedes_pid: Option<Uuid>,
    pub drafted_by: Option<String>,
    pub issued_by: Option<String>,
    pub issued_at: Option<DateTimeWithTimeZone>,
    /// `manual` or `scheduled` (drafted by the monthly schedule).
    pub origin: String,
    pub reviewed_by: Option<String>,
    pub reviewed_at: Option<DateTimeWithTimeZone>,
}
