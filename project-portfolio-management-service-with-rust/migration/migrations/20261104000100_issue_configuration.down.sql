-- m20261104_000100_issue_configuration: frozen on 11 November 2026 from the statements its sea-orm-migration `down` ran on PostgreSQL 18,
-- in order. Never edit it: it undoes what the up file beside it does, as `db down` did before.

DROP INDEX workflows_one_issue_default;
                 DROP INDEX workflows_one_per_plan;
                 UPDATE workflows SET deleted_at = now() WHERE issue_type IS NOT NULL AND deleted_at IS NULL;
                 CREATE UNIQUE INDEX workflows_one_per_plan ON workflows (plan_pid, applies_to)
                     WHERE deleted_at IS NULL AND plan_pid IS NOT NULL;
                 ALTER TABLE workflows DROP CONSTRAINT workflows_issue_type_on_issues;
                 ALTER TABLE workflows DROP COLUMN issue_type;
                 DROP TABLE organization_roles;
                 ALTER TABLE issues DROP CONSTRAINT issues_status_category;
                 ALTER TABLE issues DROP COLUMN status_category;
                 ALTER TABLE issues ADD CONSTRAINT issues_status
                     CHECK (status IN ('open','in_progress','resolved','closed'));
                 ALTER TABLE issues DROP CONSTRAINT issues_kind_type;
                 ALTER TABLE issues ADD CONSTRAINT issues_kind
                     CHECK (kind IN ('bug','risk','blocker','question','improvement','concern'));
                 DROP TABLE issue_types;
