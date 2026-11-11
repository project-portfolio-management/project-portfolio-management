-- m20261003_000009_task_parent: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

DROP INDEX IF EXISTS tasks_parent;

ALTER TABLE tasks DROP COLUMN IF EXISTS parent_task_pid;
