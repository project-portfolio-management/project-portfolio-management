//! Migration: **`notification_prefs`** — per-user control over which
//! notification kinds reach which channel, and whether a kind arrives
//! instantly or as a daily digest (PL-5). Keyed by the token `sub`
//! (a user pid) and nothing else identity-shaped.
//!
//! **No row means today's behaviour** (everything on, instant, in-app), so
//! introducing the table changes nothing for anyone who never opens it. A
//! row exists only where a user chose something; deleting it restores the
//! default.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The `notification_prefs` migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Create the table and its uniqueness + CHECK constraints.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "notification_prefs",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("sub", ColType::String),
                ("kind", ColType::String),
                ("channel", ColType::String),
                ("enabled", ColType::Boolean),
                ("digest", ColType::String),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        db.execute_unprepared(
            "CREATE UNIQUE INDEX notification_prefs_one_per_choice \
                 ON notification_prefs (sub, kind, channel);",
        )
        .await?;
        db.execute_unprepared(
            "ALTER TABLE notification_prefs ADD CONSTRAINT notification_prefs_digest \
                 CHECK (digest IN ('instant', 'daily'));",
        )
        .await?;
        Ok(())
    }

    /// Drop the table.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "notification_prefs").await
    }
}
