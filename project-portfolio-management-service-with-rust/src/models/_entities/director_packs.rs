//! `SeaORM` Entity — `director_packs`. One version of the monthly pack for one audience.

#![expect(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "director_packs")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    /// `YYYY-MM`.
    pub period: String,
    pub audience: String,
    pub version: i32,
    /// `draft` or `issued`.
    pub status: String,
    pub cut_off: DateTimeWithTimeZone,
    /// The exact JSON text computed at drafting.
    #[sea_orm(column_type = "Text")]
    pub body: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub change_note: Option<String>,
    pub supersedes_pid: Option<Uuid>,
    pub drafted_by: Option<String>,
    pub issued_by: Option<String>,
    pub issued_at: Option<DateTimeWithTimeZone>,
    /// `manual` or `scheduled` (drafted by the monthly schedule).
    pub origin: String,
    pub reviewed_by: Option<String>,
    pub reviewed_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
