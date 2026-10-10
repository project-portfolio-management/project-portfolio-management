//! `SeaORM` Entity — `custom_field_defs`. Admin-declared, typed custom fields for plans and tasks (RM-1). A definition is soft-deleted, never dropped: values already stored stay readable.

#![allow(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "custom_field_defs")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub key: String,
    pub label: String,
    pub field_type: String,
    pub applies_to: String,
    pub kinds: Json,
    pub required: bool,
    pub options: Json,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
