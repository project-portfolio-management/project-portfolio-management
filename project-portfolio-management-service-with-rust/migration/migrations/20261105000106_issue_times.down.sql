-- m20261105_000106_issue_times: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

ALTER TABLE issues
                     DROP CONSTRAINT IF EXISTS issues_occurred_before_detected,
                     DROP COLUMN reported_at,
                     DROP COLUMN acknowledged_at,
                     DROP COLUMN resolved_at,
                     DROP COLUMN closed_at,
                     DROP COLUMN occurred_at,
                     DROP COLUMN detected_at;
