//! Row — `workflow_transitions`. An **empty** set for a
//! workflow means unconstrained, preserving today's board behaviour.
//! See entity spec §5.9.1.

// The row's shape is documented by the migration.
#![expect(missing_docs)]

use super::{DateTimeWithTimeZone, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub id: i32,
    pub pid: Uuid,
    pub workflow_pid: Uuid,
    pub from_key: String,
    pub to_key: String,
    /// Roles allowed to make this move; an empty array means anyone.
    pub roles: Json,
}
