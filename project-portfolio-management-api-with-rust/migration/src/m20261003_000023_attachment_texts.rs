//! Migration: **`attachment_texts`**. The text extracted from an attachment so it can be found by search (PL-8). Extraction is best-effort and never fails an upload: a file with no extractor is recorded `unindexed` with the reason, never silently skipped. The text inherits its attachment's access: a search result is shown only to someone who may read the attachment's subject.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The `attachment_texts` migration (name derived from the module).
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
            "attachment_texts",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("attachment_pid", ColType::Uuid),
                ("status", ColType::String),
                ("reason", ColType::StringNull),
                ("body", ColType::Text),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        db.execute_unprepared(
            "CREATE UNIQUE INDEX attachment_texts_one ON attachment_texts (attachment_pid);",
        )
        .await?;
        db.execute_unprepared(
            "ALTER TABLE attachment_texts ADD CONSTRAINT attachment_texts_status CHECK (status IN ('indexed','unindexed'));",
        )
        .await?;
        db.execute_unprepared(
            "CREATE INDEX attachment_texts_fts ON attachment_texts USING GIN (to_tsvector('simple', body));",
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
        drop_table(m, "attachment_texts").await
    }
}
