//! Row — `workflow_states`. See entity spec §5.9.1.

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
    pub workflow_pid: Uuid,
    pub state_key: String,
    pub label: String,
    /// The label by language code (OC-6).
    pub labels: serde_json::Value,
    /// `todo` | `active` | `waiting` | `done`. **`NOT NULL`** — every
    /// derived view computes from this rather than from the name.
    pub category: String,
    pub wip_limit: Option<i32>,
    pub is_initial: bool,
    pub is_terminal: bool,
    /// What this state requires of an issue's fields (OC-4), as `crate::workflow::FieldRule`s.
    pub field_rules: serde_json::Value,
    pub position: i32,
}
