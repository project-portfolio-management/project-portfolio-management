//! Row — `schedule_baselines`. A schedule baseline (SC-3): a numbered, append-only snapshot of a plan's task schedule,
//! taken with a reason. Never updated or deleted; a re-baseline is the next version and the earlier one stays. Variance
//! is always measured against a named version.

#![expect(missing_docs)]

use super::{Date, DateTimeWithTimeZone};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub id: i64,
    pub pid: Uuid,
    pub plan_pid: Uuid,
    pub version: i32,
    pub reason: String,
    pub created_by: Option<String>,
    pub task_count: i32,
    pub project_finish: Option<Date>,
}
