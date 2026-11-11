-- m20261025_000096_meeting_tasks: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: **`meeting_tasks`** (BR-3). Which task was created from which outcome of a meeting. An outcome has at
-- most one task, so a second commit of the same outcome is refused by the table itself, not only by a check.

CREATE TABLE IF NOT EXISTS meeting_tasks (
                     created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     id BIGSERIAL PRIMARY KEY,
                     meeting_pid UUID NOT NULL,
                     outcome_no INTEGER NOT NULL,
                     task_pid UUID NOT NULL,
                     CONSTRAINT meeting_tasks_outcome_once UNIQUE (meeting_pid, outcome_no),
                     CONSTRAINT meeting_tasks_outcome_no CHECK (outcome_no >= 0)
                 );
                 CREATE INDEX IF NOT EXISTS meeting_tasks_task_idx ON meeting_tasks (task_pid);
