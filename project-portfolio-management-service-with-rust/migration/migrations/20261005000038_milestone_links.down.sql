-- m20261005_000038_milestone_links: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

DROP TABLE "milestone_slips";

DROP TABLE "milestone_tasks";

ALTER TABLE milestones DROP COLUMN done_reason;

ALTER TABLE milestones DROP CONSTRAINT milestones_completion_mode;

ALTER TABLE milestones DROP COLUMN completion_mode;
