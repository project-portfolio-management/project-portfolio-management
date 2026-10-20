//! Migration: **plan tolerances**. How far a project may drift on one dimension (time, cost, risk, resources) before
//! it must be referred upward, and how much of that limit may be used before the colour turns amber. A plan with no
//! row for a dimension inherits its nearest ancestor's, resolved on read and never stored.

use sea_orm_migration::prelude::*;

/// The migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Create the table.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared(
                "CREATE TABLE IF NOT EXISTS plan_tolerances (
                     created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     id BIGSERIAL PRIMARY KEY,
                     pid UUID NOT NULL UNIQUE,
                     plan_pid UUID NOT NULL,
                     dimension VARCHAR NOT NULL,
                     limit_value BIGINT NOT NULL,
                     amber_from_percent INTEGER NOT NULL DEFAULT 75,
                     note TEXT NULL,
                     set_by VARCHAR NULL,
                     CONSTRAINT plan_tolerances_one_per_dimension UNIQUE (plan_pid, dimension),
                     CONSTRAINT plan_tolerances_dimension CHECK (dimension IN ('time','cost','risk','resources')),
                     CONSTRAINT plan_tolerances_amber CHECK (amber_from_percent BETWEEN 1 AND 99),
                     CONSTRAINT plan_tolerances_limit CHECK (limit_value >= 0 AND limit_value <= 100000
                         AND (dimension <> 'risk' OR limit_value BETWEEN 1 AND 25))
                 );",
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
        m.get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS plan_tolerances;")
            .await?;
        Ok(())
    }
}
