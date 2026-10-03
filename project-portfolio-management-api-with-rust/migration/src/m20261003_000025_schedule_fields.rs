//! Migration: **schedule fields** (PJ-1, SC-1). A task gains an optional planned
//! `start_date` and a working-day `duration_days`, so it can be a node in the
//! critical-path network beside plans; a dependency gains a relationship `kind`
//! (`fs` default, so every existing edge means what it meant). Both are
//! nullable or defaulted: no existing row changes meaning.

use sea_orm_migration::prelude::*;

/// The migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Add the columns and their checks.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();
        for q in [
            "ALTER TABLE tasks ADD COLUMN start_date date",
            "ALTER TABLE tasks ADD COLUMN duration_days integer",
            "ALTER TABLE tasks ADD CONSTRAINT tasks_duration_days CHECK (duration_days IS NULL OR (duration_days >= 0 AND duration_days <= 3650))",
            "ALTER TABLE plan_dependencies ADD COLUMN kind text NOT NULL DEFAULT 'fs'",
            "ALTER TABLE plan_dependencies ADD CONSTRAINT plan_dependencies_kind CHECK (kind IN ('fs','ss','ff','sf'))",
        ] {
            db.execute_unprepared(q).await?;
        }
        Ok(())
    }

    /// Drop the columns.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();
        for q in [
            "ALTER TABLE plan_dependencies DROP CONSTRAINT plan_dependencies_kind",
            "ALTER TABLE plan_dependencies DROP COLUMN kind",
            "ALTER TABLE tasks DROP CONSTRAINT tasks_duration_days",
            "ALTER TABLE tasks DROP COLUMN duration_days",
            "ALTER TABLE tasks DROP COLUMN start_date",
        ] {
            db.execute_unprepared(q).await?;
        }
        Ok(())
    }
}
