//! Row — `benefits`. Value-realization records (PPM-11):
//! financial targets in minor units, non-financial as notes.

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
    pub title: String,
    pub category: String,
    pub currency: Option<String>,
    pub target_minor: Option<i64>,
    pub realized_minor: i64,
    pub target_note: Option<String>,
    pub realized_note: Option<String>,
    pub expected_on: Option<Date>,
    pub status: String,
    /// `recurring` or `one_off` (DP-9); none when not stated.
    pub recurrence: Option<String>,
    /// The yearly amount of a recurring benefit, in minor units.
    pub annualized_minor: Option<i64>,
    /// `cash_releasing`, `non_cash_releasing` or `societal`.
    pub classification: Option<String>,
    /// The KPI that measures a non-financial benefit.
    pub kpi_pid: Option<Uuid>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
