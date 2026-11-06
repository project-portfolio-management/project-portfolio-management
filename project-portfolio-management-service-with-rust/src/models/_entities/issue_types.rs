//! `SeaORM` Entity — `issue_types`. A deployment's issue types (OC-1). A type is retired, never deleted.

#![expect(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, sqlx::FromRow)]
#[sea_orm(table_name = "issue_types")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub pid: Uuid,
    /// The stable key an issue's `kind` stores.
    #[sea_orm(unique)]
    pub key: String,
    pub label: String,
    /// The label by language code (OC-6).
    pub labels: serde_json::Value,
    pub position: i32,
    pub retired_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
