-- 20261118000116_kpi_template_key: undo the up file beside it.

ALTER TABLE kpis DROP CONSTRAINT IF EXISTS kpi_template_key;

ALTER TABLE kpis DROP COLUMN IF EXISTS template_key;
