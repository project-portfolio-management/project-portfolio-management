//! Row — `cost_rate_tables`. A named rate table (standard, overtime, …) for one person or one role, in one currency.

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
    pub name: String,
    pub person_ref: Option<String>,
    pub role: Option<String>,
    pub currency: String,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
