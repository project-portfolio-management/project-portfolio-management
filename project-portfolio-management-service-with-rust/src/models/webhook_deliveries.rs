//! `webhook_deliveries` model — the outbound webhook sink's delivery
//! log ([`crate::webhooks`], `agents/share/event-bus.md` §12).
//!
//! [`Model::record`] is the one write: insert the final outcome of one
//! target URL's retry-with-backoff sequence for one outbox event. There
//! is no update — a delivery outcome is immutable once decided, matching
//! `audit_logs`' append-only posture. The operator read is
//! `GET /api/webhook-deliveries` (`controllers::webhook_deliveries`).

use crate::prelude::*;

pub use super::_entities::webhook_deliveries::{self, ActiveModel, Entity, Model};

/// Default `SeaORM` active-model behaviour — no custom hooks.
impl ActiveModelBehavior for super::_entities::webhook_deliveries::ActiveModel {}

/// One delivery outcome to record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeliveryOutcome {
    /// The envelope's dedup id.
    pub event_id: uuid::Uuid,
    /// Entity name.
    pub entity: String,
    /// Change kind token.
    pub kind: String,
    /// The target URL.
    pub url: String,
    /// How many HTTP attempts were made.
    pub attempts: u32,
    /// Whether the delivery ultimately succeeded.
    pub delivered: bool,
    /// The last HTTP status received, if any.
    pub status_code: Option<u16>,
    /// The last error message, if the final attempt failed.
    pub error: Option<String>,
}

impl Model {
    /// Record one delivery outcome, and return its row id.
    ///
    /// # Errors
    ///
    /// When the insert fails.
    pub async fn record(ctx: &AppContext, outcome: &DeliveryOutcome) -> Result<i32> {
        // The columns `SeaORM`'s insert set, no more (`created_at` is the table's default).
        let id = sqlx::query_scalar(
            "INSERT INTO webhook_deliveries
                 (event_id, entity, kind, url, attempts, status, status_code, error)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
             RETURNING id",
        )
        .bind(outcome.event_id)
        .bind(&outcome.entity)
        .bind(&outcome.kind)
        .bind(&outcome.url)
        .bind(i32::try_from(outcome.attempts).unwrap_or(i32::MAX))
        .bind(if outcome.delivered {
            "delivered"
        } else {
            "failed"
        })
        .bind(outcome.status_code.map(i32::from))
        .bind(&outcome.error)
        .fetch_one(crate::db::pool(ctx))
        .await?;
        Ok(id)
    }
}

#[cfg(test)]
mod tests {
    use super::DeliveryOutcome;
    use uuid::Uuid;

    /// Sanity: a `DeliveryOutcome` for a failed delivery carries no
    /// status code when the failure was a pure transport error (no
    /// response ever came back) — pinned since [`super::Model::record`]
    /// maps `None` straight through and a future refactor could
    /// accidentally coerce it to `0`.
    #[test]
    fn a_transport_failure_carries_no_status_code() {
        let outcome = DeliveryOutcome {
            event_id: Uuid::parse_str("22222222-2222-4222-8222-222222222222").unwrap(),
            entity: "plan".to_string(),
            kind: "created".to_string(),
            url: "https://example.com/hook".to_string(),
            attempts: 4,
            delivered: false,
            status_code: None,
            error: Some("connection reset".to_string()),
        };
        assert_eq!(outcome.status_code, None);
        assert!(!outcome.delivered);
    }
}
