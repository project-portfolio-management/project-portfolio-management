-- m20261006_000043_onboarding: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

DROP TABLE "access_records";

DROP TABLE "onboarding_items";

DROP TABLE "onboarding_cases";

DROP TABLE "onboarding_templates";
