-- m20261105_000104_translated_labels: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: **translated labels** (OC-6). An organization's own issue types, roles and statuses can carry a label per
-- language. A language with none falls back to the one label each already has.

ALTER TABLE issue_types
                     ADD COLUMN IF NOT EXISTS labels JSONB NOT NULL DEFAULT '{}'::jsonb;
                 ALTER TABLE organization_roles
                     ADD COLUMN IF NOT EXISTS labels JSONB NOT NULL DEFAULT '{}'::jsonb;
                 ALTER TABLE workflow_states
                     ADD COLUMN IF NOT EXISTS labels JSONB NOT NULL DEFAULT '{}'::jsonb;
