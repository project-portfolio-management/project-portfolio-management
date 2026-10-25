//! BR-5: a webhook delivery carries a timestamped V2 signature and a delivery id; an administrator can list the
//! delivery log and redeliver one entry, once, from the event the outbox still holds.
//!
//! Its **own test binary**: the integrity key and the webhook targets are read from the process environment.
//! `#[ignore]`d — needs PostgreSQL. `cargo test --test webhook_redelivery -- --ignored`.

use std::sync::{Arc, Mutex};

use axum::Router;
use axum::body::Bytes;
use axum::http::HeaderMap;
use axum::routing::post;
use chrono::Utc;
use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use project_portfolio_management::compliance::mac::{self, Domain};
use project_portfolio_management::models::_entities::{event_outbox, webhook_deliveries};
use project_portfolio_management::webhooks::{
    DELIVERY_ID_HEADER, EVENT_ID_HEADER, SIGNATURE_V2_HEADER, TIMESTAMP_HEADER, signing_input_v2,
    timestamp_is_fresh,
};
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait, PaginatorTrait};
use serde_json::{Value, json};
use uuid::Uuid;

type Seen = Arc<Mutex<Vec<(HeaderMap, Vec<u8>)>>>;

#[tokio::test]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test webhook_redelivery -- --ignored`"]
#[expect(clippy::too_many_lines)] // one linear scenario: a receiver, a logged failure, then each redelivery case
async fn a_logged_delivery_is_redelivered_once_with_a_fresh_signed_timestamp() {
    // A receiver on a loopback port, recording what it is sent.
    let seen: Seen = Arc::default();
    let sink = seen.clone();
    let app = Router::new().route(
        "/hook",
        post(move |headers: HeaderMap, body: Bytes| {
            let sink = sink.clone();
            async move {
                sink.lock().unwrap().push((headers, body.to_vec()));
                "ok"
            }
        }),
    );
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    tokio::spawn(async move { axum::serve(listener, app).await.ok() });
    let url = format!("http://127.0.0.1:{port}/hook");
    // SAFETY: this binary is single-test, so nothing races the environment.
    unsafe {
        std::env::set_var(
            "PORTFOLIO_INTEGRITY_MAC_KEY",
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        );
        std::env::set_var(
            "PROJECT_PORTFOLIO_MANAGEMENT_WEBHOOKS",
            json!([{ "url": url }]).to_string(),
        );
    }

    request::<App, _, _>(|request, ctx| async move {
        // An event the outbox still holds, and the log row of a delivery that failed.
        let event_id = Uuid::new_v4();
        event_outbox::ActiveModel {
            event_id: ActiveValue::set(event_id),
            entity: ActiveValue::set("plan".to_string()),
            entity_pid: ActiveValue::set(Uuid::new_v4()),
            kind: ActiveValue::set("created".to_string()),
            occurred_at: ActiveValue::set(Utc::now().into()),
            actor: ActiveValue::set(None),
            schema_version: ActiveValue::set(1),
            payload: ActiveValue::set(
                json!({ "event_id": event_id, "kind": "created", "name": "Redelivered" }),
            ),
            published_at: ActiveValue::set(Some(Utc::now().into())),
            ..Default::default()
        }
        .insert(&ctx.db)
        .await
        .unwrap();
        let log = |event: Uuid, target: &str| webhook_deliveries::ActiveModel {
            event_id: ActiveValue::set(event),
            entity: ActiveValue::set("plan".to_string()),
            kind: ActiveValue::set("created".to_string()),
            url: ActiveValue::set(target.to_string()),
            attempts: ActiveValue::set(4),
            status: ActiveValue::set("failed".to_string()),
            status_code: ActiveValue::set(Some(503)),
            error: ActiveValue::set(Some("HTTP 503".to_string())),
            ..Default::default()
        };
        let logged = log(event_id, &url).insert(&ctx.db).await.unwrap();

        // ---- the log lists it
        let list: Value = request
            .get(&format!("/api/webhook-deliveries?event_id={event_id}"))
            .await
            .json();
        assert_eq!(list["deliveries"].as_array().unwrap().len(), 1);
        assert_eq!(list["deliveries"][0]["status"], "failed");
        assert_eq!(
            request
                .get("/api/webhook-deliveries?status=nope")
                .await
                .status_code(),
            422
        );

        // ---- redeliver: one attempt, delivered, a new log row
        let out: Value = request
            .post(&format!("/api/webhook-deliveries/{}/redeliver", logged.id))
            .await
            .json();
        assert_eq!(out["status"], "delivered", "{out}");
        assert_eq!(out["event_id"], json!(event_id));
        let received = seen.lock().unwrap().clone();
        assert_eq!(received.len(), 1, "exactly one attempt");
        let (headers, body) = &received[0];
        let get = |name: &str| {
            headers
                .get(name)
                .and_then(|v| v.to_str().ok())
                .map(str::to_string)
        };
        assert_eq!(
            get(EVENT_ID_HEADER).unwrap(),
            event_id.to_string(),
            "the event id is the same"
        );
        assert_eq!(
            get(DELIVERY_ID_HEADER).unwrap(),
            out["delivery_id"].as_str().unwrap()
        );
        // The V2 signature verifies over "<timestamp>.<body>", and the timestamp is current.
        let ts: i64 = get(TIMESTAMP_HEADER).unwrap().parse().unwrap();
        let sig = get(SIGNATURE_V2_HEADER).unwrap();
        assert!(timestamp_is_fresh(ts, Utc::now().timestamp()));
        assert!(matches!(
            mac::verify(Domain::Webhook, Some(&sig), &signing_input_v2(ts, body)),
            integrity_mac::MacVerdict::Valid
        ));
        // A replay with a fresh timestamp header does not verify: the old signature covers the old one.
        assert!(!matches!(
            mac::verify(
                Domain::Webhook,
                Some(&sig),
                &signing_input_v2(ts + 1_000, body)
            ),
            integrity_mac::MacVerdict::Valid
        ));
        // The V1 signature (the body alone) is still sent for receivers that have not moved.
        assert!(get("X-Mxi-Signature").is_some());
        assert_eq!(
            webhook_deliveries::Entity::find()
                .count(&ctx.db)
                .await
                .unwrap(),
            2,
            "a new log row"
        );

        // ---- a second redelivery gets a new delivery id
        let again: Value = request
            .post(&format!("/api/webhook-deliveries/{}/redeliver", logged.id))
            .await
            .json();
        assert_ne!(again["delivery_id"], out["delivery_id"]);

        // ---- gone: an unknown row, an event the outbox no longer holds, a target no longer configured
        assert_eq!(
            request
                .post("/api/webhook-deliveries/999999/redeliver")
                .await
                .status_code(),
            404
        );
        let orphan = log(Uuid::new_v4(), &url).insert(&ctx.db).await.unwrap();
        assert_eq!(
            request
                .post(&format!("/api/webhook-deliveries/{}/redeliver", orphan.id))
                .await
                .status_code(),
            422
        );
        let elsewhere = log(event_id, "http://127.0.0.1:9/hook")
            .insert(&ctx.db)
            .await
            .unwrap();
        assert_eq!(
            request
                .post(&format!(
                    "/api/webhook-deliveries/{}/redeliver",
                    elsewhere.id
                ))
                .await
                .status_code(),
            422
        );
    })
    .await;
}
