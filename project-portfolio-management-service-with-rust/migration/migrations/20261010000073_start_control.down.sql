-- m20261010_000073_start_control: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

DROP TRIGGER IF EXISTS so_append_only ON start_overrides;

DROP FUNCTION IF EXISTS so_append_only();

DROP TABLE "start_overrides";

DROP TABLE "start_queue_entries";
