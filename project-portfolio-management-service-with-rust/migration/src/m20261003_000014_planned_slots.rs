//! Migration: **`planned_slots`**. A person's own time-blocking of a task (PL-4). Instants are UTC (`timestamptz`) and are rendered in the caller's zone on read. A slot never changes the task's due date, and overlapping slots are allowed — reported as clashes, not refused.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The `planned_slots` migration (name derived from the module).
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
            "planned_slots",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("sub", ColType::String),
                ("task_pid", ColType::Uuid),
                ("starts_at", ColType::TimestampWithTimeZone),
                ("ends_at", ColType::TimestampWithTimeZone),
                ("deleted_at", ColType::TimestampWithTimeZoneNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        db.execute_unprepared(
            "ALTER TABLE planned_slots ADD CONSTRAINT planned_slots_order CHECK (ends_at > starts_at);",
        )
        .await?;
        db.execute_unprepared(
            "CREATE INDEX planned_slots_person ON planned_slots (sub, starts_at) WHERE deleted_at IS NULL;",
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
        drop_table(m, "planned_slots").await
    }
}
