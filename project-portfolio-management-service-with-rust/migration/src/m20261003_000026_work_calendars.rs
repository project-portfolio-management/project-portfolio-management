//! Migration: **`work_calendars`**. A working calendar (SC-2): the weekdays that work and a list of holidays, for one plan or one person. A task is scheduled on its plan's calendar; an assigned person's own calendar (plus their recorded absences) narrows it. Also adds the per-task date constraint and deadline columns the critical-path engine reads.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The `work_calendars` migration (name derived from the module).
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
            "work_calendars",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("scope", ColType::String),
                ("subject", ColType::String),
                ("weekdays", ColType::String),
                ("holidays", ColType::JsonBinary),
                ("deleted_at", ColType::TimestampWithTimeZoneNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        db.execute_unprepared(
            "ALTER TABLE work_calendars ADD CONSTRAINT work_calendars_scope CHECK (scope IN ('plan','person'))",
        )
        .await?;
        db.execute_unprepared(
            "ALTER TABLE work_calendars ADD CONSTRAINT work_calendars_weekdays CHECK (weekdays ~ '^[01]{7}$' AND weekdays <> '0000000')",
        )
        .await?;
        db.execute_unprepared(
            "CREATE UNIQUE INDEX work_calendars_one ON work_calendars (scope, subject) WHERE deleted_at IS NULL",
        )
        .await?;
        db.execute_unprepared("ALTER TABLE tasks ADD COLUMN constraint_kind text")
            .await?;
        db.execute_unprepared("ALTER TABLE tasks ADD COLUMN constraint_date date")
            .await?;
        db.execute_unprepared("ALTER TABLE tasks ADD COLUMN deadline date")
            .await?;
        db.execute_unprepared(
            "ALTER TABLE tasks ADD CONSTRAINT tasks_constraint_kind CHECK (constraint_kind IS NULL OR constraint_kind IN ('snet','snlt','fnet','fnlt','mso','mfo'))",
        )
        .await?;
        db.execute_unprepared(
            "ALTER TABLE tasks ADD CONSTRAINT tasks_constraint_pair CHECK ((constraint_kind IS NULL) = (constraint_date IS NULL))",
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
            .execute_unprepared("ALTER TABLE tasks DROP CONSTRAINT tasks_constraint_pair")
            .await?;
        m.get_connection()
            .execute_unprepared("ALTER TABLE tasks DROP CONSTRAINT tasks_constraint_kind")
            .await?;
        m.get_connection()
            .execute_unprepared("ALTER TABLE tasks DROP COLUMN deadline")
            .await?;
        m.get_connection()
            .execute_unprepared("ALTER TABLE tasks DROP COLUMN constraint_date")
            .await?;
        m.get_connection()
            .execute_unprepared("ALTER TABLE tasks DROP COLUMN constraint_kind")
            .await?;
        drop_table(m, "work_calendars").await
    }
}
