//! `SeaORM` Entity — `milestone_signoffs`. Append-only: a sign-off or a withdrawal of one (EX-4).

#![expect(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "milestone_signoffs")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub milestone_pid: Uuid,
    pub organization_pid: Uuid,
    /// `sign` or `withdraw`.
    pub action: String,
    pub reason: Option<String>,
    pub acted_on: Date,
    pub signed_by: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
