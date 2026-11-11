-- m20261009_000049_oxford_spelling: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

UPDATE risks SET status = 'materialised' WHERE status = 'materialized';
