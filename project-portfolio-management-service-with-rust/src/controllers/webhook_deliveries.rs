//! The outbound webhook delivery log, for an administrator (BR-5).
//!
//! * `GET /api/webhook-deliveries[?event_id=][&status=][&limit=]`: the log, newest first (administrators only;
//!   anyone else gets `404`);
//! * `POST /api/webhook-deliveries/{id}/redeliver`: send a logged delivery again, once, to the same configured
//!   target, from the event the outbox still holds. The event id is the same; the delivery id is new; a new log row
//!   records the outcome. The audit row is written **first**, so a redelivery that cannot be audited does not happen.

use crate::prelude::*;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use super::unprocessable_entity as unprocessable;
use crate::auth::MaybeAuthUser;
use crate::compliance::mac::{self, Domain};
use crate::models::_entities::webhook_deliveries as rows;
use crate::models::audit_logs::Model as AuditModel;
use crate::webhooks;

/// The routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/webhook-deliveries", get(list))
        .add("/webhook-deliveries/{id}/redeliver", post(redeliver))
}

/// Allow administrators only. Everyone else gets `NotFound` rather than Forbidden, so the endpoint's existence is not revealed.
fn admin(caller: &MaybeAuthUser) -> Result<()> {
    if caller.holds_or_open(|c| crate::auth::has_attr(c, "access", "admin")) {
        Ok(())
    } else {
        Err(Error::NotFound)
    }
}

/// `GET /api/webhook-deliveries` query.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ListQuery {
    /// Only deliveries for this event id.
    #[serde(default)]
    event_id: Option<Uuid>,
    /// Only `delivered` or `failed` deliveries; anything else is a 422.
    #[serde(default)]
    status: Option<String>,
    /// Rows to return, clamped to 1 to 500; absent means 100. Newest first.
    #[serde(default)]
    limit: Option<u64>,
}

/// `GET /api/webhook-deliveries[?event_id=][&status=][&limit=]` — administrators only; anyone else gets 404.
#[debug_handler]
async fn list(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(q): Query<ListQuery>,
) -> Result<Response> {
    admin(&caller)?;
    if let Some(s) = &q.status
        && !["delivered", "failed"].contains(&s.as_str())
    {
        return Err(unprocessable("status must be delivered or failed"));
    }
    let list: Vec<rows::Model> = sqlx::query_as(
        "SELECT * FROM webhook_deliveries
         WHERE ($1::uuid IS NULL OR event_id = $1) AND ($2::text IS NULL OR status = $2)
         ORDER BY id DESC
         LIMIT $3",
    )
    .bind(q.event_id)
    .bind(&q.status)
    .bind(i64::try_from(q.limit.unwrap_or(100).clamp(1, 500)).unwrap_or(500))
    .fetch_all(crate::db::pool(&ctx))
    .await?;
    format::json(json!({ "deliveries": list.iter().map(|d| json!({
        "id": d.id, "event_id": d.event_id, "entity": d.entity, "kind": d.kind, "url": d.url,
        "attempts": d.attempts, "status": d.status, "status_code": d.status_code, "error": d.error,
        "at": d.created_at,
    })).collect::<Vec<_>>() }))
}

/// `POST /api/webhook-deliveries/{id}/redeliver` — administrators only; anyone else gets 404. One attempt, to the
/// same still-configured target, from the event the outbox still holds (`422` when either is gone). Audited first.
#[debug_handler]
async fn redeliver(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Path(id): Path<i32>,
) -> Result<Response> {
    admin(&caller)?;
    let logged: rows::Model = sqlx::query_as("SELECT * FROM webhook_deliveries WHERE id = $1")
        .bind(id)
        .fetch_optional(crate::db::pool(&ctx))
        .await?
        .ok_or(Error::NotFound)?;
    let (topic, kind, payload): (String, String, Option<serde_json::Value>) = sqlx::query_as(
        "SELECT e.topic, e.kind, p.payload FROM axa_outbox_events e \
         LEFT JOIN axa_outbox_payloads p ON p.event_id = e.event_id WHERE e.event_id = $1",
    )
    .bind(logged.event_id)
    .fetch_optional(crate::db::pool(&ctx))
    .await?
    .ok_or_else(|| unprocessable("the event is not in the outbox, so it cannot be sent again"))?;
    let payload = payload.ok_or_else(|| {
        unprocessable("the event's payload was erased, so it cannot be sent again")
    })?;
    let target = webhooks::targets()
        .into_iter()
        .find(|t| t.url == logged.url)
        .ok_or_else(|| unprocessable("that target is no longer configured"))?;
    let request = webhooks::render(target.format, &topic, &payload)
        .ok_or_else(|| unprocessable("the event could not be rendered"))?;
    let signature = mac::tag(Domain::Webhook, &request.body).ok_or_else(|| {
        Error::CustomError(
            axum::http::StatusCode::SERVICE_UNAVAILABLE,
            axa::error::ErrorDetail::new(
                "signing_unavailable",
                "no integrity key is active, so an unsigned delivery will not be sent",
            ),
        )
    })?;
    let delivery_id = Uuid::new_v4();
    // Audit first: a redelivery that cannot be recorded does not go out.
    AuditModel::record_on(
        crate::db::pool(&ctx),
        logged.event_id,
        "webhook_redelivered",
        caller.actor(),
        Some(json!({ "log_id": id, "delivery_id": delivery_id, "url": logged.url })),
    )
    .await
    .map_err(super::audit_failed)?;
    let new_log = webhooks::deliver_with_retry(
        &ctx,
        &target,
        &topic,
        &kind,
        logged.event_id,
        delivery_id,
        &signature,
        &request,
        1,
    )
    .await;
    let after: Option<rows::Model> = match new_log {
        Some(log_id) => {
            sqlx::query_as("SELECT * FROM webhook_deliveries WHERE id = $1")
                .bind(log_id)
                .fetch_optional(crate::db::pool(&ctx))
                .await?
        }
        None => None,
    };
    format::json(json!({
        "event_id": logged.event_id,
        "delivery_id": delivery_id,
        "log_id": new_log,
        "status": after.as_ref().map(|d| d.status.clone()),
        "status_code": after.as_ref().and_then(|d| d.status_code),
        "error": after.as_ref().and_then(|d| d.error.clone()),
    }))
}
