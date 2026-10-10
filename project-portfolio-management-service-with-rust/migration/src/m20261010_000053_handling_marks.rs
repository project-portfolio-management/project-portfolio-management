//! Migration: **handling marks** (RG-7). A plan's mark is its own row, so a plan without a row is unmarked (the lowest level of the configured scheme) and the plan and its access settings are untouched. Which codes are valid is the deployment's scheme, not the database's; the code is only constrained in shape.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Create the table.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "plan_handling_marks",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("plan_pid", ColType::Uuid),
                ("mark", ColType::String),
                ("set_by", ColType::StringNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        for q in [
            "CREATE UNIQUE INDEX plan_handling_marks_plan ON plan_handling_marks (plan_pid)",
            "ALTER TABLE plan_handling_marks ADD CONSTRAINT plan_handling_marks_shape CHECK (char_length(mark) BETWEEN 1 AND 40 AND mark = btrim(mark))",
        ] {
            db.execute_unprepared(q).await?;
        }
        Ok(())
    }

    /// Drop the table.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "plan_handling_marks").await
    }
}
