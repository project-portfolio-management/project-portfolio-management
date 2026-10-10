//! Migration: **`tasks.due_date`** — a task's due **calendar date** (LT-3).
//! A `DATE`, not a timestamp: "due 2026-10-03" names that day with no zone
//! (the time-zone doctrine, `src/tz.rs`). Nullable: an undated task stays
//! undated, never a made-up date. The partial index serves the "my open
//! tasks" lookup (LT-3), which filters by assignee over live, unfinished rows.

use sea_orm_migration::prelude::*;

/// The `task_due_date` migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Add the column and the index.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();
        db.execute_unprepared("ALTER TABLE tasks ADD COLUMN IF NOT EXISTS due_date DATE;")
            .await?;
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS tasks_assignee_open ON tasks (assignee_ref) \
                 WHERE deleted_at IS NULL AND done_at IS NULL AND assignee_ref IS NOT NULL;",
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
        db.execute_unprepared("DROP INDEX IF EXISTS tasks_assignee_open;")
            .await?;
        db.execute_unprepared("ALTER TABLE tasks DROP COLUMN IF EXISTS due_date;")
            .await?;
        Ok(())
    }
}
