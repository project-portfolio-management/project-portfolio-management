//! `SeaORM` Entity — `process_executions`. One run of a process on a subject, pinned to a version.

#![expect(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "process_executions")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub process_pid: Uuid,
    pub version: i32,
    pub subject_kind: String,
    pub subject_pid: Uuid,
    /// `running`, `completed`, `rejected`, `cancelled` or `failed`.
    pub status: String,
    /// The index of the step it is on (or the number of steps once finished).
    pub current_step: i32,
    pub started_by: String,
    pub finished_at: Option<DateTimeWithTimeZone>,
    pub note: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
