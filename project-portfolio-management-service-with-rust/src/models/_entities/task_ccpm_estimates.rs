//! Row — `task_ccpm_estimates`: a task's aggressive and safe estimates, in business days (CC-2).

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
    /// About 50 percent confidence; `None` when not given.
    pub aggressive_days: Option<i32>,
    /// About 90 percent confidence; `None` when not given.
    pub safe_days: Option<i32>,
}
