//! Migration: **per-status field rules** (OC-4). A workflow state can make an issue's field required or read-only, for
//! everyone or for named roles, as Redmine's workflow field permissions do. Nothing changes until a workflow declares one.

use sea_orm_migration::prelude::*;

/// The `issue_field_rules` migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Add the rules to a state.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared(
                "ALTER TABLE workflow_states
                     ADD COLUMN IF NOT EXISTS field_rules JSONB NOT NULL DEFAULT '[]'::jsonb;
                 ALTER TABLE workflow_states DROP CONSTRAINT IF EXISTS workflow_states_field_rules_array;
                 ALTER TABLE workflow_states ADD CONSTRAINT workflow_states_field_rules_array
                     CHECK (jsonb_typeof(field_rules) = 'array');",
            )
            .await?;
        Ok(())
    }

    /// Remove them.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared("ALTER TABLE workflow_states DROP COLUMN field_rules;")
            .await?;
        Ok(())
    }
}
