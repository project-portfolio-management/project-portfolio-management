//! Migration: **the monthly director pack** (DP-1). One row per pack version. `body` is the exact text that was computed at drafting; once a pack is issued the database refuses any change to it and any deletion, so what was issued is what is read, byte for byte, forever. A correction is a new row (the next version) that points at the one it replaces.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Create the table, its checks, its indexes and the freezing trigger.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "director_packs",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("period", ColType::String),
                ("audience", ColType::String),
                ("version", ColType::Integer),
                ("status", ColType::String),
                ("cut_off", ColType::TimestampWithTimeZone),
                ("body", ColType::Text),
                ("change_note", ColType::TextNull),
                ("supersedes_pid", ColType::UuidNull),
                ("drafted_by", ColType::StringNull),
                ("issued_by", ColType::StringNull),
                ("issued_at", ColType::TimestampWithTimeZoneNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        for q in [
            "ALTER TABLE director_packs ADD CONSTRAINT director_packs_period CHECK (period ~ '^[0-9]{4}-(0[1-9]|1[0-2])$')",
            "ALTER TABLE director_packs ADD CONSTRAINT director_packs_audience CHECK (audience IN ('executive','board','escalation_board','partner'))",
            "ALTER TABLE director_packs ADD CONSTRAINT director_packs_status CHECK (status IN ('draft','issued'))",
            "ALTER TABLE director_packs ADD CONSTRAINT director_packs_version CHECK (version >= 1)",
            "ALTER TABLE director_packs ADD CONSTRAINT director_packs_issued CHECK ((status = 'issued') = (issued_at IS NOT NULL AND issued_by IS NOT NULL))",
            "ALTER TABLE director_packs ADD CONSTRAINT director_packs_reissue CHECK ((version = 1 AND supersedes_pid IS NULL AND change_note IS NULL) OR (version > 1 AND supersedes_pid IS NOT NULL AND change_note IS NOT NULL))",
            "ALTER TABLE director_packs ADD CONSTRAINT director_packs_note CHECK (change_note IS NULL OR char_length(btrim(change_note)) >= 10)",
            "CREATE UNIQUE INDEX director_packs_version_once ON director_packs (period, audience, version)",
            "CREATE UNIQUE INDEX director_packs_one_draft ON director_packs (period, audience) WHERE status = 'draft'",
            // Frozen: an issued pack cannot change or go; a draft may be issued (status and the
            // issue fields) but its body, period, audience and version never change.
            "CREATE OR REPLACE FUNCTION director_packs_frozen() RETURNS trigger LANGUAGE plpgsql AS $$
             BEGIN
               IF TG_OP = 'DELETE' THEN
                 IF OLD.status = 'issued' THEN RAISE EXCEPTION 'an issued director pack cannot be deleted'; END IF;
                 RETURN OLD;
               END IF;
               IF OLD.status = 'issued' THEN
                 -- The one exception: pseudonymising a data subject (RG-6) may blank who drafted or issued it.
                 IF current_setting('ppm.audit_pseudonymise', true) = 'on'
                    AND NEW.body = OLD.body AND NEW.status = OLD.status AND NEW.period = OLD.period
                    AND NEW.audience = OLD.audience AND NEW.version = OLD.version AND NEW.cut_off = OLD.cut_off
                    AND NEW.issued_at IS NOT DISTINCT FROM OLD.issued_at
                    AND NEW.supersedes_pid IS NOT DISTINCT FROM OLD.supersedes_pid
                    AND NEW.change_note IS NOT DISTINCT FROM OLD.change_note THEN
                   RETURN NEW;
                 END IF;
                 RAISE EXCEPTION 'an issued director pack cannot be changed';
               END IF;
               IF NEW.body IS DISTINCT FROM OLD.body OR NEW.period <> OLD.period OR NEW.audience <> OLD.audience
                  OR NEW.version <> OLD.version OR NEW.cut_off <> OLD.cut_off
                  OR NEW.supersedes_pid IS DISTINCT FROM OLD.supersedes_pid
                  OR NEW.change_note IS DISTINCT FROM OLD.change_note THEN
                 RAISE EXCEPTION 'a director pack body is computed once and does not change';
               END IF;
               RETURN NEW;
             END $$",
            "CREATE TRIGGER director_packs_frozen BEFORE UPDATE OR DELETE ON director_packs FOR EACH ROW EXECUTE FUNCTION director_packs_frozen()",
        ] {
            db.execute_unprepared(q).await?;
        }
        Ok(())
    }

    /// Drop the trigger, the function and the table.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();
        db.execute_unprepared("DROP TRIGGER IF EXISTS director_packs_frozen ON director_packs")
            .await?;
        db.execute_unprepared("DROP FUNCTION IF EXISTS director_packs_frozen()")
            .await?;
        drop_table(m, "director_packs").await
    }
}
