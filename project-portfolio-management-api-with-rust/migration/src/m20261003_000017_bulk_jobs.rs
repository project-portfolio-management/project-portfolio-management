//! Migration: **`bulk_jobs`**. One bulk import or export job (T-8, spec `bulk-import-export.md` §2). `UNIQUE (entity, kind, idempotency_key)` makes a retried request return the original job. The counters always reconcile: `rows_total = created + updated + unchanged + queued_for_review + failed`. A job is never deleted; only its artifacts expire.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The `bulk_jobs` migration (name derived from the module).
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
            "bulk_jobs",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("entity", ColType::String),
                ("kind", ColType::String),
                ("status", ColType::String),
                ("format", ColType::String),
                ("idempotency_key", ColType::StringNull),
                ("requested_by", ColType::StringNull),
                ("params", ColType::JsonBinary),
                ("rows_total", ColType::BigInteger),
                ("rows_created", ColType::BigInteger),
                ("rows_updated", ColType::BigInteger),
                ("rows_unchanged", ColType::BigInteger),
                ("rows_queued_for_review", ColType::BigInteger),
                ("rows_failed", ColType::BigInteger),
                ("rows_omitted", ColType::BigInteger),
                ("source_artifact", ColType::UuidNull),
                ("result_artifact", ColType::UuidNull),
                ("error_artifact", ColType::UuidNull),
                ("started_at", ColType::TimestampWithTimeZoneNull),
                ("finished_at", ColType::TimestampWithTimeZoneNull),
                ("failure_reason", ColType::TextNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        db.execute_unprepared(
            "ALTER TABLE bulk_jobs ADD CONSTRAINT bulk_jobs_kind CHECK (kind IN ('import','export'));",
        )
        .await?;
        db.execute_unprepared(
            "ALTER TABLE bulk_jobs ADD CONSTRAINT bulk_jobs_status CHECK (status IN ('queued','running','completed','completed_with_errors','failed'));",
        )
        .await?;
        db.execute_unprepared(
            "CREATE UNIQUE INDEX bulk_jobs_idempotency ON bulk_jobs (entity, kind, idempotency_key);",
        )
        .await?;
        db.execute_unprepared(
            "CREATE INDEX bulk_jobs_status_idx ON bulk_jobs (status, updated_at);",
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
        drop_table(m, "bulk_jobs").await
    }
}
