//! `merge_records` model — record and query the record-merge history.

use crate::prelude::*;
use uuid::Uuid;

pub use super::_entities::merge_records::{self, ActiveModel, Entity, Model};

impl ActiveModelBehavior for super::_entities::merge_records::ActiveModel {}

impl Model {
    /// Record one merge. `actor` is the caller's `sub` when a verified
    /// token was presented; `transferred` is a snapshot of the merged-away
    /// duplicate's payload.
    ///
    /// # Errors
    ///
    /// When the insert fails.
    pub async fn record(
        ctx: &AppContext,
        main_pid: Uuid,
        duplicate_pid: Uuid,
        reason: Option<&str>,
        actor: Option<&str>,
        transferred: Option<serde_json::Value>,
    ) -> Result<Self> {
        Ok(sqlx::query_as(
            "INSERT INTO merge_records (main_pid, duplicate_pid, reason, actor, transferred)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING *",
        )
        .bind(main_pid)
        .bind(duplicate_pid)
        .bind(reason)
        .bind(actor)
        .bind(transferred)
        .fetch_one(crate::db::pool(ctx))
        .await?)
    }

    /// Most-recent merge records, capped at `limit`.
    ///
    /// # Errors
    ///
    /// When the query fails.
    pub async fn recent(ctx: &AppContext, limit: u64) -> Result<Vec<Self>> {
        Ok(
            sqlx::query_as("SELECT * FROM merge_records ORDER BY id DESC LIMIT $1")
                .bind(i64::try_from(limit).unwrap_or(i64::MAX))
                .fetch_all(crate::db::pool(ctx))
                .await?,
        )
    }
}
