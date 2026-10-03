//! Migration: **`report_schedules`**. A saved report run on a cadence (T-28e): each due run is stored as an expiring artifact and audited like an export, even at zero rows. Dates are UTC calendar dates (a portfolio view, not a personal one).

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The `report_schedules` migration (name derived from the module).
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
            "report_schedules",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("report_pid", ColType::Uuid),
                ("owner", ColType::String),
                ("cadence", ColType::String),
                ("format", ColType::String),
                ("anchor_date", ColType::Date),
                ("next_run_date", ColType::Date),
                ("enabled", ColType::Boolean),
                ("last_run_at", ColType::TimestampWithTimeZoneNull),
                ("last_status", ColType::StringNull),
                ("last_rows", ColType::IntegerNull),
                ("last_artifact_pid", ColType::UuidNull),
                ("last_error", ColType::TextNull),
                ("deleted_at", ColType::TimestampWithTimeZoneNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        db.execute_unprepared(
            "ALTER TABLE report_schedules ADD CONSTRAINT report_schedules_cadence CHECK (cadence IN ('daily','weekly','monthly'));",
        )
        .await?;
        db.execute_unprepared(
            "ALTER TABLE report_schedules ADD CONSTRAINT report_schedules_format CHECK (format IN ('csv','json'));",
        )
        .await?;
        db.execute_unprepared(
            "CREATE INDEX report_schedules_due ON report_schedules (next_run_date) WHERE enabled AND deleted_at IS NULL;",
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
        drop_table(m, "report_schedules").await
    }
}
