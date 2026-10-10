//! Migration: stored values move to Oxford spelling (`-ize`). A risk that
//! happened was stored as `materialised`; it is now `materialized`, the value
//! the API reads and writes. Audit snapshots are history and are not rewritten.

use sea_orm_migration::prelude::*;

/// The Oxford-spelling migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Rename the stored risk status.
    ///
    /// # Errors
    ///
    /// Propagates any DML error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared(
                "UPDATE risks SET status = 'materialized' WHERE status = 'materialised'",
            )
            .await?;
        Ok(())
    }

    /// Restore the earlier spelling.
    ///
    /// # Errors
    ///
    /// Propagates any DML error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared(
                "UPDATE risks SET status = 'materialised' WHERE status = 'materialized'",
            )
            .await?;
        Ok(())
    }
}
