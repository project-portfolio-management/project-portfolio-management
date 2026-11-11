-- m20261010_000062_pack_rhythm: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

DROP TABLE "director_pack_subscriptions";

DROP TABLE "director_pack_schedules";

ALTER TABLE director_packs DROP CONSTRAINT IF EXISTS director_packs_origin;

ALTER TABLE director_packs DROP COLUMN IF EXISTS reviewed_at;

ALTER TABLE director_packs DROP COLUMN IF EXISTS reviewed_by;

ALTER TABLE director_packs DROP COLUMN IF EXISTS origin;
