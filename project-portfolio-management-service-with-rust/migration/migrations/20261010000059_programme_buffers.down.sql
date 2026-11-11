-- m20261010_000059_programme_buffers: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

DROP TRIGGER IF EXISTS pb_append_only ON programme_buffers;

DROP FUNCTION IF EXISTS pb_append_only();

DROP TABLE "programme_buffers";
