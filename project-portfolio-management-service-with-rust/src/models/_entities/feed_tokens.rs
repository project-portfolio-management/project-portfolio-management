//! Row — `feed_tokens`. Revocable secrets that let a calendar app or feed reader read one person's own feeds without a
//! bearer token (LT-7 / RM-6). Only the SHA-256 of the token is stored, so a database read never yields a usable token;
//! revoking sets `revoked_at` and takes effect on the next fetch.

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
    pub token_hash: String,
    pub label: Option<String>,
    pub revoked_at: Option<DateTimeWithTimeZone>,
    pub last_used_at: Option<DateTimeWithTimeZone>,
}
