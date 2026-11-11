//! Row — `work_calendars`. A working calendar (SC-2): the weekdays that work and a list of holidays, for one plan or
//! one person. A task is scheduled on its plan's calendar; an assigned person's own calendar (plus their recorded
//! absences) narrows it. Also adds the per-task date constraint and deadline columns the critical-path engine reads.

#![expect(missing_docs)]

use super::{DateTimeWithTimeZone, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub id: i64,
    pub pid: Uuid,
    pub scope: String,
    pub subject: String,
    pub weekdays: String,
    pub holidays: Json,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
