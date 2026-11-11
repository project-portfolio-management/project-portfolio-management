-- m20261006_000045_job_progress_cancel: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

DROP TRIGGER IF EXISTS bulk_jobs_progress_monotone ON bulk_jobs;

DROP FUNCTION IF EXISTS bulk_jobs_progress_monotone();

UPDATE bulk_jobs SET status = 'failed' WHERE status = 'cancelled';

ALTER TABLE bulk_jobs DROP CONSTRAINT bulk_jobs_status;

ALTER TABLE bulk_jobs ADD CONSTRAINT bulk_jobs_status CHECK (status IN ('queued','running','completed','completed_with_errors','failed'));

ALTER TABLE bulk_jobs DROP COLUMN cancelled_by;

ALTER TABLE bulk_jobs DROP COLUMN cancel_requested_at;

ALTER TABLE bulk_jobs DROP CONSTRAINT bulk_jobs_progress;

ALTER TABLE bulk_jobs DROP COLUMN progress;
