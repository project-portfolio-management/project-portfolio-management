//! `SeaORM` Entity — `task_costs`. A material or fixed-cost expense on a task, with the accrual rule that spreads it over the task.

#![allow(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "task_costs")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
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

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
