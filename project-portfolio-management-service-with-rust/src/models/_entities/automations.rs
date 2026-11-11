//! Row — `automations`. One workflow rule: when a trigger
//! fires (typically a task crossing the Kanban board), apply its
//! `actions` **in declared order** (FR-32) — a JSONB array of
//! `{"kind": …, "value": …}`, never empty (schema-enforced). `plan_pid`
//! scopes the rule to a plan; `NULL` = every plan.

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
    pub plan_pid: Option<Uuid>,
    pub name: String,
    pub trigger_kind: String,
    pub from_status: Option<String>,
    pub to_status: Option<String>,
    pub actions: Json,
    pub enabled: bool,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
