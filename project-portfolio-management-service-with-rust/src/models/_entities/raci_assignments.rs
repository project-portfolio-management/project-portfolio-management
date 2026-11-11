//! Row — `raci_assignments`. A RACI assignment: one person's role on one task: Responsible, Accountable, Consulted or
//! Informed. A person may hold several letters on a task (the usual A/R); a task has at most one Accountable (enforced
//! by the service) and should have exactly one (reported when it does not). Replacing a task's row soft-deletes the old
//! assignments, so the history is kept.

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
    pub plan_pid: Uuid,
    pub task_pid: Uuid,
    pub person_ref: String,
    pub role: String,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
