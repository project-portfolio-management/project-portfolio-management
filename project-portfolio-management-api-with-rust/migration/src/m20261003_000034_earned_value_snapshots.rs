//! Migration: **`earned_value_snapshots`**. A day's earned-value figures for one plan in one currency (SC-5): what was planned, earned and spent as of that date. Written once a day by the scheduler (or on request) so the S-curve has a real history to draw: earned value and actual cost are not dated at source, so the past cannot be reconstructed, only recorded going forward. One row per plan, currency and day; a second snapshot the same day replaces the first.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The `earned_value_snapshots` migration (name derived from the module).
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
            "earned_value_snapshots",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("plan_pid", ColType::Uuid),
                ("currency", ColType::String),
                ("as_of", ColType::Date),
                ("baseline_version", ColType::Integer),
                ("bac_minor", ColType::BigInteger),
                ("pv_minor", ColType::BigInteger),
                ("ev_minor", ColType::BigInteger),
                ("ac_minor", ColType::BigIntegerNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        db.execute_unprepared(
            "CREATE UNIQUE INDEX earned_value_snapshots_day ON earned_value_snapshots (plan_pid, currency, as_of)",
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
        drop_table(m, "earned_value_snapshots").await
    }
}
