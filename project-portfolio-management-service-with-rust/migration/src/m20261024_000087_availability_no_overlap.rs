//! Migration: one person's availability ranges cannot overlap (CF-3). The service already checks this
//! under a per-person advisory lock; this is the database's own guarantee, so a second writer, a
//! script or a lost lock cannot store two ranges that cover the same day. Inclusive on both ends, as
//! the service reads them, and only for ranges not removed.
//!
//! Needs the `btree_gist` extension (equality on text inside a `GiST` index). It ships with PostgreSQL's
//! contrib and is a *trusted* extension from version 13, so a role that may create objects in the
//! database can add it. If the table already holds overlapping ranges this fails, naming the
//! constraint; remove or change one of each pair and run it again.

use sea_orm_migration::prelude::*;

/// The migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Add the extension and the exclusion constraint.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error, including existing overlapping ranges.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();
        db.execute_unprepared("CREATE EXTENSION IF NOT EXISTS btree_gist")
            .await?;
        db.execute_unprepared(
            "ALTER TABLE person_availabilities ADD CONSTRAINT person_availabilities_no_overlap
             EXCLUDE USING gist (person_ref WITH =, daterange(from_date, to_date, '[]') WITH &&)
             WHERE (deleted_at IS NULL)",
        )
        .await?;
        Ok(())
    }

    /// Drop the constraint (the extension stays: something else may use it).
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared(
                "ALTER TABLE person_availabilities DROP CONSTRAINT IF EXISTS person_availabilities_no_overlap",
            )
            .await?;
        Ok(())
    }
}
