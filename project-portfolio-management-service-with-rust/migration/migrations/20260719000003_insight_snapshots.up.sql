-- m20260719_000003_insight_snapshots: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: create `insight_snapshots` — point-in-time estate
-- captures (portfolio RAG counts, open exposure, per-currency money
-- totals) behind the board/CRO trend views. Snapshots are taken
-- explicitly (`POST /api/board/snapshots`) or by the optional
-- env-gated ticker; trends read the stored series — no invented
-- history.

CREATE TABLE IF NOT EXISTS insight_snapshots (
                     id SERIAL PRIMARY KEY,
                     taken_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     kind VARCHAR NOT NULL,
                     body JSONB NOT NULL
                 );
                 CREATE INDEX IF NOT EXISTS insight_snapshots_kind_taken
                     ON insight_snapshots (kind, taken_at);
