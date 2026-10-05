//! Migration: **progress and cancellation for bulk jobs** (PJ-11). `bulk_jobs.progress` (0–100, **never goes backwards**: a trigger clamps any update that would lower it), `cancel_requested_at` / `cancelled_by` (a cooperative cancel: the running job notices between rows), and a `cancelled` status.

use sea_orm_migration::prelude::*;

/// The job progress and cancel migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Add the columns, the status, and the monotone trigger.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();
        for q in [
            "ALTER TABLE bulk_jobs ADD COLUMN progress integer NOT NULL DEFAULT 0",
            "ALTER TABLE bulk_jobs ADD CONSTRAINT bulk_jobs_progress CHECK (progress BETWEEN 0 AND 100)",
            "ALTER TABLE bulk_jobs ADD COLUMN cancel_requested_at timestamptz",
            "ALTER TABLE bulk_jobs ADD COLUMN cancelled_by varchar",
            "ALTER TABLE bulk_jobs DROP CONSTRAINT bulk_jobs_status",
            "ALTER TABLE bulk_jobs ADD CONSTRAINT bulk_jobs_status CHECK (status IN ('queued','running','completed','completed_with_errors','failed','cancelled'))",
            "CREATE OR REPLACE FUNCTION bulk_jobs_progress_monotone() RETURNS trigger AS $$ BEGIN IF NEW.progress < OLD.progress THEN NEW.progress := OLD.progress; END IF; RETURN NEW; END; $$ LANGUAGE plpgsql",
            "DROP TRIGGER IF EXISTS bulk_jobs_progress_monotone ON bulk_jobs",
            "CREATE TRIGGER bulk_jobs_progress_monotone BEFORE UPDATE ON bulk_jobs FOR EACH ROW EXECUTE FUNCTION bulk_jobs_progress_monotone()",
        ] {
            db.execute_unprepared(q).await?;
        }
        Ok(())
    }

    /// Remove them again.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();
        for q in [
            "DROP TRIGGER IF EXISTS bulk_jobs_progress_monotone ON bulk_jobs",
            "DROP FUNCTION IF EXISTS bulk_jobs_progress_monotone()",
            "UPDATE bulk_jobs SET status = 'failed' WHERE status = 'cancelled'",
            "ALTER TABLE bulk_jobs DROP CONSTRAINT bulk_jobs_status",
            "ALTER TABLE bulk_jobs ADD CONSTRAINT bulk_jobs_status CHECK (status IN ('queued','running','completed','completed_with_errors','failed'))",
            "ALTER TABLE bulk_jobs DROP COLUMN cancelled_by",
            "ALTER TABLE bulk_jobs DROP COLUMN cancel_requested_at",
            "ALTER TABLE bulk_jobs DROP CONSTRAINT bulk_jobs_progress",
            "ALTER TABLE bulk_jobs DROP COLUMN progress",
        ] {
            db.execute_unprepared(q).await?;
        }
        Ok(())
    }
}
