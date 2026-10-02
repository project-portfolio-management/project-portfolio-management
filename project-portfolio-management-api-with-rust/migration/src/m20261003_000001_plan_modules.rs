//! Migration: **`plan_modules`** — which feature modules a plan exposes
//! (RM-7). A row exists only where someone chose a set for that plan;
//! **no row means the deployment default**, so introducing the table
//! changes nothing for any existing plan.
//!
//! Its own table rather than a column on `plans`: the toggle is operational
//! presentation, not matcher payload, and keeping it off the plan row leaves
//! the record-integrity pre-image (`content_hash`, `content_mac`) untouched.
//! Disabling a module hides its routes for that plan; it never deletes data.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The `plan_modules` migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Create the table with one row per plan.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "plan_modules",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("plan_pid", ColType::UuidUniq),
                ("modules", ColType::JsonBinary),
            ],
            &[],
        )
        .await
    }

    /// Drop the table.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "plan_modules").await
    }
}
