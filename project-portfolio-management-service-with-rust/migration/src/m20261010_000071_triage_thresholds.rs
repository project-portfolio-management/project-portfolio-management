//! Migration: **triage and pause / reset / stop thresholds** (EX-7). A dated triage per programme (Finish,
//! Fix, Pause, Stop, with the reason); thresholds per programme; and the decision a person records when a
//! threshold is crossed (pause, reset, stop, or an override with its expiry). Triage and decisions are
//! append-only. A threshold exception is a delivery exception (DP-3) that also names the threshold, and its
//! opened and worst values may be `threshold`, since it is not a confidence reading.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Create the tables, alter the exceptions, add the triggers.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "programme_triages",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("plan_pid", ColType::Uuid),
                ("triaged_on", ColType::Date),
                ("outcome", ColType::String),
                ("reason", ColType::Text),
                ("decided_by", ColType::StringNull),
            ],
            &[],
        )
        .await?;
        create_table(
            m,
            "programme_thresholds",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("plan_pid", ColType::Uuid),
                ("kind", ColType::String),
                ("count_value", ColType::IntegerNull),
                ("date_value", ColType::DateNull),
                ("amount_minor", ColType::BigIntegerNull),
                ("currency", ColType::StringNull),
            ],
            &[],
        )
        .await?;
        create_table(
            m,
            "threshold_decisions",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("exception_pid", ColType::Uuid),
                ("plan_pid", ColType::Uuid),
                ("threshold_kind", ColType::String),
                ("action", ColType::String),
                ("expiry", ColType::DateNull),
                ("reason", ColType::Text),
                ("decided_by", ColType::StringNull),
                ("decided_on", ColType::Date),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        for q in [
            "ALTER TABLE programme_triages ADD CONSTRAINT pt_outcome CHECK (outcome IN ('finish','fix','pause','stop'))",
            "ALTER TABLE programme_triages ADD CONSTRAINT pt_reason CHECK (length(btrim(reason)) >= 10)",
            "CREATE INDEX pt_by_plan ON programme_triages (plan_pid, id)",
            "ALTER TABLE programme_thresholds ADD CONSTRAINT pth_kind CHECK (kind IN ('confidence_red_packs','forecast_past','p80_cap'))",
            "ALTER TABLE programme_thresholds ADD CONSTRAINT pth_value CHECK (
                (kind = 'confidence_red_packs' AND count_value BETWEEN 1 AND 24 AND date_value IS NULL AND amount_minor IS NULL)
             OR (kind = 'forecast_past' AND date_value IS NOT NULL AND count_value IS NULL AND amount_minor IS NULL)
             OR (kind = 'p80_cap' AND amount_minor > 0 AND currency IS NOT NULL AND count_value IS NULL AND date_value IS NULL))",
            "CREATE UNIQUE INDEX pth_once ON programme_thresholds (plan_pid, kind)",
            "ALTER TABLE threshold_decisions ADD CONSTRAINT td_action CHECK (action IN ('pause','reset','stop','override'))",
            "ALTER TABLE threshold_decisions ADD CONSTRAINT td_override CHECK ((action = 'override') = (expiry IS NOT NULL))",
            "ALTER TABLE threshold_decisions ADD CONSTRAINT td_reason CHECK (length(btrim(reason)) >= 10)",
            "CREATE INDEX td_by_plan ON threshold_decisions (plan_pid, decided_on)",
            "ALTER TABLE delivery_exceptions ADD COLUMN threshold_kind text",
            "ALTER TABLE delivery_exceptions DROP CONSTRAINT dex_values",
            "ALTER TABLE delivery_exceptions ADD CONSTRAINT dex_values CHECK (opened_value IN ('amber_red','red','threshold') AND worst_value IN ('amber_red','red','threshold'))",
            "CREATE OR REPLACE FUNCTION ex7_append_only() RETURNS trigger LANGUAGE plpgsql AS $$
             BEGIN
               IF TG_OP = 'UPDATE' AND current_setting('ppm.audit_pseudonymise', true) = 'on'
                  AND NEW.pid = OLD.pid AND NEW.plan_pid = OLD.plan_pid THEN
                 RETURN NEW;
               END IF;
               RAISE EXCEPTION 'triage and threshold decisions are append-only';
             END $$",
            "CREATE TRIGGER pt_append_only BEFORE UPDATE OR DELETE ON programme_triages FOR EACH ROW EXECUTE FUNCTION ex7_append_only()",
            "CREATE TRIGGER td_append_only BEFORE UPDATE OR DELETE ON threshold_decisions FOR EACH ROW EXECUTE FUNCTION ex7_append_only()",
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
        for q in [
            "DROP TRIGGER IF EXISTS td_append_only ON threshold_decisions",
            "DROP TRIGGER IF EXISTS pt_append_only ON programme_triages",
            "DROP FUNCTION IF EXISTS ex7_append_only()",
            "DELETE FROM delivery_exceptions WHERE threshold_kind IS NOT NULL AND opened_value = 'threshold'",
            "ALTER TABLE delivery_exceptions DROP CONSTRAINT dex_values",
            "ALTER TABLE delivery_exceptions ADD CONSTRAINT dex_values CHECK (opened_value IN ('amber_red','red') AND worst_value IN ('amber_red','red'))",
            "ALTER TABLE delivery_exceptions DROP COLUMN threshold_kind",
        ] {
            db.execute_unprepared(q).await?;
        }
        drop_table(m, "threshold_decisions").await?;
        drop_table(m, "programme_thresholds").await?;
        drop_table(m, "programme_triages").await
    }
}
