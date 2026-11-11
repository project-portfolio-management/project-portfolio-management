//! Row — `buffer_plans`. Append-only: the critical chain and its buffers as planned (CC-4).

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
    pub plan_pid: Uuid,
    pub taken_on: Date,
    /// The frozen plan as JSON text: the chain, the buffers and the planned finishes.
    pub document: String,
}
