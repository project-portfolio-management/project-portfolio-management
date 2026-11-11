//! Row — `artifacts`. The artifact store's index (T-8 §7, LT-5): one row per stored file. The bytes live on disk under
//! `storage_key`; the row carries what is needed to serve and verify them — name, declared content type, size and the
//! SHA-256 of the bytes. `expires_at` lets a sweep delete a file while the row remains as a tombstone.

#![expect(missing_docs)]

use super::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub id: i64,
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
