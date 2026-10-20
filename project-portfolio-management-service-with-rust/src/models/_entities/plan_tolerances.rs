//! `SeaORM` Entity — `plan_tolerances`. How far a plan may drift on one dimension (time, cost, risk, resources)
//! before it must be referred upward. A plan with no row for a dimension inherits its nearest ancestor's.

#![allow(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "plan_tolerances")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub plan_pid: Uuid,
    pub dimension: String,
    pub limit_value: i64,
    pub amber_from_percent: i32,
    #[sea_orm(column_type = "Text", nullable)]
    pub note: Option<String>,
    pub set_by: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
