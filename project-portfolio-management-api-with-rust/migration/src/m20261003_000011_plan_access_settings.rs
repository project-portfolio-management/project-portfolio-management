//! Migration: **`plan_access_settings`** — who may see a private plan (RM-8).
//! A row exists only where someone made a plan private; no row means the plan
//! is as visible as it always was. Members are token `sub`s (user pids). Its
//! own table so the plan row and its integrity hash stay untouched.
//!
//! (Named in the plural on purpose: loco's `create_table` pluralises table
//! names, so a singular name creates a different table than the one the SQL
//! below refers to.)

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The `plan_access_settings` migration (name derived from the module).
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
            "plan_access_settings",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("plan_pid", ColType::Uuid),
                ("private", ColType::Boolean),
                ("members", ColType::JsonBinary),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        db.execute_unprepared(
            "CREATE UNIQUE INDEX plan_access_settings_plan ON plan_access_settings (plan_pid);",
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
        drop_table(m, "plan_access_settings").await
    }
}
