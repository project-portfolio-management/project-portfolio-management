//! Row — `sprint_commitments`. The set of tasks declared at
//! sprint start, written once and never updated: without it, scope
//! added mid-sprint is indistinguishable from scope committed at the
//! outset. See entity spec §5.9.3 / FR-29.

// The row's shape is documented by the migration.
#![expect(missing_docs)]

use super::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub id: i32,
    pub pid: Uuid,
    pub sprint_pid: Uuid,
    pub task_pid: Uuid,
    pub committed_at: DateTimeWithTimeZone,
}
