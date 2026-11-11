-- m20261025_000097_status_claims: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: **`status_updates.claims`** (BR-2). A status update may carry claims, each a sentence with the sources
-- that support it, stored as ids (`kind:<pid>` or `metric:<key>`) and never as titles.

ALTER TABLE status_updates ADD COLUMN IF NOT EXISTS claims JSONB NOT NULL DEFAULT '[]'::jsonb;
