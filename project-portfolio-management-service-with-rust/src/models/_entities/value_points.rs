//! Row — `value_points`. An observed value, carrying **how** it was arrived at — a
//! measured figure and an asserted one are different evidence.
//! See entity spec §5.9.6.

// The row's shape is documented by the migration.
#![expect(missing_docs)]

use super::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub id: i32,
    pub pid: Uuid,
    pub plan_pid: Uuid,
    pub benefit_pid: Option<Uuid>,
    pub observed_at: DateTimeWithTimeZone,
    pub value: i64,
    pub currency: Option<String>,
    pub is_first_measurable: bool,
    pub method: String,
    pub evidence_ref: Option<String>,
    pub actor: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
