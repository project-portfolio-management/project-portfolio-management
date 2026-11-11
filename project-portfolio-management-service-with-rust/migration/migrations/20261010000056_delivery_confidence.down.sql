-- m20261010_000056_delivery_confidence: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

DROP INDEX IF EXISTS objective_links_one_primary;

ALTER TABLE objective_links DROP COLUMN IF EXISTS is_primary;

DROP TRIGGER IF EXISTS dca_append_only ON delivery_confidence_assessments;

DROP FUNCTION IF EXISTS dca_append_only();

DROP TABLE "strategic_aims";

DROP TABLE "delivery_exceptions";

DROP TABLE "delivery_confidence_assessments";
