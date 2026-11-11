-- m20261025_000093_blueprint_sync: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

DROP TABLE IF EXISTS plan_template_links; ALTER TABLE plan_templates DROP COLUMN IF EXISTS version;
