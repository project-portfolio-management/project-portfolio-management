//! Migration: **delivery confidence, exceptions and strategic aims** (DP-2, DP-3, DP-4).
//!
//! * `delivery_confidence_assessments`: append-only. The database refuses to change or delete a row
//!   (the one exception: a data-subject pseudonymisation may blank `assessor`).
//! * `delivery_exceptions`: opened when confidence reaches amber-red or red; closed only by a recorded
//!   decision. A closed row must carry its decision, who decided and when.
//! * `strategic_aims`: objectives marked as strategic aims; `objective_links.is_primary` names a
//!   programme's one primary aim.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Create the tables, checks, indexes and triggers.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "delivery_confidence_assessments",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("plan_pid", ColType::Uuid),
                ("value", ColType::String),
                ("kind", ColType::String),
                ("rationale", ColType::Text),
                ("assessor", ColType::StringNull),
                ("assessed_on", ColType::Date),
            ],
            &[],
        )
        .await?;
        create_table(
            m,
            "delivery_exceptions",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("plan_pid", ColType::Uuid),
                ("status", ColType::String),
                ("opened_at", ColType::TimestampWithTimeZone),
                ("opened_by", ColType::StringNull),
                ("opened_value", ColType::String),
                ("worst_value", ColType::String),
                ("worsened_count", ColType::Integer),
                ("cause", ColType::TextNull),
                ("decision_needed", ColType::TextNull),
                ("owner_ref", ColType::StringNull),
                ("owner_role", ColType::StringNull),
                ("decide_by", ColType::DateNull),
                ("completed_at", ColType::TimestampWithTimeZoneNull),
                ("breach_notified_at", ColType::TimestampWithTimeZoneNull),
                ("closed_at", ColType::TimestampWithTimeZoneNull),
                ("decision", ColType::TextNull),
                ("decided_by", ColType::StringNull),
            ],
            &[],
        )
        .await?;
        create_table(
            m,
            "strategic_aims",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("objective_pid", ColType::Uuid),
                ("marked_by", ColType::StringNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        for q in [
            "ALTER TABLE delivery_confidence_assessments ADD CONSTRAINT dca_value CHECK (value IN ('green','amber_green','amber','amber_red','red'))",
            "ALTER TABLE delivery_confidence_assessments ADD CONSTRAINT dca_kind CHECK (kind IN ('self','independent'))",
            "ALTER TABLE delivery_confidence_assessments ADD CONSTRAINT dca_rationale CHECK (char_length(btrim(rationale)) >= 10)",
            "CREATE INDEX dca_by_plan ON delivery_confidence_assessments (plan_pid, assessed_on, id)",
            "ALTER TABLE delivery_exceptions ADD CONSTRAINT dex_status CHECK (status IN ('open','closed'))",
            "ALTER TABLE delivery_exceptions ADD CONSTRAINT dex_values CHECK (opened_value IN ('amber_red','red') AND worst_value IN ('amber_red','red'))",
            "ALTER TABLE delivery_exceptions ADD CONSTRAINT dex_closed CHECK ((status = 'closed') = (closed_at IS NOT NULL AND decided_by IS NOT NULL AND char_length(btrim(coalesce(decision, ''))) >= 10))",
            "ALTER TABLE delivery_exceptions ADD CONSTRAINT dex_worsened CHECK (worsened_count >= 0)",
            "CREATE UNIQUE INDEX dex_one_open ON delivery_exceptions (plan_pid) WHERE status = 'open'",
            "CREATE INDEX dex_by_status ON delivery_exceptions (status, opened_at)",
            "CREATE UNIQUE INDEX strategic_aims_once ON strategic_aims (objective_pid)",
            "ALTER TABLE objective_links ADD COLUMN is_primary boolean NOT NULL DEFAULT false",
            "CREATE UNIQUE INDEX objective_links_one_primary ON objective_links (plan_pid) WHERE is_primary",
            // Append-only history: no edit and no delete, except blanking the assessor when a data
            // subject is pseudonymised (RG-6), which changes nothing else.
            "CREATE OR REPLACE FUNCTION dca_append_only() RETURNS trigger LANGUAGE plpgsql AS $$
             BEGIN
               IF TG_OP = 'UPDATE' AND current_setting('ppm.audit_pseudonymise', true) = 'on'
                  AND NEW.plan_pid = OLD.plan_pid AND NEW.value = OLD.value AND NEW.kind = OLD.kind
                  AND NEW.rationale = OLD.rationale AND NEW.assessed_on = OLD.assessed_on
                  AND NEW.pid = OLD.pid THEN
                 RETURN NEW;
               END IF;
               RAISE EXCEPTION 'delivery confidence history is append-only';
             END $$",
            "CREATE TRIGGER dca_append_only BEFORE UPDATE OR DELETE ON delivery_confidence_assessments FOR EACH ROW EXECUTE FUNCTION dca_append_only()",
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
        db.execute_unprepared("DROP INDEX IF EXISTS objective_links_one_primary")
            .await?;
        db.execute_unprepared("ALTER TABLE objective_links DROP COLUMN IF EXISTS is_primary")
            .await?;
        db.execute_unprepared(
            "DROP TRIGGER IF EXISTS dca_append_only ON delivery_confidence_assessments",
        )
        .await?;
        db.execute_unprepared("DROP FUNCTION IF EXISTS dca_append_only()")
            .await?;
        drop_table(m, "strategic_aims").await?;
        drop_table(m, "delivery_exceptions").await?;
        drop_table(m, "delivery_confidence_assessments").await
    }
}
