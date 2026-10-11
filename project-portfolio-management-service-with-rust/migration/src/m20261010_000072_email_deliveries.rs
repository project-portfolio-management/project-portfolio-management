//! Migration: **per-recipient email deliveries**. One row per message per recipient: what was sent to whom,
//! through which sender, how many times it was tried, and how it ended. A message to three people is three
//! rows, so one bad address never hides a good one. The unique key (kind, subject, recipient) makes a
//! repeated dispatch of the same message to the same person a no-op rather than a second email.

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
            "email_deliveries",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("kind", ColType::String),
                ("subject_ref", ColType::Uuid),
                ("recipient", ColType::String),
                ("subject", ColType::String),
                ("body_text", ColType::Text),
                ("status", ColType::String),
                ("attempts", ColType::Integer),
                ("last_error", ColType::TextNull),
                ("sender", ColType::StringNull),
                ("next_attempt_at", ColType::TimestampWithTimeZoneNull),
                ("sent_at", ColType::TimestampWithTimeZoneNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        for q in [
            "ALTER TABLE email_deliveries ADD CONSTRAINT ed_status CHECK (status IN ('queued','retry','sent','failed'))",
            "ALTER TABLE email_deliveries ADD CONSTRAINT ed_sent CHECK ((status = 'sent') = (sent_at IS NOT NULL))",
            "CREATE UNIQUE INDEX ed_once ON email_deliveries (kind, subject_ref, lower(recipient))",
            "CREATE INDEX ed_due ON email_deliveries (status, next_attempt_at)",
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
        drop_table(m, "email_deliveries").await
    }
}
