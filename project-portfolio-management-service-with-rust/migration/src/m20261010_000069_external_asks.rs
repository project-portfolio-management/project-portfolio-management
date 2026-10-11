//! Migration: **external asks need a named trade-off** (EX-13). A proposal may be tagged as an external ask
//! (from the sponsor); approving it then needs a statement of what it displaces, which the service checks.

use sea_orm_migration::prelude::*;

/// The migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Add the columns.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();
        db.execute_unprepared(
            "ALTER TABLE proposals ADD COLUMN external_ask boolean NOT NULL DEFAULT false",
        )
        .await?;
        db.execute_unprepared("ALTER TABLE proposals ADD COLUMN tradeoff text")
            .await?;
        db.execute_unprepared("ALTER TABLE proposals ADD CONSTRAINT proposals_tradeoff_len CHECK (tradeoff IS NULL OR length(tradeoff) <= 2000)").await?;
        Ok(())
    }

    /// Drop the columns.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();
        db.execute_unprepared("ALTER TABLE proposals DROP CONSTRAINT proposals_tradeoff_len")
            .await?;
        db.execute_unprepared("ALTER TABLE proposals DROP COLUMN tradeoff")
            .await?;
        db.execute_unprepared("ALTER TABLE proposals DROP COLUMN external_ask")
            .await?;
        Ok(())
    }
}
