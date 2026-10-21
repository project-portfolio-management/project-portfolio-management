//! Migration: a fifth tolerance dimension, `schedule`: how many calendar days the forecast finish is after the finish
//! recorded in the plan's latest schedule baseline. (`time` stays the slip against the plan's `target_date`.) The two
//! checks that list the dimensions are widened, in the tolerances table and on escalations that cite a tolerance.

use sea_orm_migration::prelude::*;

/// The migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Widen the checks.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared(
                "ALTER TABLE plan_tolerances DROP CONSTRAINT IF EXISTS plan_tolerances_dimension;
                 ALTER TABLE plan_tolerances ADD CONSTRAINT plan_tolerances_dimension
                     CHECK (dimension IN ('time','schedule','cost','risk','resources'));
                 ALTER TABLE escalations DROP CONSTRAINT IF EXISTS escalations_dimension;
                 ALTER TABLE escalations ADD CONSTRAINT escalations_dimension CHECK (tolerance_dimension IS NULL OR
                     (reason_kind = 'tolerance' AND tolerance_dimension IN ('time','schedule','cost','risk','resources')));",
            )
            .await?;
        Ok(())
    }

    /// Remove `schedule` tolerances and escalations' citations of it, and restore the narrower checks.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared(
                "DELETE FROM plan_tolerances WHERE dimension = 'schedule';
                 UPDATE escalations SET tolerance_dimension = NULL, reason_kind = 'other'
                     WHERE tolerance_dimension = 'schedule';
                 ALTER TABLE plan_tolerances DROP CONSTRAINT IF EXISTS plan_tolerances_dimension;
                 ALTER TABLE plan_tolerances ADD CONSTRAINT plan_tolerances_dimension
                     CHECK (dimension IN ('time','cost','risk','resources'));
                 ALTER TABLE escalations DROP CONSTRAINT IF EXISTS escalations_dimension;
                 ALTER TABLE escalations ADD CONSTRAINT escalations_dimension CHECK (tolerance_dimension IS NULL OR
                     (reason_kind = 'tolerance' AND tolerance_dimension IN ('time','cost','risk','resources')));",
            )
            .await?;
        Ok(())
    }
}
