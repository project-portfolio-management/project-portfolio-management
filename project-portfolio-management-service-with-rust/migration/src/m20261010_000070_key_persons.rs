//! Migration: **the key-person register** (EX-11). Single points of failure (a person or a role), who would
//! take over and how ready they are, and which people are contingent (contractors and the like), so that the
//! contingent share of a skill pool can be counted. For resource managers only; the pack shows counts.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Create the tables and checks.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "key_person_entries",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("subject_kind", ColType::String),
                ("subject", ColType::String),
                ("successor_ref", ColType::StringNull),
                ("readiness", ColType::String),
                ("skill_pool_pid", ColType::UuidNull),
                ("note", ColType::TextNull),
            ],
            &[],
        )
        .await?;
        create_table(
            m,
            "contingent_workers",
            &[("id", ColType::PkAuto), ("pid", ColType::UuidUniq), ("person_ref", ColType::StringUniq)],
            &[],
        )
        .await?;
        let db = m.get_connection();
        for q in [
            "ALTER TABLE key_person_entries ADD CONSTRAINT kpe_kind CHECK (subject_kind IN ('person','role'))",
            "ALTER TABLE key_person_entries ADD CONSTRAINT kpe_readiness CHECK (readiness IN ('ready','partly','none'))",
            "ALTER TABLE key_person_entries ADD CONSTRAINT kpe_subject CHECK (length(btrim(subject)) BETWEEN 1 AND 200)",
            "CREATE UNIQUE INDEX kpe_once ON key_person_entries (subject_kind, lower(btrim(subject)))",
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
        drop_table(m, "contingent_workers").await?;
        drop_table(m, "key_person_entries").await
    }
}
