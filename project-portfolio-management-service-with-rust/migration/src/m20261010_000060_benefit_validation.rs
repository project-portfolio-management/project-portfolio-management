//! Migration: **validated benefits** (DP-9). A benefit gains a recurrence, an annualized amount, a
//! classification and an optional KPI. Validations are append-only: the one exception is blanking the
//! validator when a data subject is pseudonymised.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Alter `benefits`; create `benefit_validations` with its checks and trigger.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "benefit_validations",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("benefit_pid", ColType::Uuid),
                ("plan_pid", ColType::Uuid),
                ("amount_minor", ColType::BigInteger),
                ("currency", ColType::String),
                ("validator", ColType::StringNull),
                ("evidence_attachment_pid", ColType::UuidNull),
                ("validated_on", ColType::Date),
                ("note", ColType::TextNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        for q in [
            "ALTER TABLE benefits ADD COLUMN recurrence text",
            "ALTER TABLE benefits ADD COLUMN annualized_minor bigint",
            "ALTER TABLE benefits ADD COLUMN classification text",
            "ALTER TABLE benefits ADD COLUMN kpi_pid uuid",
            "ALTER TABLE benefits ADD CONSTRAINT benefits_recurrence CHECK (recurrence IS NULL OR recurrence IN ('recurring','one_off'))",
            "ALTER TABLE benefits ADD CONSTRAINT benefits_classification CHECK (classification IS NULL OR classification IN ('cash_releasing','non_cash_releasing','societal'))",
            "ALTER TABLE benefits ADD CONSTRAINT benefits_annualized CHECK (annualized_minor IS NULL OR (recurrence = 'recurring' AND annualized_minor >= 0))",
            "ALTER TABLE benefit_validations ADD CONSTRAINT bv_amount CHECK (amount_minor >= 0)",
            "CREATE INDEX bv_by_benefit ON benefit_validations (benefit_pid, id)",
            "CREATE OR REPLACE FUNCTION bv_guard() RETURNS trigger LANGUAGE plpgsql AS $$
             BEGIN
               IF TG_OP = 'INSERT' THEN
                 IF NEW.validator IS NULL OR btrim(NEW.validator) = '' THEN
                   RAISE EXCEPTION 'a benefit validation names who validated it';
                 END IF;
                 RETURN NEW;
               END IF;
               IF TG_OP = 'UPDATE' AND current_setting('ppm.audit_pseudonymise', true) = 'on'
                  AND NEW.benefit_pid = OLD.benefit_pid AND NEW.amount_minor = OLD.amount_minor
                  AND NEW.currency = OLD.currency AND NEW.validated_on = OLD.validated_on
                  AND NEW.pid = OLD.pid THEN
                 RETURN NEW;
               END IF;
               RAISE EXCEPTION 'benefit validations are append-only';
             END $$",
            "CREATE TRIGGER bv_guard BEFORE INSERT OR UPDATE OR DELETE ON benefit_validations FOR EACH ROW EXECUTE FUNCTION bv_guard()",
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
        db.execute_unprepared("DROP TRIGGER IF EXISTS bv_guard ON benefit_validations")
            .await?;
        db.execute_unprepared("DROP FUNCTION IF EXISTS bv_guard()")
            .await?;
        drop_table(m, "benefit_validations").await?;
        for q in [
            "ALTER TABLE benefits DROP CONSTRAINT IF EXISTS benefits_annualized",
            "ALTER TABLE benefits DROP CONSTRAINT IF EXISTS benefits_classification",
            "ALTER TABLE benefits DROP CONSTRAINT IF EXISTS benefits_recurrence",
            "ALTER TABLE benefits DROP COLUMN IF EXISTS kpi_pid",
            "ALTER TABLE benefits DROP COLUMN IF EXISTS classification",
            "ALTER TABLE benefits DROP COLUMN IF EXISTS annualized_minor",
            "ALTER TABLE benefits DROP COLUMN IF EXISTS recurrence",
        ] {
            db.execute_unprepared(q).await?;
        }
        Ok(())
    }
}
