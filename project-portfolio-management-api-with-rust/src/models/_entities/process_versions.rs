//! `SeaORM` Entity — `process_versions`. One immutable version of a process: its ordered steps.

#![allow(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "process_versions")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub process_pid: Uuid,
    pub version: i32,
    pub steps: Json,
    pub created_by: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
