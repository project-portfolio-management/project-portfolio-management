-- m20261010_000069_external_asks: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: **external asks need a named trade-off** (EX-13). A proposal may be tagged as an external ask
-- (from the sponsor); approving it then needs a statement of what it displaces, which the service checks.

ALTER TABLE proposals ADD COLUMN external_ask boolean NOT NULL DEFAULT false;

ALTER TABLE proposals ADD COLUMN tradeoff text;

ALTER TABLE proposals ADD CONSTRAINT proposals_tradeoff_len CHECK (tradeoff IS NULL OR length(tradeoff) <= 2000);
