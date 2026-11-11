-- m20261025_000089_scoring_inputs: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: **`scoring_inputs`** (NC-6). The estimated numbers behind two prioritization models beside the Smart
-- Score: WSJF (business value, time criticality, risk reduction, job size) and RICE (reach, impact, confidence,
-- effort). One row per plan; every column is optional, because an input nobody has estimated is absent, not zero,
-- and the score says which it is waiting for. The score itself is never stored.

CREATE TABLE IF NOT EXISTS scoring_inputs (
                     created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     id BIGSERIAL PRIMARY KEY,
                     plan_pid UUID NOT NULL UNIQUE,
                     business_value SMALLINT NULL,
                     time_criticality SMALLINT NULL,
                     risk_reduction SMALLINT NULL,
                     job_size SMALLINT NULL,
                     reach INTEGER NULL,
                     impact DOUBLE PRECISION NULL,
                     confidence_percent SMALLINT NULL,
                     effort DOUBLE PRECISION NULL,
                     set_by VARCHAR NULL,
                     CONSTRAINT scoring_inputs_wsjf CHECK (
                         (business_value IS NULL OR business_value BETWEEN 1 AND 20)
                         AND (time_criticality IS NULL OR time_criticality BETWEEN 1 AND 20)
                         AND (risk_reduction IS NULL OR risk_reduction BETWEEN 1 AND 20)
                         AND (job_size IS NULL OR job_size BETWEEN 1 AND 20)),
                     CONSTRAINT scoring_inputs_rice CHECK (
                         (reach IS NULL OR reach >= 0)
                         AND (impact IS NULL OR impact IN (0.25, 0.5, 1, 2, 3))
                         AND (confidence_percent IS NULL OR confidence_percent BETWEEN 0 AND 100)
                         AND (effort IS NULL OR effort > 0))
                 );
