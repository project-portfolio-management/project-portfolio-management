-- m20260720_000001_engineering: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

DROP TABLE IF EXISTS tasks;
                 DROP TABLE IF EXISTS sprints;
                 ALTER TABLE milestones DROP COLUMN IF EXISTS kind;
