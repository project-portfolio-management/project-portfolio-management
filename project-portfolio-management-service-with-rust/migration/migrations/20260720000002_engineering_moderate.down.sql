-- m20260720_000002_engineering_moderate: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

DROP TABLE IF EXISTS devops_events;
                 DROP TABLE IF EXISTS sprint_notes;
                 ALTER TABLE tasks DROP COLUMN IF EXISTS points;
