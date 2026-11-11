//! Row — `notification_prefs` (PL-5). Per-user notification
//! kind × channel choices, keyed by the token `sub`. No row ⇒ the default.

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
    pub sub: String,
    pub kind: String,
    pub channel: String,
    pub enabled: bool,
    pub digest: String,
}
