//! Row — `notification_sinks`. A person's own delivery destination for their notifications (LT-6, RM-11): a signed
//! webhook, a chat incoming-webhook (Slack, Mattermost, Discord, Telegram) or an email address. The destination is a
//! secret of its owner and is never returned in full by any read.

#![expect(missing_docs)]

use super::{DateTimeWithTimeZone, Json};
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
    pub destination: String,
    pub settings: Json,
    pub enabled: bool,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
