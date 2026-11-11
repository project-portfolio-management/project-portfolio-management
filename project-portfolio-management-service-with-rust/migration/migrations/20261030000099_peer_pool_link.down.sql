-- m20261030_000099_peer_pool_link: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

DROP INDEX IF EXISTS skill_pools_peer_pool_ref;

ALTER TABLE skill_pools DROP CONSTRAINT IF EXISTS skill_pools_peer_pool_ref_shape, DROP COLUMN IF EXISTS peer_pool_ref;

DROP TABLE "peer_demand_sends";
