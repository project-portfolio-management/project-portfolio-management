//! Migration: **`components`**. Named groupings of tasks inside a plan (PL-11), corroborated by Redmine's issue categories. A task may name one `component_pid` of its own plan; deleting a component that still has tasks is refused, never silently orphaning them.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The `components` migration (name derived from the module).
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
            "components",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("plan_pid", ColType::Uuid),
                ("name", ColType::String),
                ("lead_ref", ColType::StringNull),
                ("deleted_at", ColType::TimestampWithTimeZoneNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        db.execute_unprepared(
            "CREATE UNIQUE INDEX components_plan_name ON components (plan_pid, lower(name)) WHERE deleted_at IS NULL;",
        )
        .await?;
        db.execute_unprepared("ALTER TABLE tasks ADD COLUMN IF NOT EXISTS component_pid UUID;")
            .await?;
        db.execute_unprepared(
            "CREATE INDEX tasks_component ON tasks (component_pid) WHERE deleted_at IS NULL;",
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
        m.get_connection()
            .execute_unprepared("ALTER TABLE tasks DROP COLUMN IF EXISTS component_pid;")
            .await?;
        drop_table(m, "components").await
    }
}
