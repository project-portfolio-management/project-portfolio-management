-- m20261003_000009_task_parent: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: **`tasks.parent_task_pid`** — subtasks (RM-5). A parent's
-- dates, estimate and progress are **derived on read** from its
-- descendants, never stored, so they cannot drift from the work they
-- summarize.

ALTER TABLE tasks ADD COLUMN IF NOT EXISTS parent_task_pid UUID;

CREATE INDEX IF NOT EXISTS tasks_parent ON tasks (parent_task_pid) WHERE deleted_at IS NULL;
