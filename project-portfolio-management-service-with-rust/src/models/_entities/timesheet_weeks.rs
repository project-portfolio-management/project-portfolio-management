//! Row — `timesheet_weeks`. The approval state of one person's week of time entries (LT-11). A row exists only once a
//! week has been submitted; no row means the week is open. Approved weeks are locked against new entries. `week_start`
//! is the ISO Monday, a zone-free calendar date.

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
    pub person_ref: String,
    pub week_start: Date,
    pub status: String,
    pub submitted_at: Option<DateTimeWithTimeZone>,
    pub decided_by: Option<String>,
    pub decided_at: Option<DateTimeWithTimeZone>,
    pub decision_comment: Option<String>,
}
