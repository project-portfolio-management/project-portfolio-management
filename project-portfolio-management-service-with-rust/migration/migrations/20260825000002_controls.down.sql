-- m20260825_000002_controls: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

DROP TABLE IF EXISTS control_actions;
                 DROP TABLE IF EXISTS control_readings;
                 DROP TABLE IF EXISTS controls;
