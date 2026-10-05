//! Migration: **milestones tied to the schedule** (milestone concept). A milestone may be `linked` to tasks (`milestone_tasks`): it is then achieved when they are all done, and its forecast comes from the critical-path analysis. Every reschedule is recorded (`milestone_slips`, append-only). `done_reason` records why a milestone was completed by hand while linked tasks were still open.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The milestone-links migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Add the columns and create both tables.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();
        for q in [
            "ALTER TABLE milestones ADD COLUMN completion_mode text NOT NULL DEFAULT 'manual'",
            "ALTER TABLE milestones ADD CONSTRAINT milestones_completion_mode CHECK (completion_mode IN ('manual','linked'))",
            "ALTER TABLE milestones ADD COLUMN done_reason text",
        ] {
            db.execute_unprepared(q).await?;
        }
        create_table(
            m,
            "milestone_tasks",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("milestone_pid", ColType::Uuid),
                ("task_pid", ColType::Uuid),
            ],
            &[],
        )
        .await?;
        create_table(
            m,
            "milestone_slips",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("milestone_pid", ColType::Uuid),
                ("from_due", ColType::Date),
                ("to_due", ColType::Date),
                ("reason", ColType::TextNull),
                ("actor", ColType::StringNull),
            ],
            &[],
        )
        .await?;
        for q in [
            "CREATE UNIQUE INDEX milestone_tasks_pair ON milestone_tasks (milestone_pid, task_pid)",
            "CREATE INDEX milestone_tasks_task ON milestone_tasks (task_pid)",
            "CREATE INDEX milestone_slips_milestone ON milestone_slips (milestone_pid, id)",
        ] {
            db.execute_unprepared(q).await?;
        }
        Ok(())
    }

    /// Drop the tables and columns.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "milestone_slips").await?;
        drop_table(m, "milestone_tasks").await?;
        let db = m.get_connection();
        for q in [
            "ALTER TABLE milestones DROP COLUMN done_reason",
            "ALTER TABLE milestones DROP CONSTRAINT milestones_completion_mode",
            "ALTER TABLE milestones DROP COLUMN completion_mode",
        ] {
            db.execute_unprepared(q).await?;
        }
        Ok(())
    }
}
