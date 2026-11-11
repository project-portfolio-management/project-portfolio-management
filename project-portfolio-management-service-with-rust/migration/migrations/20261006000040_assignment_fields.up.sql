-- m20261006_000040_assignment_fields: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: **assignment fields on tasks** (PJ-5, PJ-6). A task with an assignee may carry the three quantities scheduling tools relate — `duration_days` (already there), `units_percent` (how much of the assignee's day) and `work_minutes` — with a `scheduling_rule` saying which of the three stays fixed when another is edited, and a `contour` saying how its work is spread over its business days (`custom` carries its own weights). All optional; every existing task is unchanged.

ALTER TABLE tasks ADD COLUMN work_minutes bigint;

ALTER TABLE tasks ADD COLUMN units_percent integer;

ALTER TABLE tasks ADD COLUMN scheduling_rule text;

ALTER TABLE tasks ADD COLUMN contour text NOT NULL DEFAULT 'flat';

ALTER TABLE tasks ADD COLUMN contour_custom jsonb;

ALTER TABLE tasks ADD CONSTRAINT tasks_work_range CHECK (work_minutes IS NULL OR (work_minutes >= 0 AND work_minutes <= 17520000));

ALTER TABLE tasks ADD CONSTRAINT tasks_units_range CHECK (units_percent IS NULL OR (units_percent >= 1 AND units_percent <= 1000));

ALTER TABLE tasks ADD CONSTRAINT tasks_scheduling_rule CHECK (scheduling_rule IS NULL OR scheduling_rule IN ('fixed_units','fixed_duration','fixed_work'));

ALTER TABLE tasks ADD CONSTRAINT tasks_contour CHECK (contour IN ('flat','front_loaded','back_loaded','bell','turtle','custom'));
