-- m20261006_000045_job_progress_cancel: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: **progress and cancellation for bulk jobs** (PJ-11). `bulk_jobs.progress` (0–100, **never goes backwards**: a trigger clamps any update that would lower it), `cancel_requested_at` / `cancelled_by` (a cooperative cancel: the running job notices between rows), and a `cancelled` status.

ALTER TABLE bulk_jobs ADD COLUMN progress integer NOT NULL DEFAULT 0;

ALTER TABLE bulk_jobs ADD CONSTRAINT bulk_jobs_progress CHECK (progress BETWEEN 0 AND 100);

ALTER TABLE bulk_jobs ADD COLUMN cancel_requested_at timestamptz;

ALTER TABLE bulk_jobs ADD COLUMN cancelled_by varchar;

ALTER TABLE bulk_jobs DROP CONSTRAINT bulk_jobs_status;

ALTER TABLE bulk_jobs ADD CONSTRAINT bulk_jobs_status CHECK (status IN ('queued','running','completed','completed_with_errors','failed','cancelled'));

CREATE OR REPLACE FUNCTION bulk_jobs_progress_monotone() RETURNS trigger AS $$ BEGIN IF NEW.progress < OLD.progress THEN NEW.progress := OLD.progress; END IF; RETURN NEW; END; $$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS bulk_jobs_progress_monotone ON bulk_jobs;

CREATE TRIGGER bulk_jobs_progress_monotone BEFORE UPDATE ON bulk_jobs FOR EACH ROW EXECUTE FUNCTION bulk_jobs_progress_monotone();
