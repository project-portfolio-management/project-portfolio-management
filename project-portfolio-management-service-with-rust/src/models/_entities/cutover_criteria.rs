//! Row — `cutover_criteria`. A stop criterion of a cutover decision, published before the decision (EX-6).

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
    pub decision_pid: Uuid,
    pub position: i32,
    pub criterion: String,
    pub met: Option<bool>,
    pub evidence: Option<String>,
    pub marked_by: Option<String>,
    pub marked_on: Option<Date>,
}
