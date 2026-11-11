-- m20261010_000060_benefit_validation: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

DROP TRIGGER IF EXISTS bv_guard ON benefit_validations;

DROP FUNCTION IF EXISTS bv_guard();

DROP TABLE "benefit_validations";

ALTER TABLE benefits DROP CONSTRAINT IF EXISTS benefits_annualized;

ALTER TABLE benefits DROP CONSTRAINT IF EXISTS benefits_classification;

ALTER TABLE benefits DROP CONSTRAINT IF EXISTS benefits_recurrence;

ALTER TABLE benefits DROP COLUMN IF EXISTS kpi_pid;

ALTER TABLE benefits DROP COLUMN IF EXISTS classification;

ALTER TABLE benefits DROP COLUMN IF EXISTS annualized_minor;

ALTER TABLE benefits DROP COLUMN IF EXISTS recurrence;
