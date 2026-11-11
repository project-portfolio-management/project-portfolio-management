//! Row — `time_entry_costs`. The cost of a recorded time entry as worked out when it was recorded: the rate used is
//! stored with it, so a later rate change never rewrites an actual.

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
    pub time_entry_pid: Uuid,
    pub plan_pid: Uuid,
    pub person_ref: String,
    pub table_pid: Uuid,
    pub spent_on: Date,
    pub minutes: i64,
    pub per_hour_minor: i64,
    pub currency: String,
    pub cost_minor: i64,
}
