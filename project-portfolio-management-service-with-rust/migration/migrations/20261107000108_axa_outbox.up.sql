-- m20261107_000108_axa_outbox: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: the durable event bus's outbox moves to **`axa_outbox`**, the table `axa-outbox` 0.1 relayed from, made
-- by that release's own setup statements, kept here as [`V01_SETUP`] so that this migration makes the same table
-- whichever `axa` the service builds with (0.2 makes other tables; migration 109 moves the rows into them). Every `event_outbox` row moves across in its order:
-- an unpublished one as `pending`, so the new relay sends it, and a published one as `sent`, so a webhook redelivery
-- can still find it until the sweep. The old table is left empty, and is dropped in a later release, once no
-- binary that writes it is running.

CREATE TABLE IF NOT EXISTS axa_outbox (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    event_id UUID NOT NULL UNIQUE,
    topic TEXT NOT NULL,
    kind TEXT NOT NULL,
    key TEXT NOT NULL,
    payload JSONB NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    attempts INTEGER NOT NULL DEFAULT 0,
    error TEXT,
    run_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    sent_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS axa_outbox_ready ON axa_outbox (id) WHERE status = 'pending';

CREATE INDEX IF NOT EXISTS axa_outbox_owed
    ON axa_outbox (topic, key, id) WHERE status <> 'sent';

CREATE INDEX IF NOT EXISTS axa_outbox_claimed
    ON axa_outbox (updated_at) WHERE status = 'claimed';

CREATE INDEX IF NOT EXISTS axa_outbox_sent ON axa_outbox (sent_at) WHERE status = 'sent';

INSERT INTO axa_outbox (event_id, topic, kind, key, payload, status, sent_at, created_at, updated_at)
     SELECT event_id, entity, kind, entity_pid::text, payload,
            CASE WHEN published_at IS NULL THEN 'pending' ELSE 'sent' END,
            published_at, occurred_at, updated_at
     FROM event_outbox
     ORDER BY id
     ON CONFLICT (event_id) DO NOTHING;
     DELETE FROM event_outbox;
