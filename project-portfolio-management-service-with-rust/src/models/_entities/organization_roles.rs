//! `SeaORM` Entity — `organization_roles`. The role names a deployment recognizes (OC-1). **A role here grants
//! nothing:** authority comes from the caller's token.

#![expect(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "organization_roles")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub pid: Uuid,
    /// The name, as a token's `role` attribute carries it.
    #[sea_orm(unique)]
    pub key: String,
    pub label: String,
    pub description: Option<String>,
    pub position: i32,
    pub retired_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
