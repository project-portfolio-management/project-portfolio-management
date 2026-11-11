-- m20260719_000002_insight_columns: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

ALTER TABLE budget_lines DROP COLUMN IF EXISTS gate;
                 ALTER TABLE budget_lines DROP COLUMN IF EXISTS released_at;
                 ALTER TABLE risks DROP COLUMN IF EXISTS category;
                 ALTER TABLE milestones DROP COLUMN IF EXISTS done_at;
