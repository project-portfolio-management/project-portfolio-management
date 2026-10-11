//! Migration: **organizations** (EX-3). The delivery organization, a partner body, a government sponsor or
//! a supplier, as records rather than labels. A capacity commitment may name one; an organization can be
//! the audience of a director pack (`org:<pid>`). Existing free-text partner labels become partner
//! organizations and their commitments are linked; a record with no organization behaves as before.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

const AUD: &str = "audience IN ('executive','board','escalation_board','partner') OR audience ~ '^org:[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$'";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Create the table; link commitments; allow an organization as an audience.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "organizations",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("name", ColType::String),
                ("kind", ColType::String),
                ("active", ColType::Boolean),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        for q in [
            "ALTER TABLE organizations ADD CONSTRAINT org_kind CHECK (kind IN ('delivery','partner','sponsor','supplier'))",
            "ALTER TABLE organizations ADD CONSTRAINT org_name CHECK (length(btrim(name)) BETWEEN 1 AND 200)",
            "CREATE UNIQUE INDEX org_name_once ON organizations (lower(btrim(name)))",
            "ALTER TABLE capacity_commitments ADD COLUMN organization_pid uuid",
            "INSERT INTO organizations (pid, name, kind, active, created_at, updated_at)
               SELECT gen_random_uuid(), min(btrim(organization)), 'partner', true, now(), now()
               FROM capacity_commitments WHERE organization IS NOT NULL AND btrim(organization) <> ''
               GROUP BY lower(btrim(organization))",
            "UPDATE capacity_commitments c SET organization_pid = o.pid
               FROM organizations o WHERE c.organization IS NOT NULL AND lower(btrim(c.organization)) = lower(o.name)",
            "CREATE INDEX cc_by_org ON capacity_commitments (organization_pid)",
        ] {
            db.execute_unprepared(q).await?;
        }
        for (table, name) in [
            ("director_packs", "director_packs_audience"),
            ("director_pack_schedules", "dps_audience"),
            ("director_pack_subscriptions", "dsub_audience"),
        ] {
            db.execute_unprepared(&format!("ALTER TABLE {table} DROP CONSTRAINT {name}")).await?;
            db.execute_unprepared(&format!("ALTER TABLE {table} ADD CONSTRAINT {name} CHECK ({AUD})")).await?;
        }
        Ok(())
    }

    /// Drop what `up` made.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();
        for (table, name) in [
            ("director_packs", "director_packs_audience"),
            ("director_pack_schedules", "dps_audience"),
            ("director_pack_subscriptions", "dsub_audience"),
        ] {
            db.execute_unprepared(&format!("DELETE FROM {table} WHERE audience LIKE 'org:%'")).await?;
            db.execute_unprepared(&format!("ALTER TABLE {table} DROP CONSTRAINT {name}")).await?;
            db.execute_unprepared(&format!(
                "ALTER TABLE {table} ADD CONSTRAINT {name} CHECK (audience IN ('executive','board','escalation_board','partner'))"
            ))
            .await?;
        }
        db.execute_unprepared("ALTER TABLE capacity_commitments DROP COLUMN organization_pid").await?;
        drop_table(m, "organizations").await
    }
}
