-- m20261010_000075_cutover_decisions: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

DROP TRIGGER IF EXISTS cutover_cosign_guard ON cutover_cosignatures;

DROP TRIGGER IF EXISTS cutover_decision_guard ON cutover_decisions;

DROP TRIGGER IF EXISTS cutover_criteria_guard ON cutover_criteria;

DROP FUNCTION IF EXISTS cutover_cosign_guard();

DROP FUNCTION IF EXISTS cutover_decision_guard();

DROP FUNCTION IF EXISTS cutover_criteria_guard();

DROP TABLE "cutover_cosignatures";

DROP TABLE "cutover_criteria";

DROP TABLE "cutover_decisions";
