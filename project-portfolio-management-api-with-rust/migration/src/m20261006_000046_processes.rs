//! Migration: **multi-step approval processes** (PL-2). A `process` is a named, ordered list of steps (`approve`, `todo`, `notify`); each edit adds a new immutable `process_versions` row, and an `execution` is **pinned** to the version it started on. All state lives in these tables (so an execution survives a restart), and `process_execution_logs` is **append-only**: the database refuses an update or a delete of a log row. An approval request made by a process carries which execution and step it belongs to.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The processes migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Create the tables, triggers and checks.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "processes",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("name", ColType::String),
                ("current_version", ColType::Integer),
                ("created_by", ColType::StringNull),
                ("deleted_at", ColType::TimestampWithTimeZoneNull),
            ],
            &[],
        )
        .await?;
        create_table(
            m,
            "process_versions",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("process_pid", ColType::Uuid),
                ("version", ColType::Integer),
                ("steps", ColType::Json),
                ("created_by", ColType::StringNull),
            ],
            &[],
        )
        .await?;
        create_table(
            m,
            "process_executions",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("process_pid", ColType::Uuid),
                ("version", ColType::Integer),
                ("subject_kind", ColType::String),
                ("subject_pid", ColType::Uuid),
                ("status", ColType::String),
                ("current_step", ColType::Integer),
                ("started_by", ColType::String),
                ("finished_at", ColType::TimestampWithTimeZoneNull),
                ("note", ColType::StringNull),
            ],
            &[],
        )
        .await?;
        create_table(
            m,
            "process_execution_logs",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("execution_pid", ColType::Uuid),
                ("step_index", ColType::IntegerNull),
                ("event", ColType::String),
                ("actor", ColType::StringNull),
                ("note", ColType::StringNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        for q in [
            "ALTER TABLE processes ADD CONSTRAINT processes_version CHECK (current_version >= 1)",
            "CREATE UNIQUE INDEX process_versions_unique ON process_versions (process_pid, version)",
            "ALTER TABLE process_executions ADD CONSTRAINT process_executions_status CHECK (status IN ('running','completed','rejected','cancelled','failed'))",
            "CREATE INDEX process_executions_subject ON process_executions (subject_kind, subject_pid)",
            "CREATE INDEX process_execution_log_execution ON process_execution_logs (execution_pid, id)",
            "ALTER TABLE process_execution_logs ADD CONSTRAINT process_execution_log_event CHECK (event IN ('execution_started','step_started','step_completed','step_skipped','step_failed','step_rejected','execution_completed','execution_rejected','execution_cancelled','execution_failed'))",
            // A version, once written, never changes; a log row, once written, is never changed or removed.
            "CREATE OR REPLACE FUNCTION processes_refuse_change() RETURNS trigger AS $$ BEGIN RAISE EXCEPTION '% is append-only', TG_TABLE_NAME; END; $$ LANGUAGE plpgsql",
            "DROP TRIGGER IF EXISTS process_versions_immutable ON process_versions",
            "CREATE TRIGGER process_versions_immutable BEFORE UPDATE OR DELETE ON process_versions FOR EACH ROW EXECUTE FUNCTION processes_refuse_change()",
            "DROP TRIGGER IF EXISTS process_execution_log_append_only ON process_execution_logs",
            "CREATE TRIGGER process_execution_log_append_only BEFORE UPDATE OR DELETE ON process_execution_logs FOR EACH ROW EXECUTE FUNCTION processes_refuse_change()",
            "ALTER TABLE approval_requests ADD COLUMN process_execution_pid uuid",
            "ALTER TABLE approval_requests ADD COLUMN process_step integer",
        ] {
            db.execute_unprepared(q).await?;
        }
        Ok(())
    }

    /// Drop it all again.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();
        db.execute_unprepared("ALTER TABLE approval_requests DROP COLUMN IF EXISTS process_step")
            .await?;
        db.execute_unprepared(
            "ALTER TABLE approval_requests DROP COLUMN IF EXISTS process_execution_pid",
        )
        .await?;
        drop_table(m, "process_execution_logs").await?;
        drop_table(m, "process_executions").await?;
        drop_table(m, "process_versions").await?;
        drop_table(m, "processes").await?;
        db.execute_unprepared("DROP FUNCTION IF EXISTS processes_refuse_change()")
            .await?;
        Ok(())
    }
}
