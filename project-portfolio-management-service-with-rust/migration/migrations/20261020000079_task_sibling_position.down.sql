-- m20261020_000079_task_sibling_position: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

ALTER TABLE tasks DROP COLUMN IF EXISTS sibling_position;
