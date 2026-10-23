//! Migration: **go / no-go for safety-critical cutovers** (EX-6). A cutover decision has stop criteria that
//! are published before the decision opens and cannot be edited afterwards (a new version is made instead),
//! each marked met or not while it is open, and co-signatures from configured roles. "Go" needs every
//! criterion met and every co-signature; "no-go" and "delay for safety" are decisions in their own right.
//! The database holds the freeze: criteria change only while the decision is a draft.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
#[expect(clippy::too_many_lines)]
impl MigrationTrait for Migration {
    /// Create the tables, checks and triggers.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "cutover_decisions",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("plan_pid", ColType::Uuid),
                ("title", ColType::String),
                ("version", ColType::Integer),
                ("supersedes_pid", ColType::UuidNull),
                ("status", ColType::String),
                ("published_on", ColType::DateNull),
                ("opened_on", ColType::DateNull),
                ("outcome", ColType::StringNull),
                ("decided_on", ColType::DateNull),
                ("decided_by", ColType::StringNull),
                ("decision_reason", ColType::TextNull),
            ],
            &[],
        )
        .await?;
        create_table(
            m,
            "cutover_criteria",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("decision_pid", ColType::Uuid),
                ("position", ColType::Integer),
                ("criterion", ColType::Text),
                ("met", ColType::BooleanNull),
                ("evidence", ColType::TextNull),
                ("marked_by", ColType::StringNull),
                ("marked_on", ColType::DateNull),
            ],
            &[],
        )
        .await?;
        create_table(
            m,
            "cutover_cosignatures",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("decision_pid", ColType::Uuid),
                ("role", ColType::String),
                ("signed_by", ColType::StringNull),
                ("signed_on", ColType::Date),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        for q in [
            "ALTER TABLE cutover_decisions ADD CONSTRAINT cd_status CHECK (status IN ('draft','published','open','decided','superseded'))",
            "ALTER TABLE cutover_decisions ADD CONSTRAINT cd_outcome CHECK (outcome IS NULL OR outcome IN ('go','no_go','delay_for_safety'))",
            "ALTER TABLE cutover_decisions ADD CONSTRAINT cd_decided CHECK ((status = 'decided') = (outcome IS NOT NULL AND decided_on IS NOT NULL AND length(btrim(coalesce(decision_reason, ''))) >= 10))",
            "CREATE UNIQUE INDEX cd_version ON cutover_decisions (plan_pid, lower(title), version)",
            "CREATE UNIQUE INDEX cc_position ON cutover_criteria (decision_pid, position)",
            "CREATE UNIQUE INDEX ccs_once ON cutover_cosignatures (decision_pid, role)",
            // The freeze: criteria are added, changed or removed only while their decision is a draft. Marking
            // (met, evidence, marked_by, marked_on) is allowed while it is open.
            "CREATE OR REPLACE FUNCTION cutover_criteria_guard() RETURNS trigger LANGUAGE plpgsql AS $$
             DECLARE st text;
             BEGIN
               IF TG_OP = 'INSERT' THEN
                 SELECT status INTO st FROM cutover_decisions WHERE pid = NEW.decision_pid;
                 IF st IS DISTINCT FROM 'draft' THEN RAISE EXCEPTION 'the criteria of a published cutover decision cannot be added to: make a new version'; END IF;
                 RETURN NEW;
               END IF;
               SELECT status INTO st FROM cutover_decisions WHERE pid = OLD.decision_pid;
               IF TG_OP = 'DELETE' THEN
                 IF st IS DISTINCT FROM 'draft' THEN RAISE EXCEPTION 'the criteria of a published cutover decision cannot be removed: make a new version'; END IF;
                 RETURN OLD;
               END IF;
               IF NEW.criterion IS DISTINCT FROM OLD.criterion OR NEW.position IS DISTINCT FROM OLD.position OR NEW.decision_pid IS DISTINCT FROM OLD.decision_pid THEN
                 IF coalesce(current_setting('ppm.audit_pseudonymise', true), '') <> 'on' AND st IS DISTINCT FROM 'draft' THEN
                   RAISE EXCEPTION 'the criteria of a published cutover decision cannot be edited: make a new version';
                 END IF;
               END IF;
               IF (NEW.met IS DISTINCT FROM OLD.met OR NEW.evidence IS DISTINCT FROM OLD.evidence) AND st IS DISTINCT FROM 'open'
                  AND coalesce(current_setting('ppm.audit_pseudonymise', true), '') <> 'on' THEN
                 RAISE EXCEPTION 'a criterion is marked only while the decision is open';
               END IF;
               RETURN NEW;
             END $$",
            "CREATE TRIGGER cutover_criteria_guard BEFORE INSERT OR UPDATE OR DELETE ON cutover_criteria FOR EACH ROW EXECUTE FUNCTION cutover_criteria_guard()",
            // A decision moves forward only, and a decided one is final.
            "CREATE OR REPLACE FUNCTION cutover_decision_guard() RETURNS trigger LANGUAGE plpgsql AS $$
             BEGIN
               IF TG_OP = 'DELETE' THEN RAISE EXCEPTION 'a cutover decision is kept'; END IF;
               IF OLD.status IN ('decided','superseded') AND coalesce(current_setting('ppm.audit_pseudonymise', true), '') <> 'on' THEN
                 RAISE EXCEPTION 'a decided cutover decision is final';
               END IF;
               IF OLD.status <> 'draft' AND NEW.title IS DISTINCT FROM OLD.title THEN RAISE EXCEPTION 'a published cutover decision cannot be retitled'; END IF;
               RETURN NEW;
             END $$",
            "CREATE TRIGGER cutover_decision_guard BEFORE UPDATE OR DELETE ON cutover_decisions FOR EACH ROW EXECUTE FUNCTION cutover_decision_guard()",
            "CREATE OR REPLACE FUNCTION cutover_cosign_guard() RETURNS trigger LANGUAGE plpgsql AS $$
             BEGIN
               IF current_setting('ppm.audit_pseudonymise', true) = 'on' AND TG_OP = 'UPDATE' THEN RETURN NEW; END IF;
               RAISE EXCEPTION 'co-signatures are append-only';
             END $$",
            "CREATE TRIGGER cutover_cosign_guard BEFORE UPDATE OR DELETE ON cutover_cosignatures FOR EACH ROW EXECUTE FUNCTION cutover_cosign_guard()",
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
            "DROP TRIGGER IF EXISTS cutover_cosign_guard ON cutover_cosignatures",
            "DROP TRIGGER IF EXISTS cutover_decision_guard ON cutover_decisions",
            "DROP TRIGGER IF EXISTS cutover_criteria_guard ON cutover_criteria",
            "DROP FUNCTION IF EXISTS cutover_cosign_guard()",
            "DROP FUNCTION IF EXISTS cutover_decision_guard()",
            "DROP FUNCTION IF EXISTS cutover_criteria_guard()",
        ] {
            db.execute_unprepared(q).await?;
        }
        drop_table(m, "cutover_cosignatures").await?;
        drop_table(m, "cutover_criteria").await?;
        drop_table(m, "cutover_decisions").await
    }
}
