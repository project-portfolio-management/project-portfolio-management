//! Row — `plan_status_cadences`. How often a plan expects a status update (NC-4).

#![expect(missing_docs)]

use super::{Date, DateTimeWithTimeZone};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub id: i64,
    pub plan_pid: Uuid,
    pub cadence_days: i32,
    pub set_on: Date,
    pub set_by: Option<String>,
    pub reminded_for: Option<Date>,
}
