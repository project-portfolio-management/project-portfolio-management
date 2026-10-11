//! Migration: **the monthly rhythm of the director pack** (DP-12). A schedule drafts a pack for the month
//! that just ended; the portfolio office reviews the draft; a person issues it; and the audience's
//! subscribers are told with a link. A pack drafted by the schedule records its origin and who reviewed it.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Alter `director_packs`; create the schedule and subscription tables.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "director_pack_schedules",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("audience", ColType::String),
                ("day_of_month", ColType::Integer),
                ("next_run_date", ColType::Date),
                ("enabled", ColType::Boolean),
                ("last_run_on", ColType::DateNull),
                ("last_result", ColType::TextNull),
                ("created_by", ColType::StringNull),
            ],
            &[],
        )
        .await?;
        create_table(
            m,
            "director_pack_subscriptions",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("audience", ColType::String),
                ("person_ref", ColType::String),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        for q in [
            "ALTER TABLE director_packs ADD COLUMN origin text NOT NULL DEFAULT 'manual'",
            "ALTER TABLE director_packs ADD COLUMN reviewed_by text",
            "ALTER TABLE director_packs ADD COLUMN reviewed_at timestamptz",
            "ALTER TABLE director_packs ADD CONSTRAINT director_packs_origin CHECK (origin IN ('manual','scheduled'))",
            "ALTER TABLE director_pack_schedules ADD CONSTRAINT dps_day CHECK (day_of_month BETWEEN 1 AND 28)",
            "ALTER TABLE director_pack_schedules ADD CONSTRAINT dps_audience CHECK (audience IN ('executive','board','escalation_board','partner'))",
            "CREATE UNIQUE INDEX dps_one_per_audience ON director_pack_schedules (audience)",
            "ALTER TABLE director_pack_subscriptions ADD CONSTRAINT dsub_audience CHECK (audience IN ('executive','board','escalation_board','partner'))",
            "CREATE UNIQUE INDEX dsub_once ON director_pack_subscriptions (audience, lower(person_ref))",
        ] {
            db.execute_unprepared(q).await?;
        }
        Ok(())
    }

    /// Drop what `up` made.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "director_pack_subscriptions").await?;
        drop_table(m, "director_pack_schedules").await?;
        let db = m.get_connection();
        for q in [
            "ALTER TABLE director_packs DROP CONSTRAINT IF EXISTS director_packs_origin",
            "ALTER TABLE director_packs DROP COLUMN IF EXISTS reviewed_at",
            "ALTER TABLE director_packs DROP COLUMN IF EXISTS reviewed_by",
            "ALTER TABLE director_packs DROP COLUMN IF EXISTS origin",
        ] {
            db.execute_unprepared(q).await?;
        }
        Ok(())
    }
}
