//! `SeaORM` Entity — `plan_role_members`. Who holds which role on a plan (OC-3).

#![expect(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, sqlx::FromRow)]
#[sea_orm(table_name = "plan_role_members")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub plan_pid: Uuid,
    /// The person's token subject, a lowercase UUID.
    pub person_ref: String,
    /// A key in `organization_roles`.
    pub role_key: String,
    pub added_by: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
