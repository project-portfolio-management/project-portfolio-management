//! Row — `earned_value_settings`. A plan's earned-value setting (SC-4): how its tasks are counted by default
//! (`zero_hundred`, `fifty_fifty`, or `points`, which weights by story points). A plan with no row counts
//! `zero_hundred`. Also adds the per-task override columns: a task may be counted another way, or by a person-entered
//! percent with a mandatory note.

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
    pub default_method: String,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
