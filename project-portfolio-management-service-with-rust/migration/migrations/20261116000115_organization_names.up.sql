-- 20261116000115_organization_names (RE-7): an organization's short name, and its names in other languages.
--
-- `short_name` is optional and not unique (the short `code` is the unique one): a health board's "Swansea Bay" beside
-- its "Swansea Bay University Health Board". `translations` holds the names an organization goes by in other
-- languages, keyed by language code: `{ "cy": { "name": "...", "short_name": "..." }, "zh-hans": { "name": "..." } }`.
-- A language's `name` is required and its `short_name` optional; the service checks the shape, the database checks
-- that it is an object.

ALTER TABLE organizations ADD COLUMN short_name varchar NULL;

ALTER TABLE organizations ADD CONSTRAINT org_short_name
    CHECK (short_name IS NULL OR length(btrim(short_name)) BETWEEN 1 AND 50);

ALTER TABLE organizations ADD COLUMN translations jsonb NOT NULL DEFAULT '{}'::jsonb;

ALTER TABLE organizations ADD CONSTRAINT org_translations CHECK (jsonb_typeof(translations) = 'object');
