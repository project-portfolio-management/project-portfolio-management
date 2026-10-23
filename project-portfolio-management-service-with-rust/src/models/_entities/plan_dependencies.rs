//! `SeaORM` Entity — `plan_dependencies`. Finish-start edges
//! between plans (PPM-6), with lag; cycle-free (app-enforced).

#![expect(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "plan_dependencies")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub predecessor_pid: Uuid,
    pub successor_pid: Uuid,
    pub lag_days: i32,
    /// `fs` (default), `ss`, `ff` or `sf`.
    pub kind: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
