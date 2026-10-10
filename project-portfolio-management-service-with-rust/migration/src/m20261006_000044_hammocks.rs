//! Migration: **hammock activities** (PJ-7). `tasks.is_hammock` marks a task whose span is derived from the tasks it is linked to; `hammock_links` are those links (a hammock → a member task or another hammock). A hammock is not work and is not in the dependency network.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The hammock migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Add the column, create the table and its checks.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();
        db.execute_unprepared(
            "ALTER TABLE tasks ADD COLUMN is_hammock boolean NOT NULL DEFAULT false",
        )
        .await?;
        create_table(
            m,
            "hammock_links",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("hammock_pid", ColType::Uuid),
                ("member_pid", ColType::Uuid),
            ],
            &[],
        )
        .await?;
        for q in [
            "ALTER TABLE hammock_links ADD CONSTRAINT hammock_links_distinct CHECK (hammock_pid <> member_pid)",
            "CREATE UNIQUE INDEX hammock_links_pair ON hammock_links (hammock_pid, member_pid)",
            "CREATE INDEX hammock_links_member ON hammock_links (member_pid)",
            // A hammock is a derived span, not work: it carries no duration or work of its own.
            "ALTER TABLE tasks ADD CONSTRAINT tasks_hammock_is_derived CHECK (NOT is_hammock OR (duration_days IS NULL AND work_minutes IS NULL AND constraint_kind IS NULL))",
        ] {
            db.execute_unprepared(q).await?;
        }
        Ok(())
    }

    /// Drop the table and the column.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "hammock_links").await?;
        let db = m.get_connection();
        db.execute_unprepared(
            "ALTER TABLE tasks DROP CONSTRAINT IF EXISTS tasks_hammock_is_derived",
        )
        .await?;
        db.execute_unprepared("ALTER TABLE tasks DROP COLUMN is_hammock")
            .await?;
        Ok(())
    }
}
