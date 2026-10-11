//! Migration: **buffer management** (CC-4). A buffer plan freezes the critical chain and its buffers as they
//! were planned; snapshots record each buffer's reading at a moment; recovery actions are what people
//! agreed to do about a buffer in the red. Plans and snapshots are append-only; an action may be closed once
//! and never deleted. The one exception, for all three: a data-subject pseudonymisation may blank `owner_ref`.

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
            "buffer_plans",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("plan_pid", ColType::Uuid),
                ("taken_on", ColType::Date),
                ("document", ColType::Text),
            ],
            &[],
        )
        .await?;
        create_table(
            m,
            "buffer_snapshots",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("plan_pid", ColType::Uuid),
                ("buffer_plan_pid", ColType::Uuid),
                ("taken_on", ColType::Date),
                ("document", ColType::Text),
            ],
            &[],
        )
        .await?;
        create_table(
            m,
            "buffer_recovery_actions",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("plan_pid", ColType::Uuid),
                ("buffer_plan_pid", ColType::Uuid),
                ("buffer_key", ColType::String),
                ("action", ColType::Text),
                ("owner_ref", ColType::StringNull),
                ("due_date", ColType::DateNull),
                ("status", ColType::String),
                ("closed_on", ColType::DateNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        for q in [
            "CREATE INDEX bp_by_plan ON buffer_plans (plan_pid, id)",
            "CREATE INDEX bs_by_plan ON buffer_snapshots (plan_pid, id)",
            "CREATE INDEX bra_by_plan ON buffer_recovery_actions (plan_pid, id)",
            "ALTER TABLE buffer_recovery_actions ADD CONSTRAINT bra_status CHECK (status IN ('open','done','dropped'))",
            "ALTER TABLE buffer_recovery_actions ADD CONSTRAINT bra_closed CHECK ((status = 'open') = (closed_on IS NULL))",
            "CREATE OR REPLACE FUNCTION buffer_append_only() RETURNS trigger LANGUAGE plpgsql AS $$
             BEGIN
               RAISE EXCEPTION 'buffer plans and snapshots are append-only';
             END $$",
            "CREATE TRIGGER bp_append_only BEFORE UPDATE OR DELETE ON buffer_plans FOR EACH ROW EXECUTE FUNCTION buffer_append_only()",
            "CREATE TRIGGER bs_append_only BEFORE UPDATE OR DELETE ON buffer_snapshots FOR EACH ROW EXECUTE FUNCTION buffer_append_only()",
            "CREATE OR REPLACE FUNCTION bra_guard() RETURNS trigger LANGUAGE plpgsql AS $$
             BEGIN
               IF TG_OP = 'DELETE' THEN
                 RAISE EXCEPTION 'a recovery action is closed, never deleted';
               END IF;
               IF current_setting('ppm.audit_pseudonymise', true) = 'on'
                  AND NEW.pid = OLD.pid AND NEW.action = OLD.action AND NEW.status = OLD.status
                  AND NEW.buffer_key = OLD.buffer_key THEN
                 RETURN NEW;
               END IF;
               IF OLD.status = 'open' AND NEW.status IN ('done','dropped')
                  AND NEW.pid = OLD.pid AND NEW.plan_pid = OLD.plan_pid AND NEW.buffer_key = OLD.buffer_key
                  AND NEW.action = OLD.action AND NEW.owner_ref IS NOT DISTINCT FROM OLD.owner_ref
                  AND NEW.due_date IS NOT DISTINCT FROM OLD.due_date THEN
                 RETURN NEW;
               END IF;
               RAISE EXCEPTION 'a recovery action can only be closed, once';
             END $$",
            "CREATE TRIGGER bra_guard BEFORE UPDATE OR DELETE ON buffer_recovery_actions FOR EACH ROW EXECUTE FUNCTION bra_guard()",
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
            "DROP TRIGGER IF EXISTS bra_guard ON buffer_recovery_actions",
            "DROP TRIGGER IF EXISTS bs_append_only ON buffer_snapshots",
            "DROP TRIGGER IF EXISTS bp_append_only ON buffer_plans",
            "DROP FUNCTION IF EXISTS bra_guard()",
            "DROP FUNCTION IF EXISTS buffer_append_only()",
        ] {
            db.execute_unprepared(q).await?;
        }
        drop_table(m, "buffer_recovery_actions").await?;
        drop_table(m, "buffer_snapshots").await?;
        drop_table(m, "buffer_plans").await
    }
}
