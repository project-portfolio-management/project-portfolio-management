//! Migration: **`notification_sinks`**. A person's own delivery destination for their notifications (LT-6, RM-11): a signed webhook, a chat incoming-webhook (Slack, Mattermost, Discord, Telegram) or an email address. The destination is a secret of its owner and is never returned in full by any read.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The `notification_sinks` migration (name derived from the module).
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
            "notification_sinks",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("sub", ColType::String),
                ("kind", ColType::String),
                ("destination", ColType::String),
                ("settings", ColType::JsonBinary),
                ("enabled", ColType::Boolean),
                ("deleted_at", ColType::TimestampWithTimeZoneNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        db.execute_unprepared(
            "ALTER TABLE notification_sinks ADD CONSTRAINT notification_sinks_kind CHECK (kind IN ('webhook','slack','mattermost','discord','telegram','email'));",
        )
        .await?;
        db.execute_unprepared(
            "CREATE INDEX notification_sinks_sub ON notification_sinks (sub) WHERE deleted_at IS NULL;",
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
        drop_table(m, "notification_sinks").await
    }
}
