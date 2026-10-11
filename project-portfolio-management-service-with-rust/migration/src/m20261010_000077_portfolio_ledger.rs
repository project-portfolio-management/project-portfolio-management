//! Migration: **the portfolio ledger** (EX-12). A funding allocation (a source, an amount, a date) that
//! budget lines can be attached to, so that "what did this allocation buy?" has an answer in one query; and a
//! map from the finance system's cost codes to budget lines (the register's identifier), so that a ledger
//! export can be reconciled. Allocations are append-only.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Create the tables, alter the budget lines, add the trigger.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "funding_allocations",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("funding_source_pid", ColType::Uuid),
                ("amount_minor", ColType::BigInteger),
                ("currency", ColType::String),
                ("allocated_on", ColType::Date),
                ("reference", ColType::StringNull),
                ("created_by", ColType::StringNull),
            ],
            &[],
        )
        .await?;
        create_table(
            m,
            "cost_code_maps",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("cost_code", ColType::String),
                ("budget_line_pid", ColType::Uuid),
                ("created_by", ColType::StringNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        for q in [
            "ALTER TABLE funding_allocations ADD CONSTRAINT fa_amount CHECK (amount_minor > 0)",
            "ALTER TABLE funding_allocations ADD CONSTRAINT fa_currency CHECK (currency ~ '^[A-Z]{3}$')",
            "CREATE INDEX fa_by_source ON funding_allocations (funding_source_pid, allocated_on)",
            "ALTER TABLE cost_code_maps ADD CONSTRAINT ccm_code CHECK (length(btrim(cost_code)) BETWEEN 1 AND 60)",
            "CREATE UNIQUE INDEX ccm_once ON cost_code_maps (lower(btrim(cost_code)))",
            "ALTER TABLE budget_lines ADD COLUMN funding_allocation_pid uuid",
            "CREATE INDEX bl_by_allocation ON budget_lines (funding_allocation_pid)",
            "CREATE OR REPLACE FUNCTION fa_append_only() RETURNS trigger LANGUAGE plpgsql AS $$
             BEGIN
               IF TG_OP = 'UPDATE' AND coalesce(current_setting('ppm.audit_pseudonymise', true), '') = 'on' THEN RETURN NEW; END IF;
               RAISE EXCEPTION 'funding allocations are append-only';
             END $$",
            "CREATE TRIGGER fa_append_only BEFORE UPDATE OR DELETE ON funding_allocations FOR EACH ROW EXECUTE FUNCTION fa_append_only()",
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
        db.execute_unprepared("ALTER TABLE budget_lines DROP COLUMN funding_allocation_pid").await?;
        db.execute_unprepared("DROP TRIGGER IF EXISTS fa_append_only ON funding_allocations").await?;
        db.execute_unprepared("DROP FUNCTION IF EXISTS fa_append_only()").await?;
        drop_table(m, "cost_code_maps").await?;
        drop_table(m, "funding_allocations").await
    }
}
