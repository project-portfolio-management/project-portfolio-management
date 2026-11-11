-- m20261005_000039_cost_risk_inputs: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

ALTER TABLE risks DROP CONSTRAINT risks_cost_impact;

ALTER TABLE risks DROP COLUMN cost_currency;

ALTER TABLE risks DROP COLUMN cost_impact_minor;

ALTER TABLE budget_lines DROP CONSTRAINT budget_lines_estimate_order;

ALTER TABLE budget_lines DROP COLUMN high_minor;

ALTER TABLE budget_lines DROP COLUMN low_minor;
