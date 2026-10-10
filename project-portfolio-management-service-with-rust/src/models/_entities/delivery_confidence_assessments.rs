//! `SeaORM` Entity — `delivery_confidence_assessments`. Append-only history of a plan's stated delivery confidence.

#![allow(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "delivery_confidence_assessments")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub plan_pid: Uuid,
    /// `green`, `amber_green`, `amber`, `amber_red` or `red`.
    pub value: String,
    /// `self` or `independent`.
    pub kind: String,
    #[sea_orm(column_type = "Text")]
    pub rationale: String,
    pub assessor: Option<String>,
    pub assessed_on: Date,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
