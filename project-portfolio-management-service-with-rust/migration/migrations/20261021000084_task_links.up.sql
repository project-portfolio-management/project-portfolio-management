-- m20261021_000084_task_links: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: **links on a task**. A link may belong to one task of the plan (`task_pid`), for example the Jira or
-- Linear issue the task is tracked as. Plan-level links have no `task_pid`. A URL may be linked once per plan-level
-- list and once per task, so the same address can appear on a plan and on one of its tasks.

ALTER TABLE plan_links ADD COLUMN IF NOT EXISTS task_pid UUID NULL;
                 DROP INDEX IF EXISTS plan_links_one;
                 CREATE UNIQUE INDEX plan_links_one ON plan_links
                     (plan_pid, COALESCE(task_pid, '00000000-0000-0000-0000-000000000000'::uuid), url)
                     WHERE deleted_at IS NULL;
                 CREATE INDEX IF NOT EXISTS plan_links_task ON plan_links (task_pid) WHERE task_pid IS NOT NULL;
