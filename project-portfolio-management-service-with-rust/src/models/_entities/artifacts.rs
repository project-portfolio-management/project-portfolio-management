//! `SeaORM` Entity — `artifacts`. The artifact store's index (T-8 §7, LT-5): one row per stored file. The bytes live on disk under `storage_key`; the row carries what is needed to serve and verify them — name, declared content type, size and the SHA-256 of the bytes. `expires_at` lets a sweep delete a file while the row remains as a tombstone.

#![expect(missing_docs)]

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "artifacts")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub pid: Uuid,
    pub kind: String,
    pub filename: String,
    pub content_type: String,
    pub size_bytes: i64,
    pub sha256: String,
    pub storage_key: Option<String>,
    pub created_by: Option<String>,
    pub expires_at: Option<DateTimeWithTimeZone>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
