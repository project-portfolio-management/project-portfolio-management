//! `SeaORM` Entity — `announcements`. A message to everyone, with a severity, an optional pin and an optional window.

#![expect(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "announcements")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub title: String,
    #[sea_orm(column_type = "Text")]
    pub body: String,
    /// `info`, `important` or `urgent`.
    pub severity: String,
    pub pinned: bool,
    pub publish_at: DateTimeWithTimeZone,
    pub expires_at: Option<DateTimeWithTimeZone>,
    pub author_ref: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
