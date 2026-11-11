-- m20260720_000001_engineering: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: the engineering-team tables — `tasks` (the spec-§13
-- operational sub-resource behind the Kanban board, with status /
-- completion stamps for honest flow metrics) and `sprints`
-- (time-boxed iterations behind the burndown view) — plus the
-- milestone `kind` column (demo / release / checkpoint calendar).

CREATE TABLE IF NOT EXISTS sprints (
                     created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     id SERIAL PRIMARY KEY,
                     pid UUID NOT NULL UNIQUE,
                     plan_pid UUID NOT NULL,
                     name VARCHAR NOT NULL,
                     starts_on DATE NOT NULL,
                     ends_on DATE NOT NULL,
                     deleted_at TIMESTAMPTZ NULL
                 );
                 CREATE INDEX IF NOT EXISTS sprints_item ON sprints (plan_pid);
                 CREATE TABLE IF NOT EXISTS tasks (
                     created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     id SERIAL PRIMARY KEY,
                     pid UUID NOT NULL UNIQUE,
                     plan_pid UUID NOT NULL,
                     sprint_pid UUID NULL,
                     title VARCHAR NOT NULL,
                     description VARCHAR NULL,
                     status VARCHAR NOT NULL DEFAULT 'todo',
                     assignee_ref VARCHAR NULL,
                     status_changed_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     done_at TIMESTAMPTZ NULL,
                     deleted_at TIMESTAMPTZ NULL
                 );
                 CREATE INDEX IF NOT EXISTS tasks_item ON tasks (plan_pid);
                 CREATE INDEX IF NOT EXISTS tasks_status ON tasks (status);
                 ALTER TABLE milestones ADD COLUMN IF NOT EXISTS kind VARCHAR NULL;
