//! Migration: **`workflow_transitions.roles`** — who may make a move (RM-2).
//! A JSON array of role names; **empty means anyone**, preserving T-15's
//! opt-in constraint, so every existing workflow behaves exactly as before.

use sea_orm_migration::prelude::*;

/// The `workflow_transition_roles` migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Add the `NOT NULL DEFAULT '[]'` column.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared(
                "ALTER TABLE workflow_transitions \
                     ADD COLUMN IF NOT EXISTS roles JSONB NOT NULL DEFAULT '[]';",
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
            .execute_unprepared("ALTER TABLE workflow_transitions DROP COLUMN IF EXISTS roles;")
            .await?;
        Ok(())
    }
}
