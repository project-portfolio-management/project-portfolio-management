//! Migration: **skill pools and capacity commitments** (DP-10). A pool groups roles into something a
//! director can read; a commitment is capacity a partner (or the organization itself) has promised to
//! a programme for a month, or work owed. Until partner organizations exist (EX-3) the organization is
//! a free-text label; a commitment with none is the organization's own.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Create the tables, checks and indexes.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "skill_pools",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("name", ColType::String),
                ("active", ColType::Boolean),
            ],
            &[],
        )
        .await?;
        create_table(
            m,
            "skill_pool_members",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("pool_pid", ColType::Uuid),
                ("kind", ColType::String),
                ("value", ColType::String),
            ],
            &[],
        )
        .await?;
        create_table(
            m,
            "capacity_commitments",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("pool_pid", ColType::Uuid),
                ("plan_pid", ColType::Uuid),
                ("organization", ColType::StringNull),
                ("kind", ColType::String),
                ("month", ColType::Date),
                ("committed_milli_fte", ColType::Integer),
                ("delivered_milli_fte", ColType::IntegerNull),
                ("note", ColType::TextNull),
                ("updated_by", ColType::StringNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        for q in [
            "CREATE UNIQUE INDEX skill_pools_name ON skill_pools (lower(name))",
            "ALTER TABLE skill_pool_members ADD CONSTRAINT spm_kind CHECK (kind IN ('role','skill'))",
            // A role or skill sits in at most one pool, so demand is never counted twice.
            "CREATE UNIQUE INDEX spm_once ON skill_pool_members (kind, lower(value))",
            "ALTER TABLE capacity_commitments ADD CONSTRAINT cc_kind CHECK (kind IN ('supply','demand'))",
            "ALTER TABLE capacity_commitments ADD CONSTRAINT cc_month CHECK (month = date_trunc('month', month)::date)",
            "ALTER TABLE capacity_commitments ADD CONSTRAINT cc_amounts CHECK (committed_milli_fte BETWEEN 0 AND 1000000 AND (delivered_milli_fte IS NULL OR delivered_milli_fte BETWEEN 0 AND 1000000))",
            "CREATE UNIQUE INDEX cc_once ON capacity_commitments (pool_pid, plan_pid, coalesce(lower(organization), ''), kind, month)",
            "CREATE INDEX cc_by_month ON capacity_commitments (month)",
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
        drop_table(m, "capacity_commitments").await?;
        drop_table(m, "skill_pool_members").await?;
        drop_table(m, "skill_pools").await
    }
}
