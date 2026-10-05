//! Migration: **cost-risk inputs**. A budget line may carry a low and a high estimate of its final cost (its likely one is `planned_minor`); a risk may carry the cost it adds in a named currency if it happens (its probability is the same `schedule_probability_percent` the schedule simulation uses, so one event strikes once and drives both). All optional; none changes any existing behaviour.

use sea_orm_migration::prelude::*;

/// The cost-risk inputs migration (name derived from the module).
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
            "ALTER TABLE budget_lines ADD COLUMN low_minor bigint",
            "ALTER TABLE budget_lines ADD COLUMN high_minor bigint",
            "ALTER TABLE budget_lines ADD CONSTRAINT budget_lines_estimate_order CHECK ((low_minor IS NULL OR (low_minor >= 0 AND low_minor <= planned_minor)) AND (high_minor IS NULL OR high_minor >= planned_minor))",
            "ALTER TABLE risks ADD COLUMN cost_impact_minor bigint",
            "ALTER TABLE risks ADD COLUMN cost_currency text",
            "ALTER TABLE risks ADD CONSTRAINT risks_cost_impact CHECK ((cost_impact_minor IS NULL AND cost_currency IS NULL) OR (cost_impact_minor > 0 AND cost_currency IS NOT NULL))",
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
            "ALTER TABLE risks DROP CONSTRAINT risks_cost_impact",
            "ALTER TABLE risks DROP COLUMN cost_currency",
            "ALTER TABLE risks DROP COLUMN cost_impact_minor",
            "ALTER TABLE budget_lines DROP CONSTRAINT budget_lines_estimate_order",
            "ALTER TABLE budget_lines DROP COLUMN high_minor",
            "ALTER TABLE budget_lines DROP COLUMN low_minor",
        ] {
            db.execute_unprepared(q).await?;
        }
        Ok(())
    }
}
