//! Migration: **`feed_tokens`**. Revocable secrets that let a calendar app or feed reader read one person's own feeds without a bearer token (LT-7 / RM-6). Only the SHA-256 of the token is stored, so a database read never yields a usable token; revoking sets `revoked_at` and takes effect on the next fetch.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The `feed_tokens` migration (name derived from the module).
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
            "feed_tokens",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("sub", ColType::String),
                ("token_hash", ColType::String),
                ("label", ColType::StringNull),
                ("revoked_at", ColType::TimestampWithTimeZoneNull),
                ("last_used_at", ColType::TimestampWithTimeZoneNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        db.execute_unprepared("CREATE UNIQUE INDEX feed_tokens_hash ON feed_tokens (token_hash);")
            .await?;
        db.execute_unprepared("CREATE INDEX feed_tokens_sub ON feed_tokens (sub);")
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
        drop_table(m, "feed_tokens").await
    }
}
