-- m20261009_000049_oxford_spelling: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: stored values move to Oxford spelling (`-ize`). A risk that
-- happened was stored as `materialised`; it is now `materialized`, the value
-- the API reads and writes. Audit snapshots are history and are not rewritten.

UPDATE risks SET status = 'materialized' WHERE status = 'materialised';
