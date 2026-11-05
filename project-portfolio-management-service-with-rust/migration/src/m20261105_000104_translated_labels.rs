//! Migration: **translated labels** (OC-6). An organization's own issue types, roles and statuses can carry a label per
//! language. A language with none falls back to the one label each already has.

use sea_orm_migration::prelude::*;

/// The `translated_labels` migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Add the translations.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared(
                "ALTER TABLE issue_types
                     ADD COLUMN IF NOT EXISTS labels JSONB NOT NULL DEFAULT '{}'::jsonb;
                 ALTER TABLE organization_roles
                     ADD COLUMN IF NOT EXISTS labels JSONB NOT NULL DEFAULT '{}'::jsonb;
                 ALTER TABLE workflow_states
                     ADD COLUMN IF NOT EXISTS labels JSONB NOT NULL DEFAULT '{}'::jsonb;",
            )
            .await?;
        Ok(())
    }

    /// Remove them. The one label each row already had stays.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared(
                "ALTER TABLE issue_types DROP COLUMN labels;
                 ALTER TABLE organization_roles DROP COLUMN labels;
                 ALTER TABLE workflow_states DROP COLUMN labels;",
            )
            .await?;
        Ok(())
    }
}
