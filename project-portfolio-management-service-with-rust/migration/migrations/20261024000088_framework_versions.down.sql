-- m20261024_000088_framework_versions: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

DROP TABLE IF EXISTS framework_versions;
                 DROP FUNCTION IF EXISTS framework_versions_append_only();
                 ALTER TABLE plan_frameworks DROP COLUMN IF EXISTS version;
                 ALTER TABLE frameworks DROP COLUMN IF EXISTS version;
