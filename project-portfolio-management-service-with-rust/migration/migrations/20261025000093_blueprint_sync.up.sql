-- m20261025_000093_blueprint_sync: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: **blueprint sync** (NC-12). A template gains a `version`; every plan made from one records the
-- template, its place in the template's tree, the version it was last brought up to, a `baseline` of the template's
-- tasks for that plan, and which of the plan's tasks came from which template task.

ALTER TABLE plan_templates ADD COLUMN IF NOT EXISTS version INTEGER NOT NULL DEFAULT 1;
                 CREATE TABLE IF NOT EXISTS plan_template_links (
                     created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     id BIGSERIAL PRIMARY KEY,
                     plan_pid UUID NOT NULL UNIQUE,
                     template_pid UUID NOT NULL,
                     node_key INTEGER NOT NULL,
                     template_version INTEGER NOT NULL,
                     baseline JSONB NOT NULL,
                     task_map JSONB NOT NULL,
                     CONSTRAINT plan_template_links_version CHECK (template_version >= 1)
                 );
                 CREATE INDEX IF NOT EXISTS plan_template_links_template_idx ON plan_template_links (template_pid);
