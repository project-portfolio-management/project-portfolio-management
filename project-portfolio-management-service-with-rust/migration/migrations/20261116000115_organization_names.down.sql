-- 20261116000115_organization_names (RE-7): undo the up file beside it.

ALTER TABLE organizations DROP CONSTRAINT IF EXISTS org_translations;

ALTER TABLE organizations DROP COLUMN IF EXISTS translations;

ALTER TABLE organizations DROP CONSTRAINT IF EXISTS org_short_name;

ALTER TABLE organizations DROP COLUMN IF EXISTS short_name;
