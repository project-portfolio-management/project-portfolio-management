-- m20261020_000082_plan_tolerances: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: **plan tolerances**. How far a project may drift on one dimension (time, cost, risk, resources) before
-- it must be referred upward, and how much of that limit may be used before the colour turns amber. A plan with no
-- row for a dimension inherits its nearest ancestor's, resolved on read and never stored.

CREATE TABLE IF NOT EXISTS plan_tolerances (
                     created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     id BIGSERIAL PRIMARY KEY,
                     pid UUID NOT NULL UNIQUE,
                     plan_pid UUID NOT NULL,
                     dimension VARCHAR NOT NULL,
                     limit_value BIGINT NOT NULL,
                     amber_from_percent INTEGER NOT NULL DEFAULT 75,
                     note TEXT NULL,
                     set_by VARCHAR NULL,
                     CONSTRAINT plan_tolerances_one_per_dimension UNIQUE (plan_pid, dimension),
                     CONSTRAINT plan_tolerances_dimension CHECK (dimension IN ('time','cost','risk','resources')),
                     CONSTRAINT plan_tolerances_amber CHECK (amber_from_percent BETWEEN 1 AND 99),
                     CONSTRAINT plan_tolerances_limit CHECK (limit_value >= 0 AND limit_value <= 100000
                         AND (dimension <> 'risk' OR limit_value BETWEEN 1 AND 25))
                 );
