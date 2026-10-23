//! `SeaORM` Entity — `schedule_baselines`. A schedule baseline (SC-3): a numbered, append-only snapshot of a plan's task schedule, taken with a reason. Never updated or deleted; a re-baseline is the next version and the earlier one stays. Variance is always measured against a named version.

#![expect(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "schedule_baselines")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub plan_pid: Uuid,
    pub version: i32,
    pub reason: String,
    pub created_by: Option<String>,
    pub task_count: i32,
    pub project_finish: Option<Date>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
