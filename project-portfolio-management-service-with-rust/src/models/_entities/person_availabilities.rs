//! Row — `person_availabilities`. A person's availability over a date range (PJ-3): the most of their time (0-100
//! percent) they can commit between two dates, for part-time spells, leave blocks and ramp-up. Where no range applies
//! capacity is 100 percent, so nothing stored before this existed changes. Ranges for one person may not overlap
//! (checked by the service).

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
    pub from_date: Date,
    pub to_date: Date,
    pub max_percent: i32,
    pub note: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
