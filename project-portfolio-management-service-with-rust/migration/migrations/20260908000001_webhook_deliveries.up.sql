-- m20260908_000001_webhook_deliveries: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: create the `webhook_deliveries` table — the delivery log
-- for the outbound webhook sink (`agents/share/event-bus.md` §12, repo
-- `tasks.md` EV-3, this crate's own T-28m).
--
-- One row per **delivery attempt-set** (a target URL's full
-- retry-with-backoff sequence for one outbox event), written once the
-- sequence reaches its final outcome — not one row per HTTP attempt.
-- `attempts` is how many were made. This is the operator-facing record
-- of what was (or was not) delivered; it does not gate outbox
-- progression (see [`crate::webhooks`] — webhook delivery is best-effort
-- and never re-blocks the durable-bus row it fanned out from).

CREATE TABLE IF NOT EXISTS webhook_deliveries (
                     created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     id SERIAL PRIMARY KEY,
                     -- The envelope's event_id (dedup key), so a delivery
                     -- log row is traceable to its event_outbox row.
                     event_id UUID NOT NULL,
                     -- Entity name, e.g. "plan".
                     entity VARCHAR NOT NULL,
                     -- created / updated / deleted / merged.
                     kind VARCHAR NOT NULL,
                     -- The configured target URL this row is about.
                     url VARCHAR NOT NULL,
                     -- How many HTTP attempts this delivery took.
                     attempts INTEGER NOT NULL,
                     -- 'delivered' | 'failed' (final outcome only).
                     status VARCHAR NOT NULL,
                     -- The last HTTP status received, if any response came
                     -- back at all (NULL on a pure transport failure).
                     status_code INTEGER NULL,
                     -- The last error message, if the final attempt failed.
                     error VARCHAR NULL
                 );
                 CREATE INDEX IF NOT EXISTS webhook_deliveries_event
                     ON webhook_deliveries (event_id);
