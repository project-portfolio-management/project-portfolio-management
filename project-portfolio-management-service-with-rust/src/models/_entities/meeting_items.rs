//! Row — `meeting_items`. One agenda item or outcome of a meeting (NC-14).

#![expect(missing_docs)]

use super::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub id: i64,
    pub meeting_pid: Uuid,
    pub part: String,
    pub outcome_kind: Option<String>,
    pub text: String,
    pub link_kind: Option<String>,
    pub link_pid: Option<Uuid>,
}
