//! Migration: the **issues register** (IR-1). An issue is something that has already happened and needs managing (a risk is something that might). It belongs to a plan, has a kind, a severity, a status and an optional owner, and may point back at the risk or the blocked task it came from. Closing one records a resolution. Deleting is a soft delete.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The issues migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Create the table, its checks and its indexes.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "issues",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("plan_pid", ColType::Uuid),
                ("title", ColType::String),
                ("description", ColType::TextNull),
                ("kind", ColType::String),
                ("severity", ColType::String),
                ("priority", ColType::StringNull),
                ("status", ColType::String),
                ("owner_ref", ColType::StringNull),
                ("raised_by", ColType::StringNull),
                ("raised_on", ColType::Date),
                ("due_date", ColType::DateNull),
                ("resolution", ColType::TextNull),
                ("resolved_on", ColType::DateNull),
                ("source_risk_pid", ColType::UuidNull),
                ("source_task_pid", ColType::UuidNull),
                ("change_request_pid", ColType::UuidNull),
                ("deleted_at", ColType::TimestampWithTimeZoneNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        for q in [
            "ALTER TABLE issues ADD CONSTRAINT issues_kind CHECK (kind IN ('bug','risk','blocker','question','improvement','concern'))",
            "ALTER TABLE issues ADD CONSTRAINT issues_severity CHECK (severity IN ('low','med','high','critical'))",
            "ALTER TABLE issues ADD CONSTRAINT issues_status CHECK (status IN ('open','in_progress','resolved','closed'))",
            "ALTER TABLE issues ADD CONSTRAINT issues_closed_resolution CHECK (status <> 'closed' OR (resolution IS NOT NULL AND btrim(resolution) <> ''))",
            "ALTER TABLE issues ADD CONSTRAINT issues_due_after_raised CHECK (due_date IS NULL OR due_date >= raised_on)",
            "CREATE INDEX issues_plan ON issues (plan_pid, status) WHERE deleted_at IS NULL",
            "CREATE INDEX issues_owner ON issues (owner_ref) WHERE deleted_at IS NULL AND owner_ref IS NOT NULL",
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
        drop_table(m, "issues").await
    }
}
