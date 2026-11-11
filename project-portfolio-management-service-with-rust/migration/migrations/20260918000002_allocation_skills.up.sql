-- m20260918_000002_allocation_skills: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: **`allocations.skills_required`** — the short tags an
-- allocation declares it needs (T-28c). A JSON array of short
-- strings, matching the family's `tags`/`must_include` convention
-- (`m20220101_000007_strategy.rs`) rather than a Postgres native
-- array.
--
-- ## Why no skill *data* lands here
--
-- Only the **requirement** tags are stored. Whether the assigned
-- person actually holds a tag is resolved live, on read, against the
-- worker service by `EntityRef` (`src/workers_client.rs`) — never
-- copied into this row. People stay references (family doctrine); a
-- stored "has skill X" flag would drift the moment the worker
-- service's own record changed.

ALTER TABLE allocations
                     ADD COLUMN IF NOT EXISTS skills_required JSONB NOT NULL DEFAULT '[]';
