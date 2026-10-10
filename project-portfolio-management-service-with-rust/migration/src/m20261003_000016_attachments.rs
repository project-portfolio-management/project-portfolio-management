//! Migration: **`attachments`**. A file attached to a plan, task, risk, proposal, idea, canvas item, budget line or scenario (LT-5). The bytes are an artifact; this row is the link, so removing an attachment (soft-delete) never loses the audit trail of what was attached.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The `attachments` migration (name derived from the module).
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
            "attachments",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("subject_kind", ColType::String),
                ("subject_pid", ColType::Uuid),
                ("artifact_pid", ColType::Uuid),
                ("uploaded_by", ColType::StringNull),
                ("deleted_at", ColType::TimestampWithTimeZoneNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        db.execute_unprepared(
            "CREATE INDEX attachments_subject ON attachments (subject_kind, subject_pid) WHERE deleted_at IS NULL;",
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
        drop_table(m, "attachments").await
    }
}
