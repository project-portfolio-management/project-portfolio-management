-- m20261021_000084_task_links: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

DELETE FROM plan_links WHERE task_pid IS NOT NULL;
                 DROP INDEX IF EXISTS plan_links_task;
                 DROP INDEX IF EXISTS plan_links_one;
                 CREATE UNIQUE INDEX plan_links_one ON plan_links (plan_pid, url) WHERE deleted_at IS NULL;
                 ALTER TABLE plan_links DROP COLUMN IF EXISTS task_pid;
