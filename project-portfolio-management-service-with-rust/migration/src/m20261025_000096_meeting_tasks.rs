//! Migration: **`meeting_tasks`** (BR-3). Which task was created from which outcome of a meeting. An outcome has at
//! most one task, so a second commit of the same outcome is refused by the table itself, not only by a check.

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
                "CREATE TABLE IF NOT EXISTS meeting_tasks (
                     created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     id BIGSERIAL PRIMARY KEY,
                     meeting_pid UUID NOT NULL,
                     outcome_no INTEGER NOT NULL,
                     task_pid UUID NOT NULL,
                     CONSTRAINT meeting_tasks_outcome_once UNIQUE (meeting_pid, outcome_no),
                     CONSTRAINT meeting_tasks_outcome_no CHECK (outcome_no >= 0)
                 );
                 CREATE INDEX IF NOT EXISTS meeting_tasks_task_idx ON meeting_tasks (task_pid);",
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
            .execute_unprepared("DROP TABLE IF EXISTS meeting_tasks")
            .await?;
        Ok(())
    }
}
