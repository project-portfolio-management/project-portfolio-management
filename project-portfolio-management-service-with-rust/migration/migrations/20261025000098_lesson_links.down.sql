-- m20261025_000098_lesson_links: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

DROP INDEX IF EXISTS idx_meeting_items_link;

DROP INDEX IF EXISTS idx_lessons_link;

ALTER TABLE lessons DROP COLUMN IF EXISTS link_pid, DROP COLUMN IF EXISTS link_kind;
