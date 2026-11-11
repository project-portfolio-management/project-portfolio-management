//! Row — `workflows`. A configurable task or issue state
//! vocabulary. See entity spec §5.9.1 / FR-26.

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
    /// `None` scopes the workflow deployment-wide; a value scopes it to
    /// that plan, which then overrides the default.
    pub plan_pid: Option<Uuid>,
    pub name: String,
    /// `task` | `issue`.
    pub applies_to: String,
    /// For an issue workflow, the one issue type it governs; `None` governs every type without its own.
    pub issue_type: Option<String>,
    pub is_default: bool,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
