//! Row — `controls`. The Controlling-process register.
//! See entity spec §5.9.8 / FR-38.

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
    pub name: String,
    /// `feedforward` | `concurrent` | `feedback` — decides what a
    /// failing reading may do.
    pub timing: String,
    pub metric: String,
    pub target_value: i64,
    pub comparator: String,
    pub tolerance: Option<i64>,
    pub unit: Option<String>,
    pub currency: Option<String>,
    pub source_kind: String,
    pub source_ref: Option<String>,
    /// `None` means no cadence is declared, so a reading can never be
    /// *overdue*.
    pub cadence_days: Option<i64>,
    pub owner_ref: Option<String>,
    pub enabled: bool,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
