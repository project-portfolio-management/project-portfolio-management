//! Migration: **times on an issue** (SM-2). An issue had only the dates it was raised and resolved, which no service level can
//! be measured from. Existing issues keep their dates and show no time: nothing is back-filled by guessing, so the default on
//! `reported_at` is set after the column is added and reaches new rows only.

use sea_orm_migration::prelude::*;

/// The `issue_times` migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Add the times.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared(
                "ALTER TABLE issues
                     ADD COLUMN IF NOT EXISTS reported_at TIMESTAMPTZ,
                     ADD COLUMN IF NOT EXISTS acknowledged_at TIMESTAMPTZ,
                     ADD COLUMN IF NOT EXISTS resolved_at TIMESTAMPTZ,
                     ADD COLUMN IF NOT EXISTS closed_at TIMESTAMPTZ,
                     ADD COLUMN IF NOT EXISTS occurred_at TIMESTAMPTZ,
                     ADD COLUMN IF NOT EXISTS detected_at TIMESTAMPTZ;
                 ALTER TABLE issues ALTER COLUMN reported_at SET DEFAULT now();
                 ALTER TABLE issues DROP CONSTRAINT IF EXISTS issues_occurred_before_detected;
                 ALTER TABLE issues ADD CONSTRAINT issues_occurred_before_detected
                     CHECK (occurred_at IS NULL OR detected_at IS NULL OR occurred_at <= detected_at);",
            )
            .await?;
        Ok(())
    }

    /// Remove them.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared(
                "ALTER TABLE issues
                     DROP CONSTRAINT IF EXISTS issues_occurred_before_detected,
                     DROP COLUMN reported_at,
                     DROP COLUMN acknowledged_at,
                     DROP COLUMN resolved_at,
                     DROP COLUMN closed_at,
                     DROP COLUMN occurred_at,
                     DROP COLUMN detected_at;",
            )
            .await?;
        Ok(())
    }
}
