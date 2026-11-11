//! BR-5: a webhook delivery carries a timestamped V2 signature and a delivery id; an administrator can list the
//! delivery log and redeliver one entry, once, from the event the outbox still holds.
//!
//! Its **own test binary**: the integrity key and the webhook targets are read from the process environment.
//! `#[ignore]`d — needs PostgreSQL. `cargo test --test webhook_redelivery -- --ignored`.

use std::sync::{Arc, Mutex};

use axa_test::prelude::*;
use axum::Router;
use axum::body::Bytes;
use axum::http::HeaderMap;
use axum::routing::post;
use chrono::Utc;
use project_portfolio_management::app::App;
use project_portfolio_management::compliance::mac::{self, Domain};
use project_portfolio_management::models::_entities::webhook_deliveries;
use project_portfolio_management::webhooks::{
    DELIVERY_ID_HEADER, EVENT_ID_HEADER, SIGNATURE_V2_HEADER, TIMESTAMP_HEADER, signing_input_v2,
    timestamp_is_fresh,
};
use serde_json::{Value, json};
use uuid::Uuid;

type Seen = Arc<Mutex<Vec<(HeaderMap, Vec<u8>)>>>;

#[tokio::test]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test --test webhook_redelivery -- --ignored`"]
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
        // As a handler writes it; the record keeps it whatever the relay has done since.
        let mut tx = project_portfolio_management::db::begin(&ctx).await.unwrap();
        tx.write_outbox(&axa::outbox::Event {
                event_id,
                ..axa::outbox::Event::new(
                    "plan",
                    "created",
                    Uuid::new_v4().to_string(),
                    json!({ "event_id": event_id, "kind": "created", "name": "Redelivered" }),
                )
        })
        .await
        .unwrap();
        tx.commit().await.unwrap();
        let log = |event: Uuid, target: String| {
            let db = project_portfolio_management::db::pool(&ctx);
            async move {
                sqlx::query_as::<_, webhook_deliveries::Model>(
                    "INSERT INTO webhook_deliveries (event_id, entity, kind, url, attempts, status, status_code, error)
                     VALUES ($1, 'plan', 'created', $2, 4, 'failed', 503, 'HTTP 503') RETURNING *",
                )
                .bind(event)
                .bind(target)
                .fetch_one(db)
                .await
            }
        };
        let logged = log(event_id, url.clone()).await.unwrap();

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
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM webhook_deliveries")
                .fetch_one(project_portfolio_management::db::pool(&ctx))
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
        let orphan = log(Uuid::new_v4(), url.clone()).await.unwrap();
        assert_eq!(
            request
                .post(&format!("/api/webhook-deliveries/{}/redeliver", orphan.id))
                .await
                .status_code(),
            422
        );
        let elsewhere = log(event_id, "http://127.0.0.1:9/hook".to_string())
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
