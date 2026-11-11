-- m20261010_000058_funding_sources: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

DROP INDEX IF EXISTS budget_lines_by_source;

ALTER TABLE budget_lines DROP COLUMN IF EXISTS funding_source_pid;

DROP TABLE "funding_sources";
