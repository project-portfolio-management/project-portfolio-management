//! Row — `plan_tolerances`. How far a plan may drift on one dimension (time, cost, risk, resources)
//! before it must be referred upward. A plan with no row for a dimension inherits its nearest ancestor's.

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
    pub plan_pid: Uuid,
    pub dimension: String,
    pub limit_value: i64,
    pub amber_from_percent: i32,
    pub note: Option<String>,
    pub set_by: Option<String>,
}
