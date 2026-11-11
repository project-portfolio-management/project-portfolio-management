-- m20261004_000036_schedule_risk_inputs: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

ALTER TABLE risks DROP CONSTRAINT risks_schedule_range;

ALTER TABLE risks DROP COLUMN schedule_probability_percent;

ALTER TABLE risks DROP COLUMN schedule_impact_days;

ALTER TABLE risks DROP COLUMN task_pid;

ALTER TABLE tasks DROP CONSTRAINT tasks_estimate_range;

ALTER TABLE tasks DROP COLUMN duration_pessimistic_days;

ALTER TABLE tasks DROP COLUMN duration_optimistic_days;
