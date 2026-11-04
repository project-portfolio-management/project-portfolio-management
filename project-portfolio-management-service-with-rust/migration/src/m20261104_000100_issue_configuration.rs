//! Migration: **organization-defined issue types, statuses and roles** (OC-1).
//!
//! * `issue_types`: the catalogue an issue's `kind` must come from, seeded with the six built-in kinds. It replaces
//!   the `issues_kind` check with a foreign key, so the database still refuses an unknown kind. A type is retired,
//!   never deleted: issues that carry it stay valid.
//! * `issues.status_category`: what a status **means** (`todo`, `active`, `waiting`, `done`), stored on the issue. A
//!   status is now a state of the workflow in force, so "open", "done" and "overdue" can no longer be read from the
//!   status's name. The `issues_status` check goes; the category has one of its own.
//! * `organization_roles`: the role names the deployment recognizes. **A role here grants nothing.** Authority still
//!   comes from the token; the catalogue only names roles, so that a workflow cannot be restricted to a misspelt one.
//! * `workflows.issue_type`: a workflow for one issue type, which is the per-type half of Redmine's workflow matrix.
//!
//! With nothing configured, every issue behaves as it did before. **An issue workflow registered before this migration is
//! withdrawn by it.** Such a row did nothing, and anyone who could write could create one; left in place it would start
//! deciding who may move an issue, with nobody having decided that. An administrator registers the ones wanted.

use sea_orm_migration::prelude::*;

/// The `issue_configuration` migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Create the catalogues, move `issues` onto them and let a workflow name a type.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared(
                "DO $$ BEGIN
                     IF NOT EXISTS (SELECT 1 FROM information_schema.columns
                                    WHERE table_name = 'workflows' AND column_name = 'issue_type') THEN
                         UPDATE workflows SET deleted_at = now()
                             WHERE applies_to = 'issue' AND deleted_at IS NULL;
                     END IF;
                 END $$;

                 CREATE TABLE IF NOT EXISTS issue_types (
                     created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     id SERIAL PRIMARY KEY,
                     pid UUID NOT NULL UNIQUE DEFAULT gen_random_uuid(),
                     key VARCHAR NOT NULL UNIQUE CHECK (key ~ '^[a-z][a-z0-9_]{0,39}$'),
                     label VARCHAR NOT NULL CHECK (btrim(label) <> ''),
                     position INTEGER NOT NULL DEFAULT 0,
                     retired_at TIMESTAMPTZ NULL
                 );
                 INSERT INTO issue_types (key, label, position) VALUES
                     ('bug', 'Bug', 0), ('risk', 'Risk', 1), ('blocker', 'Blocker', 2),
                     ('question', 'Question', 3), ('improvement', 'Improvement', 4), ('concern', 'Concern', 5)
                     ON CONFLICT (key) DO NOTHING;

                 ALTER TABLE issues DROP CONSTRAINT IF EXISTS issues_kind;
                 ALTER TABLE issues DROP CONSTRAINT IF EXISTS issues_kind_type;
                 ALTER TABLE issues ADD CONSTRAINT issues_kind_type
                     FOREIGN KEY (kind) REFERENCES issue_types (key);

                 ALTER TABLE issues DROP CONSTRAINT IF EXISTS issues_status;
                 ALTER TABLE issues ADD COLUMN IF NOT EXISTS status_category VARCHAR NULL;
                 UPDATE issues SET status_category = CASE status
                     WHEN 'open' THEN 'todo'
                     WHEN 'in_progress' THEN 'active'
                     ELSE 'done' END
                     WHERE status_category IS NULL;
                 ALTER TABLE issues ALTER COLUMN status_category SET NOT NULL;
                 ALTER TABLE issues DROP CONSTRAINT IF EXISTS issues_status_category;
                 ALTER TABLE issues ADD CONSTRAINT issues_status_category
                     CHECK (status_category IN ('todo', 'active', 'waiting', 'done'));

                 CREATE TABLE IF NOT EXISTS organization_roles (
                     created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     id SERIAL PRIMARY KEY,
                     pid UUID NOT NULL UNIQUE DEFAULT gen_random_uuid(),
                     key VARCHAR NOT NULL UNIQUE CHECK (key ~ '^[a-z][a-z0-9_-]{0,63}$'),
                     label VARCHAR NOT NULL CHECK (btrim(label) <> ''),
                     description VARCHAR NULL,
                     position INTEGER NOT NULL DEFAULT 0,
                     retired_at TIMESTAMPTZ NULL
                 );

                 ALTER TABLE workflows ADD COLUMN IF NOT EXISTS issue_type VARCHAR NULL REFERENCES issue_types (key);
                 ALTER TABLE workflows DROP CONSTRAINT IF EXISTS workflows_issue_type_on_issues;
                 ALTER TABLE workflows ADD CONSTRAINT workflows_issue_type_on_issues
                     CHECK (issue_type IS NULL OR applies_to = 'issue');
                 DROP INDEX IF EXISTS workflows_one_per_plan;
                 CREATE UNIQUE INDEX workflows_one_per_plan
                     ON workflows (plan_pid, applies_to, COALESCE(issue_type, ''))
                     WHERE deleted_at IS NULL AND plan_pid IS NOT NULL;
                 -- One deployment default per issue type (and one for all types), so the workflow in force is
                 -- never ambiguous. Task defaults are left as they were.
                 CREATE UNIQUE INDEX IF NOT EXISTS workflows_one_issue_default
                     ON workflows (COALESCE(issue_type, ''))
                     WHERE deleted_at IS NULL AND plan_pid IS NULL AND is_default AND applies_to = 'issue';",
            )
            .await?;
        Ok(())
    }

    /// Remove the catalogues and restore the checks. Fails, by design, while any issue carries a custom type or
    /// status: the old checks could not hold it, and quietly rewriting those issues would lose what they say.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared(
                "DROP INDEX workflows_one_issue_default;
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
                 DROP TABLE issue_types;",
            )
            .await?;
        Ok(())
    }
}
