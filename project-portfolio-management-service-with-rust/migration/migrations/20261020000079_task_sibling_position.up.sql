-- m20261020_000079_task_sibling_position: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: **sibling order of tasks**. An optional position among the tasks that share a parent
-- (`parent_task_pid`, or none for the top level), set by `PUT /api/plans/{pid}/task-order`, so a work breakdown
-- structure can be reordered. A task with no position sorts after the positioned ones, oldest first.

ALTER TABLE tasks ADD COLUMN IF NOT EXISTS sibling_position INTEGER NULL;
                 ALTER TABLE tasks DROP CONSTRAINT IF EXISTS tasks_sibling_position_check;
                 ALTER TABLE tasks ADD CONSTRAINT tasks_sibling_position_check
                     CHECK (sibling_position IS NULL OR sibling_position >= 0);
