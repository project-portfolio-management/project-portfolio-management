//! Migration: **`plan_templates`**. A reusable plan tree (LT-2). The snapshot is self-contained JSON: once saved, a template does not depend on the plan it was captured from. It carries **no person reference and no identity** (leads, identifiers, codes), and every date is an **offset** from a base date.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The `plan_templates` migration (name derived from the module).
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
            "plan_templates",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("name", ColType::String),
                ("description", ColType::TextNull),
                ("snapshot", ColType::JsonBinary),
                ("created_by", ColType::StringNull),
                ("deleted_at", ColType::TimestampWithTimeZoneNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        db.execute_unprepared(
            "CREATE UNIQUE INDEX plan_templates_name ON plan_templates (lower(name)) WHERE deleted_at IS NULL;",
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
        drop_table(m, "plan_templates").await
    }
}
