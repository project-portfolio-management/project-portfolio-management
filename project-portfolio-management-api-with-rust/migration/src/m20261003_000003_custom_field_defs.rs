//! Migration: **`custom_field_defs`**. Admin-declared, typed custom fields for plans and tasks (RM-1). A definition is soft-deleted, never dropped: values already stored stay readable.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The `custom_field_defs` migration (name derived from the module).
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
            "custom_field_defs",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("key", ColType::String),
                ("label", ColType::String),
                ("field_type", ColType::String),
                ("applies_to", ColType::String),
                ("kinds", ColType::JsonBinary),
                ("required", ColType::Boolean),
                ("options", ColType::JsonBinary),
                ("deleted_at", ColType::TimestampWithTimeZoneNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        db.execute_unprepared(
            "CREATE UNIQUE INDEX custom_field_defs_key ON custom_field_defs (applies_to, key) WHERE deleted_at IS NULL;",
        )
        .await?;
        db.execute_unprepared(
            "ALTER TABLE custom_field_defs ADD CONSTRAINT custom_field_defs_type CHECK (field_type IN ('text','date','bool','integer','list','multi_list'));",
        )
        .await?;
        db.execute_unprepared(
            "ALTER TABLE custom_field_defs ADD CONSTRAINT custom_field_defs_applies CHECK (applies_to IN ('plan','task'));",
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
        drop_table(m, "custom_field_defs").await
    }
}
