-- m20261105_000101_plan_roles: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

DROP TABLE plan_role_members;
                 ALTER TABLE organization_roles DROP COLUMN permissions;
