//! Row — `meetings`. An agenda, minutes and outcomes for one plan (NC-14).

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
    pub scheduled_on: Date,
    pub status: String,
    pub minutes: Option<String>,
    pub held_on: Option<Date>,
    pub created_by: Option<String>,
    pub held_by: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
