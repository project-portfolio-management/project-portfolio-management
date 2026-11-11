//! Row — `phase_transitions`. The **append-only** project
//! phase log, from which per-phase durations are derived. See entity
//! spec §5.9.4 / FR-30.

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
    pub plan_pid: Uuid,
    /// `None` marks the plan's first phase.
    pub from_phase: Option<String>,
    pub to_phase: String,
    pub occurred_at: DateTimeWithTimeZone,
    pub actor: Option<String>,
    /// Required for a backward move: re-planning is normal, an
    /// unexplained regression is not.
    pub reason: Option<String>,
}
