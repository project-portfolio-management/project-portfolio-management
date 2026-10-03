//! Migration: **`schedule_baseline_tasks`**. One task's row in a schedule baseline (SC-3): its title and, when it was schedulable then, its early start, early finish, working duration and total float. A task that could not be scheduled is kept with null dates and the reason, so it is neither lost nor given a date it never had. Append-only.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The `schedule_baseline_tasks` migration (name derived from the module).
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
            "schedule_baseline_tasks",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("baseline_pid", ColType::Uuid),
                ("task_pid", ColType::Uuid),
                ("title", ColType::String),
                ("start_date", ColType::DateNull),
                ("finish_date", ColType::DateNull),
                ("duration_days", ColType::IntegerNull),
                ("total_float", ColType::IntegerNull),
                ("unscheduled_reason", ColType::StringNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        db.execute_unprepared(
            "CREATE UNIQUE INDEX schedule_baseline_tasks_once ON schedule_baseline_tasks (baseline_pid, task_pid)",
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
        drop_table(m, "schedule_baseline_tasks").await
    }
}
