//! Migration: **`earned_value_settings`**. A plan's earned-value setting (SC-4): how its tasks are counted by default (`zero_hundred`, `fifty_fifty`, or `points`, which weights by story points). A plan with no row counts `zero_hundred`. Also adds the per-task override columns: a task may be counted another way, or by a person-entered percent with a mandatory note.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The `earned_value_settings` migration (name derived from the module).
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
            "earned_value_settings",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("plan_pid", ColType::Uuid),
                ("default_method", ColType::String),
                ("deleted_at", ColType::TimestampWithTimeZoneNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        db.execute_unprepared(
            "ALTER TABLE earned_value_settings ADD CONSTRAINT earned_value_settings_method CHECK (default_method IN ('zero_hundred','fifty_fifty','points'))",
        )
        .await?;
        db.execute_unprepared(
            "CREATE UNIQUE INDEX earned_value_settings_one ON earned_value_settings (plan_pid) WHERE deleted_at IS NULL",
        )
        .await?;
        db.execute_unprepared("ALTER TABLE tasks ADD COLUMN percent_method text")
            .await?;
        db.execute_unprepared("ALTER TABLE tasks ADD COLUMN percent_manual integer")
            .await?;
        db.execute_unprepared("ALTER TABLE tasks ADD COLUMN percent_note text")
            .await?;
        db.execute_unprepared(
            "ALTER TABLE tasks ADD CONSTRAINT tasks_percent_method CHECK (percent_method IS NULL OR percent_method IN ('zero_hundred','fifty_fifty','manual'))",
        )
        .await?;
        db.execute_unprepared(
            "ALTER TABLE tasks ADD CONSTRAINT tasks_percent_manual CHECK ((percent_method = 'manual') = (percent_manual IS NOT NULL) AND (percent_manual IS NULL OR (percent_manual >= 0 AND percent_manual <= 100)))",
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
            .execute_unprepared("ALTER TABLE tasks DROP CONSTRAINT tasks_percent_manual")
            .await?;
        m.get_connection()
            .execute_unprepared("ALTER TABLE tasks DROP CONSTRAINT tasks_percent_method")
            .await?;
        m.get_connection()
            .execute_unprepared("ALTER TABLE tasks DROP COLUMN percent_note")
            .await?;
        m.get_connection()
            .execute_unprepared("ALTER TABLE tasks DROP COLUMN percent_manual")
            .await?;
        m.get_connection()
            .execute_unprepared("ALTER TABLE tasks DROP COLUMN percent_method")
            .await?;
        drop_table(m, "earned_value_settings").await
    }
}
