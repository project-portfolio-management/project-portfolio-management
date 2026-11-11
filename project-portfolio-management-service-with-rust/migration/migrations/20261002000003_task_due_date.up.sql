-- m20261002_000003_task_due_date: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: **`tasks.due_date`** — a task's due **calendar date** (LT-3).
-- A `DATE`, not a timestamp: "due 2026-10-03" names that day with no zone
-- (the time-zone doctrine, `src/tz.rs`). Nullable: an undated task stays
-- undated, never a made-up date. The partial index serves the "my open
-- tasks" lookup (LT-3), which filters by assignee over live, unfinished rows.

ALTER TABLE tasks ADD COLUMN IF NOT EXISTS due_date DATE;

CREATE INDEX IF NOT EXISTS tasks_assignee_open ON tasks (assignee_ref) WHERE deleted_at IS NULL AND done_at IS NULL AND assignee_ref IS NOT NULL;
