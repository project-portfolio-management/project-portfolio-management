//! `SeaORM` Entity — `raci_assignments`. A RACI assignment: one person's role on one task: Responsible, Accountable, Consulted or Informed. A person may hold several letters on a task (the usual A/R); a task has at most one Accountable (enforced by the service) and should have exactly one (reported when it does not). Replacing a task's row soft-deletes the old assignments, so the history is kept.

#![expect(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "raci_assignments")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub plan_pid: Uuid,
    pub task_pid: Uuid,
    pub person_ref: String,
    pub role: String,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
