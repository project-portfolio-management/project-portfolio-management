//! Row — `status_updates`. One on-track / at-risk / off-track call on a plan (NC-4). Append-only.

#![expect(missing_docs)]

use super::{Date, DateTimeWithTimeZone, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub id: i64,
    pub pid: Uuid,
    pub plan_pid: Uuid,
    pub status: String,
    pub narrative: String,
    pub posted_on: Date,
    pub posted_by: Option<String>,
    /// Claims with their sources, as ids (BR-2).
    pub claims: Json,
}
