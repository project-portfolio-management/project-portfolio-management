-- m20261020_000083_escalations: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: **project escalations**. A project referring something upward at a level of its governance escalation
-- path, with a state (open, acknowledged, resolved, withdrawn) and an append-only history of what happened.

CREATE TABLE IF NOT EXISTS escalations (
                     created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     id BIGSERIAL PRIMARY KEY,
                     pid UUID NOT NULL UNIQUE,
                     plan_pid UUID NOT NULL,
                     title VARCHAR NOT NULL,
                     description TEXT NULL,
                     reason_kind VARCHAR NOT NULL,
                     tolerance_dimension VARCHAR NULL,
                     level INTEGER NOT NULL,
                     escalated_to VARCHAR NULL,
                     owner_ref VARCHAR NULL,
                     status VARCHAR NOT NULL DEFAULT 'open',
                     raised_by VARCHAR NULL,
                     raised_on DATE NOT NULL,
                     respond_by DATE NULL,
                     outcome VARCHAR NULL,
                     response TEXT NULL,
                     resolved_on DATE NULL,
                     deleted_at TIMESTAMPTZ NULL,
                     CONSTRAINT escalations_reason CHECK (reason_kind IN
                         ('tolerance','risk','issue','dependency','decision','resource','other')),
                     CONSTRAINT escalations_dimension CHECK (tolerance_dimension IS NULL OR
                         (reason_kind = 'tolerance' AND tolerance_dimension IN ('time','cost','risk','resources'))),
                     CONSTRAINT escalations_level CHECK (level BETWEEN 1 AND 9),
                     CONSTRAINT escalations_status CHECK (status IN ('open','acknowledged','resolved','withdrawn')),
                     CONSTRAINT escalations_outcome CHECK (outcome IS NULL OR outcome IN ('decided','guidance_given','rejected')),
                     CONSTRAINT escalations_resolved_has_answer CHECK (status <> 'resolved' OR
                         (outcome IS NOT NULL AND response IS NOT NULL AND resolved_on IS NOT NULL))
                 );
                 CREATE INDEX IF NOT EXISTS escalations_plan ON escalations (plan_pid, status);
                 CREATE TABLE IF NOT EXISTS escalation_events (
                     created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     id BIGSERIAL PRIMARY KEY,
                     pid UUID NOT NULL UNIQUE,
                     escalation_pid UUID NOT NULL,
                     event VARCHAR NOT NULL,
                     level INTEGER NOT NULL,
                     actor VARCHAR NULL,
                     note TEXT NULL,
                     CONSTRAINT escalation_events_event CHECK (event IN
                         ('raised','acknowledged','level_raised','resolved','withdrawn'))
                 );
                 CREATE INDEX IF NOT EXISTS escalation_events_escalation ON escalation_events (escalation_pid, id);
                 -- The history is append-only: no change or deletion of an event, ever.
                 CREATE OR REPLACE FUNCTION escalation_events_append_only() RETURNS trigger AS $$
                 BEGIN RAISE EXCEPTION 'escalation_events is append-only'; END; $$ LANGUAGE plpgsql;
                 DROP TRIGGER IF EXISTS escalation_events_no_change ON escalation_events;
                 CREATE TRIGGER escalation_events_no_change BEFORE UPDATE OR DELETE ON escalation_events
                     FOR EACH ROW EXECUTE FUNCTION escalation_events_append_only();
