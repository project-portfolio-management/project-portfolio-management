//! Migration: **`resource_roles`**. A person's one primary role for capacity planning (SC-7): the role whose supply they count toward in the portfolio resource view. One row per person; changing it replaces the row, and the change is audited.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The `resource_roles` migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Create the table and its index.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "resource_roles",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("person_ref", ColType::String),
                ("role", ColType::String),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        db.execute_unprepared(
            "CREATE UNIQUE INDEX resource_roles_person ON resource_roles (person_ref)",
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
        drop_table(m, "resource_roles").await
    }
}
