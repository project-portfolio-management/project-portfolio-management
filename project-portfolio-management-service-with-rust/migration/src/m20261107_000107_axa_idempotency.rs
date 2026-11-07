//! Migration: idempotency keys move to **`axa_idempotency`** (BR-1, NFR-7), the table `axa::idempotency` keeps them
//! in, made by its own [`SETUP_STATEMENTS`](axa::idempotency::SETUP_STATEMENTS). The old `idempotency_keys` table is
//! dropped: its rows live a day, and a retry across the deploy runs as a first request.

use sea_orm_migration::prelude::*;

/// The migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Create `axa_idempotency` and drop `idempotency_keys`.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();
        for statement in axa::idempotency::SETUP_STATEMENTS {
            db.execute_unprepared(statement).await?;
        }
        db.execute_unprepared("DROP TABLE IF EXISTS idempotency_keys")
            .await?;
        Ok(())
    }

    /// Drop `axa_idempotency` and make the old table again, empty.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS axa_idempotency")
            .await?;
        super::m20261025_000095_idempotency_keys::Migration
            .up(m)
            .await
    }
}
