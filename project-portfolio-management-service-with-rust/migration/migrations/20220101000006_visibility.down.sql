-- m20220101_000006_visibility: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

DROP TABLE "report_definitions";

DROP TABLE "allocations";

DROP TABLE "milestones";

DROP TABLE "plan_dependencies";
