//! Migration: **funding sources** (DP-6). An owner-supplied reference list, and an optional funding
//! source on each budget line. A line with none is reported as "no funding source", not hidden.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Create `funding_sources` and add `budget_lines.funding_source_pid`.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "funding_sources",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("name", ColType::String),
                ("active", ColType::Boolean),
                ("created_by", ColType::StringNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        for q in [
            "CREATE UNIQUE INDEX funding_sources_name ON funding_sources (lower(name))",
            "ALTER TABLE funding_sources ADD CONSTRAINT funding_sources_name_present CHECK (char_length(btrim(name)) >= 2)",
            "ALTER TABLE budget_lines ADD COLUMN funding_source_pid uuid",
            "CREATE INDEX budget_lines_by_source ON budget_lines (funding_source_pid)",
        ] {
            db.execute_unprepared(q).await?;
        }
        Ok(())
    }

    /// Drop what `up` made.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();
        db.execute_unprepared("DROP INDEX IF EXISTS budget_lines_by_source")
            .await?;
        db.execute_unprepared("ALTER TABLE budget_lines DROP COLUMN IF EXISTS funding_source_pid")
            .await?;
        drop_table(m, "funding_sources").await
    }
}
