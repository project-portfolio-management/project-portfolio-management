-- 20261118000116_kpi_template_key (EX-10, NC-1 slice 12 part a): which scorecard-template measure a KPI was made from.
--
-- `POST /api/scorecards/{pid}/from-template` stores a measure's name as the service's English text. `template_key`
-- keeps the measure's key (`milestones_met_on_time`, ...), so a client can name the KPI from its own catalogue in the
-- reader's language. Null for a KPI made any other way. No foreign key: the template is code, not a table, and a key
-- the template no longer offers stays as it was recorded.

ALTER TABLE kpis ADD COLUMN template_key varchar NULL;

ALTER TABLE kpis ADD CONSTRAINT kpi_template_key
    CHECK (template_key IS NULL OR template_key ~ '^[a-z][a-z0-9_]{0,79}$');
