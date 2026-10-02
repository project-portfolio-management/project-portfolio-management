//! Migration: **`approval_requests`**. Ask a named person to approve something, and record their decision (PL-1). Generic over the same subject kinds as comments, plus budget lines and scenarios. A decision is final and carries a comment (required on reject).

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The `approval_requests` migration (name derived from the module).
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
            "approval_requests",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("subject_kind", ColType::String),
                ("subject_pid", ColType::Uuid),
                ("requested_by", ColType::String),
                ("approver_ref", ColType::String),
                ("message", ColType::StringNull),
                ("status", ColType::String),
                ("decision_comment", ColType::StringNull),
                ("decided_by", ColType::StringNull),
                ("decided_at", ColType::TimestampWithTimeZoneNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        db.execute_unprepared(
            "ALTER TABLE approval_requests ADD CONSTRAINT approval_requests_status CHECK (status IN ('pending','approved','rejected','withdrawn'));",
        )
        .await?;
        db.execute_unprepared(
            "CREATE INDEX approval_requests_approver ON approval_requests (approver_ref, status);",
        )
        .await?;
        db.execute_unprepared(
            "CREATE INDEX approval_requests_subject ON approval_requests (subject_kind, subject_pid);",
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
        drop_table(m, "approval_requests").await
    }
}
