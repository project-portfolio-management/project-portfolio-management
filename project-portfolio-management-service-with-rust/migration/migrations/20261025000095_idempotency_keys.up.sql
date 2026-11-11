-- m20261025_000095_idempotency_keys: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: **`idempotency_keys`** (BR-1). One row per `Idempotency-Key` a caller sent on a `POST`: the hash of the
-- request, and once it succeeded the stored answer, so a retry returns the first answer and a different request under
-- the same key is refused. Short-lived by design (retention class `GEN`, one day).

CREATE TABLE IF NOT EXISTS idempotency_keys (
                     created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     id BIGSERIAL PRIMARY KEY,
                     caller VARCHAR NOT NULL,
                     key VARCHAR(200) NOT NULL,
                     method VARCHAR NOT NULL,
                     path VARCHAR NOT NULL,
                     request_hash VARCHAR NOT NULL,
                     state VARCHAR NOT NULL DEFAULT 'in_progress',
                     status INTEGER NULL,
                     content_type VARCHAR NULL,
                     body BYTEA NULL,
                     CONSTRAINT idempotency_keys_state CHECK (state IN ('in_progress', 'done')),
                     CONSTRAINT idempotency_keys_caller_key UNIQUE (caller, key)
                 );
                 CREATE INDEX IF NOT EXISTS idempotency_keys_created_idx ON idempotency_keys (created_at);
