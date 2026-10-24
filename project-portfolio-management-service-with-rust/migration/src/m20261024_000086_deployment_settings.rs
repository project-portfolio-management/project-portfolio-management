//! Migration: `deployment_settings`, a small key/value table for settings an administrator changes at
//! run time. The first key is `today_time_zone` (the IANA zone that defines "today" for portfolio-wide
//! views). Absent means the default (UTC).

use sea_orm_migration::prelude::*;

/// The migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Create the table.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared(
                "CREATE TABLE IF NOT EXISTS deployment_settings (
                     key        text PRIMARY KEY,
                     value      text NOT NULL,
                     updated_by text,
                     updated_at timestamptz NOT NULL DEFAULT now()
                 )",
            )
            .await?;
        Ok(())
    }

    /// Drop the table.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS deployment_settings")
            .await?;
        Ok(())
    }
}
