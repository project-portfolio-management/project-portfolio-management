-- m20261003_000026_work_calendars: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

ALTER TABLE tasks DROP CONSTRAINT tasks_constraint_pair;

ALTER TABLE tasks DROP CONSTRAINT tasks_constraint_kind;

ALTER TABLE tasks DROP COLUMN deadline;

ALTER TABLE tasks DROP COLUMN constraint_date;

ALTER TABLE tasks DROP COLUMN constraint_kind;

DROP TABLE "work_calendars";
