//! `SeaORM` Entity — `task_ccpm_estimates`: a task's aggressive and safe estimates, in business days (CC-2).

#![allow(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "task_ccpm_estimates")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub plan_pid: Uuid,
    #[sea_orm(unique)]
    pub task_pid: Uuid,
    /// About 50 percent confidence; `None` when not given.
    pub aggressive_days: Option<i32>,
    /// About 90 percent confidence; `None` when not given.
    pub safe_days: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
