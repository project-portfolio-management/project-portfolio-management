-- m20260920_000001_report_group_by: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

ALTER TABLE report_definitions DROP COLUMN IF EXISTS group_by;
