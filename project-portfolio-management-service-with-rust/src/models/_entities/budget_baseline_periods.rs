//! Row — `budget_baseline_periods`. Planned cost for one
//! window of a [`super::budget_baselines`] version.

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
    pub baseline_pid: Uuid,
    pub period_start: Date,
    pub period_end: Date,
    pub planned_minor: i64,
}
