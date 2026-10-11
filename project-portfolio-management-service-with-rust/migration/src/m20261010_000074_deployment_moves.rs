//! Migration: **deployment-date change control** (EX-5). For a programme delivered as a central platform
//! with local deployment, a move of one deployment's go-live is a request with its knock-on computed at the
//! time, decided by a named approver. Until it is approved nothing about the schedule changes. A decision
//! needs a reason; the approver and the decider are recorded.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Create the table and checks.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "deployment_moves",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("plan_pid", ColType::Uuid),
                ("programme_pid", ColType::Uuid),
                ("task_pid", ColType::Uuid),
                ("from_start", ColType::Date),
                ("to_start", ColType::Date),
                ("reason", ColType::Text),
                ("requested_by", ColType::StringNull),
                ("organization_pid", ColType::UuidNull),
                ("approver_ref", ColType::String),
                ("status", ColType::String),
                ("decided_by", ColType::StringNull),
                ("decided_on", ColType::DateNull),
                ("decision_reason", ColType::TextNull),
                ("knock_on", ColType::Text),
                ("knock_on_cost", ColType::Text),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        for q in [
            "ALTER TABLE deployment_moves ADD CONSTRAINT dm_status CHECK (status IN ('requested','approved','refused'))",
            "ALTER TABLE deployment_moves ADD CONSTRAINT dm_decided CHECK ((status = 'requested') = (decided_on IS NULL) AND (status = 'requested' OR (decided_by IS NOT NULL AND length(btrim(coalesce(decision_reason, ''))) >= 10)))",
            "ALTER TABLE deployment_moves ADD CONSTRAINT dm_reason CHECK (length(btrim(reason)) >= 10)",
            "ALTER TABLE deployment_moves ADD CONSTRAINT dm_changes CHECK (from_start <> to_start)",
            "CREATE UNIQUE INDEX dm_one_open ON deployment_moves (task_pid) WHERE status = 'requested'",
            "CREATE INDEX dm_by_plan ON deployment_moves (plan_pid, id)",
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
        drop_table(m, "deployment_moves").await
    }
}
