//! Row — `start_queue_entries`. A programme waiting to start (EX-8).

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
    /// `waiting`, `started` or `withdrawn`.
    pub status: String,
    /// Why it was refused, as JSON text.
    pub refusals: String,
    pub queued_on: Date,
    pub resolved_on: Option<Date>,
}
