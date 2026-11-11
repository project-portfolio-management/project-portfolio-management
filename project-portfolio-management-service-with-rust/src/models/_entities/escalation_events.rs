//! Row — `escalation_events`. The append-only history of an escalation: raised, acknowledged, taken to a
//! higher level, resolved or withdrawn, with who and why. A database trigger refuses any change or deletion.

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
    pub escalation_pid: Uuid,
    pub event: String,
    pub level: i32,
    pub actor: Option<String>,
    pub note: Option<String>,
}
