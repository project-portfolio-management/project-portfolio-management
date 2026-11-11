//! Row — `automation_runs`. The append-only log of what an
//! automation actually did (or refused to do) on one subject: the
//! honest record behind "the platform updated this for you". One
//! firing of an N-action rule writes N rows sharing everything but
//! `action_index` (0-based, matching the rule's `actions` array
//! position) — FR-32's "each action's outcome logged separately".

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
    pub automation_pid: Uuid,
    pub subject_kind: String,
    pub subject_pid: Uuid,
    pub action_index: i32,
    pub outcome: String,
    pub detail: Json,
}
