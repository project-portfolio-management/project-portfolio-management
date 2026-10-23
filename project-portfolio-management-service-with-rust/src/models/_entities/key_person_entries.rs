//! `SeaORM` Entity — `key_person_entries`: a single point of failure, a person or a role (EX-11).

#![expect(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "key_person_entries")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    /// `person` or `role`.
    pub subject_kind: String,
    /// A person reference, or a role name.
    pub subject: String,
    pub successor_ref: Option<String>,
    /// `ready`, `partly` or `none`.
    pub readiness: String,
    pub skill_pool_pid: Option<Uuid>,
    pub note: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
