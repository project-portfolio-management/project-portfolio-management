-- m20261021_000085_schedule_tolerance: frozen on 11 November 2026 from the statements its sea-orm-migration `up` ran on PostgreSQL 18,
-- in order. Never edit it: `_sqlx_migrations` holds its checksum. A change is a new migration.
--
-- What the Rust migration said of itself:
--
-- Migration: a fifth tolerance dimension, `schedule`: how many calendar days the forecast finish is after the finish
-- recorded in the plan's latest schedule baseline. (`time` stays the slip against the plan's `target_date`.) The two
-- checks that list the dimensions are widened, in the tolerances table and on escalations that cite a tolerance.

ALTER TABLE plan_tolerances DROP CONSTRAINT IF EXISTS plan_tolerances_dimension;
                 ALTER TABLE plan_tolerances ADD CONSTRAINT plan_tolerances_dimension
                     CHECK (dimension IN ('time','schedule','cost','risk','resources'));
                 ALTER TABLE escalations DROP CONSTRAINT IF EXISTS escalations_dimension;
                 ALTER TABLE escalations ADD CONSTRAINT escalations_dimension CHECK (tolerance_dimension IS NULL OR
                     (reason_kind = 'tolerance' AND tolerance_dimension IN ('time','schedule','cost','risk','resources')));
