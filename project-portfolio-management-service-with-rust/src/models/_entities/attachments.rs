//! Row — `attachments`. A file attached to a plan, task, risk, proposal, idea, canvas item, budget line or scenario
//! (LT-5). The bytes are an artifact; this row is the link, so removing an attachment (soft-delete) never loses the
//! audit trail of what was attached.

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
    pub subject_kind: String,
    pub subject_pid: Uuid,
    pub artifact_pid: Uuid,
    pub uploaded_by: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
