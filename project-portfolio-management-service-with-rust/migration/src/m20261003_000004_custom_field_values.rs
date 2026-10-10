//! Migration: **`custom_field_values`**. The values of custom fields, one row per (subject, field key). A separate table, not the plan payload: a custom value can never reach the matcher (the matchable / operational partition holds by construction).

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The `custom_field_values` migration (name derived from the module).
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
            "custom_field_values",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("subject_kind", ColType::String),
                ("subject_pid", ColType::Uuid),
                ("field_key", ColType::String),
                ("value", ColType::JsonBinary),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        db.execute_unprepared(
            "CREATE UNIQUE INDEX custom_field_values_one ON custom_field_values (subject_kind, subject_pid, field_key);",
        )
        .await?;
        db.execute_unprepared(
            "CREATE INDEX custom_field_values_key ON custom_field_values (subject_kind, field_key);",
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
        drop_table(m, "custom_field_values").await
    }
}
