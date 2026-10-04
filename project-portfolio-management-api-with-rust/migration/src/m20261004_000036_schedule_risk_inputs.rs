//! Migration: **schedule-risk inputs** (SC-8). A task may carry an optimistic and a pessimistic duration (its likely one is `duration_days`); a risk may name the task it threatens, how many working days it adds if it happens, and its probability as a percent. All optional; none changes any existing behaviour.

use sea_orm_migration::prelude::*;

/// The schedule-risk inputs migration (name derived from the module).
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
            "ALTER TABLE tasks ADD COLUMN duration_optimistic_days integer",
            "ALTER TABLE tasks ADD COLUMN duration_pessimistic_days integer",
            "ALTER TABLE tasks ADD CONSTRAINT tasks_estimate_range CHECK ((duration_optimistic_days IS NULL OR (duration_optimistic_days >= 0 AND duration_optimistic_days <= 3650)) AND (duration_pessimistic_days IS NULL OR (duration_pessimistic_days >= 0 AND duration_pessimistic_days <= 3650)))",
            "ALTER TABLE risks ADD COLUMN task_pid uuid",
            "ALTER TABLE risks ADD COLUMN schedule_impact_days integer",
            "ALTER TABLE risks ADD COLUMN schedule_probability_percent integer",
            "ALTER TABLE risks ADD CONSTRAINT risks_schedule_range CHECK ((schedule_impact_days IS NULL OR (schedule_impact_days >= 1 AND schedule_impact_days <= 365)) AND (schedule_probability_percent IS NULL OR (schedule_probability_percent >= 0 AND schedule_probability_percent <= 100)))",
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
            "ALTER TABLE risks DROP CONSTRAINT risks_schedule_range",
            "ALTER TABLE risks DROP COLUMN schedule_probability_percent",
            "ALTER TABLE risks DROP COLUMN schedule_impact_days",
            "ALTER TABLE risks DROP COLUMN task_pid",
            "ALTER TABLE tasks DROP CONSTRAINT tasks_estimate_range",
            "ALTER TABLE tasks DROP COLUMN duration_pessimistic_days",
            "ALTER TABLE tasks DROP COLUMN duration_optimistic_days",
        ] {
            db.execute_unprepared(q).await?;
        }
        Ok(())
    }
}
