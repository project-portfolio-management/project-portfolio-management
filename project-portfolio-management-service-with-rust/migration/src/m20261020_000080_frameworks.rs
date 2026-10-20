//! Migration: **frameworks**. Named, reusable descriptions of how a team works (`family = ways_of_working`: methodology,
//! cadence, WIP limit, definitions of ready and done, ceremonies) or is governed (`family = governance`: decision model,
//! review and reporting cadence, escalation path, decision rights, forums). A framework is organisation-wide or owned
//! by one plan; a plan follows at most one per family through `plan_frameworks`, and a plan with no link inherits the
//! nearest ancestor's (resolved on read, never stored). The family's own fields are in `body`, checked by the service.

use sea_orm_migration::prelude::*;

/// The migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Create both tables.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared(
                "CREATE TABLE IF NOT EXISTS frameworks (
                     created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     id BIGSERIAL PRIMARY KEY,
                     pid UUID NOT NULL UNIQUE,
                     family VARCHAR NOT NULL,
                     name VARCHAR NOT NULL,
                     description TEXT NULL,
                     body JSONB NOT NULL DEFAULT '{}'::jsonb,
                     owner_plan_pid UUID NULL,
                     created_by VARCHAR NULL,
                     deleted_at TIMESTAMPTZ NULL,
                     CONSTRAINT frameworks_family CHECK (family IN ('ways_of_working','governance')),
                     CONSTRAINT frameworks_name CHECK (char_length(btrim(name)) BETWEEN 1 AND 120)
                 );
                 -- One name per family and owner (a plan, or the organisation) among live records.
                 CREATE UNIQUE INDEX IF NOT EXISTS frameworks_name_once
                     ON frameworks (family, COALESCE(owner_plan_pid, '00000000-0000-0000-0000-000000000000'::uuid), lower(btrim(name)))
                     WHERE deleted_at IS NULL;
                 CREATE TABLE IF NOT EXISTS plan_frameworks (
                     created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     id BIGSERIAL PRIMARY KEY,
                     pid UUID NOT NULL UNIQUE,
                     plan_pid UUID NOT NULL,
                     family VARCHAR NOT NULL,
                     framework_pid UUID NOT NULL,
                     set_by VARCHAR NULL,
                     CONSTRAINT plan_frameworks_one_per_family UNIQUE (plan_pid, family)
                 );
                 CREATE INDEX IF NOT EXISTS plan_frameworks_target ON plan_frameworks (framework_pid);",
            )
            .await?;
        Ok(())
    }

    /// Drop both tables.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared(
                "DROP TABLE IF EXISTS plan_frameworks; DROP TABLE IF EXISTS frameworks;",
            )
            .await?;
        Ok(())
    }
}
