-- m20261010_000067_milestone_signoffs: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

DROP TRIGGER IF EXISTS mso_append_only ON milestone_signoffs;

DROP FUNCTION IF EXISTS mso_append_only();

DROP TABLE "milestone_signoffs";

DROP TABLE "milestone_signoff_requirements";
