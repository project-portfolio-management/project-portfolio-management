-- m20260728_000001_integrity_digests: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: add the row-level integrity columns.
--
-- Three values per row, all nullable and all **never back-filled**:
--
-- - `content_hash` — SHA-256 (FIPS 180-4) over the row's pre-image.
-- - `content_hash_sha3` — SHA3-256 (FIPS 202). Kept for structural
--   diversity: a sponge construction, unrelated to SHA-256's
--   Merkle-Damgard chaining, so a cryptanalytic advance against one
--   design family does not transfer.
-- - `content_mac` / `mac` — HMAC-SHA256 (FIPS 198-1) over the same
--   pre-image, keyed.
--
-- The MAC is the one that matters against a deliberate edit. The two
-- digests are **unkeyed** and their pre-image format is published, so
-- anyone who can write SQL defeats them: edit the row, recompute both,
-- update both columns. What they detect is careless or unaware
-- modification — a bug, a manual fix, a restore from the wrong backup.
-- The MAC raises that bar to a key held in the service environment and
-- never written to this database, so a stolen backup, a replica, a
-- SQL-injection foothold, or a DBA without application-server access
-- cannot forge one.
--
-- Stored as `"<scheme>.<key id>:<hex>"`. The key id is what makes
-- rotation survivable: without it, changing the key would invalidate
-- every historical row at once, which is indistinguishable from mass
-- tampering.
--
-- Nullable and never back-filled, because a digest computed later from
-- current content would certify whatever that content now is — which is
-- the claim it exists to test. Rows without one are reported as
-- unhashed or `mac_absent`, never as mismatches.

ALTER TABLE "audit_logs" ADD COLUMN IF NOT EXISTS "hash" varchar NULL, ADD COLUMN IF NOT EXISTS "hash_sha3" varchar NULL, ADD COLUMN IF NOT EXISTS "mac" varchar NULL;

ALTER TABLE "plans" ADD COLUMN IF NOT EXISTS "content_hash" varchar NULL, ADD COLUMN IF NOT EXISTS "content_hash_sha3" varchar NULL, ADD COLUMN IF NOT EXISTS "content_mac" varchar NULL;
