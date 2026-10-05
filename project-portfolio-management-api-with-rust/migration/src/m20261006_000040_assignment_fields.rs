//! Migration: **assignment fields on tasks** (PJ-5, PJ-6). A task with an assignee may carry the three quantities scheduling tools relate — `duration_days` (already there), `units_percent` (how much of the assignee's day) and `work_minutes` — with a `scheduling_rule` saying which of the three stays fixed when another is edited, and a `contour` saying how its work is spread over its working days (`custom` carries its own weights). All optional; every existing task is unchanged.

use sea_orm_migration::prelude::*;

/// The assignment-fields migration (name derived from the module).
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
            "ALTER TABLE tasks ADD COLUMN work_minutes bigint",
            "ALTER TABLE tasks ADD COLUMN units_percent integer",
            "ALTER TABLE tasks ADD COLUMN scheduling_rule text",
            "ALTER TABLE tasks ADD COLUMN contour text NOT NULL DEFAULT 'flat'",
            "ALTER TABLE tasks ADD COLUMN contour_custom jsonb",
            "ALTER TABLE tasks ADD CONSTRAINT tasks_work_range CHECK (work_minutes IS NULL OR (work_minutes >= 0 AND work_minutes <= 17520000))",
            "ALTER TABLE tasks ADD CONSTRAINT tasks_units_range CHECK (units_percent IS NULL OR (units_percent >= 1 AND units_percent <= 1000))",
            "ALTER TABLE tasks ADD CONSTRAINT tasks_scheduling_rule CHECK (scheduling_rule IS NULL OR scheduling_rule IN ('fixed_units','fixed_duration','fixed_work'))",
            "ALTER TABLE tasks ADD CONSTRAINT tasks_contour CHECK (contour IN ('flat','front_loaded','back_loaded','bell','turtle','custom'))",
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
            "ALTER TABLE tasks DROP CONSTRAINT tasks_contour",
            "ALTER TABLE tasks DROP CONSTRAINT tasks_scheduling_rule",
            "ALTER TABLE tasks DROP CONSTRAINT tasks_units_range",
            "ALTER TABLE tasks DROP CONSTRAINT tasks_work_range",
            "ALTER TABLE tasks DROP COLUMN contour_custom",
            "ALTER TABLE tasks DROP COLUMN contour",
            "ALTER TABLE tasks DROP COLUMN scheduling_rule",
            "ALTER TABLE tasks DROP COLUMN units_percent",
            "ALTER TABLE tasks DROP COLUMN work_minutes",
        ] {
            db.execute_unprepared(q).await?;
        }
        Ok(())
    }
}
