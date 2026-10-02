//! `SeaORM` Entity — `feed_tokens`. Revocable secrets that let a calendar app or feed reader read one person's own feeds without a bearer token (LT-7 / RM-6). Only the SHA-256 of the token is stored, so a database read never yields a usable token; revoking sets `revoked_at` and takes effect on the next fetch.

#![allow(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "feed_tokens")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub sub: String,
    pub token_hash: String,
    pub label: Option<String>,
    pub revoked_at: Option<DateTimeWithTimeZone>,
    pub last_used_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
