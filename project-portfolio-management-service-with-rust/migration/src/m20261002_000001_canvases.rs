//! Migration: **`canvases`** and **`canvas_items`** — strategy canvases
//! (LT-1). One generic pair of tables serves every canvas type (Lean,
//! Business Model, SWOT, …): a canvas names a `template` key, and each
//! item names a `box_key` within that template. The template (an ordered
//! box list) lives in code (`src/canvas.rs`), so adding a canvas type is
//! data, not a migration.
//!
//! A canvas is optionally scoped to a plan (`plan_pid`); a free-standing
//! canvas is the pre-proposal "what are we trying to do?" case. An item
//! may later be **promoted** to an idea, risk or objective; the link back
//! is recorded on the item (`promoted_to_*`) so a promotion is
//! idempotent-by-refusal, never silently repeated.

use loco_rs::schema::{create_table, drop_table, ColType};
use sea_orm_migration::prelude::*;

/// The `canvases` migration (name derived from the module).
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Create both tables and their lookup indexes.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "canvases",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("plan_pid", ColType::UuidNull),
                ("template", ColType::String),
                ("title", ColType::String),
                ("description", ColType::TextNull),
                ("deleted_at", ColType::TimestampWithTimeZoneNull),
            ],
            &[],
        )
        .await?;
        create_table(
            m,
            "canvas_items",
            &[
                ("id", ColType::PkAuto),
                ("pid", ColType::UuidUniq),
                ("canvas_pid", ColType::Uuid),
                ("box_key", ColType::String),
                ("body", ColType::Text),
                ("owner_ref", ColType::StringNull),
                ("position", ColType::Integer),
                ("promoted_to_kind", ColType::StringNull),
                ("promoted_to_pid", ColType::UuidNull),
                ("deleted_at", ColType::TimestampWithTimeZoneNull),
            ],
            &[],
        )
        .await?;
        let db = m.get_connection();
        db.execute_unprepared(
            "CREATE INDEX canvases_plan ON canvases (plan_pid) WHERE deleted_at IS NULL;",
        )
        .await?;
        db.execute_unprepared(
            "CREATE INDEX canvas_items_canvas ON canvas_items (canvas_pid, box_key, position) \
                 WHERE deleted_at IS NULL;",
        )
        .await?;
        // A promotion target is recorded as a pair or not at all.
        db.execute_unprepared(
            "ALTER TABLE canvas_items ADD CONSTRAINT canvas_items_promotion_pair \
                 CHECK ((promoted_to_kind IS NULL) = (promoted_to_pid IS NULL));",
        )
        .await?;
        Ok(())
    }

    /// Drop both tables.
    ///
    /// # Errors
    ///
    /// Propagates any DDL error.
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "canvas_items").await?;
        drop_table(m, "canvases").await
    }
}
