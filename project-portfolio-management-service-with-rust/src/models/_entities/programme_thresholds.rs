//! Row — `programme_thresholds`. A programme's pause, reset or stop threshold (EX-7).

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
    /// `confidence_red_packs`, `forecast_past` or `p80_cap`.
    pub kind: String,
    pub count_value: Option<i32>,
    pub date_value: Option<Date>,
    pub amount_minor: Option<i64>,
    pub currency: Option<String>,
}
