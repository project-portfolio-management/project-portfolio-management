//! Migration: **programme profiles** (EX-2). What kind of delivery a programme is (central product, central
//! platform with local deployment, or a local programme under central standards) and which organization
//! owns the delivery. One row per programme; changes are audited by the service.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Create the table and checks.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "programme_profiles",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("plan_pid", ColType::UuidUniq),
                ("delivery_model", ColType::StringNull),
                ("delivery_organization_pid", ColType::UuidNull),
                // EX-14: how the programme reports (evm or product); the API for it comes with EX-14.
                ("method", ColType::String),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        db.execute_unprepared("ALTER TABLE programme_profiles ADD CONSTRAINT pp_model CHECK (delivery_model IS NULL OR delivery_model IN ('central_product','central_platform_local_deployment','local_programme_central_standards'))").await?;
        db.execute_unprepared(
            "ALTER TABLE programme_profiles ALTER COLUMN method SET DEFAULT 'evm'",
        )
        .await?;
        db.execute_unprepared("ALTER TABLE programme_profiles ADD CONSTRAINT pp_method CHECK (method IN ('evm','product'))").await?;
        Ok(())
    }

    /// Drop what `up` made.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "programme_profiles").await
    }
}
