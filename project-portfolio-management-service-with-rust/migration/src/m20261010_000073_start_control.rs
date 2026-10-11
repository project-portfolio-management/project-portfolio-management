//! Migration: **start control** (EX-8). The queue of programmes waiting to start (in the order they asked) and
//! the overrides by which a refused start went ahead anyway. Overrides are append-only; a pseudonymisation
//! may blank who decided.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Create the tables, checks and trigger.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "start_queue_entries",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("plan_pid", ColType::Uuid),
                ("status", ColType::String),
                ("refusals", ColType::Text),
                ("queued_on", ColType::Date),
                ("resolved_on", ColType::DateNull),
            ],
            &[],
        )
        .await?;
        create_table(
            m,
            "start_overrides",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("plan_pid", ColType::Uuid),
                ("reason", ColType::Text),
                ("refusals", ColType::Text),
                ("decided_by", ColType::StringNull),
                ("decided_on", ColType::Date),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        for q in [
            "ALTER TABLE start_queue_entries ADD CONSTRAINT sq_status CHECK (status IN ('waiting','started','withdrawn'))",
            "ALTER TABLE start_queue_entries ADD CONSTRAINT sq_resolved CHECK ((status = 'waiting') = (resolved_on IS NULL))",
            "CREATE UNIQUE INDEX sq_one_waiting ON start_queue_entries (plan_pid) WHERE status = 'waiting'",
            "ALTER TABLE start_overrides ADD CONSTRAINT so_reason CHECK (length(btrim(reason)) >= 10)",
            "CREATE OR REPLACE FUNCTION so_append_only() RETURNS trigger LANGUAGE plpgsql AS $$
             BEGIN
               IF TG_OP = 'UPDATE' AND current_setting('ppm.audit_pseudonymise', true) = 'on'
                  AND NEW.pid = OLD.pid AND NEW.plan_pid = OLD.plan_pid AND NEW.reason = OLD.reason THEN
                 RETURN NEW;
               END IF;
               RAISE EXCEPTION 'start overrides are append-only';
             END $$",
            "CREATE TRIGGER so_append_only BEFORE UPDATE OR DELETE ON start_overrides FOR EACH ROW EXECUTE FUNCTION so_append_only()",
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
        db.execute_unprepared("DROP TRIGGER IF EXISTS so_append_only ON start_overrides").await?;
        db.execute_unprepared("DROP FUNCTION IF EXISTS so_append_only()").await?;
        drop_table(m, "start_overrides").await?;
        drop_table(m, "start_queue_entries").await
    }
}
