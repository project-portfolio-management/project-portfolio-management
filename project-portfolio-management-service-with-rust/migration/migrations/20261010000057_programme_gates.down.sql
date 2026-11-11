-- m20261010_000057_programme_gates: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

DROP TRIGGER IF EXISTS psd_guard ON programme_status_decisions;

DROP FUNCTION IF EXISTS psd_guard();

DROP TABLE "programme_status_decisions";

DROP TABLE "planned_gates";
