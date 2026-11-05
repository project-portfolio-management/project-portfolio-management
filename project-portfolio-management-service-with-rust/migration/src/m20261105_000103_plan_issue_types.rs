//! Migration: **per-plan issue types** (OC-5). A plan can limit itself to some of the deployment's issue types, as a
//! Redmine project selects its trackers. No rows means every type, so nothing changes until a plan chooses.

use sea_orm_migration::prelude::*;

/// The `plan_issue_types` migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Add the table.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared(
                "CREATE TABLE IF NOT EXISTS plan_issue_types (
                     created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     id SERIAL PRIMARY KEY,
                     pid UUID NOT NULL UNIQUE DEFAULT gen_random_uuid(),
                     plan_pid UUID NOT NULL REFERENCES plans (pid) ON DELETE CASCADE,
                     type_key VARCHAR NOT NULL REFERENCES issue_types (key),
                     UNIQUE (plan_pid, type_key)
                 );",
            )
            .await?;
        Ok(())
    }

    /// Remove it, so every plan offers every type again.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared("DROP TABLE plan_issue_types;")
            .await?;
        Ok(())
    }
}
