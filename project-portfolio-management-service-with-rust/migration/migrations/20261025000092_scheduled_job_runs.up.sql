-- m20261025_000092_scheduled_job_runs: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: **`scheduled_job_runs`** (NC-5). One row per run of a job the service schedules for itself, so that
-- replicas can tell a job has just run, and an operator can see what ran and how it ended.

CREATE TABLE IF NOT EXISTS scheduled_job_runs (
                     created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     id BIGSERIAL PRIMARY KEY,
                     pid UUID NOT NULL UNIQUE,
                     job VARCHAR NOT NULL,
                     started_at TIMESTAMPTZ NOT NULL,
                     finished_at TIMESTAMPTZ NULL,
                     outcome VARCHAR NOT NULL DEFAULT 'running',
                     detail TEXT NULL,
                     CONSTRAINT scheduled_job_runs_outcome CHECK (outcome IN ('running', 'ok', 'failed'))
                 );
                 CREATE INDEX IF NOT EXISTS scheduled_job_runs_job_idx ON scheduled_job_runs (job, started_at DESC);
