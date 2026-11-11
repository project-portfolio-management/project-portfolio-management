//! Row — `deployment_moves`: a request to move a deployment's go-live (EX-5).

#![expect(missing_docs)]

use super::{Date, DateTimeWithTimeZone};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub id: i64,
    pub pid: Uuid,
    /// The deployment's plan.
    pub plan_pid: Uuid,
    /// The programme it belongs to.
    pub programme_pid: Uuid,
    /// The go-live task.
    pub task_pid: Uuid,
    pub from_start: Date,
    pub to_start: Date,
    pub reason: String,
    pub requested_by: Option<String>,
    pub organization_pid: Option<Uuid>,
    /// `person:<id>` or `role:<name>`.
    pub approver_ref: String,
    /// `requested`, `approved` or `refused`.
    pub status: String,
    pub decided_by: Option<String>,
    pub decided_on: Option<Date>,
    pub decision_reason: Option<String>,
    /// What moves as a result, as JSON text, worked out when the request was made.
    pub knock_on: String,
    /// The knock-on labour cost per currency, as JSON text.
    pub knock_on_cost: String,
}
