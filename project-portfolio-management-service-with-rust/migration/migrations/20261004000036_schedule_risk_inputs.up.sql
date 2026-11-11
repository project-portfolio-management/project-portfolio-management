-- m20261004_000036_schedule_risk_inputs: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: **schedule-risk inputs** (SC-8). A task may carry an optimistic and a pessimistic duration (its likely one is `duration_days`); a risk may name the task it threatens, how many business days it adds if it happens, and its probability as a percent. All optional; none changes any existing behaviour.

ALTER TABLE tasks ADD COLUMN duration_optimistic_days integer;

ALTER TABLE tasks ADD COLUMN duration_pessimistic_days integer;

ALTER TABLE tasks ADD CONSTRAINT tasks_estimate_range CHECK ((duration_optimistic_days IS NULL OR (duration_optimistic_days >= 0 AND duration_optimistic_days <= 3650)) AND (duration_pessimistic_days IS NULL OR (duration_pessimistic_days >= 0 AND duration_pessimistic_days <= 3650)));

ALTER TABLE risks ADD COLUMN task_pid uuid;

ALTER TABLE risks ADD COLUMN schedule_impact_days integer;

ALTER TABLE risks ADD COLUMN schedule_probability_percent integer;

ALTER TABLE risks ADD CONSTRAINT risks_schedule_range CHECK ((schedule_impact_days IS NULL OR (schedule_impact_days >= 1 AND schedule_impact_days <= 365)) AND (schedule_probability_percent IS NULL OR (schedule_probability_percent >= 0 AND schedule_probability_percent <= 100)));
