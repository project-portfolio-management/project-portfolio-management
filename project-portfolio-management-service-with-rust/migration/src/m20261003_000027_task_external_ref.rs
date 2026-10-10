//! Migration: **`tasks.external_ref`** (PJ-10). A stable reference to where a
//! task came from (`mspdi:<UID>`), unique per plan, so re-importing the same MS
//! Project file updates its tasks instead of duplicating them.

use sea_orm_migration::prelude::*;

/// The migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Add the column and its uniqueness.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();
        db.execute_unprepared("ALTER TABLE tasks ADD COLUMN external_ref text")
            .await?;
        db.execute_unprepared(
            "CREATE UNIQUE INDEX tasks_external_ref_once ON tasks (plan_pid, external_ref) WHERE external_ref IS NOT NULL AND deleted_at IS NULL",
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
        let db = m.get_connection();
        db.execute_unprepared("DROP INDEX tasks_external_ref_once")
            .await?;
        db.execute_unprepared("ALTER TABLE tasks DROP COLUMN external_ref")
            .await?;
        Ok(())
    }
}
