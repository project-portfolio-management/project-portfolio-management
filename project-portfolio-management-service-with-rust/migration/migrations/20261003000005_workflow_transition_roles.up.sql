-- m20261003_000005_workflow_transition_roles: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: **`workflow_transitions.roles`** — who may make a move (RM-2).
-- A JSON array of role names; **empty means anyone**, preserving T-15's
-- opt-in constraint, so every existing workflow behaves exactly as before.

ALTER TABLE workflow_transitions ADD COLUMN IF NOT EXISTS roles JSONB NOT NULL DEFAULT '[]';
