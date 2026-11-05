//! Migration: **role permissions and plan role membership** (OC-3).
//!
//! * `organization_roles.permissions`: the issue actions a role allows, from a fixed vocabulary the service checks.
//! * `plan_role_members`: who holds which role on which plan. One person can hold several roles on a plan, and the
//!   same person can hold different roles on different plans.
//!
//! No existing behaviour changes: every role starts with no permissions, and a plan with no members is not enforced.

use sea_orm_migration::prelude::*;

/// The `plan_roles` migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Add the permissions column and the membership table.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared(
                "ALTER TABLE organization_roles
                     ADD COLUMN IF NOT EXISTS permissions JSONB NOT NULL DEFAULT '[]'::jsonb;
                 ALTER TABLE organization_roles DROP CONSTRAINT IF EXISTS organization_roles_permissions_array;
                 ALTER TABLE organization_roles ADD CONSTRAINT organization_roles_permissions_array
                     CHECK (jsonb_typeof(permissions) = 'array');

                 CREATE TABLE IF NOT EXISTS plan_role_members (
                     created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                     id SERIAL PRIMARY KEY,
                     pid UUID NOT NULL UNIQUE DEFAULT gen_random_uuid(),
                     plan_pid UUID NOT NULL REFERENCES plans (pid) ON DELETE CASCADE,
                     person_ref VARCHAR NOT NULL CHECK (btrim(person_ref) <> ''),
                     role_key VARCHAR NOT NULL REFERENCES organization_roles (key),
                     added_by VARCHAR NULL,
                     UNIQUE (plan_pid, person_ref, role_key)
                 );
                 CREATE INDEX IF NOT EXISTS plan_role_members_person ON plan_role_members (person_ref);",
            )
            .await?;
        Ok(())
    }

    /// Remove them. The grants go with them, so every plan is unenforced again.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.get_connection()
            .execute_unprepared(
                "DROP TABLE plan_role_members;
                 ALTER TABLE organization_roles DROP COLUMN permissions;",
            )
            .await?;
        Ok(())
    }
}
