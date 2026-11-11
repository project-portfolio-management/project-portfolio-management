-- m20261003_000027_task_external_ref: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

DROP INDEX tasks_external_ref_once;

ALTER TABLE tasks DROP COLUMN external_ref;
