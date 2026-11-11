-- m20261105_000106_issue_times: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: **times on an issue** (SM-2). An issue had only the dates it was raised and resolved, which no service level can
-- be measured from. Existing issues keep their dates and show no time: nothing is back-filled by guessing, so the default on
-- `reported_at` is set after the column is added and reaches new rows only.

ALTER TABLE issues
                     ADD COLUMN IF NOT EXISTS reported_at TIMESTAMPTZ,
                     ADD COLUMN IF NOT EXISTS acknowledged_at TIMESTAMPTZ,
                     ADD COLUMN IF NOT EXISTS resolved_at TIMESTAMPTZ,
                     ADD COLUMN IF NOT EXISTS closed_at TIMESTAMPTZ,
                     ADD COLUMN IF NOT EXISTS occurred_at TIMESTAMPTZ,
                     ADD COLUMN IF NOT EXISTS detected_at TIMESTAMPTZ;
                 ALTER TABLE issues ALTER COLUMN reported_at SET DEFAULT now();
                 ALTER TABLE issues DROP CONSTRAINT IF EXISTS issues_occurred_before_detected;
                 ALTER TABLE issues ADD CONSTRAINT issues_occurred_before_detected
                     CHECK (occurred_at IS NULL OR detected_at IS NULL OR occurred_at <= detected_at);
