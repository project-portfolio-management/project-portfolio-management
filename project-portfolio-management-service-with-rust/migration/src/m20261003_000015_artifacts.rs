//! Migration: **`artifacts`**. The artifact store's index (T-8 §7, LT-5): one row per stored file. The bytes live on disk under `storage_key`; the row carries what is needed to serve and verify them — name, declared content type, size and the SHA-256 of the bytes. `expires_at` lets a sweep delete a file while the row remains as a tombstone.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The `artifacts` migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Create the table and its indexes.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "artifacts",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("kind", ColType::String),
                ("filename", ColType::String),
                ("content_type", ColType::String),
                ("size_bytes", ColType::BigInteger),
                ("sha256", ColType::String),
                ("storage_key", ColType::StringNull),
                ("created_by", ColType::StringNull),
                ("expires_at", ColType::TimestampWithTimeZoneNull),
                ("deleted_at", ColType::TimestampWithTimeZoneNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        db.execute_unprepared(
            "CREATE INDEX artifacts_expiry ON artifacts (expires_at) WHERE expires_at IS NOT NULL AND storage_key IS NOT NULL;",
        )
        .await?;
        let _ = db;
        Ok(())
    }

    /// Drop the table.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "artifacts").await
    }
}
