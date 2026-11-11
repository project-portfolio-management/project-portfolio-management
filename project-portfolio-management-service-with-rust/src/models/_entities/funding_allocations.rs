//! Row — `funding_allocations`. Append-only: money a funding source allocated, on a date (EX-12).

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
    pub funding_source_pid: Uuid,
    pub amount_minor: i64,
    pub currency: String,
    pub allocated_on: Date,
    pub reference: Option<String>,
    pub created_by: Option<String>,
}
