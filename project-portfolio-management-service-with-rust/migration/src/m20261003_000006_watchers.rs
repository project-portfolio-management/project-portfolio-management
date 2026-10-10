//! Migration: **`watchers`**. Who follows a subject (RM-3). One row per (subject, user); unwatching deletes the row. A watcher joins the comment and status-change notification targets; it grants no access of its own.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The `watchers` migration (name derived from the module).
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
            "watchers",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("subject_kind", ColType::String),
                ("subject_pid", ColType::Uuid),
                ("sub", ColType::String),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        db.execute_unprepared(
            "CREATE UNIQUE INDEX watchers_one ON watchers (subject_kind, subject_pid, sub);",
        )
        .await?;
        db.execute_unprepared("CREATE INDEX watchers_sub ON watchers (sub);")
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
        drop_table(m, "watchers").await
    }
}
