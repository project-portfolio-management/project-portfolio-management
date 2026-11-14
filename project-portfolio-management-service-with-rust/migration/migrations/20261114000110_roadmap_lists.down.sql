-- 20261114000110_roadmap_lists (RE-7): undo the up file beside it.

DROP TABLE IF EXISTS roadmap_categories;

DROP INDEX IF EXISTS org_one_national;

DROP INDEX IF EXISTS org_code_once;

ALTER TABLE organizations DROP CONSTRAINT IF EXISTS org_code;

ALTER TABLE organizations DROP COLUMN IF EXISTS national;

ALTER TABLE organizations DROP COLUMN IF EXISTS code;
