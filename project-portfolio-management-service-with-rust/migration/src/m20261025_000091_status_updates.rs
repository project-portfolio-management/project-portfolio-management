//! Migration: **`plan_status_cadences`** and **`status_updates`** (NC-4). A plan says how often it expects a status
//! update; each update is an on-track / at-risk / off-track call with a narrative. Updates are append-only: a wrong
//! one is answered by a newer one, not edited.

use sea_orm_migration::prelude::*;

/// The migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Create the tables.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared(
                "CREATE TABLE IF NOT EXISTS plan_status_cadences (
                     created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     id BIGSERIAL PRIMARY KEY,
                     plan_pid UUID NOT NULL UNIQUE,
                     cadence_days INTEGER NOT NULL,
                     set_on DATE NOT NULL,
                     set_by VARCHAR NULL,
                     reminded_for DATE NULL,
                     CONSTRAINT plan_status_cadences_days CHECK (cadence_days BETWEEN 1 AND 92)
                 );
                 CREATE TABLE IF NOT EXISTS status_updates (
                     created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     id BIGSERIAL PRIMARY KEY,
                     pid UUID NOT NULL UNIQUE,
                     plan_pid UUID NOT NULL,
                     status VARCHAR NOT NULL,
                     narrative TEXT NOT NULL,
                     posted_on DATE NOT NULL,
                     posted_by VARCHAR NULL,
                     CONSTRAINT status_updates_status CHECK (status IN ('on_track', 'at_risk', 'off_track')),
                     CONSTRAINT status_updates_narrative CHECK (char_length(btrim(narrative)) > 0)
                 );
                 CREATE INDEX IF NOT EXISTS status_updates_plan_idx ON status_updates (plan_pid, id DESC);",
            )
            .await?;
        Ok(())
    }

    /// Drop the tables.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared(
                "DROP TABLE IF EXISTS status_updates; DROP TABLE IF EXISTS plan_status_cadences",
            )
            .await?;
        Ok(())
    }
}
