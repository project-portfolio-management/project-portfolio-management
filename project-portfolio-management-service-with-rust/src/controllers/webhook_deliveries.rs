//! The outbound webhook delivery log, for an administrator (BR-5).
//!
//! * `GET /api/webhook-deliveries[?event_id=][&status=][&limit=]`: the log, newest first (administrators only;
//!   anyone else gets `404`);
//! * `POST /api/webhook-deliveries/{id}/redeliver`: send a logged delivery again, once, to the same configured
//!   target, from the event the outbox still holds. The event id is the same; the delivery id is new; a new log row
//!   records the outcome. The audit row is written **first**, so a redelivery that cannot be audited does not happen.

use loco_rs::prelude::*;
use sea_orm::{QueryOrder, QuerySelect};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use super::{db_err, unprocessable_entity as unprocessable};
use crate::auth::MaybeAuthUser;
use crate::compliance::mac::{self, Domain};
use crate::models::_entities::{event_outbox, webhook_deliveries as rows};
use crate::models::audit_logs::Model as AuditModel;
use crate::webhooks;

/// The routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/webhook-deliveries", get(list))
        .add("/webhook-deliveries/{id}/redeliver", post(redeliver))
}

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
    #[serde(default)]
    event_id: Option<Uuid>,
    #[serde(default)]
    status: Option<String>,
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
    let mut find = rows::Entity::find().order_by_desc(rows::Column::Id);
    if let Some(e) = q.event_id {
        find = find.filter(rows::Column::EventId.eq(e));
    }
    if let Some(s) = &q.status {
        if !["delivered", "failed"].contains(&s.as_str()) {
            return Err(unprocessable("status must be delivered or failed"));
        }
        find = find.filter(rows::Column::Status.eq(s.as_str()));
    }
    let list = find
        .limit(q.limit.unwrap_or(100).clamp(1, 500))
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
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
    let logged = rows::Entity::find_by_id(id)
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or(Error::NotFound)?;
    let event = event_outbox::Entity::find()
        .filter(event_outbox::Column::EventId.eq(logged.event_id))
        .one(&ctx.db)
        .await
        .map_err(db_err)?
        .ok_or_else(|| {
            unprocessable("the event is no longer kept in the outbox, so it cannot be sent again")
        })?;
    let target = webhooks::targets()
        .into_iter()
        .find(|t| t.url == logged.url)
        .ok_or_else(|| unprocessable("that target is no longer configured"))?;
    let request = webhooks::render(target.format, &event.entity, &event.payload)
        .ok_or_else(|| unprocessable("the event could not be rendered"))?;
    let signature = mac::tag(Domain::Webhook, &request.body).ok_or_else(|| {
        Error::CustomError(
            axum::http::StatusCode::SERVICE_UNAVAILABLE,
            loco_rs::controller::ErrorDetail::new(
                "signing_unavailable",
                "no integrity key is active, so an unsigned delivery will not be sent",
            ),
        )
    })?;
    let delivery_id = Uuid::new_v4();
    // Audit first: a redelivery that cannot be recorded does not go out.
    AuditModel::record(
        &ctx.db,
        logged.event_id,
        "webhook_redelivered",
        caller.actor(),
        Some(json!({ "log_id": id, "delivery_id": delivery_id, "url": logged.url })),
    )
    .await
    .map_err(super::audit_failed)?;
    let new_log = webhooks::deliver_with_retry(
        &ctx.db,
        &target,
        &event.entity,
        &event.kind,
        logged.event_id,
        delivery_id,
        &signature,
        &request,
        1,
    )
    .await;
    let after = match new_log {
        Some(log_id) => rows::Entity::find_by_id(log_id)
            .one(&ctx.db)
            .await
            .map_err(db_err)?,
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
