//! Row — `kpis`. A standing measure with a target and a red line (KPI). Its status is derived on read from the latest
//! reading, never stored.

#![expect(missing_docs)]

use super::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub id: i64,
    pub pid: Uuid,
    /// The plan it measures; none for a portfolio-level KPI.
    pub plan_pid: Option<Uuid>,
    /// The objective it supports, if any.
    pub objective_pid: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub unit: Option<String>,
    /// `higher_better` or `lower_better`.
    pub direction: String,
    pub target_value: i64,
    /// The line at which it turns red (beyond it is red).
    pub red_value: i64,
    /// How often it should be read: `daily`, `weekly`, `monthly`, `quarterly`.
    pub cadence: Option<String>,
    pub owner_ref: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
