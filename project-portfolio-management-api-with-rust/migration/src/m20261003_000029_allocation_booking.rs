//! Migration: **allocation booking** (PJ-4). An allocation is `committed`
//! (firm: the default, so every existing row means what it meant) or `proposed`
//! (a hoped-for booking that does not count toward committed utilisation), and
//! may be flagged `requested` (asked of the resource manager).

use sea_orm_migration::prelude::*;

/// The migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Add the columns and the check.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();
        for q in [
            "ALTER TABLE allocations ADD COLUMN booking text NOT NULL DEFAULT 'committed'",
            "ALTER TABLE allocations ADD COLUMN requested boolean NOT NULL DEFAULT false",
            "ALTER TABLE allocations ADD CONSTRAINT allocations_booking CHECK (booking IN ('proposed','committed'))",
        ] {
            db.execute_unprepared(q).await?;
        }
        Ok(())
    }

    /// Drop the columns.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();
        for q in [
            "ALTER TABLE allocations DROP CONSTRAINT allocations_booking",
            "ALTER TABLE allocations DROP COLUMN requested",
            "ALTER TABLE allocations DROP COLUMN booking",
        ] {
            db.execute_unprepared(q).await?;
        }
        Ok(())
    }
}
