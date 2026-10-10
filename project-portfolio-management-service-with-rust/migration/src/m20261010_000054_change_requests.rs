//! Migration: **change requests** (IR-4). A controlled proposal to change an approved baseline: scope, schedule, cost, quality or something else. It has a requester, one named approver who is somebody else, a decision with a stated reason, and the issues and risks it answers. The database refuses a cost without its currency, an approval without a decider and a reason, and a status it does not know. Rows are soft-deleted only.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The migration (name derived from the module).
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
            "change_requests",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("plan_pid", ColType::Uuid),
                ("title", ColType::String),
                ("description", ColType::Text),
                ("reason", ColType::Text),
                ("kind", ColType::String),
                ("impact_schedule_days", ColType::IntegerNull),
                ("impact_cost_minor", ColType::BigIntegerNull),
                ("cost_currency", ColType::StringNull),
                ("impact_summary", ColType::TextNull),
                ("edits", ColType::JsonBinaryNull),
                ("status", ColType::String),
                ("requested_by", ColType::String),
                ("requested_on", ColType::Date),
                ("approver_ref", ColType::StringNull),
                ("decided_by", ColType::StringNull),
                ("decided_on", ColType::DateNull),
                ("decision_reason", ColType::TextNull),
                ("implemented_on", ColType::DateNull),
                ("baseline_pid", ColType::UuidNull),
                ("deleted_at", ColType::TimestampWithTimeZoneNull),
            ],
            &[],
        )
        .await?;
        create_table(
            m,
            "change_request_risks",
            &[
                ("id", ColType::PkAuto),
                ("change_request_pid", ColType::Uuid),
                ("risk_pid", ColType::Uuid),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        for q in [
            "ALTER TABLE change_requests ADD CONSTRAINT change_requests_kind CHECK (kind IN ('scope','schedule','cost','quality','other'))",
            "ALTER TABLE change_requests ADD CONSTRAINT change_requests_status CHECK (status IN ('draft','submitted','under_review','approved','rejected','implemented','withdrawn'))",
            "ALTER TABLE change_requests ADD CONSTRAINT change_requests_cost_pair CHECK ((impact_cost_minor IS NULL) = (cost_currency IS NULL))",
            "ALTER TABLE change_requests ADD CONSTRAINT change_requests_currency CHECK (cost_currency IS NULL OR cost_currency ~ '^[A-Z]{3}$')",
            "ALTER TABLE change_requests ADD CONSTRAINT change_requests_days CHECK (impact_schedule_days IS NULL OR impact_schedule_days BETWEEN -3650 AND 3650)",
            "ALTER TABLE change_requests ADD CONSTRAINT change_requests_decided CHECK (status NOT IN ('approved','rejected','implemented') OR (decided_by IS NOT NULL AND decided_on IS NOT NULL AND char_length(btrim(coalesce(decision_reason,''))) >= 10))",
            "ALTER TABLE change_requests ADD CONSTRAINT change_requests_submitted CHECK (status IN ('draft','withdrawn') OR approver_ref IS NOT NULL)",
            "CREATE INDEX change_requests_plan ON change_requests (plan_pid, status) WHERE deleted_at IS NULL",
            "CREATE UNIQUE INDEX change_request_risks_once ON change_request_risks (change_request_pid, risk_pid)",
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
        drop_table(m, "change_request_risks").await?;
        drop_table(m, "change_requests").await
    }
}
