//! `SeaORM` Entity — `planned_slots`. A person's own time-blocking of a task (PL-4). Instants are UTC (`timestamptz`) and are rendered in the caller's zone on read. A slot never changes the task's due date, and overlapping slots are allowed — reported as clashes, not refused.

#![allow(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "planned_slots")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub sub: String,
    pub task_pid: Uuid,
    pub starts_at: DateTimeWithTimeZone,
    pub ends_at: DateTimeWithTimeZone,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
