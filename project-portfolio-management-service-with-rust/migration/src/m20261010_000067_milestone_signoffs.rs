//! Migration: **milestone sign-offs** (EX-4). A milestone may require sign-off from named organizations;
//! each sign-off or withdrawal is an append-only event, so the history is kept and a withdrawal carries its
//! reason. The one exception: a data-subject pseudonymisation may blank `signed_by`.

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
            "milestone_signoff_requirements",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("milestone_pid", ColType::Uuid),
                ("organization_pid", ColType::Uuid),
            ],
            &[],
        )
        .await?;
        create_table(
            m,
            "milestone_signoffs",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("milestone_pid", ColType::Uuid),
                ("organization_pid", ColType::Uuid),
                ("action", ColType::String),
                ("reason", ColType::TextNull),
                ("acted_on", ColType::Date),
                ("signed_by", ColType::StringNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        for q in [
            "CREATE UNIQUE INDEX msr_once ON milestone_signoff_requirements (milestone_pid, organization_pid)",
            "CREATE INDEX mso_by_milestone ON milestone_signoffs (milestone_pid, id)",
            "ALTER TABLE milestone_signoffs ADD CONSTRAINT mso_action CHECK (action IN ('sign','withdraw'))",
            "ALTER TABLE milestone_signoffs ADD CONSTRAINT mso_reason CHECK (action = 'sign' OR length(btrim(coalesce(reason, ''))) >= 5)",
            "CREATE OR REPLACE FUNCTION mso_append_only() RETURNS trigger LANGUAGE plpgsql AS $$
             BEGIN
               IF TG_OP = 'UPDATE' AND current_setting('ppm.audit_pseudonymise', true) = 'on'
                  AND NEW.pid = OLD.pid AND NEW.action = OLD.action AND NEW.milestone_pid = OLD.milestone_pid
                  AND NEW.organization_pid = OLD.organization_pid AND NEW.acted_on = OLD.acted_on THEN
                 RETURN NEW;
               END IF;
               RAISE EXCEPTION 'milestone sign-offs are append-only';
             END $$",
            "CREATE TRIGGER mso_append_only BEFORE UPDATE OR DELETE ON milestone_signoffs FOR EACH ROW EXECUTE FUNCTION mso_append_only()",
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
        db.execute_unprepared("DROP TRIGGER IF EXISTS mso_append_only ON milestone_signoffs").await?;
        db.execute_unprepared("DROP FUNCTION IF EXISTS mso_append_only()").await?;
        drop_table(m, "milestone_signoffs").await?;
        drop_table(m, "milestone_signoff_requirements").await
    }
}
