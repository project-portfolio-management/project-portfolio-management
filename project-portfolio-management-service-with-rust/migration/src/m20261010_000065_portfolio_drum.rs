//! Migration: **the portfolio drum** (CC-5). The skill pool the portfolio office has named as the drum;
//! append-only, the latest row wins, and a row with no pool means the naming was cleared (the drum is then
//! the most overloaded pool). No person is stored (the audit trail names who).

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Create the table and trigger.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "portfolio_drums",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("skill_pool_pid", ColType::UuidNull),
                ("declared_on", ColType::Date),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        for q in [
            "CREATE OR REPLACE FUNCTION drum_append_only() RETURNS trigger LANGUAGE plpgsql AS $$
             BEGIN
               RAISE EXCEPTION 'the drum history is append-only';
             END $$",
            "CREATE TRIGGER drum_append_only BEFORE UPDATE OR DELETE ON portfolio_drums FOR EACH ROW EXECUTE FUNCTION drum_append_only()",
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
        db.execute_unprepared("DROP TRIGGER IF EXISTS drum_append_only ON portfolio_drums")
            .await?;
        db.execute_unprepared("DROP FUNCTION IF EXISTS drum_append_only()")
            .await?;
        drop_table(m, "portfolio_drums").await
    }
}
