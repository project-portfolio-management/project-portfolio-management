//! Row — `adoption_snapshots`. Active over target users, storing its own `definition` and
//! `window_days` because "active user" is the term most easily redefined.
//! See entity spec §5.9.6.

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
    pub observed_at: DateTimeWithTimeZone,
    pub active_users: i64,
    pub target_users: i64,
    pub window_days: i32,
    pub definition: String,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
