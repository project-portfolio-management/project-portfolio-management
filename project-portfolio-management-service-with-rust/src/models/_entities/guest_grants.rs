//! `SeaORM` Entity — `guest_grants`. A scoped, time-boxed, revocable grant letting an external collaborator (a token whose role attribute includes `guest`) see — and optionally comment on — one plan (PL-7). A guest sees only what is granted and gets 404 everywhere else; never people references. `revoked_at` and `expires_at` take effect on the next request.

#![allow(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "guest_grants")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub guest_sub: String,
    pub subject_kind: String,
    pub subject_pid: Uuid,
    pub access: String,
    pub expires_at: Option<DateTimeWithTimeZone>,
    pub granted_by: Option<String>,
    pub revoked_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
