//! Migration: **`plan_links`**. Typed external links on a plan (LT-8): where the plan's real documents live. The service stores a link, never the document, and never fetches it. Only `http` and `https` URLs are accepted, so a link can never be a script or file reference.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The `plan_links` migration (name derived from the module).
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
            "plan_links",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("plan_pid", ColType::Uuid),
                ("kind", ColType::String),
                ("url", ColType::String),
                ("label", ColType::String),
                ("created_by", ColType::StringNull),
                ("deleted_at", ColType::TimestampWithTimeZoneNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        db.execute_unprepared(
            "ALTER TABLE plan_links ADD CONSTRAINT plan_links_kind CHECK (kind IN ('docs','repo','ticket','design','chat','other'));",
        )
        .await?;
        db.execute_unprepared(
            "CREATE UNIQUE INDEX plan_links_one ON plan_links (plan_pid, url) WHERE deleted_at IS NULL;",
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
        drop_table(m, "plan_links").await
    }
}
