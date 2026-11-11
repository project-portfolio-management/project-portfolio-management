//! Row — `sink_deliveries`. One attempt-tracked delivery of one notification to one sink (LT-6). Enqueued when the
//! notification is written and sent later, so a slow or dead destination can never delay or fail the notification
//! itself. Bounded retries with growing backoff; the last error is kept.

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
    pub sink_pid: Uuid,
    pub notification_pid: Uuid,
    pub status: String,
    pub attempts: i32,
    pub next_attempt_at: Option<DateTimeWithTimeZone>,
    pub last_error: Option<String>,
    pub delivered_at: Option<DateTimeWithTimeZone>,
}
