-- m20260826_000001_effort: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

DROP TABLE IF EXISTS non_working_periods;
                 DROP TABLE IF EXISTS working_time_configs;
                 DROP TABLE IF EXISTS time_entries;
