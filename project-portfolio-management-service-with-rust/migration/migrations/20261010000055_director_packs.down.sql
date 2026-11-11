-- m20261010_000055_director_packs: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

DROP TRIGGER IF EXISTS director_packs_frozen ON director_packs;

DROP FUNCTION IF EXISTS director_packs_frozen();

DROP TABLE "director_packs";
