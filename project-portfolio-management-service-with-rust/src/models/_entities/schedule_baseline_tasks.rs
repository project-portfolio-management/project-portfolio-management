//! `SeaORM` Entity — `schedule_baseline_tasks`. One task's row in a schedule baseline (SC-3): its title and, when it was schedulable then, its early start, early finish, working duration and total float. A task that could not be scheduled is kept with null dates and the reason, so it is neither lost nor given a date it never had. Append-only.

#![expect(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "schedule_baseline_tasks")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
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

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
