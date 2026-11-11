-- m20261105_000103_plan_issue_types: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: **per-plan issue types** (OC-5). A plan can limit itself to some of the deployment's issue types, as a
-- Redmine project selects its trackers. No rows means every type, so nothing changes until a plan chooses.

CREATE TABLE IF NOT EXISTS plan_issue_types (
                     created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     id SERIAL PRIMARY KEY,
                     pid UUID NOT NULL UNIQUE DEFAULT gen_random_uuid(),
                     plan_pid UUID NOT NULL REFERENCES plans (pid) ON DELETE CASCADE,
                     type_key VARCHAR NOT NULL REFERENCES issue_types (key),
                     UNIQUE (plan_pid, type_key)
                 );
