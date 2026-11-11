-- m20261025_000090_lessons: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: **`lessons`** (NC-8). The lessons-learned register: what a plan learned, recorded at a phase gate or
-- at closure, so the next similar proposal can be shown them. Append-only: a lesson is never edited, only
-- withdrawn (soft-deleted), so the audit trail and what a past reader saw stay true.

CREATE TABLE IF NOT EXISTS lessons (
                     created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     id BIGSERIAL PRIMARY KEY,
                     pid UUID NOT NULL UNIQUE,
                     plan_pid UUID NOT NULL,
                     occasion VARCHAR NOT NULL,
                     category VARCHAR NOT NULL,
                     summary VARCHAR NOT NULL,
                     detail TEXT NULL,
                     recorded_by VARCHAR NULL,
                     deleted_at TIMESTAMPTZ NULL,
                     CONSTRAINT lessons_category CHECK (category IN ('went_well', 'went_badly', 'recommendation'))
                 );
                 CREATE INDEX IF NOT EXISTS lessons_plan_idx ON lessons (plan_pid) WHERE deleted_at IS NULL;
