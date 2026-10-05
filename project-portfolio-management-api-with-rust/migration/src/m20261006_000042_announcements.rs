//! Migration: the **announcement feed**. An announcement is a short message to everyone, with a severity, an optional pin, and an optional window (`publish_at` to `expires_at`). Retracting one is a soft delete.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The announcements migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Create the table and its checks.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "announcements",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("title", ColType::String),
                ("body", ColType::Text),
                ("severity", ColType::String),
                ("pinned", ColType::Boolean),
                ("publish_at", ColType::TimestampWithTimeZone),
                ("expires_at", ColType::TimestampWithTimeZoneNull),
                ("author_ref", ColType::StringNull),
                ("deleted_at", ColType::TimestampWithTimeZoneNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        for q in [
            "ALTER TABLE announcements ADD CONSTRAINT announcements_severity CHECK (severity IN ('info','important','urgent'))",
            "ALTER TABLE announcements ADD CONSTRAINT announcements_window CHECK (expires_at IS NULL OR expires_at > publish_at)",
            "CREATE INDEX announcements_live ON announcements (publish_at DESC) WHERE deleted_at IS NULL",
        ] {
            db.execute_unprepared(q).await?;
        }
        Ok(())
    }

    /// Drop the table.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "announcements").await
    }
}
