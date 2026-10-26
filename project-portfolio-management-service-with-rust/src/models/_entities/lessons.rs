//! `SeaORM` Entity — `lessons`. One lesson learned by a plan, recorded at a gate or at closure (NC-8).

#![expect(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "lessons")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub plan_pid: Uuid,
    pub occasion: String,
    pub category: String,
    pub summary: String,
    pub detail: Option<String>,
    pub recorded_by: Option<String>,
    /// The kind of record the lesson points at, when it does (BR-4).
    pub link_kind: Option<String>,
    /// That record's `pid`.
    pub link_pid: Option<Uuid>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
