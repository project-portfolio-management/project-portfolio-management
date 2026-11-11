//! Row — `escalation_levels`. Append-only: the escalation level as the sponsor set it (EX-10).

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
    pub level: String,
    pub note: Option<String>,
    pub set_by: Option<String>,
    pub set_on: Date,
}
