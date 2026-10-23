//! `SeaORM` Entity — `document_versions`. One version of a controlled document.

#![expect(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "document_versions")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub document_pid: Uuid,
    pub version: i32,
    #[sea_orm(column_type = "Text")]
    pub body: String,
    pub change_summary: Option<String>,
    /// `draft`, `in_review`, `approved`, `effective` or `superseded`.
    pub status: String,
    pub approval_pid: Option<Uuid>,
    pub effective_from: Option<Date>,
    pub superseded_on: Option<Date>,
    pub superseded_by: Option<Uuid>,
    pub created_by: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
