-- m20260826_000003_value: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

DROP TABLE IF EXISTS satisfaction_responses;
                 DROP TABLE IF EXISTS adoption_snapshots;
                 DROP TABLE IF EXISTS value_points;
                 DROP TABLE IF EXISTS business_case_targets;
