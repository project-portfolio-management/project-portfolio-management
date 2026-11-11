-- m20261105_000105_plan_template_source: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

ALTER TABLE plan_templates DROP COLUMN source_plan_pid;
