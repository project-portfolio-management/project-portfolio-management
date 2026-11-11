//! Row — `change_requests`. A controlled proposal to change an approved baseline.

#![expect(missing_docs)]

use super::{Date, DateTimeWithTimeZone, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub id: i64,
    pub pid: Uuid,
    pub plan_pid: Uuid,
    pub title: String,
    pub description: String,
    pub reason: String,
    /// `scope`, `schedule`, `cost`, `quality` or `other`.
    pub kind: String,
    /// Business days; negative pulls the finish in.
    pub impact_schedule_days: Option<i32>,
    pub impact_cost_minor: Option<i64>,
    pub cost_currency: Option<String>,
    pub impact_summary: Option<String>,
    /// What-if edits (the sandbox's edit list) whose forecast is shown as the impact.
    pub edits: Option<Json>,
    pub status: String,
    pub requested_by: String,
    pub requested_on: Date,
    pub approver_ref: Option<String>,
    pub decided_by: Option<String>,
    pub decided_on: Option<Date>,
    pub decision_reason: Option<String>,
    pub implemented_on: Option<Date>,
    pub baseline_pid: Option<Uuid>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
