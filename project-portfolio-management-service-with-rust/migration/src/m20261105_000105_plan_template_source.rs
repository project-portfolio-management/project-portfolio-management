//! Migration: **the plan a template came from** (OC-20). A snapshot carries a plan's tasks and dates, so a template of a
//! private plan is hidden with that plan. No foreign key: when a plan is purged its template stays hidden from whoever the
//! access row still names, and the template outlives the plan, as it always has. Rows saved before this have no source, which
//! cannot be worked out afterwards, so they stay as visible as they were.

use sea_orm_migration::prelude::*;

/// The `plan_template_source` migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Add the column.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared(
                "ALTER TABLE plan_templates ADD COLUMN IF NOT EXISTS source_plan_pid UUID;",
            )
            .await?;
        Ok(())
    }

    /// Remove it, so templates are visible to everyone again.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared("ALTER TABLE plan_templates DROP COLUMN source_plan_pid;")
            .await?;
        Ok(())
    }
}
