//! Row — `milestone_signoffs`. Append-only: a sign-off or a withdrawal of one (EX-4).

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
    pub milestone_pid: Uuid,
    pub organization_pid: Uuid,
    /// `sign` or `withdraw`.
    pub action: String,
    pub reason: Option<String>,
    pub acted_on: Date,
    pub signed_by: Option<String>,
}
