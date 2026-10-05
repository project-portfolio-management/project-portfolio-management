//! `SeaORM` Entity — `milestones`. Dated checkpoints on a plan
//! (PPM-6).

#![allow(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "milestones")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub plan_pid: Uuid,
    pub name: String,
    pub due: Date,
    pub done: bool,
    pub done_at: Option<DateTimeWithTimeZone>,
    pub kind: Option<String>,
    /// `manual` (a person marks it done) or `linked` (achieved when its linked tasks are all done).
    pub completion_mode: String,
    /// Why it was completed by hand while linked tasks were still open.
    pub done_reason: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
