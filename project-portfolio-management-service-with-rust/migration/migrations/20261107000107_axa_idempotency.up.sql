-- m20261107_000107_axa_idempotency: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: idempotency keys move to **`axa_idempotency`** (BR-1, NFR-7), the table `axa::idempotency` keeps them
-- in, made by its own [`SETUP_STATEMENTS`](axa::idempotency::SETUP_STATEMENTS). The old `idempotency_keys` table is
-- dropped: its rows live a day, and a retry across the deploy runs as a first request.

CREATE TABLE IF NOT EXISTS axa_idempotency (
    caller TEXT NOT NULL,
    key TEXT NOT NULL,
    fingerprint TEXT NOT NULL,
    claim UUID NOT NULL,
    state TEXT NOT NULL DEFAULT 'running' CHECK (state IN ('running', 'done', 'not_kept')),
    status INTEGER,
    headers JSONB,
    body BYTEA,
    lease_until TIMESTAMPTZ NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (caller, key)
);

CREATE INDEX IF NOT EXISTS axa_idempotency_expiry ON axa_idempotency (expires_at);

DROP TABLE IF EXISTS idempotency_keys;
