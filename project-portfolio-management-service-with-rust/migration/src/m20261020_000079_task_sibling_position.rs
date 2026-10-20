//! Migration: **sibling order of tasks**. An optional position among the tasks that share a parent
//! (`parent_task_pid`, or none for the top level), set by `PUT /api/plans/{pid}/task-order`, so a work breakdown
//! structure can be reordered. A task with no position sorts after the positioned ones, oldest first.

use sea_orm_migration::prelude::*;

/// The migration (name derived from the module).
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
                "ALTER TABLE tasks ADD COLUMN IF NOT EXISTS sibling_position INTEGER NULL;
                 ALTER TABLE tasks DROP CONSTRAINT IF EXISTS tasks_sibling_position_check;
                 ALTER TABLE tasks ADD CONSTRAINT tasks_sibling_position_check
                     CHECK (sibling_position IS NULL OR sibling_position >= 0);",
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
            .execute_unprepared("ALTER TABLE tasks DROP COLUMN IF EXISTS sibling_position;")
            .await?;
        Ok(())
    }
}
