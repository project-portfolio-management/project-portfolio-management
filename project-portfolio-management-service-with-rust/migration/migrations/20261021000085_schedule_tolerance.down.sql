-- m20261021_000085_schedule_tolerance: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

DELETE FROM plan_tolerances WHERE dimension = 'schedule';
                 UPDATE escalations SET tolerance_dimension = NULL, reason_kind = 'other'
                     WHERE tolerance_dimension = 'schedule';
                 ALTER TABLE plan_tolerances DROP CONSTRAINT IF EXISTS plan_tolerances_dimension;
                 ALTER TABLE plan_tolerances ADD CONSTRAINT plan_tolerances_dimension
                     CHECK (dimension IN ('time','cost','risk','resources'));
                 ALTER TABLE escalations DROP CONSTRAINT IF EXISTS escalations_dimension;
                 ALTER TABLE escalations ADD CONSTRAINT escalations_dimension CHECK (tolerance_dimension IS NULL OR
                     (reason_kind = 'tolerance' AND tolerance_dimension IN ('time','cost','risk','resources')));
