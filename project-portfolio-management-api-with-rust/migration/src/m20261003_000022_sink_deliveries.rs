//! Migration: **`sink_deliveries`**. One attempt-tracked delivery of one notification to one sink (LT-6). Enqueued when the notification is written and sent later, so a slow or dead destination can never delay or fail the notification itself. Bounded retries with growing backoff; the last error is kept.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The `sink_deliveries` migration (name derived from the module).
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
            "sink_deliveries",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("sink_pid", ColType::Uuid),
                ("notification_pid", ColType::Uuid),
                ("status", ColType::String),
                ("attempts", ColType::Integer),
                ("next_attempt_at", ColType::TimestampWithTimeZoneNull),
                ("last_error", ColType::TextNull),
                ("delivered_at", ColType::TimestampWithTimeZoneNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        db.execute_unprepared(
            "ALTER TABLE sink_deliveries ADD CONSTRAINT sink_deliveries_status CHECK (status IN ('pending','sent','failed','skipped'));",
        )
        .await?;
        db.execute_unprepared(
            "CREATE INDEX sink_deliveries_due ON sink_deliveries (next_attempt_at) WHERE status = 'pending';",
        )
        .await?;
        db.execute_unprepared(
            "CREATE UNIQUE INDEX sink_deliveries_once ON sink_deliveries (sink_pid, notification_pid);",
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
        drop_table(m, "sink_deliveries").await
    }
}
