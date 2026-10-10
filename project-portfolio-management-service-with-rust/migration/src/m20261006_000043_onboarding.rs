//! Migration: **onboarding and offboarding**. `onboarding_templates` are custom checklist templates (a built-in one exists for each kind); `onboarding_cases` are one onboarding or offboarding of a person on a plan, or of a plan itself; `onboarding_items` are the checklist items of a case, each with an owner, a due date and a resolution; `access_records` are an **append-only** record of access granted and revoked (this service records it; it cannot change an identity provider's accounts).

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The onboarding migration (name derived from the module).
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
            "onboarding_templates",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("kind", ColType::String),
                ("items", ColType::Json),
            ],
            &[],
        )
        .await?;
        create_table(
            m,
            "onboarding_cases",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("kind", ColType::String),
                ("plan_pid", ColType::Uuid),
                ("subject_ref", ColType::StringNull),
                ("title", ColType::String),
                ("effective_date", ColType::Date),
                ("status", ColType::String),
                ("created_by", ColType::StringNull),
                ("closed_at", ColType::TimestampWithTimeZoneNull),
                ("closed_reason", ColType::StringNull),
            ],
            &[],
        )
        .await?;
        create_table(
            m,
            "onboarding_items",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("case_pid", ColType::Uuid),
                ("position", ColType::Integer),
                ("title", ColType::String),
                ("owner_ref", ColType::StringNull),
                ("due_date", ColType::DateNull),
                ("resolution", ColType::StringNull),
                ("note", ColType::StringNull),
                ("resolved_at", ColType::TimestampWithTimeZoneNull),
                ("resolved_by", ColType::StringNull),
            ],
            &[],
        )
        .await?;
        create_table(
            m,
            "access_records",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("plan_pid", ColType::Uuid),
                ("case_pid", ColType::UuidNull),
                ("person_ref", ColType::String),
                ("access", ColType::String),
                ("action", ColType::String),
                ("recorded_on", ColType::Date),
                ("recorded_by", ColType::StringNull),
                ("note", ColType::StringNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        for q in [
            "ALTER TABLE onboarding_templates ADD CONSTRAINT onboarding_templates_kind CHECK (kind IN ('person_join','person_leave','plan_intake','plan_closeout'))",
            "CREATE UNIQUE INDEX onboarding_templates_kind_unique ON onboarding_templates (kind)",
            "ALTER TABLE onboarding_cases ADD CONSTRAINT onboarding_cases_kind CHECK (kind IN ('person_join','person_leave','plan_intake','plan_closeout'))",
            "ALTER TABLE onboarding_cases ADD CONSTRAINT onboarding_cases_status CHECK (status IN ('open','completed','cancelled'))",
            "ALTER TABLE onboarding_cases ADD CONSTRAINT onboarding_cases_subject CHECK ((kind LIKE 'person_%') = (subject_ref IS NOT NULL))",
            "CREATE INDEX onboarding_cases_plan ON onboarding_cases (plan_pid, status)",
            "ALTER TABLE onboarding_items ADD CONSTRAINT onboarding_items_resolution CHECK (resolution IS NULL OR resolution IN ('done','skipped'))",
            "CREATE INDEX onboarding_items_case ON onboarding_items (case_pid, position)",
            "ALTER TABLE access_records ADD CONSTRAINT access_records_action CHECK (action IN ('granted','revoked'))",
            "CREATE INDEX access_records_plan ON access_records (plan_pid, person_ref)",
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
        drop_table(m, "access_records").await?;
        drop_table(m, "onboarding_items").await?;
        drop_table(m, "onboarding_cases").await?;
        drop_table(m, "onboarding_templates").await
    }
}
