//! Migration: **`tasks.parent_task_pid`** — subtasks (RM-5). A parent's
//! dates, estimate and progress are **derived on read** from its
//! descendants, never stored, so they cannot drift from the work they
//! summarize.

use sea_orm_migration::prelude::*;

/// The `task_parent` migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Add the nullable column and its index.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();
        db.execute_unprepared("ALTER TABLE tasks ADD COLUMN IF NOT EXISTS parent_task_pid UUID;")
            .await?;
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS tasks_parent ON tasks (parent_task_pid) \
                 WHERE deleted_at IS NULL;",
        )
        .await?;
        Ok(())
    }

    /// Drop the index and the column.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();
        db.execute_unprepared("DROP INDEX IF EXISTS tasks_parent;")
            .await?;
        db.execute_unprepared("ALTER TABLE tasks DROP COLUMN IF EXISTS parent_task_pid;")
            .await?;
        Ok(())
    }
}
