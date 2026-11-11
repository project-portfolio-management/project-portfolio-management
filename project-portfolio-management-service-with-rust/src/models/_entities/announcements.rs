//! Row — `announcements`. A message to everyone, with a severity, an optional pin and an optional window.

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
    pub title: String,
    pub body: String,
    /// `info`, `important` or `urgent`.
    pub severity: String,
    pub pinned: bool,
    pub publish_at: DateTimeWithTimeZone,
    pub expires_at: Option<DateTimeWithTimeZone>,
    pub author_ref: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
