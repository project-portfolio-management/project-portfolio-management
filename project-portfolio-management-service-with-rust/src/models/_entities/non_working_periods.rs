//! Row — `non_working_periods`. Leave, holiday, study leave
//! and non-project duty, which **subtract from the utilization
//! denominator** rather than counting as idle capacity. See entity spec
//! §5.9.6 / FR-35.

// The row's shape is documented by the migration.
#![expect(missing_docs)]

use super::{Date, DateTimeWithTimeZone};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub id: i32,
    pub pid: Uuid,
    pub person_ref: String,
    pub starts_on: Date,
    pub ends_on: Date,
    pub kind: String,
    pub note: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
