//! Migration: **`task_relations`**. Typed relations between tasks (LT-10), with Redmine's concrete behaviours: `blocks` stops the blocked task closing while the blocker is open; `precedes` forbids a successor due on or before its predecessor; `duplicates` closes the duplicate when the primary closes; `related` and `copied_from` are informational. Cross-plan relations are allowed.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The `task_relations` migration (name derived from the module).
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
            "task_relations",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("from_task_pid", ColType::Uuid),
                ("to_task_pid", ColType::Uuid),
                ("kind", ColType::String),
                ("deleted_at", ColType::TimestampWithTimeZoneNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        db.execute_unprepared(
            "ALTER TABLE task_relations ADD CONSTRAINT task_relations_kind CHECK (kind IN ('related','blocks','precedes','duplicates','copied_from'));",
        )
        .await?;
        db.execute_unprepared(
            "ALTER TABLE task_relations ADD CONSTRAINT task_relations_not_self CHECK (from_task_pid <> to_task_pid);",
        )
        .await?;
        db.execute_unprepared(
            "CREATE UNIQUE INDEX task_relations_one ON task_relations (from_task_pid, to_task_pid, kind) WHERE deleted_at IS NULL;",
        )
        .await?;
        db.execute_unprepared(
            "CREATE INDEX task_relations_to ON task_relations (to_task_pid) WHERE deleted_at IS NULL;",
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
        drop_table(m, "task_relations").await
    }
}
