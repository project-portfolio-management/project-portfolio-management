//! Migration: **`report_definitions.group_by`** — the one field a saved
//! report groups by (T-28e). Nullable: `NULL` keeps today's ungrouped
//! behaviour, so every existing definition runs exactly as before.

use sea_orm_migration::prelude::*;

/// The `report_group_by` migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Add the nullable column.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared(
                "ALTER TABLE report_definitions ADD COLUMN IF NOT EXISTS group_by TEXT;",
            )
            .await?;
        Ok(())
    }

    /// Drop the column.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared("ALTER TABLE report_definitions DROP COLUMN IF EXISTS group_by;")
            .await?;
        Ok(())
    }
}
