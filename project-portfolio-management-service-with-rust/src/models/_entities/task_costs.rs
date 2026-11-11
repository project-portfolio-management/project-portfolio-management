//! Row — `task_costs`. A material or fixed-cost expense on a task, with the accrual rule that spreads it over the task.

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
    pub task_pid: Uuid,
    /// `material` or `fixed`.
    pub kind: String,
    pub description: String,
    pub amount_minor: i64,
    pub currency: String,
    /// `start`, `prorated` or `end`.
    pub accrual: String,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
