//! Row — `schedule_baseline_tasks`. One task's row in a schedule baseline (SC-3): its title and, when it was
//! schedulable then, its early start, early finish, working duration and total float. A task that could not be
//! scheduled is kept with null dates and the reason, so it is neither lost nor given a date it never had. Append-only.

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
    pub baseline_pid: Uuid,
    pub task_pid: Uuid,
    pub title: String,
    pub start_date: Option<Date>,
    pub finish_date: Option<Date>,
    pub duration_days: Option<i32>,
    pub total_float: Option<i32>,
    pub unscheduled_reason: Option<String>,
}
