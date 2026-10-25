//! Migration: **blueprint sync** (NC-12). A template gains a `version`; every plan made from one records the
//! template, its place in the template's tree, the version it was last brought up to, a `baseline` of the template's
//! tasks for that plan, and which of the plan's tasks came from which template task.

use sea_orm_migration::prelude::*;

/// The migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Add the column and create the table.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared(
                "ALTER TABLE plan_templates ADD COLUMN IF NOT EXISTS version INTEGER NOT NULL DEFAULT 1;
                 CREATE TABLE IF NOT EXISTS plan_template_links (
                     created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     id BIGSERIAL PRIMARY KEY,
                     plan_pid UUID NOT NULL UNIQUE,
                     template_pid UUID NOT NULL,
                     node_key INTEGER NOT NULL,
                     template_version INTEGER NOT NULL,
                     baseline JSONB NOT NULL,
                     task_map JSONB NOT NULL,
                     CONSTRAINT plan_template_links_version CHECK (template_version >= 1)
                 );
                 CREATE INDEX IF NOT EXISTS plan_template_links_template_idx ON plan_template_links (template_pid);",
            )
            .await?;
        Ok(())
    }

    /// Drop the table and the column.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared(
                "DROP TABLE IF EXISTS plan_template_links; ALTER TABLE plan_templates DROP COLUMN IF EXISTS version",
            )
            .await?;
        Ok(())
    }
}
