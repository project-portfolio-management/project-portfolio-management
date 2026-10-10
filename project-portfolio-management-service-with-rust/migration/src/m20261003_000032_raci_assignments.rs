//! Migration: **`raci_assignments`**. A RACI assignment: one person's role on one task: Responsible, Accountable, Consulted or Informed. A person may hold several letters on a task (the usual A/R); a task has at most one Accountable (enforced by the service) and should have exactly one (reported when it does not). Replacing a task's row soft-deletes the old assignments, so the history is kept.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The `raci_assignments` migration (name derived from the module).
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
            "raci_assignments",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("plan_pid", ColType::Uuid),
                ("task_pid", ColType::Uuid),
                ("person_ref", ColType::String),
                ("role", ColType::String),
                ("deleted_at", ColType::TimestampWithTimeZoneNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        db.execute_unprepared(
            "ALTER TABLE raci_assignments ADD CONSTRAINT raci_assignments_role CHECK (role IN ('R','A','C','I'))",
        )
        .await?;
        db.execute_unprepared(
            "CREATE UNIQUE INDEX raci_assignments_once ON raci_assignments (task_pid, person_ref, role) WHERE deleted_at IS NULL",
        )
        .await?;
        db.execute_unprepared(
            "CREATE UNIQUE INDEX raci_assignments_one_accountable ON raci_assignments (task_pid) WHERE role = 'A' AND deleted_at IS NULL",
        )
        .await?;
        db.execute_unprepared(
            "CREATE INDEX raci_assignments_plan ON raci_assignments (plan_pid) WHERE deleted_at IS NULL",
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
        drop_table(m, "raci_assignments").await
    }
}
