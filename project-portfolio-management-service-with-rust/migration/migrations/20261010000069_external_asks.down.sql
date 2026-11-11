-- m20261010_000069_external_asks: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

ALTER TABLE proposals DROP CONSTRAINT proposals_tradeoff_len;

ALTER TABLE proposals DROP COLUMN tradeoff;

ALTER TABLE proposals DROP COLUMN external_ask;
