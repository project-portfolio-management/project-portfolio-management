//! `SeaORM` Entity — `buffer_recovery_actions`. What was agreed about a buffer in the red; closed once, never deleted (CC-4).

#![expect(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "buffer_recovery_actions")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub plan_pid: Uuid,
    pub buffer_plan_pid: Uuid,
    /// `project` or `feeding:<join task>:<first task>` in the buffer plan.
    pub buffer_key: String,
    pub action: String,
    pub owner_ref: Option<String>,
    pub due_date: Option<Date>,
    /// `open`, `done` or `dropped`.
    pub status: String,
    pub closed_on: Option<Date>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
