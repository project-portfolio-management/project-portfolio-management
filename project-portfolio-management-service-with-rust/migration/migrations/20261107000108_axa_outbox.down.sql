-- m20261107_000108_axa_outbox: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

INSERT INTO event_outbox (event_id, entity, entity_pid, kind, occurred_at, actor,
            schema_version, payload, published_at, created_at, updated_at)
     SELECT event_id, topic, key::uuid, kind, created_at, payload->>'actor',
            COALESCE((payload->>'schema_version')::int, 1), payload,
            CASE WHEN status = 'sent' THEN sent_at END, created_at, updated_at
     FROM axa_outbox
     ORDER BY id
     ON CONFLICT (event_id) DO NOTHING;

DROP TABLE IF EXISTS axa_outbox;
