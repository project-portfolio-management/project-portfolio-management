//! Migration: **`scheduled_job_runs`** (NC-5). One row per run of a job the service schedules for itself, so that
//! replicas can tell a job has just run, and an operator can see what ran and how it ended.

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
                "CREATE TABLE IF NOT EXISTS scheduled_job_runs (
                     created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     id BIGSERIAL PRIMARY KEY,
                     pid UUID NOT NULL UNIQUE,
                     job VARCHAR NOT NULL,
                     started_at TIMESTAMPTZ NOT NULL,
                     finished_at TIMESTAMPTZ NULL,
                     outcome VARCHAR NOT NULL DEFAULT 'running',
                     detail TEXT NULL,
                     CONSTRAINT scheduled_job_runs_outcome CHECK (outcome IN ('running', 'ok', 'failed'))
                 );
                 CREATE INDEX IF NOT EXISTS scheduled_job_runs_job_idx ON scheduled_job_runs (job, started_at DESC);",
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
            .execute_unprepared("DROP TABLE IF EXISTS scheduled_job_runs")
            .await?;
        Ok(())
    }
}
