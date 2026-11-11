//! Row — `plan_template_links`. Where a plan came from (NC-12): the template, the plan's node in the
//! template tree, the version it was last brought up to, the baseline tasks, and the task-to-task map.

#![expect(missing_docs)]

use super::{DateTimeWithTimeZone, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub id: i64,
    pub plan_pid: Uuid,
    pub template_pid: Uuid,
    pub node_key: i32,
    pub template_version: i32,
    pub baseline: Json,
    pub task_map: Json,
}
