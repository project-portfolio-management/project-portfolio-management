-- m20260920_000001_report_group_by: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: **`report_definitions.group_by`** — the one field a saved
-- report groups by (T-28e). Nullable: `NULL` keeps today's ungrouped
-- behaviour, so every existing definition runs exactly as before.

ALTER TABLE report_definitions ADD COLUMN IF NOT EXISTS group_by TEXT;
