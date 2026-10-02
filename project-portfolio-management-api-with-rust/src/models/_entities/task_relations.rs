//! `SeaORM` Entity — `task_relations`. Typed relations between tasks (LT-10), with Redmine's concrete behaviours: `blocks` stops the blocked task closing while the blocker is open; `precedes` forbids a successor due on or before its predecessor; `duplicates` closes the duplicate when the primary closes; `related` and `copied_from` are informational. Cross-plan relations are allowed.

#![allow(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "task_relations")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub from_task_pid: Uuid,
    pub to_task_pid: Uuid,
    pub kind: String,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
