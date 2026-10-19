//! Migration: **why a task was blocked**. An optional reason on the transition into `blocked`, from a fixed
//! vocabulary, so blocked time can be reported by reason (`spec/time-based-analysis.md` §17). The log stays
//! append-only; a transition recorded before this migration, or without a reason, simply has none.

use sea_orm_migration::prelude::*;

/// The migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Add the column and the check that keeps it to the vocabulary and to `blocked` transitions.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared(
                "ALTER TABLE task_transitions ADD COLUMN IF NOT EXISTS blocked_reason VARCHAR NULL;
                 ALTER TABLE task_transitions DROP CONSTRAINT IF EXISTS task_transitions_blocked_reason_check;
                 ALTER TABLE task_transitions ADD CONSTRAINT task_transitions_blocked_reason_check
                     CHECK (blocked_reason IS NULL OR (to_status = 'blocked' AND blocked_reason IN
                         ('waiting_on_person', 'external_dependency', 'awaiting_decision',
                          'awaiting_resource', 'other')));",
            )
            .await?;
        Ok(())
    }

    /// Drop the column (and its check with it).
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared(
                "ALTER TABLE task_transitions DROP COLUMN IF EXISTS blocked_reason;",
            )
            .await?;
        Ok(())
    }
}
