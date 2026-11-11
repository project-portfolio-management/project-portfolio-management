-- m20261003_000033_earned_value: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

ALTER TABLE tasks DROP CONSTRAINT tasks_percent_manual;

ALTER TABLE tasks DROP CONSTRAINT tasks_percent_method;

ALTER TABLE tasks DROP COLUMN percent_note;

ALTER TABLE tasks DROP COLUMN percent_manual;

ALTER TABLE tasks DROP COLUMN percent_method;

DROP TABLE "earned_value_settings";
