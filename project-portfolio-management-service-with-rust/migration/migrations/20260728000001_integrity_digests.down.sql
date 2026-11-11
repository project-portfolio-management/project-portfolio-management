-- m20260728_000001_integrity_digests: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

ALTER TABLE "audit_logs" DROP COLUMN "hash", DROP COLUMN "hash_sha3", DROP COLUMN "mac";

ALTER TABLE "plans" DROP COLUMN "content_hash", DROP COLUMN "content_hash_sha3", DROP COLUMN "content_mac";
