-- m20261003_000027_task_external_ref: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: **`tasks.external_ref`** (PJ-10). A stable reference to where a
-- task came from (`mspdi:<UID>`), unique per plan, so re-importing the same MS
-- Project file updates its tasks instead of duplicating them.

ALTER TABLE tasks ADD COLUMN external_ref text;

CREATE UNIQUE INDEX tasks_external_ref_once ON tasks (plan_pid, external_ref) WHERE external_ref IS NOT NULL AND deleted_at IS NULL;
