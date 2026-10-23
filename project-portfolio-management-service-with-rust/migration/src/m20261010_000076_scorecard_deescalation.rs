//! Migration: **the outcome scorecard and the de-escalation tracker** (EX-10). A scorecard is a named set of
//! KPIs read by quarter. The tracker records the escalation level as the sponsor set it (append-only), the
//! criteria for coming down, and per quarter whether each is met with a link to the evidence (append-only;
//! the latest record for a criterion and quarter speaks).

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
            "scorecards",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("name", ColType::String),
                ("for_deescalation", ColType::Boolean),
                ("active", ColType::Boolean),
            ],
            &[],
        )
        .await?;
        create_table(
            m,
            "scorecard_kpis",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("scorecard_pid", ColType::Uuid),
                ("kpi_pid", ColType::Uuid),
            ],
            &[],
        )
        .await?;
        create_table(
            m,
            "escalation_levels",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("level", ColType::String),
                ("note", ColType::TextNull),
                ("set_by", ColType::StringNull),
                ("set_on", ColType::Date),
            ],
            &[],
        )
        .await?;
        create_table(
            m,
            "deescalation_criteria",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("criterion", ColType::Text),
                ("active", ColType::Boolean),
            ],
            &[],
        )
        .await?;
        create_table(
            m,
            "criterion_evidence",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("criterion_pid", ColType::Uuid),
                ("quarter", ColType::String),
                ("met", ColType::Boolean),
                ("link", ColType::TextNull),
                ("note", ColType::Text),
                ("recorded_by", ColType::StringNull),
                ("recorded_on", ColType::Date),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        for q in [
            "CREATE UNIQUE INDEX sc_name ON scorecards (lower(btrim(name)))",
            "CREATE UNIQUE INDEX sc_one_deescalation ON scorecards (for_deescalation) WHERE for_deescalation",
            "CREATE UNIQUE INDEX sck_once ON scorecard_kpis (scorecard_pid, kpi_pid)",
            "ALTER TABLE escalation_levels ADD CONSTRAINT el_level CHECK (length(btrim(level)) BETWEEN 1 AND 60)",
            "ALTER TABLE criterion_evidence ADD CONSTRAINT ce_quarter CHECK (quarter ~ '^[0-9]{4}-Q[1-4]$')",
            "ALTER TABLE criterion_evidence ADD CONSTRAINT ce_note CHECK (length(btrim(note)) >= 5)",
            "ALTER TABLE criterion_evidence ADD CONSTRAINT ce_link CHECK (link IS NULL OR link ~ '^https?://[^ ]+$')",
            "CREATE INDEX ce_by_criterion ON criterion_evidence (criterion_pid, quarter, id)",
            "CREATE OR REPLACE FUNCTION ex10_append_only() RETURNS trigger LANGUAGE plpgsql AS $$
             BEGIN
               IF TG_OP = 'UPDATE' AND coalesce(current_setting('ppm.audit_pseudonymise', true), '') = 'on' THEN RETURN NEW; END IF;
               RAISE EXCEPTION 'escalation levels and evidence are append-only';
             END $$",
            "CREATE TRIGGER el_append_only BEFORE UPDATE OR DELETE ON escalation_levels FOR EACH ROW EXECUTE FUNCTION ex10_append_only()",
            "CREATE TRIGGER ce_append_only BEFORE UPDATE OR DELETE ON criterion_evidence FOR EACH ROW EXECUTE FUNCTION ex10_append_only()",
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
        db.execute_unprepared("DROP TRIGGER IF EXISTS ce_append_only ON criterion_evidence")
            .await?;
        db.execute_unprepared("DROP TRIGGER IF EXISTS el_append_only ON escalation_levels")
            .await?;
        db.execute_unprepared("DROP FUNCTION IF EXISTS ex10_append_only()")
            .await?;
        drop_table(m, "criterion_evidence").await?;
        drop_table(m, "deescalation_criteria").await?;
        drop_table(m, "escalation_levels").await?;
        drop_table(m, "scorecard_kpis").await?;
        drop_table(m, "scorecards").await
    }
}
