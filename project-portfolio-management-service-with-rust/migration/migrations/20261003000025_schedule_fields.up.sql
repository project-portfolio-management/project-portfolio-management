-- m20261003_000025_schedule_fields: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: **schedule fields** (PJ-1, SC-1). A task gains an optional planned
-- `start_date` and a business-day `duration_days`, so it can be a node in the
-- critical-path network beside plans; a dependency gains a relationship `kind`
-- (`fs` default, so every existing edge means what it meant). Both are
-- nullable or defaulted: no existing row changes meaning.

ALTER TABLE tasks ADD COLUMN start_date date;

ALTER TABLE tasks ADD COLUMN duration_days integer;

ALTER TABLE tasks ADD CONSTRAINT tasks_duration_days CHECK (duration_days IS NULL OR (duration_days >= 0 AND duration_days <= 3650));

ALTER TABLE plan_dependencies ADD COLUMN kind text NOT NULL DEFAULT 'fs';

ALTER TABLE plan_dependencies ADD CONSTRAINT plan_dependencies_kind CHECK (kind IN ('fs','ss','ff','sf'));
