//! Row — `components`. Named groupings of tasks inside a plan (PL-11), corroborated by Redmine's issue categories. A
//! task may name one `component_pid` of its own plan; deleting a component that still has tasks is refused, never
//! silently orphaning them.

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
    pub plan_pid: Uuid,
    pub name: String,
    pub lead_ref: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
