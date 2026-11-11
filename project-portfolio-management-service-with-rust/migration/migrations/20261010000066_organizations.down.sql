-- m20261010_000066_organizations: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

DELETE FROM director_packs WHERE audience LIKE 'org:%';

ALTER TABLE director_packs DROP CONSTRAINT director_packs_audience;

ALTER TABLE director_packs ADD CONSTRAINT director_packs_audience CHECK (audience IN ('executive','board','escalation_board','partner'));

DELETE FROM director_pack_schedules WHERE audience LIKE 'org:%';

ALTER TABLE director_pack_schedules DROP CONSTRAINT dps_audience;

ALTER TABLE director_pack_schedules ADD CONSTRAINT dps_audience CHECK (audience IN ('executive','board','escalation_board','partner'));

DELETE FROM director_pack_subscriptions WHERE audience LIKE 'org:%';

ALTER TABLE director_pack_subscriptions DROP CONSTRAINT dsub_audience;

ALTER TABLE director_pack_subscriptions ADD CONSTRAINT dsub_audience CHECK (audience IN ('executive','board','escalation_board','partner'));

ALTER TABLE capacity_commitments DROP COLUMN organization_pid;

DROP TABLE "organizations";
