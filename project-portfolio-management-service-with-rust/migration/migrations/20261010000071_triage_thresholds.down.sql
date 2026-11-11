-- m20261010_000071_triage_thresholds: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

DROP TRIGGER IF EXISTS td_append_only ON threshold_decisions;

DROP TRIGGER IF EXISTS pt_append_only ON programme_triages;

DROP FUNCTION IF EXISTS ex7_append_only();

DELETE FROM delivery_exceptions WHERE threshold_kind IS NOT NULL AND opened_value = 'threshold';

ALTER TABLE delivery_exceptions DROP CONSTRAINT dex_values;

ALTER TABLE delivery_exceptions ADD CONSTRAINT dex_values CHECK (opened_value IN ('amber_red','red') AND worst_value IN ('amber_red','red'));

ALTER TABLE delivery_exceptions DROP COLUMN threshold_kind;

DROP TABLE "threshold_decisions";

DROP TABLE "programme_thresholds";

DROP TABLE "programme_triages";
