-- m20260719_000002_insight_columns: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: the moderate-fit executive-insight columns —
-- stage-gated funding tranches on `budget_lines` (`gate` +
-- `released_at`), the risk `category` (technical-debt register), and
-- the milestone completion timestamp `done_at` (delivery-flow
-- metrics). All nullable, so existing rows keep their behaviour:
-- an ungated line is never held, an uncategorized risk reads as
-- `delivery`, and a milestone completed before this migration counts
-- but is not timed.

ALTER TABLE budget_lines ADD COLUMN IF NOT EXISTS gate VARCHAR NULL;
                 ALTER TABLE budget_lines ADD COLUMN IF NOT EXISTS released_at TIMESTAMPTZ NULL;
                 ALTER TABLE risks ADD COLUMN IF NOT EXISTS category VARCHAR NULL;
                 ALTER TABLE milestones ADD COLUMN IF NOT EXISTS done_at TIMESTAMPTZ NULL;
