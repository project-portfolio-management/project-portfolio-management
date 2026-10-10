//! Migration: **cost rates** (PJ-2). `cost_rate_tables` are named rate tables (standard, overtime, …) for one person or one role, in one currency; `cost_rates` are the rates in them, each **effective from a date** and append-only. `task_costs` are material and fixed-cost expenses on a task with an accrual rule. `time_entry_costs` are the cost of a recorded time entry **as it was worked out at the time**: the rate used is stored with it, so a later rate change never rewrites an existing actual.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The cost-rates migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Create the tables, their checks and indexes.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "cost_rate_tables",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("name", ColType::String),
                ("person_ref", ColType::StringNull),
                ("role", ColType::StringNull),
                ("currency", ColType::String),
                ("deleted_at", ColType::TimestampWithTimeZoneNull),
            ],
            &[],
        )
        .await?;
        create_table(
            m,
            "cost_rates",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("table_pid", ColType::Uuid),
                ("effective_from", ColType::Date),
                ("per_hour_minor", ColType::BigInteger),
                ("created_by", ColType::StringNull),
            ],
            &[],
        )
        .await?;
        create_table(
            m,
            "task_costs",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("task_pid", ColType::Uuid),
                ("kind", ColType::String),
                ("description", ColType::String),
                ("amount_minor", ColType::BigInteger),
                ("currency", ColType::String),
                ("accrual", ColType::String),
                ("deleted_at", ColType::TimestampWithTimeZoneNull),
            ],
            &[],
        )
        .await?;
        create_table(
            m,
            "time_entry_costs",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("time_entry_pid", ColType::Uuid),
                ("plan_pid", ColType::Uuid),
                ("person_ref", ColType::String),
                ("table_pid", ColType::Uuid),
                ("spent_on", ColType::Date),
                ("minutes", ColType::BigInteger),
                ("per_hour_minor", ColType::BigInteger),
                ("currency", ColType::String),
                ("cost_minor", ColType::BigInteger),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        for q in [
            "ALTER TABLE cost_rate_tables ADD CONSTRAINT cost_rate_tables_scope CHECK ((person_ref IS NOT NULL) <> (role IS NOT NULL))",
            "ALTER TABLE cost_rate_tables ADD CONSTRAINT cost_rate_tables_currency CHECK (currency ~ '^[A-Z]{3}$')",
            "CREATE UNIQUE INDEX cost_rate_tables_person ON cost_rate_tables (name, person_ref) WHERE person_ref IS NOT NULL AND deleted_at IS NULL",
            "CREATE UNIQUE INDEX cost_rate_tables_role ON cost_rate_tables (name, role) WHERE role IS NOT NULL AND deleted_at IS NULL",
            "ALTER TABLE cost_rates ADD CONSTRAINT cost_rates_nonnegative CHECK (per_hour_minor >= 0)",
            "CREATE UNIQUE INDEX cost_rates_day ON cost_rates (table_pid, effective_from)",
            "ALTER TABLE task_costs ADD CONSTRAINT task_costs_kind CHECK (kind IN ('material','fixed'))",
            "ALTER TABLE task_costs ADD CONSTRAINT task_costs_accrual CHECK (accrual IN ('start','prorated','end'))",
            "ALTER TABLE task_costs ADD CONSTRAINT task_costs_amount CHECK (amount_minor > 0)",
            "CREATE INDEX task_costs_task ON task_costs (task_pid) WHERE deleted_at IS NULL",
            "CREATE UNIQUE INDEX time_entry_costs_entry ON time_entry_costs (time_entry_pid)",
            "CREATE INDEX time_entry_costs_plan ON time_entry_costs (plan_pid)",
        ] {
            db.execute_unprepared(q).await?;
        }
        Ok(())
    }

    /// Drop the tables.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "time_entry_costs").await?;
        drop_table(m, "task_costs").await?;
        drop_table(m, "cost_rates").await?;
        drop_table(m, "cost_rate_tables").await
    }
}
