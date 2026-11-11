//! Row — `kpi_readings`. One observation of a KPI, dated by when it was observed. Append-only: a correction is a newer
//! reading.

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
    pub kpi_pid: Uuid,
    pub observed_on: Date,
    pub value: i64,
    pub note: Option<String>,
    pub actor: Option<String>,
}
