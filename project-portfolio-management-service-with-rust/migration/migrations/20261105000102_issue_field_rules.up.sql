-- m20261105_000102_issue_field_rules: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: **per-status field rules** (OC-4). A workflow state can make an issue's field required or read-only, for
-- everyone or for named roles, as Redmine's workflow field permissions do. Nothing changes until a workflow declares one.

ALTER TABLE workflow_states
                     ADD COLUMN IF NOT EXISTS field_rules JSONB NOT NULL DEFAULT '[]'::jsonb;
                 ALTER TABLE workflow_states DROP CONSTRAINT IF EXISTS workflow_states_field_rules_array;
                 ALTER TABLE workflow_states ADD CONSTRAINT workflow_states_field_rules_array
                     CHECK (jsonb_typeof(field_rules) = 'array');
