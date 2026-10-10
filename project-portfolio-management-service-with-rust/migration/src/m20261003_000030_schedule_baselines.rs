//! Migration: **`schedule_baselines`**. A schedule baseline (SC-3): a numbered, append-only snapshot of a plan's task schedule, taken with a reason. Never updated or deleted; a re-baseline is the next version and the earlier one stays. Variance is always measured against a named version.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The `schedule_baselines` migration (name derived from the module).
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
            "schedule_baselines",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("plan_pid", ColType::Uuid),
                ("version", ColType::Integer),
                ("reason", ColType::Text),
                ("created_by", ColType::StringNull),
                ("task_count", ColType::Integer),
                ("project_finish", ColType::DateNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        db.execute_unprepared(
            "CREATE UNIQUE INDEX schedule_baselines_version ON schedule_baselines (plan_pid, version)",
        )
        .await?;
        db.execute_unprepared(
            "ALTER TABLE schedule_baselines ADD CONSTRAINT schedule_baselines_version_pos CHECK (version >= 1)",
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
        drop_table(m, "schedule_baselines").await
    }
}
