-- m20261020_000081_link_systems: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

DELETE FROM plan_links WHERE kind = 'tracker';
                 ALTER TABLE plan_links DROP CONSTRAINT IF EXISTS plan_links_tracker_has_system;
                 ALTER TABLE plan_links DROP CONSTRAINT IF EXISTS plan_links_system;
                 ALTER TABLE plan_links DROP CONSTRAINT IF EXISTS plan_links_kind;
                 ALTER TABLE plan_links ADD CONSTRAINT plan_links_kind
                     CHECK (kind IN ('docs','repo','ticket','design','chat','other'));
                 ALTER TABLE plan_links DROP COLUMN IF EXISTS external_id;
                 ALTER TABLE plan_links DROP COLUMN IF EXISTS system;
