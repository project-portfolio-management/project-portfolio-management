//! Migration: **`comments`**, **`comment_edits`** and **`reactions`**
//! (LT-4). One generic pair keyed `(subject_kind, subject_pid)` serves
//! every commentable thing (plan, task, risk, proposal, idea, canvas
//! item), so a new subject kind is code, not a migration.
//!
//! - `comments` soft-delete; the author is the token `sub` (nothing
//!   identity-shaped beyond it) and may be absent only when auth is off.
//! - `comment_edits` is append-only: the previous body of every edit is
//!   kept, so a discussion cannot be silently rewritten.
//! - `reactions` are one per `(subject, sub, kind)` — the unique index
//!   makes a repeat reaction idempotent by construction.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The `comments_reactions` migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Create the three tables and their indexes.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "comments",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("subject_kind", ColType::String),
                ("subject_pid", ColType::Uuid),
                ("author_sub", ColType::StringNull),
                ("body", ColType::Text),
                ("edited_at", ColType::TimestampWithTimeZoneNull),
                ("deleted_at", ColType::TimestampWithTimeZoneNull),
            ],
            &[],
        )
        .await?;
        create_table(
            m,
            "comment_edits",
            &[
                ("id", ColType::PkAuto),
                ("comment_pid", ColType::Uuid),
                ("previous_body", ColType::Text),
                ("edited_by", ColType::StringNull),
            ],
            &[],
        )
        .await?;
        create_table(
            m,
            "reactions",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("subject_kind", ColType::String),
                ("subject_pid", ColType::Uuid),
                ("sub", ColType::String),
                ("kind", ColType::String),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        db.execute_unprepared(
            "CREATE INDEX comments_subject ON comments (subject_kind, subject_pid, id) \
                 WHERE deleted_at IS NULL;",
        )
        .await?;
        db.execute_unprepared(
            "CREATE INDEX comment_edits_comment ON comment_edits (comment_pid, id);",
        )
        .await?;
        db.execute_unprepared(
            "CREATE UNIQUE INDEX reactions_one_per_user \
                 ON reactions (subject_kind, subject_pid, sub, kind);",
        )
        .await?;
        Ok(())
    }

    /// Drop the three tables.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "reactions").await?;
        drop_table(m, "comment_edits").await?;
        drop_table(m, "comments").await
    }
}
