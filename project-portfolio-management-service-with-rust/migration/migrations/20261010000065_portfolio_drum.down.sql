-- m20261010_000065_portfolio_drum: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

DROP TRIGGER IF EXISTS drum_append_only ON portfolio_drums;

DROP FUNCTION IF EXISTS drum_append_only();

DROP TABLE "portfolio_drums";
