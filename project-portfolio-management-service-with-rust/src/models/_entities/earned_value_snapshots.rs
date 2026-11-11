//! Row — `earned_value_snapshots`. A day's earned-value figures for one plan in one currency (SC-5): what was planned,
//! earned and spent as of that date. Written once a day by the scheduler (or on request) so the S-curve has a real
//! history to draw: earned value and actual cost are not dated at source, so the past cannot be reconstructed, only
//! recorded going forward. One row per plan, currency and day; a second snapshot the same day replaces the first.

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
    pub currency: String,
    pub as_of: Date,
    pub baseline_version: i32,
    pub bac_minor: i64,
    pub pv_minor: i64,
    pub ev_minor: i64,
    pub ac_minor: Option<i64>,
}
