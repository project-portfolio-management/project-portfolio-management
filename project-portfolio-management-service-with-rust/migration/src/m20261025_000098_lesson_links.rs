//! Migration: **`lessons.link_kind` and `lessons.link_pid`** (BR-4). A lesson may point at one record of its plan, so
//! the plan can ask which meetings and lessons mention a task, risk, issue or milestone.

use sea_orm_migration::prelude::*;

/// The migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Add the columns and the index.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();
        db.execute_unprepared("ALTER TABLE lessons ADD COLUMN IF NOT EXISTS link_kind TEXT, ADD COLUMN IF NOT EXISTS link_pid UUID")
            .await?;
        db.execute_unprepared("CREATE INDEX IF NOT EXISTS idx_lessons_link ON lessons (link_pid) WHERE link_pid IS NOT NULL")
            .await?;
        db.execute_unprepared("CREATE INDEX IF NOT EXISTS idx_meeting_items_link ON meeting_items (link_pid) WHERE link_pid IS NOT NULL")
            .await?;
        Ok(())
    }

    /// Drop them.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();
        db.execute_unprepared("DROP INDEX IF EXISTS idx_meeting_items_link")
            .await?;
        db.execute_unprepared("DROP INDEX IF EXISTS idx_lessons_link")
            .await?;
        db.execute_unprepared(
            "ALTER TABLE lessons DROP COLUMN IF EXISTS link_pid, DROP COLUMN IF EXISTS link_kind",
        )
        .await?;
        Ok(())
    }
}
