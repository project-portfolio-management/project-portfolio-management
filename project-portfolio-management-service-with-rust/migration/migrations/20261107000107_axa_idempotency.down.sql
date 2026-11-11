-- m20261107_000107_axa_idempotency: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

DROP TABLE IF EXISTS axa_idempotency;

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
