-- m20261109_000109_axa_outbox_record: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: the outbox becomes **a record** (`axa-outbox` 0.2). An event and everything that happens to it are only
-- ever inserted, into `axa_outbox_events`, `axa_outbox_payloads` and `axa_outbox_attempts`, whose triggers refuse
-- `UPDATE`, `DELETE` and `TRUNCATE`. [`SETUP_STATEMENTS`](axa::outbox::SETUP_STATEMENTS) make them, and
-- [`IMPORT_STATEMENTS`](axa::outbox::IMPORT_STATEMENTS) copy every `axa_outbox` row across in its order, with one
-- attempt for its state: `delivered` for `sent`, `failed` for `failed`, and `imported` for one still owed, which the
-- relay then sends. Both are safe to repeat.
--
-- `axa-outbox` 0.1 named an index `axa_outbox_owed`, the name 0.2 gives a table. `CREATE TABLE IF NOT EXISTS` then
-- sees the name taken, makes nothing, and the next statement fails on a column the index does not have. So
-- [`FREE_THE_NAME`] first renames that index, if it is one, to `axa_outbox_v01_owed`; it still serves `axa_outbox`.
--
-- Stop every relay before this runs. `axa_outbox` is left as it was, and is dropped in a later release, once no
-- binary that writes it is running.

DO $$ BEGIN
    IF EXISTS (SELECT 1 FROM pg_class WHERE relname = 'axa_outbox_owed' AND relkind = 'i') THEN
        ALTER INDEX axa_outbox_owed RENAME TO axa_outbox_v01_owed;
    END IF;
END $$;

CREATE TABLE IF NOT EXISTS axa_outbox_events (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    event_id UUID NOT NULL UNIQUE,
    topic TEXT NOT NULL,
    kind TEXT NOT NULL,
    key TEXT NOT NULL,
    fingerprint TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS axa_outbox_payloads (
    event_id UUID PRIMARY KEY REFERENCES axa_outbox_events (event_id),
    payload JSONB,
    sealed BYTEA,
    CHECK (payload IS NULL OR sealed IS NULL)
);

CREATE TABLE IF NOT EXISTS axa_outbox_attempts (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    event_id UUID NOT NULL REFERENCES axa_outbox_events (event_id),
    outcome TEXT NOT NULL CHECK (outcome IN ('claimed', 'returned', 'delivered', 'refused',
        'rejected', 'failed', 'lost', 'abandoned', 'released', 'erased', 'imported')),
    relay UUID NOT NULL,
    claim UUID NOT NULL,
    reason TEXT CHECK (length(reason) <= 200 AND reason !~ '[[:cntrl:]]'),
    by TEXT CHECK (length(by) <= 200 AND by !~ '[[:cntrl:]]'),
    retry_at TIMESTAMPTZ,
    at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS axa_outbox_owed (
    event_id UUID PRIMARY KEY REFERENCES axa_outbox_events (event_id),
    id BIGINT NOT NULL,
    topic TEXT NOT NULL,
    key TEXT NOT NULL,
    state TEXT NOT NULL DEFAULT 'pending' CHECK (state IN ('pending', 'claimed', 'failed')),
    claim UUID,
    charges INTEGER NOT NULL DEFAULT 0,
    reason TEXT,
    retry_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    held_since TIMESTAMPTZ,
    recheck_at TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS axa_outbox_leases (
    claim UUID PRIMARY KEY,
    relay UUID NOT NULL,
    heard_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS axa_outbox_owed_ready
    ON axa_outbox_owed (id) WHERE state = 'pending';

CREATE INDEX IF NOT EXISTS axa_outbox_owed_line
    ON axa_outbox_owed (topic, key, id) WHERE state <> 'failed';

CREATE INDEX IF NOT EXISTS axa_outbox_owed_claimed
    ON axa_outbox_owed (claim) WHERE state = 'claimed';

CREATE INDEX IF NOT EXISTS axa_outbox_attempts_event
    ON axa_outbox_attempts (event_id, id);

CREATE INDEX IF NOT EXISTS axa_outbox_attempts_recent
    ON axa_outbox_attempts (outcome, at) WHERE outcome IN ('delivered', 'refused');

CREATE OR REPLACE FUNCTION axa_outbox_fingerprint(
    event_id UUID, topic TEXT, kind TEXT, key TEXT, payload JSONB, sealed BYTEA)
RETURNS TEXT LANGUAGE sql IMMUTABLE PARALLEL SAFE AS $$
    SELECT encode(sha256(
        uuid_send(event_id)
        || convert_to(topic, 'UTF8') || '\x00'::bytea
        || convert_to(kind, 'UTF8') || '\x00'::bytea
        || convert_to(key, 'UTF8') || '\x00'::bytea
        || CASE WHEN sealed IS NOT NULL THEN '\x73'::bytea || sealed
                ELSE '\x6a'::bytea || convert_to(payload::text, 'UTF8') END
    ), 'hex')
$$;

CREATE OR REPLACE FUNCTION axa_outbox_refuse() RETURNS trigger
LANGUAGE plpgsql AS $$
BEGIN
    RAISE EXCEPTION '% on % is refused: the outbox record is only ever added to',
        TG_OP, TG_TABLE_NAME;
END
$$;

CREATE OR REPLACE FUNCTION axa_outbox_payloads_guard() RETURNS trigger
LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'UPDATE'
        AND NEW.payload IS NULL AND NEW.sealed IS NULL
        AND (OLD.payload IS NOT NULL OR OLD.sealed IS NOT NULL)
        AND to_jsonb(NEW) - 'payload' - 'sealed' = to_jsonb(OLD) - 'payload' - 'sealed'
        AND EXISTS (
            SELECT 1 FROM axa_outbox_attempts erased
            WHERE erased.event_id = OLD.event_id AND erased.outcome = 'erased'
        )
    THEN
        RETURN NEW;
    END IF;
    RAISE EXCEPTION '% on axa_outbox_payloads is refused: a payload is only blanked, once erased',
        TG_OP;
END
$$;

CREATE OR REPLACE FUNCTION axa_outbox_owed_guard() RETURNS trigger
LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'TRUNCATE' THEN
        RAISE EXCEPTION 'TRUNCATE on axa_outbox_owed is refused: it is derived from the record';
    END IF;
    IF pg_trigger_depth() > 1 THEN
        RETURN coalesce(NEW, OLD);
    END IF;
    IF TG_OP = 'UPDATE'
        AND to_jsonb(NEW) - 'held_since' - 'recheck_at'
            = to_jsonb(OLD) - 'held_since' - 'recheck_at'
        AND (NEW.held_since IS NULL OR NEW.held_since <= now())
        AND (NEW.recheck_at IS NULL OR NEW.recheck_at <= now() + interval '1 day')
    THEN
        RETURN NEW;
    END IF;
    RAISE EXCEPTION '% on axa_outbox_owed is refused: it changes only as the record does',
        TG_OP;
END
$$;

CREATE OR REPLACE FUNCTION axa_outbox_leases_guard() RETURNS trigger
LANGUAGE plpgsql AS $$
BEGIN
    IF NEW.heard_at > now() THEN
        RAISE EXCEPTION 'a lease in axa_outbox_leases cannot be heard from in the future';
    END IF;
    IF TG_OP = 'UPDATE' AND (NEW.claim <> OLD.claim OR NEW.relay <> OLD.relay) THEN
        RAISE EXCEPTION 'a lease in axa_outbox_leases cannot change its claim or relay';
    END IF;
    RETURN NEW;
END
$$;

CREATE OR REPLACE FUNCTION axa_outbox_owe() RETURNS trigger
LANGUAGE plpgsql AS $$
BEGIN
    INSERT INTO axa_outbox_owed (event_id, id, topic, key)
    VALUES (NEW.event_id, NEW.id, NEW.topic, NEW.key);
    RETURN NULL;
END
$$;

CREATE OR REPLACE FUNCTION axa_outbox_follow() RETURNS trigger
LANGUAGE plpgsql AS $$
BEGIN
    IF NEW.outcome = 'claimed' THEN
        UPDATE axa_outbox_owed
        SET state = 'claimed', claim = NEW.claim, reason = NULL,
            held_since = NULL, recheck_at = NULL
        WHERE event_id = NEW.event_id;
    ELSIF NEW.outcome IN ('returned', 'imported') THEN
        UPDATE axa_outbox_owed
        SET state = 'pending', claim = NULL, reason = NEW.reason,
            retry_at = coalesce(NEW.retry_at, NEW.at)
        WHERE event_id = NEW.event_id;
    ELSIF NEW.outcome IN ('rejected', 'lost') THEN
        UPDATE axa_outbox_owed
        SET state = 'pending', claim = NULL, reason = NEW.reason, charges = charges + 1,
            retry_at = coalesce(NEW.retry_at, NEW.at)
        WHERE event_id = NEW.event_id;
    ELSIF NEW.outcome IN ('failed', 'abandoned') THEN
        UPDATE axa_outbox_owed
        SET state = 'failed', claim = NULL, reason = NEW.reason, charges = charges + 1,
            held_since = NULL, recheck_at = NULL
        WHERE event_id = NEW.event_id;
    ELSIF NEW.outcome = 'released' THEN
        UPDATE axa_outbox_owed
        SET state = 'pending', claim = NULL, reason = NULL, charges = 0, retry_at = NEW.at
        WHERE event_id = NEW.event_id;
    ELSE
        -- delivered, refused, and erased: nothing more is owed.
        DELETE FROM axa_outbox_owed WHERE event_id = NEW.event_id;
    END IF;
    RETURN NULL;
END
$$;

CREATE OR REPLACE TRIGGER axa_outbox_events_refuse
    BEFORE UPDATE OR DELETE ON axa_outbox_events
    FOR EACH ROW EXECUTE FUNCTION axa_outbox_refuse();

CREATE OR REPLACE TRIGGER axa_outbox_events_refuse_truncate
    BEFORE TRUNCATE ON axa_outbox_events
    FOR EACH STATEMENT EXECUTE FUNCTION axa_outbox_refuse();

CREATE OR REPLACE TRIGGER axa_outbox_events_owe
    AFTER INSERT ON axa_outbox_events
    FOR EACH ROW EXECUTE FUNCTION axa_outbox_owe();

CREATE OR REPLACE TRIGGER axa_outbox_attempts_refuse
    BEFORE UPDATE OR DELETE ON axa_outbox_attempts
    FOR EACH ROW EXECUTE FUNCTION axa_outbox_refuse();

CREATE OR REPLACE TRIGGER axa_outbox_attempts_refuse_truncate
    BEFORE TRUNCATE ON axa_outbox_attempts
    FOR EACH STATEMENT EXECUTE FUNCTION axa_outbox_refuse();

CREATE OR REPLACE TRIGGER axa_outbox_attempts_follow
    AFTER INSERT ON axa_outbox_attempts
    FOR EACH ROW EXECUTE FUNCTION axa_outbox_follow();

CREATE OR REPLACE TRIGGER axa_outbox_payloads_guard
    BEFORE UPDATE OR DELETE ON axa_outbox_payloads
    FOR EACH ROW EXECUTE FUNCTION axa_outbox_payloads_guard();

CREATE OR REPLACE TRIGGER axa_outbox_payloads_refuse_truncate
    BEFORE TRUNCATE ON axa_outbox_payloads
    FOR EACH STATEMENT EXECUTE FUNCTION axa_outbox_refuse();

CREATE OR REPLACE TRIGGER axa_outbox_owed_guard
    BEFORE INSERT OR UPDATE OR DELETE ON axa_outbox_owed
    FOR EACH ROW EXECUTE FUNCTION axa_outbox_owed_guard();

CREATE OR REPLACE TRIGGER axa_outbox_owed_refuse_truncate
    BEFORE TRUNCATE ON axa_outbox_owed
    FOR EACH STATEMENT EXECUTE FUNCTION axa_outbox_owed_guard();

CREATE OR REPLACE TRIGGER axa_outbox_leases_guard
    BEFORE INSERT OR UPDATE ON axa_outbox_leases
    FOR EACH ROW EXECUTE FUNCTION axa_outbox_leases_guard();

ALTER TABLE axa_outbox_events
    ENABLE ALWAYS TRIGGER axa_outbox_events_refuse,
    ENABLE ALWAYS TRIGGER axa_outbox_events_refuse_truncate;

ALTER TABLE axa_outbox_attempts
    ENABLE ALWAYS TRIGGER axa_outbox_attempts_refuse,
    ENABLE ALWAYS TRIGGER axa_outbox_attempts_refuse_truncate;

ALTER TABLE axa_outbox_payloads
    ENABLE ALWAYS TRIGGER axa_outbox_payloads_guard,
    ENABLE ALWAYS TRIGGER axa_outbox_payloads_refuse_truncate;

ALTER TABLE axa_outbox_owed
    ENABLE ALWAYS TRIGGER axa_outbox_owed_guard,
    ENABLE ALWAYS TRIGGER axa_outbox_owed_refuse_truncate;

ALTER TABLE axa_outbox_leases ENABLE ALWAYS TRIGGER axa_outbox_leases_guard;

INSERT INTO axa_outbox_events
        (event_id, topic, kind, key, fingerprint, created_at)
    SELECT old.event_id, old.topic, old.kind, old.key,
        axa_outbox_fingerprint(old.event_id, old.topic, old.kind, old.key, old.payload, NULL),
        old.created_at
    FROM axa_outbox old
    WHERE NOT EXISTS (SELECT 1 FROM axa_outbox_events e WHERE e.event_id = old.event_id)
    ORDER BY old.id;

INSERT INTO axa_outbox_payloads (event_id, payload)
    SELECT old.event_id, old.payload FROM axa_outbox old
    WHERE NOT EXISTS (SELECT 1 FROM axa_outbox_payloads p WHERE p.event_id = old.event_id)
    ORDER BY old.id;

INSERT INTO axa_outbox_attempts
        (event_id, outcome, relay, claim, reason, by, retry_at, at)
    SELECT old.event_id,
        CASE old.status WHEN 'sent' THEN 'delivered' WHEN 'failed' THEN 'failed'
            ELSE 'imported' END,
        '00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000000',
        'imported: ' || CASE old.status WHEN 'claimed' THEN 'pending' ELSE old.status END
            || ', ' || old.attempts || ' attempts',
        'axa-transactional-outbox 0.1.0',
        CASE WHEN old.status IN ('pending', 'claimed') THEN old.run_at END,
        coalesce(old.sent_at, old.updated_at)
    FROM axa_outbox old
    WHERE NOT EXISTS (SELECT 1 FROM axa_outbox_attempts a WHERE a.event_id = old.event_id)
    ORDER BY old.id;
