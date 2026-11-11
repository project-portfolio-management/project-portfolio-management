//! Row — `cutover_cosignatures`. Append-only: a role's co-signature of a go decision (EX-6).

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
    pub role: String,
    pub signed_by: Option<String>,
    pub signed_on: Date,
}
