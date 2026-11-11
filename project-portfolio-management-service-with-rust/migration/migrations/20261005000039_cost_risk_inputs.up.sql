-- m20261005_000039_cost_risk_inputs: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: **cost-risk inputs**. A budget line may carry a low and a high estimate of its final cost (its likely one is `planned_minor`); a risk may carry the cost it adds in a named currency if it happens (its probability is the same `schedule_probability_percent` the schedule simulation uses, so one event strikes once and drives both). All optional; none changes any existing behaviour.

ALTER TABLE budget_lines ADD COLUMN low_minor bigint;

ALTER TABLE budget_lines ADD COLUMN high_minor bigint;

ALTER TABLE budget_lines ADD CONSTRAINT budget_lines_estimate_order CHECK ((low_minor IS NULL OR (low_minor >= 0 AND low_minor <= planned_minor)) AND (high_minor IS NULL OR high_minor >= planned_minor));

ALTER TABLE risks ADD COLUMN cost_impact_minor bigint;

ALTER TABLE risks ADD COLUMN cost_currency text;

ALTER TABLE risks ADD CONSTRAINT risks_cost_impact CHECK ((cost_impact_minor IS NULL AND cost_currency IS NULL) OR (cost_impact_minor > 0 AND cost_currency IS NOT NULL));
