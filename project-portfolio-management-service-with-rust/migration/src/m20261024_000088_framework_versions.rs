//! Migration: **framework versions** (CF-4). Editing a framework used to change it for every plan that follows it.
//! Now each edit appends a version to `framework_versions` (append-only, enforced by a trigger), `frameworks` keeps
//! the current one, and a link in `plan_frameworks` pins the version its plan follows, so a plan keeps the version it
//! chose until it is moved. Every existing framework becomes version 1 and every existing link is pinned to 1, which
//! is the content they have now, so nothing changes on upgrade.

use sea_orm_migration::prelude::*;

/// The migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Add the versions table and the two version columns, and backfill version 1.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared(
                "ALTER TABLE frameworks ADD COLUMN IF NOT EXISTS version INTEGER NOT NULL DEFAULT 1;
                 ALTER TABLE plan_frameworks ADD COLUMN IF NOT EXISTS version INTEGER NOT NULL DEFAULT 1;
                 CREATE TABLE IF NOT EXISTS framework_versions (
                     created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     id BIGSERIAL PRIMARY KEY,
                     framework_pid UUID NOT NULL,
                     version INTEGER NOT NULL,
                     name VARCHAR NOT NULL,
                     description TEXT NULL,
                     body JSONB NOT NULL DEFAULT '{}'::jsonb,
                     created_by VARCHAR NULL,
                     CONSTRAINT framework_versions_once UNIQUE (framework_pid, version),
                     CONSTRAINT framework_versions_positive CHECK (version >= 1)
                 );
                 INSERT INTO framework_versions (framework_pid, version, name, description, body, created_by, created_at)
                     SELECT pid, 1, name, description, body, created_by, created_at FROM frameworks
                     ON CONFLICT (framework_pid, version) DO NOTHING;
                 CREATE OR REPLACE FUNCTION framework_versions_append_only() RETURNS trigger AS $$
                 BEGIN RAISE EXCEPTION 'framework_versions is append-only'; END; $$ LANGUAGE plpgsql;
                 DROP TRIGGER IF EXISTS framework_versions_no_change ON framework_versions;
                 CREATE TRIGGER framework_versions_no_change BEFORE UPDATE OR DELETE ON framework_versions
                     FOR EACH ROW EXECUTE FUNCTION framework_versions_append_only();",
            )
            .await?;
        Ok(())
    }

    /// Drop the versions table and the columns.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared(
                "DROP TABLE IF EXISTS framework_versions;
                 DROP FUNCTION IF EXISTS framework_versions_append_only();
                 ALTER TABLE plan_frameworks DROP COLUMN IF EXISTS version;
                 ALTER TABLE frameworks DROP COLUMN IF EXISTS version;",
            )
            .await?;
        Ok(())
    }
}
