//! Row — `task_relations`. Typed relations between tasks (LT-10), with Redmine's concrete behaviours: `blocks` stops
//! the blocked task closing while the blocker is open; `precedes` forbids a successor due on or before its predecessor;
//! `duplicates` closes the duplicate when the primary closes; `related` and `copied_from` are informational. Cross-plan
//! relations are allowed.

#![expect(missing_docs)]

use super::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub id: i64,
    pub pid: Uuid,
    pub from_task_pid: Uuid,
    pub to_task_pid: Uuid,
    pub kind: String,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
