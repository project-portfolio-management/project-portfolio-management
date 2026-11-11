-- m20260720_000002_engineering_moderate: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: the engineering moderate fits — task story `points`
-- (team-local; velocity derives from them), `sprint_notes` (retro /
-- feedback log per sprint, convertible to tasks), and
-- `devops_events` (ingested deploy / incident / recovery events —
-- the only source the DORA-style metrics ever derive from).

ALTER TABLE tasks ADD COLUMN IF NOT EXISTS points INTEGER NULL;
                 CREATE TABLE IF NOT EXISTS sprint_notes (
                     created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     id SERIAL PRIMARY KEY,
                     pid UUID NOT NULL UNIQUE,
                     sprint_pid UUID NOT NULL,
                     category VARCHAR NOT NULL,
                     body VARCHAR NOT NULL,
                     task_pid UUID NULL,
                     deleted_at TIMESTAMPTZ NULL
                 );
                 CREATE INDEX IF NOT EXISTS sprint_notes_sprint
                     ON sprint_notes (sprint_pid);
                 CREATE TABLE IF NOT EXISTS devops_events (
                     created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     id SERIAL PRIMARY KEY,
                     pid UUID NOT NULL UNIQUE,
                     plan_pid UUID NOT NULL,
                     kind VARCHAR NOT NULL,
                     environment VARCHAR NULL,
                     version VARCHAR NULL,
                     reference VARCHAR NULL,
                     incident_pid UUID NULL,
                     caused_by_deploy_pid UUID NULL,
                     occurred_at TIMESTAMPTZ NOT NULL DEFAULT now()
                 );
                 CREATE INDEX IF NOT EXISTS devops_events_kind
                     ON devops_events (kind, occurred_at);
