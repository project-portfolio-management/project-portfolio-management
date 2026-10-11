//! Migration: **aggressive and safe estimates** per task (CC-2). One row per task; the task row itself is
//! untouched (its duration stays the likely one). Changes are audited by the service. No person column.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Create the table and checks.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "task_ccpm_estimates",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("plan_pid", ColType::Uuid),
                ("task_pid", ColType::UuidUniq),
                ("aggressive_days", ColType::IntegerNull),
                ("safe_days", ColType::IntegerNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        for q in [
            "ALTER TABLE task_ccpm_estimates ADD CONSTRAINT tce_aggressive CHECK (aggressive_days IS NULL OR aggressive_days BETWEEN 1 AND 3650)",
            "ALTER TABLE task_ccpm_estimates ADD CONSTRAINT tce_safe CHECK (safe_days IS NULL OR safe_days BETWEEN 1 AND 3650)",
            "ALTER TABLE task_ccpm_estimates ADD CONSTRAINT tce_order CHECK (aggressive_days IS NULL OR safe_days IS NULL OR aggressive_days <= safe_days)",
            "CREATE INDEX tce_by_plan ON task_ccpm_estimates (plan_pid)",
        ] {
            db.execute_unprepared(q).await?;
        }
        Ok(())
    }

    /// Drop what `up` made.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "task_ccpm_estimates").await
    }
}
