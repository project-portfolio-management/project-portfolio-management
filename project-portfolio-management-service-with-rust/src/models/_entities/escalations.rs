//! Row — `escalations`. A project referring something upward at a level of its governance escalation
//! path. Its history is in `escalation_events`.

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
    pub title: String,
    pub description: Option<String>,
    pub reason_kind: String,
    pub tolerance_dimension: Option<String>,
    pub level: i32,
    pub escalated_to: Option<String>,
    pub owner_ref: Option<String>,
    pub status: String,
    pub raised_by: Option<String>,
    pub raised_on: Date,
    pub respond_by: Option<Date>,
    pub outcome: Option<String>,
    pub response: Option<String>,
    pub resolved_on: Option<Date>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
