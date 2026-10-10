//! Migration: **break-glass access** (RG-10). A named, eligible person may open a private plan for a limited time by stating a reason; a second person is told at once; every use is audited. A grant makes the grantee a temporary member for *visibility* only (what they may do still depends on their attributes) and ends at `expires_at` or when it is revoked. Rows are never deleted.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The migration (name derived from the module).
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
            "break_glass_grants",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("plan_pid", ColType::Uuid),
                ("sub", ColType::String),
                ("reason", ColType::Text),
                ("notify_ref", ColType::String),
                ("expires_at", ColType::TimestampWithTimeZone),
                ("revoked_at", ColType::TimestampWithTimeZoneNull),
                ("revoked_by", ColType::StringNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        for q in [
            "ALTER TABLE break_glass_grants ADD CONSTRAINT break_glass_reason CHECK (char_length(btrim(reason)) >= 20)",
            "ALTER TABLE break_glass_grants ADD CONSTRAINT break_glass_window CHECK (expires_at > created_at AND expires_at <= created_at + interval '4 hours')",
            "CREATE INDEX break_glass_live ON break_glass_grants (plan_pid, sub, expires_at)",
            "CREATE INDEX break_glass_by_sub ON break_glass_grants (sub, created_at)",
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
        drop_table(m, "break_glass_grants").await
    }
}
