//! Row — `cost_rates`. A rate in a table, effective from a date. Append-only.

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
    pub table_pid: Uuid,
    pub effective_from: Date,
    /// Minor units of the table currency per hour.
    pub per_hour_minor: i64,
    pub created_by: Option<String>,
}
