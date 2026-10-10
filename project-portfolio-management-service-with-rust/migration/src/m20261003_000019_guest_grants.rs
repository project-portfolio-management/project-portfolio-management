//! Migration: **`guest_grants`**. A scoped, time-boxed, revocable grant letting an external collaborator (a token whose role attribute includes `guest`) see — and optionally comment on — one plan (PL-7). A guest sees only what is granted and gets 404 everywhere else; never people references. `revoked_at` and `expires_at` take effect on the next request.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The `guest_grants` migration (name derived from the module).
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
            "guest_grants",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("guest_sub", ColType::String),
                ("subject_kind", ColType::String),
                ("subject_pid", ColType::Uuid),
                ("access", ColType::String),
                ("expires_at", ColType::TimestampWithTimeZoneNull),
                ("granted_by", ColType::StringNull),
                ("revoked_at", ColType::TimestampWithTimeZoneNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        db.execute_unprepared(
            "ALTER TABLE guest_grants ADD CONSTRAINT guest_grants_access CHECK (access IN ('read','comment'));",
        )
        .await?;
        db.execute_unprepared(
            "CREATE INDEX guest_grants_guest ON guest_grants (guest_sub, subject_kind, subject_pid) WHERE revoked_at IS NULL;",
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
        drop_table(m, "guest_grants").await
    }
}
