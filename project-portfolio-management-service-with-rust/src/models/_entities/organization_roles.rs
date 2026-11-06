//! `SeaORM` Entity — `organization_roles`. The roles a deployment recognizes (OC-1), and the issue actions each
//! allows (OC-3). A role can only narrow what the caller's token already allows; see `crate::plan_roles`.

#![expect(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, sqlx::FromRow)]
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
    /// The label by language code (OC-6).
    pub labels: serde_json::Value,
    pub description: Option<String>,
    /// Permission names from `crate::plan_roles::PERMISSIONS`.
    pub permissions: serde_json::Value,
    pub position: i32,
    pub retired_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
