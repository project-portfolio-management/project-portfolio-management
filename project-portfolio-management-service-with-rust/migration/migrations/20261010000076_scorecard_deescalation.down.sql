-- m20261010_000076_scorecard_deescalation: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

DROP TRIGGER IF EXISTS ce_append_only ON criterion_evidence;

DROP TRIGGER IF EXISTS el_append_only ON escalation_levels;

DROP FUNCTION IF EXISTS ex10_append_only();

DROP TABLE criterion_evidence;

DROP TABLE "deescalation_criteria";

DROP TABLE "escalation_levels";

DROP TABLE "scorecard_kpis";

DROP TABLE "scorecards";
