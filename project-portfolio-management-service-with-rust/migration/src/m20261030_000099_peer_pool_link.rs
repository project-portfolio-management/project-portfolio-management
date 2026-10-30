//! Migration: **the link from a skill pool to the workforce service's pool, and the record of demand sent there**
//! (FB-1, FB-3). `skill_pools.peer_pool_ref` is WPM's id for the pool that means the same skills: nullable, set by a
//! person, never matched by name, one PPM pool per WPM pool. `peer_demand_sends` keeps the latest outcome of sending
//! a programme's demand on a linked pool for a month (one row per pool, programme and month); it holds no token and
//! names no person.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Add the column, its checks and the table.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "peer_demand_sends",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("pool_pid", ColType::Uuid),
                ("plan_pid", ColType::Uuid),
                ("month", ColType::Date),
                ("peer_pool_ref", ColType::String),
                ("fte_centi", ColType::BigIntegerNull),
                ("peer_event_id", ColType::String),
                ("version_at", ColType::TimestampWithTimeZone),
                ("outcome", ColType::String),
                ("detail", ColType::TextNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        for q in [
            "ALTER TABLE skill_pools ADD COLUMN IF NOT EXISTS peer_pool_ref TEXT",
            "ALTER TABLE skill_pools ADD CONSTRAINT skill_pools_peer_pool_ref_shape CHECK (peer_pool_ref IS NULL OR peer_pool_ref ~ '^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$')",
            "CREATE UNIQUE INDEX IF NOT EXISTS skill_pools_peer_pool_ref ON skill_pools (peer_pool_ref) WHERE peer_pool_ref IS NOT NULL",
            "CREATE UNIQUE INDEX IF NOT EXISTS peer_demand_sends_once ON peer_demand_sends (pool_pid, plan_pid, month)",
            "ALTER TABLE peer_demand_sends ADD CONSTRAINT peer_demand_sends_outcome CHECK (outcome IN ('started','sent','duplicate','refused','unreachable','not_configured','not_sent_no_token','not_sent_zero','not_sent_too_large'))",
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
        let db = m.get_connection();
        db.execute_unprepared("DROP INDEX IF EXISTS skill_pools_peer_pool_ref")
            .await?;
        db.execute_unprepared(
            "ALTER TABLE skill_pools DROP CONSTRAINT IF EXISTS skill_pools_peer_pool_ref_shape, DROP COLUMN IF EXISTS peer_pool_ref",
        )
        .await?;
        drop_table(m, "peer_demand_sends").await
    }
}
