-- m20261010_000077_portfolio_ledger: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

ALTER TABLE budget_lines DROP COLUMN funding_allocation_pid;

DROP TRIGGER IF EXISTS fa_append_only ON funding_allocations;

DROP FUNCTION IF EXISTS fa_append_only();

DROP TABLE "cost_code_maps";

DROP TABLE "funding_allocations";
