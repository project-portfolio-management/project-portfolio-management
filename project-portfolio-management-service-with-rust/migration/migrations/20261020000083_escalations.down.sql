-- m20261020_000083_escalations: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

DROP TABLE IF EXISTS escalation_events; DROP FUNCTION IF EXISTS escalation_events_append_only(); DROP TABLE IF EXISTS escalations;
