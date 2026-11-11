-- m20261010_000064_buffer_management: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

DROP TRIGGER IF EXISTS bra_guard ON buffer_recovery_actions;

DROP TRIGGER IF EXISTS bs_append_only ON buffer_snapshots;

DROP TRIGGER IF EXISTS bp_append_only ON buffer_plans;

DROP FUNCTION IF EXISTS bra_guard();

DROP FUNCTION IF EXISTS buffer_append_only();

DROP TABLE "buffer_recovery_actions";

DROP TABLE "buffer_snapshots";

DROP TABLE "buffer_plans";
