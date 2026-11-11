-- m20220101_000005_governance: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

ALTER TABLE plans DROP COLUMN IF EXISTS stage;

DROP TABLE "budget_lines";

DROP TABLE "risks";

DROP TABLE "gate_reviews";

DROP TABLE "proposals";
