//! Migration: **`kpis`** and **`kpi_readings`**. A KPI is a standing measure (unlike a key result, it has no period and is not scored toward an objective): a name, a unit, a direction, a target and a red line. Its status is derived on read from the latest reading, never stored. Readings are appended with the date they were observed.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The KPI tables migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Create both tables, their checks and indexes.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "kpis",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("plan_pid", ColType::UuidNull),
                ("objective_pid", ColType::UuidNull),
                ("name", ColType::String),
                ("description", ColType::TextNull),
                ("unit", ColType::StringNull),
                ("direction", ColType::String),
                ("target_value", ColType::BigInteger),
                ("red_value", ColType::BigInteger),
                ("cadence", ColType::StringNull),
                ("owner_ref", ColType::StringNull),
                ("deleted_at", ColType::TimestampWithTimeZoneNull),
            ],
            &[],
        )
        .await?;
        create_table(
            m,
            "kpi_readings",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("kpi_pid", ColType::Uuid),
                ("observed_on", ColType::Date),
                ("value", ColType::BigInteger),
                ("note", ColType::TextNull),
                ("actor", ColType::StringNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        for q in [
            "ALTER TABLE kpis ADD CONSTRAINT kpis_direction CHECK (direction IN ('higher_better','lower_better'))",
            "ALTER TABLE kpis ADD CONSTRAINT kpis_cadence CHECK (cadence IS NULL OR cadence IN ('daily','weekly','monthly','quarterly'))",
            "ALTER TABLE kpis ADD CONSTRAINT kpis_red_line CHECK ((direction = 'higher_better' AND red_value < target_value) OR (direction = 'lower_better' AND red_value > target_value))",
            "CREATE INDEX kpis_plan ON kpis (plan_pid) WHERE deleted_at IS NULL",
            "CREATE INDEX kpi_readings_kpi ON kpi_readings (kpi_pid, observed_on)",
        ] {
            db.execute_unprepared(q).await?;
        }
        let _ = db;
        Ok(())
    }

    /// Drop both tables.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "kpi_readings").await?;
        drop_table(m, "kpis").await
    }
}
