//! Row — `key_results`. The measurable half of an OKR.
//! See entity spec §5.9.2 / FR-27.

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
    pub objective_pid: Uuid,
    pub title: String,
    pub metric: String,
    pub direction: String,
    /// The baseline, captured once. **Never updated** — progress
    /// measured from a moving baseline is not progress.
    pub start_value: i64,
    pub target_value: i64,
    pub current_value: i64,
    pub tolerance: Option<i64>,
    pub unit: Option<String>,
    pub currency: Option<String>,
    pub owner_ref: Option<String>,
    pub due_date: Option<Date>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
