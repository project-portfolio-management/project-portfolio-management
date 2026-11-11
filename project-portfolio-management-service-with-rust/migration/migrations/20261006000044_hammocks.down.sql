-- m20261006_000044_hammocks: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

DROP TABLE "hammock_links";

ALTER TABLE tasks DROP CONSTRAINT IF EXISTS tasks_hammock_is_derived;

ALTER TABLE tasks DROP COLUMN is_hammock;
