//! Migration: **`review_queue_items`**. A row held for a human decision (T-8): a keyless imported plan that the matcher scores as a probable duplicate of an existing one. Nothing is created or merged automatically (spec §5.1). `provenance` is `import` here.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The `review_queue_items` migration (name derived from the module).
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
            "review_queue_items",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("status", ColType::String),
                ("provenance", ColType::String),
                ("candidate", ColType::JsonBinary),
                ("matches", ColType::JsonBinary),
                ("job_pid", ColType::UuidNull),
                ("source_line", ColType::Integer),
                ("resolved_by", ColType::StringNull),
                ("resolved_at", ColType::TimestampWithTimeZoneNull),
                ("resolution_pid", ColType::UuidNull),
                ("resolution_comment", ColType::TextNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        db.execute_unprepared(
            "ALTER TABLE review_queue_items ADD CONSTRAINT review_queue_status CHECK (status IN ('pending','confirmed','rejected'));",
        )
        .await?;
        db.execute_unprepared(
            "CREATE INDEX review_queue_status_idx ON review_queue_items (status, id);",
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
        drop_table(m, "review_queue_items").await
    }
}
