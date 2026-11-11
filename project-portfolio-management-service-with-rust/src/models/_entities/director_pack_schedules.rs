//! Row — `director_pack_schedules`. A monthly schedule that drafts (never issues) a pack.

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
    pub audience: String,
    pub day_of_month: i32,
    pub next_run_date: Date,
    pub enabled: bool,
    pub last_run_on: Option<Date>,
    pub last_result: Option<String>,
    pub created_by: Option<String>,
}
