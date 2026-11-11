-- m20261025_000098_lesson_links: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: **`lessons.link_kind` and `lessons.link_pid`** (BR-4). A lesson may point at one record of its plan, so
-- the plan can ask which meetings and lessons mention a task, risk, issue or milestone.

ALTER TABLE lessons ADD COLUMN IF NOT EXISTS link_kind TEXT, ADD COLUMN IF NOT EXISTS link_pid UUID;

CREATE INDEX IF NOT EXISTS idx_lessons_link ON lessons (link_pid) WHERE link_pid IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_meeting_items_link ON meeting_items (link_pid) WHERE link_pid IS NOT NULL;
