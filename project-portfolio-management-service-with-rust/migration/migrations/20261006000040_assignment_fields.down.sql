-- m20261006_000040_assignment_fields: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

ALTER TABLE tasks DROP CONSTRAINT tasks_contour;

ALTER TABLE tasks DROP CONSTRAINT tasks_scheduling_rule;

ALTER TABLE tasks DROP CONSTRAINT tasks_units_range;

ALTER TABLE tasks DROP CONSTRAINT tasks_work_range;

ALTER TABLE tasks DROP COLUMN contour_custom;

ALTER TABLE tasks DROP COLUMN contour;

ALTER TABLE tasks DROP COLUMN scheduling_rule;

ALTER TABLE tasks DROP COLUMN units_percent;

ALTER TABLE tasks DROP COLUMN work_minutes;
