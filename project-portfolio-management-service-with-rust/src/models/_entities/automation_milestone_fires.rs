//! Row — `automation_milestone_fires`. The exactly-once
//! claim for the `milestone_due` trigger (FR-32): one row per
//! `(automation_pid, milestone_pid)` that has fired. Inserting is the
//! claim — `ON CONFLICT DO NOTHING`, so only the caller that actually
//! adds the row applies the rule's actions.

// The row's shape is documented by the migration.
#![expect(missing_docs)]

use super::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub id: i32,
    pub automation_pid: Uuid,
    pub milestone_pid: Uuid,
}
