//! Row — `business_case_targets`. The charter or gate-approved target. `approved_at` is the
//! Time-to-Value clock start and is never updated.
//! See entity spec §5.9.6.

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
    pub plan_pid: Uuid,
    pub metric: String,
    pub baseline_value: i64,
    pub target_value: i64,
    pub unit: Option<String>,
    pub currency: Option<String>,
    pub promised_by: Option<Date>,
    pub source: String,
    pub approved_at: DateTimeWithTimeZone,
    pub approved_by_ref: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
