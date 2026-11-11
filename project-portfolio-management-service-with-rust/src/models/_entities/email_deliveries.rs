//! Row — `email_deliveries`: one email to one recipient, and how it ended.

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
    /// What sent it (`sink`, ...).
    pub kind: String,
    /// The record it is about (for a sink, the sink delivery).
    pub subject_ref: Uuid,
    pub recipient: String,
    pub subject: String,
    pub body_text: String,
    /// `queued`, `retry`, `sent` or `failed`.
    pub status: String,
    pub attempts: i32,
    pub last_error: Option<String>,
    /// The sender that made the last attempt (`smtp`, `none`, ...).
    pub sender: Option<String>,
    pub next_attempt_at: Option<DateTimeWithTimeZone>,
    pub sent_at: Option<DateTimeWithTimeZone>,
}
