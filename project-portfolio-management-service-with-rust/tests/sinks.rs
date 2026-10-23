//! Notification sinks (LT-6, RM-11), end to end with real PASETO identities
//! and a real local HTTP receiver: enqueue-then-send, retries, per-channel
//! preferences, privacy between users, masked secrets, SSRF refusal, email.
//!
//! Its **own test binary**, for the same reason `enforcement.rs` is:
//! `verifier` is a process-wide `OnceLock`, read once on first use —
//! sharing the `tests/requests/mod.rs` binary would race whichever
//! test happens to touch an authenticated route first.
//!
//! `#[ignore]`d — boots the app, so it needs PostgreSQL via
//! `config/test.yaml` / `DATABASE_URL`. Run with
//! `cargo test --test sinks -- --ignored`.

#![expect(
    clippy::too_many_lines,
    reason = "request-level test: long, linear, short names; recorded under CQ-14"
)]
#![cfg(feature = "paseto")]

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use ed25519_dalek::SigningKey;
use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use rusty_paseto::core::{Footer, Key, Paseto, PasetoAsymmetricPrivateKey, Payload, Public, V4};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

const SEED: [u8; 32] = [31; 32];
const ISSUER: &str = "authentication-service";
const AUDIENCE: &str = "main-x-service";

/// The published Ed25519 key set plus its `kid`, derived the way the
/// verifier derives it (SHA-256 of the public key, base64url).
fn keys_and_kid() -> (Value, String) {
    let public = SigningKey::from_bytes(&SEED).verifying_key().to_bytes();
    let kid = URL_SAFE_NO_PAD.encode(Sha256::digest(public));
    let keys = json!({
        "keys": [{ "kty": "OKP", "crv": "Ed25519", "use": "sig",
                   "kid": kid, "x": URL_SAFE_NO_PAD.encode(public) }]
    });
    (keys, kid)
}

/// Mint a `v4.public` token for `sub`.
fn mint(kid: &str, sub: &str, access: &[&str]) -> String {
    let iat: i64 = 1_700_000_000;
    let payload = json!({
        "sub": sub,
        "email": format!("{sub}@example.com"), "name": sub,
        "iss": ISSUER, "aud": AUDIENCE,
        "exp": iat + 10_000_000_000_i64, "iat": iat,
        "sid": "test-sid", "attrs": { "access": access },
    })
    .to_string();
    let keypair = SigningKey::from_bytes(&SEED).to_keypair_bytes();
    let key = Key::<64>::from(keypair);
    let private = PasetoAsymmetricPrivateKey::<V4, Public>::from(&key);
    let footer = format!(r#"{{"kid":"{kid}"}}"#);
    let mut builder = Paseto::<V4, Public>::builder();
    builder.set_payload(Payload::from(payload.as_str()));
    builder.set_footer(Footer::from(footer.as_str()));
    builder.try_sign(&private).expect("sign")
}

fn auth_header(token: &str) -> (axum::http::HeaderName, axum::http::HeaderValue) {
    (
        axum::http::header::AUTHORIZATION,
        format!("Bearer {token}").parse().unwrap(),
    )
}

use std::sync::atomic::{AtomicU16, Ordering};
use std::sync::{Arc, Mutex};

const ALICE: &str = "3f2c6b7e-0000-4000-8000-00000000000a";
const BOB: &str = "3f2c6b7e-0000-4000-8000-00000000000b";

type Seen = Arc<Mutex<Vec<(String, axum::http::HeaderMap, String)>>>;

/// A local receiver: records `(path, headers, body)` and answers with whatever
/// status the test currently sets.
async fn receiver(status: Arc<AtomicU16>) -> (String, Seen) {
    let seen: Seen = Arc::new(Mutex::new(Vec::new()));
    let sink = seen.clone();
    let app = axum::Router::new().fallback(move |req: axum::extract::Request| {
        let (sink, status) = (sink.clone(), status.clone());
        async move {
            let path = req.uri().path().to_string();
            let headers = req.headers().clone();
            let body = axum::body::to_bytes(req.into_body(), 1 << 20)
                .await
                .unwrap_or_default();
            sink.lock()
                .unwrap()
                .push((path, headers, String::from_utf8_lossy(&body).to_string()));
            axum::http::StatusCode::from_u16(status.load(Ordering::SeqCst)).unwrap()
        }
    });
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    tokio::spawn(async move { axum::serve(listener, app).await.ok() });
    (base, seen)
}

/// LT-6 / RM-11 acceptance.
#[tokio::test]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test sinks -- --ignored`"]
async fn deliveries_are_enqueued_sent_retried_filtered_and_private() {
    let (keys, kid) = keys_and_kid();
    let status = Arc::new(AtomicU16::new(200));
    let (base, seen) = receiver(status.clone()).await;
    // SAFETY: this binary is single-test, so no other test races the
    // process-wide verifier `OnceLock` or these variables.
    unsafe {
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS", keys.to_string());
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_SINKS_ALLOW_LOOPBACK", "1");
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_SINKS_TICK_SECS", "0");
        std::env::set_var(
            "PROJECT_PORTFOLIO_MANAGEMENT_EMAIL_FROM",
            "noreply@ppm.example.com",
        );
    }
    let (ak, av) = auth_header(&mint(&kid, ALICE, &["write"]));
    let (bk, bv) = auth_header(&mint(&kid, BOB, &["write"]));
    let (ck, cv) = auth_header(&mint(&kid, "carol", &["write"]));

    request::<App, _, _>(|request, ctx| async move {
        let plan: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Apollo" }))
            .await
            .json();
        let p = plan["pid"].as_str().expect("plan pid").to_string();
        let assign_to = |who: &str| {
            let (request, p) = (&request, p.clone());
            let body = json!({ "assignee_ref": format!("person:{who}") });
            async move {
                let t: Value = request.post(&format!("/api/plans/{p}/tasks")).json(&json!({ "title": "Ship" })).await.json();
                let t = t["pid"].as_str().unwrap().to_string();
                request.post(&format!("/api/plans/{p}/tasks/{t}/assign")).json(&body).await.assert_status_ok();
            }
        };
        let add = |k: axum::http::HeaderName, v: axum::http::HeaderValue, body: Value| {
            let request = &request;
            async move { request.post("/api/me/notification-sinks").add_header(k, v).json(&body).await }
        };

        // Refusals: an internal address, a plain-http public host, a name that
        // is internal, telegram without a chat id, and an identity with no user id.
        for bad in [
            json!({ "kind": "slack", "destination": "https://10.0.0.5/hook" }),
            json!({ "kind": "slack", "destination": "https://169.254.169.254/latest" }),
            json!({ "kind": "webhook", "destination": "http://hooks.example.com/x" }),
            json!({ "kind": "slack", "destination": "https://db.internal/x" }),
            json!({ "kind": "telegram", "destination": "https://api.telegram.org/bot1:x/sendMessage" }),
            json!({ "kind": "email", "destination": "a@example.com\r\nBcc: x@y.z" }),
            json!({ "kind": "fax", "destination": "x" }),
        ] {
            assert_eq!(add(ak.clone(), av.clone(), bad.clone()).await.status_code(), 422, "{bad}");
        }
        assert_eq!(add(ck.clone(), cv.clone(), json!({ "kind": "email", "destination": "c@example.com" })).await.status_code(), 422, "non-UUID sub");
        assert_eq!(request.post("/api/me/notification-sinks").json(&json!({})).await.status_code(), 401);

        // Register a webhook, a slack sink and (for Bob) a webhook of his own.
        let hook: Value = add(ak.clone(), av.clone(), json!({ "kind": "webhook", "destination": format!("{base}/hook") })).await.json();
        let slack: Value = add(ak.clone(), av.clone(), json!({ "kind": "slack", "destination": format!("{base}/slack/SECRET9999") })).await.json();
        add(bk.clone(), bv.clone(), json!({ "kind": "webhook", "destination": format!("{base}/bobs-hook") })).await.assert_status_ok();
        assert!(slack["destination"].as_str().unwrap().ends_with("…9999"), "masked: {slack}");
        assert!(!slack["destination"].as_str().unwrap().contains("SECRET"), "never the full secret");
        let listed: Value = request.get("/api/me/notification-sinks").add_header(ak.clone(), av.clone()).await.json();
        assert_eq!(listed.as_array().unwrap().len(), 2, "only Alice's own");
        assert!(!format!("{listed}").contains("SECRET"));

        // A notification ENQUEUES; nothing is sent in its path.
        assign_to(ALICE).await;
        assert!(seen.lock().unwrap().is_empty(), "no delivery happened inside the notification");
        let sent = ctx_run(&ctx).await;
        assert_eq!(sent.sent, 2, "webhook + slack");
        {
            let log = seen.lock().unwrap();
            assert_eq!(log.len(), 2);
            let hook_req = log.iter().find(|(path, _, _)| path == "/hook").expect("webhook");
            assert!(hook_req.1.get("x-mxi-event-id").is_some());
            let body: Value = serde_json::from_str(&hook_req.2).unwrap();
            assert_eq!(body["kind"], "task_assigned");
            let slack_req = log.iter().find(|(path, _, _)| path.starts_with("/slack")).expect("slack");
            assert!(slack_req.2.starts_with(r#"{"text":"You were assigned"#), "{}", slack_req.2);
            assert!(!log.iter().any(|(path, _, _)| path == "/bobs-hook"), "Bob's sink got none of Alice's notifications");
        }

        // A failing destination is retried with backoff and never lost.
        seen.lock().unwrap().clear();
        status.store(500, Ordering::SeqCst);
        assign_to(ALICE).await;
        let r = ctx_run(&ctx).await;
        assert_eq!((r.sent, r.retrying), (0, 2));
        let sid = hook["pid"].as_str().unwrap();
        let deliveries: Value = request
            .get(&format!("/api/me/notification-sinks/{sid}/deliveries"))
            .add_header(ak.clone(), av.clone())
            .await
            .json();
        assert_eq!(deliveries[0]["status"], "pending");
        assert_eq!(deliveries[0]["attempts"], 1);
        assert!(deliveries[0]["last_error"].as_str().unwrap().starts_with("HTTP 500"));
        // Not due yet: another pass sends nothing.
        assert_eq!(ctx_run(&ctx).await.retrying + ctx_run(&ctx).await.sent, 0);
        // Make them due, recover the destination, and they go through.
        {
            use project_portfolio_management::models::_entities::sink_deliveries;
            use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, sea_query::Expr};
            sink_deliveries::Entity::update_many()
                .col_expr(sink_deliveries::Column::NextAttemptAt, Expr::value(chrono::Utc::now() - chrono::Duration::minutes(1)))
                .filter(sink_deliveries::Column::Status.eq("pending"))
                .exec(&ctx.db)
                .await
                .unwrap();
        }
        status.store(200, Ordering::SeqCst);
        assert_eq!(ctx_run(&ctx).await.sent, 2, "delivered on the retry");

        // Per-channel preferences: chat switched off for this kind ⇒ only the webhook.
        seen.lock().unwrap().clear();
        request
            .put("/api/me/notification-prefs")
            .add_header(ak.clone(), av.clone())
            .json(&json!({ "kind": "task_assigned", "channel": "chat", "enabled": false }))
            .await
            .assert_status_ok();
        assign_to(ALICE).await;
        assert_eq!(ctx_run(&ctx).await.sent, 1);
        assert!(seen.lock().unwrap().iter().all(|(path, _, _)| path == "/hook"), "no chat message");

        // Email: delivered through the mailer when configured, and carries only the text.
        let before = ctx.mailer.as_ref().expect("a stub mailer in test").deliveries().count;
        add(ak.clone(), av.clone(), json!({ "kind": "email", "destination": "alice@example.com" })).await.assert_status_ok();
        assign_to(ALICE).await;
        let r = ctx_run(&ctx).await;
        assert!(r.sent >= 2, "webhook + email: {r:?}");
        let after = ctx.mailer.as_ref().unwrap().deliveries();
        assert_eq!(after.count, before + 1);
        assert!(after.messages.last().unwrap().contains("You were assigned"));

        // Removing a sink (POST twin) stops its deliveries; Bob cannot remove Alice's.
        assert_eq!(
            request.post(&format!("/api/me/notification-sinks/{sid}/delete")).add_header(bk.clone(), bv.clone()).await.status_code(),
            404
        );
        request.post(&format!("/api/me/notification-sinks/{sid}/delete")).add_header(ak.clone(), av.clone()).await.assert_status_ok();
        seen.lock().unwrap().clear();
        assign_to(ALICE).await;
        ctx_run(&ctx).await;
        assert!(seen.lock().unwrap().iter().all(|(path, _, _)| path != "/hook"), "the removed webhook gets nothing");
    })
    .await;
}

async fn ctx_run(
    ctx: &loco_rs::app::AppContext,
) -> project_portfolio_management::controllers::sinks::Dispatched {
    project_portfolio_management::controllers::sinks::run_due(ctx, 100)
        .await
        .expect("dispatch")
}
