//! `SeaORM` Entity — `criterion_evidence`. Append-only: whether a criterion was met in a quarter, with the evidence (EX-10).

#![allow(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "criterion_evidence")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub criterion_pid: Uuid,
    /// `YYYY-Qn`.
    pub quarter: String,
    pub met: bool,
    pub link: Option<String>,
    pub note: String,
    pub recorded_by: Option<String>,
    pub recorded_on: Date,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
