//! Migration: **planned gate dates and programme status decisions** (DP-5).
//!
//! * `planned_gates`: the date a plan expects to be reviewed at a gate; one per plan and gate.
//! * `programme_status_decisions`: append-only. A programme is active until a recorded decision pauses
//!   or stops it. `stopped` is terminal, a repeat of the current status is refused, and a decision
//!   names who decided it (the one exception to the append-only rule: a data-subject pseudonymisation
//!   may blank `decided_by`).

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Create the tables, checks and triggers.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "planned_gates",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("plan_pid", ColType::Uuid),
                ("gate", ColType::String),
                ("planned_on", ColType::Date),
                ("set_by", ColType::StringNull),
            ],
            &[],
        )
        .await?;
        create_table(
            m,
            "programme_status_decisions",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("plan_pid", ColType::Uuid),
                ("status", ColType::String),
                ("reason", ColType::Text),
                ("decided_by", ColType::StringNull),
                ("decided_on", ColType::Date),
                ("gate_review_pid", ColType::UuidNull),
                ("exception_pid", ColType::UuidNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        for q in [
            "CREATE UNIQUE INDEX planned_gates_once ON planned_gates (plan_pid, gate)",
            "ALTER TABLE programme_status_decisions ADD CONSTRAINT psd_status CHECK (status IN ('active','paused','stopped'))",
            "ALTER TABLE programme_status_decisions ADD CONSTRAINT psd_reason CHECK (char_length(btrim(reason)) >= 10)",
            "CREATE INDEX psd_by_plan ON programme_status_decisions (plan_pid, id)",
            "CREATE OR REPLACE FUNCTION psd_guard() RETURNS trigger LANGUAGE plpgsql AS $$
             DECLARE last_status text;
             BEGIN
               IF TG_OP = 'INSERT' THEN
                 IF NEW.decided_by IS NULL OR btrim(NEW.decided_by) = '' THEN
                   RAISE EXCEPTION 'a programme status decision names who decided it';
                 END IF;
                 SELECT status INTO last_status FROM programme_status_decisions
                   WHERE plan_pid = NEW.plan_pid ORDER BY id DESC LIMIT 1;
                 IF last_status = 'stopped' THEN
                   RAISE EXCEPTION 'a stopped programme stays stopped';
                 END IF;
                 IF coalesce(last_status, 'active') = NEW.status THEN
                   RAISE EXCEPTION 'the programme already has that status';
                 END IF;
                 RETURN NEW;
               END IF;
               IF TG_OP = 'UPDATE' AND current_setting('ppm.audit_pseudonymise', true) = 'on'
                  AND NEW.plan_pid = OLD.plan_pid AND NEW.status = OLD.status
                  AND NEW.reason = OLD.reason AND NEW.decided_on = OLD.decided_on
                  AND NEW.pid = OLD.pid THEN
                 RETURN NEW;
               END IF;
               RAISE EXCEPTION 'programme status history is append-only';
             END $$",
            "CREATE TRIGGER psd_guard BEFORE INSERT OR UPDATE OR DELETE ON programme_status_decisions FOR EACH ROW EXECUTE FUNCTION psd_guard()",
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
        db.execute_unprepared("DROP TRIGGER IF EXISTS psd_guard ON programme_status_decisions")
            .await?;
        db.execute_unprepared("DROP FUNCTION IF EXISTS psd_guard()")
            .await?;
        drop_table(m, "programme_status_decisions").await?;
        drop_table(m, "planned_gates").await
    }
}
