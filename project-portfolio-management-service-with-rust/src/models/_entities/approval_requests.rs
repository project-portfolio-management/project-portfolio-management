//! Row — `approval_requests`. Ask a named person to approve something, and record their decision (PL-1). Generic over
//! the same subject kinds as comments, plus budget lines and scenarios. A decision is final and carries a comment
//! (required on reject).

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
    pub requested_by: String,
    pub approver_ref: String,
    pub message: Option<String>,
    pub status: String,
    pub decision_comment: Option<String>,
    pub decided_by: Option<String>,
    pub decided_at: Option<DateTimeWithTimeZone>,
    pub process_execution_pid: Option<Uuid>,
    pub process_step: Option<i32>,
}
