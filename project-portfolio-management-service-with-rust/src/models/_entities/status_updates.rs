//! `SeaORM` Entity — `status_updates`. One on-track / at-risk / off-track call on a plan (NC-4). Append-only.

#![expect(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "status_updates")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub plan_pid: Uuid,
    pub status: String,
    #[sea_orm(column_type = "Text")]
    pub narrative: String,
    pub posted_on: Date,
    pub posted_by: Option<String>,
    /// Claims with their sources, as ids (BR-2).
    pub claims: Json,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
