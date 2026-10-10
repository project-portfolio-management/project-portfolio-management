//! Migration: **`timesheet_weeks`**. The approval state of one person's week of time entries (LT-11). A row exists only once a week has been submitted; no row means the week is open. Approved weeks are locked against new entries. `week_start` is the ISO Monday, a zone-free calendar date.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The `timesheet_weeks` migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Create the table and its indexes.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "timesheet_weeks",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("person_ref", ColType::String),
                ("week_start", ColType::Date),
                ("status", ColType::String),
                ("submitted_at", ColType::TimestampWithTimeZoneNull),
                ("decided_by", ColType::StringNull),
                ("decided_at", ColType::TimestampWithTimeZoneNull),
                ("decision_comment", ColType::StringNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        db.execute_unprepared(
            "CREATE UNIQUE INDEX timesheet_weeks_one ON timesheet_weeks (person_ref, week_start);",
        )
        .await?;
        db.execute_unprepared(
            "ALTER TABLE timesheet_weeks ADD CONSTRAINT timesheet_weeks_status CHECK (status IN ('submitted','approved','rejected'));",
        )
        .await?;
        db.execute_unprepared(
            "CREATE INDEX timesheet_weeks_status_idx ON timesheet_weeks (status);",
        )
        .await?;
        let _ = db;
        Ok(())
    }

    /// Drop the table.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "timesheet_weeks").await
    }
}
