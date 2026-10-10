//! Migration: **`person_availabilities`**. A person's availability over a date range (PJ-3): the most of their time (0-100 percent) they can commit between two dates, for part-time spells, leave blocks and ramp-up. Where no range applies capacity is 100 percent, so nothing stored before this existed changes. Ranges for one person may not overlap (checked by the service).

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The `person_availabilities` migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Create the table and its indexes.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "person_availabilities",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("person_ref", ColType::String),
                ("from_date", ColType::Date),
                ("to_date", ColType::Date),
                ("max_percent", ColType::Integer),
                ("note", ColType::TextNull),
                ("deleted_at", ColType::TimestampWithTimeZoneNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        db.execute_unprepared(
            "ALTER TABLE person_availabilities ADD CONSTRAINT person_availabilities_range CHECK (to_date >= from_date)",
        )
        .await?;
        db.execute_unprepared(
            "ALTER TABLE person_availabilities ADD CONSTRAINT person_availabilities_percent CHECK (max_percent >= 0 AND max_percent <= 100)",
        )
        .await?;
        db.execute_unprepared(
            "CREATE INDEX person_availabilities_person ON person_availabilities (person_ref, from_date) WHERE deleted_at IS NULL",
        )
        .await?;
        let _ = db;
        Ok(())
    }

    /// Drop the table.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "person_availabilities").await
    }
}
