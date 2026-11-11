-- m20261105_000105_plan_template_source: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: **the plan a template came from** (OC-20). A snapshot carries a plan's tasks and dates, so a template of a
-- private plan is hidden with that plan. No foreign key: when a plan is purged its template stays hidden from whoever the
-- access row still names, and the template outlives the plan, as it always has. Rows saved before this have no source, which
-- cannot be worked out afterwards, so they stay as visible as they were.

ALTER TABLE plan_templates ADD COLUMN IF NOT EXISTS source_plan_pid UUID;
