-- 20261115000114_axa_outbox_tenant: the outbox of `axa-transactional-outbox` 0.4 (axa 0.14), which writes a `tenant`
-- column on `axa_outbox_events` and `axa_outbox_owed` and keeps a second form of `axa_outbox_fingerprint` that covers
-- it. These are that crate's `SETUP_STATEMENTS`, printed from 0.4.0 unchanged and in order; each is safe to repeat,
-- so on the tables migrations 108 and 109 made they add only what 0.4 lacks. `Outbox::connect_existing`, which the
-- relay uses, refuses tables without them ("Upgrading to Axa 0.14").
--
-- Events already written have no tenant and keep their fingerprints. This service names no tenant, so every new
-- event has none too. One way: the outbox is a record whose rows are never rewritten, so there is no down step.

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

ALTER TABLE axa_outbox_events ADD COLUMN IF NOT EXISTS tenant TEXT
    CONSTRAINT axa_outbox_events_tenant_id
    CHECK (tenant ~ '^[a-z0-9]([a-z0-9-]{0,61}[a-z0-9])?$');

ALTER TABLE axa_outbox_owed ADD COLUMN IF NOT EXISTS tenant TEXT;

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

CREATE OR REPLACE FUNCTION axa_outbox_fingerprint(
    event_id UUID, topic TEXT, kind TEXT, key TEXT, tenant TEXT, payload JSONB, sealed BYTEA)
RETURNS TEXT LANGUAGE sql IMMUTABLE PARALLEL SAFE AS $$
    SELECT encode(sha256(
        uuid_send(event_id)
        || convert_to(topic, 'UTF8') || '\x00'::bytea
        || convert_to(kind, 'UTF8') || '\x00'::bytea
        || convert_to(key, 'UTF8') || '\x00'::bytea
        || CASE WHEN tenant IS NULL THEN ''::bytea
                ELSE '\x74'::bytea || convert_to(tenant, 'UTF8') || '\x00'::bytea END
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
    INSERT INTO axa_outbox_owed (event_id, id, topic, key, tenant)
    VALUES (NEW.event_id, NEW.id, NEW.topic, NEW.key, NEW.tenant);
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

