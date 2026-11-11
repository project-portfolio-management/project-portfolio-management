-- m20261024_000087_availability_no_overlap: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

ALTER TABLE person_availabilities DROP CONSTRAINT IF EXISTS person_availabilities_no_overlap;
