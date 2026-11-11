-- m20261019_000078_blocked_reason: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: **why a task was blocked**. An optional reason on the transition into `blocked`, from a fixed
-- vocabulary, so blocked time can be reported by reason (`spec/time-based-analysis.md` §17). The log stays
-- append-only; a transition recorded before this migration, or without a reason, simply has none.

ALTER TABLE task_transitions ADD COLUMN IF NOT EXISTS blocked_reason VARCHAR NULL;
                 ALTER TABLE task_transitions DROP CONSTRAINT IF EXISTS task_transitions_blocked_reason_check;
                 ALTER TABLE task_transitions ADD CONSTRAINT task_transitions_blocked_reason_check
                     CHECK (blocked_reason IS NULL OR (to_status = 'blocked' AND blocked_reason IN
                         ('waiting_on_person', 'external_dependency', 'awaiting_decision',
                          'awaiting_resource', 'other')));
