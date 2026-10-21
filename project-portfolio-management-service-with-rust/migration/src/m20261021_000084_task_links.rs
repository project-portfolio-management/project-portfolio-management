//! Migration: **links on a task**. A link may belong to one task of the plan (`task_pid`), for example the Jira or
//! Linear issue the task is tracked as. Plan-level links have no `task_pid`. A URL may be linked once per plan-level
//! list and once per task, so the same address can appear on a plan and on one of its tasks.

use sea_orm_migration::prelude::*;

/// The migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Add the column and replace the uniqueness index.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared(
                "ALTER TABLE plan_links ADD COLUMN IF NOT EXISTS task_pid UUID NULL;
                 DROP INDEX IF EXISTS plan_links_one;
                 CREATE UNIQUE INDEX plan_links_one ON plan_links
                     (plan_pid, COALESCE(task_pid, '00000000-0000-0000-0000-000000000000'::uuid), url)
                     WHERE deleted_at IS NULL;
                 CREATE INDEX IF NOT EXISTS plan_links_task ON plan_links (task_pid) WHERE task_pid IS NOT NULL;",
            )
            .await?;
        Ok(())
    }

    /// Drop the task links and the column, and restore the plan-wide uniqueness index.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared(
                "DELETE FROM plan_links WHERE task_pid IS NOT NULL;
                 DROP INDEX IF EXISTS plan_links_task;
                 DROP INDEX IF EXISTS plan_links_one;
                 CREATE UNIQUE INDEX plan_links_one ON plan_links (plan_pid, url) WHERE deleted_at IS NULL;
                 ALTER TABLE plan_links DROP COLUMN IF EXISTS task_pid;",
            )
            .await?;
        Ok(())
    }
}
