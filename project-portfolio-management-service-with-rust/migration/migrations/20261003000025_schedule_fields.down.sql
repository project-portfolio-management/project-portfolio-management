-- m20261003_000025_schedule_fields: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

ALTER TABLE plan_dependencies DROP CONSTRAINT plan_dependencies_kind;

ALTER TABLE plan_dependencies DROP COLUMN kind;

ALTER TABLE tasks DROP CONSTRAINT tasks_duration_days;

ALTER TABLE tasks DROP COLUMN duration_days;

ALTER TABLE tasks DROP COLUMN start_date;
