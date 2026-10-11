//! Migration: **project buffers** for the buffer fever chart (DP-7). Append-only: a programme's buffer
//! is the latest row, and an earlier declaration stays on record. The one exception: a data-subject
//! pseudonymisation may blank `declared_by`.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Create the table, checks and trigger.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "programme_buffers",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("plan_pid", ColType::Uuid),
                ("buffer_days", ColType::Integer),
                ("source", ColType::String),
                ("rule", ColType::StringNull),
                ("note", ColType::TextNull),
                ("declared_by", ColType::StringNull),
                ("declared_on", ColType::Date),
                ("baseline_version", ColType::IntegerNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        for q in [
            "ALTER TABLE programme_buffers ADD CONSTRAINT pb_days CHECK (buffer_days BETWEEN 1 AND 2000)",
            "ALTER TABLE programme_buffers ADD CONSTRAINT pb_source CHECK (source IN ('declared','rule'))",
            "ALTER TABLE programme_buffers ADD CONSTRAINT pb_rule CHECK ((source = 'rule') = (rule IS NOT NULL))",
            "CREATE INDEX pb_by_plan ON programme_buffers (plan_pid, id)",
            "CREATE OR REPLACE FUNCTION pb_append_only() RETURNS trigger LANGUAGE plpgsql AS $$
             BEGIN
               IF TG_OP = 'UPDATE' AND current_setting('ppm.audit_pseudonymise', true) = 'on'
                  AND NEW.plan_pid = OLD.plan_pid AND NEW.buffer_days = OLD.buffer_days
                  AND NEW.source = OLD.source AND NEW.declared_on = OLD.declared_on
                  AND NEW.pid = OLD.pid THEN
                 RETURN NEW;
               END IF;
               RAISE EXCEPTION 'project buffer history is append-only';
             END $$",
            "CREATE TRIGGER pb_append_only BEFORE UPDATE OR DELETE ON programme_buffers FOR EACH ROW EXECUTE FUNCTION pb_append_only()",
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
        db.execute_unprepared("DROP TRIGGER IF EXISTS pb_append_only ON programme_buffers")
            .await?;
        db.execute_unprepared("DROP FUNCTION IF EXISTS pb_append_only()")
            .await?;
        drop_table(m, "programme_buffers").await
    }
}
