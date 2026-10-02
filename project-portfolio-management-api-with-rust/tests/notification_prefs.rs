//! Notification preferences (PL-5), end to end over the real router with
//! real PASETO identities: opting a kind out, the default when nothing is
//! chosen, daily digests that list every member, and privacy between users.
//!
//! Its **own test binary**, for the same reason `enforcement.rs` is:
//! `verifier` is a process-wide `OnceLock`, read once on first use —
//! sharing the `tests/requests/mod.rs` binary would race whichever
//! test happens to touch an authenticated route first.
//!
//! `#[ignore]`d — boots the app, so it needs PostgreSQL via
//! `config/test.yaml` / `DATABASE_URL`. Run with
//! `cargo test --test notification_prefs -- --ignored`.
#![cfg(feature = "paseto")]

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use ed25519_dalek::SigningKey;
use loco_rs::testing::prelude::*;
use project_portfolio_management_service::app::App;
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
fn mint(kid: &str, sub: &str) -> String {
    let iat: i64 = 1_700_000_000;
    let payload = json!({
        "sub": sub,
        "email": format!("{sub}@example.com"), "name": sub,
        "iss": ISSUER, "aud": AUDIENCE,
        "exp": iat + 10_000_000_000_i64, "iat": iat,
        "sid": "test-sid", "attrs": {},
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

const ALICE: &str = "3f2c6b7e-0000-4000-8000-00000000000a";
const BOB: &str = "3f2c6b7e-0000-4000-8000-00000000000b";

/// PL-5 acceptance. A kind switched off writes nothing for that user and
/// leaves everyone else alone; a user with no preferences gets today's
/// behaviour; a daily digest collapses N events into one line that lists
/// all N; preferences are private to their owner; bad input is refused.
#[tokio::test]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test notification_prefs -- --ignored`"]
async fn notification_preferences_suppress_digest_and_stay_private() {
    let (keys, kid) = keys_and_kid();
    // SAFETY: this binary is single-test, so no other test races the
    // process-wide verifier `OnceLock` it sets.
    unsafe {
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS", keys.to_string());
    }
    let (ak, av) = auth_header(&mint(&kid, ALICE));
    let (bk, bv) = auth_header(&mint(&kid, BOB));
    let (ck, cv) = auth_header(&mint(&kid, "carol"));

    request::<App, _, _>(|request, _ctx| async move {
        let alice = format!("person:{ALICE}");
        let bob = format!("person:{BOB}");
        let plan: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Apollo" }))
            .await
            .json();
        let p = plan["pid"].as_str().expect("plan pid").to_string();
        let mut tasks = Vec::new();
        for title in ["one", "two"] {
            let t: Value = request
                .post(&format!("/api/plans/{p}/tasks"))
                .json(&json!({ "title": title }))
                .await
                .json();
            tasks.push(t["pid"].as_str().expect("task pid").to_string());
        }
        let assign = |task: String, who: String| {
            let (request, p) = (&request, p.clone());
            async move {
                request
                    .post(&format!("/api/plans/{p}/tasks/{task}/assign"))
                    .json(&json!({ "assignee_ref": who }))
                    .await
                    .assert_status_ok();
            }
        };
        let inbox = |k: axum::http::HeaderName, v: axum::http::HeaderValue| {
            let request = &request;
            async move {
                let r = request.get("/api/me/inbox?tz=UTC").add_header(k, v).await;
                r.assert_status_ok();
                r.json::<Value>()
            }
        };

        // Nothing chosen ⇒ today's behaviour: the assignment notifies.
        assign(tasks[0].clone(), alice.clone()).await;
        assert_eq!(
            inbox(ak.clone(), av.clone()).await["items"]
                .as_array()
                .unwrap()
                .len(),
            1
        );

        // Alice switches `task_assigned` off: she gets nothing more,
        // and Bob is unaffected.
        request
            .put("/api/me/notification-prefs")
            .add_header(ak.clone(), av.clone())
            .json(&json!({ "kind": "task_assigned", "enabled": false }))
            .await
            .assert_status_ok();
        assign(tasks[1].clone(), alice.clone()).await;
        assign(tasks[1].clone(), bob.clone()).await;
        let a = inbox(ak.clone(), av.clone()).await;
        assert_eq!(
            a["items"].as_array().unwrap().len(),
            1,
            "no new one for Alice"
        );
        let b = inbox(bk.clone(), bv.clone()).await;
        assert_eq!(
            b["items"].as_array().unwrap().len(),
            1,
            "Bob still notified"
        );

        // Reset restores the default.
        request
            .delete("/api/me/notification-prefs?kind=task_assigned")
            .add_header(ak.clone(), av.clone())
            .await
            .assert_status_ok();
        for _ in 0..2 {
            assign(tasks[0].clone(), bob.clone()).await;
            assign(tasks[0].clone(), alice.clone()).await;
        }
        let a = inbox(ak.clone(), av.clone()).await;
        assert_eq!(
            a["items"].as_array().unwrap().len(),
            3,
            "1 earlier + 2 new, instant"
        );

        // Daily digest: the three events on task one become ONE line that
        // lists all three.
        request
            .put("/api/me/notification-prefs")
            .add_header(ak.clone(), av.clone())
            .json(&json!({ "kind": "task_assigned", "digest": "daily" }))
            .await
            .assert_status_ok();
        let a = inbox(ak.clone(), av.clone()).await;
        let items = a["items"].as_array().unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0]["count"], 3);
        assert_eq!(
            items[0]["pids"].as_array().unwrap().len(),
            3,
            "every member listed"
        );
        assert_eq!(items[0]["digest"], true);
        assert_eq!(
            a["unread_count"], 3,
            "unread counts individual notifications"
        );

        // Privacy: Bob sees none of Alice's preferences.
        let bp: Value = request
            .get("/api/me/notification-prefs")
            .add_header(bk.clone(), bv.clone())
            .await
            .json();
        assert!(bp["prefs"].as_array().unwrap().is_empty());
        let ap: Value = request
            .get("/api/me/notification-prefs")
            .add_header(ak.clone(), av.clone())
            .await
            .json();
        assert_eq!(ap["prefs"].as_array().unwrap().len(), 1);
        assert_eq!(ap["default"]["enabled"], true);

        // Refusals.
        for body in [
            json!({ "kind": "nope" }),
            json!({ "kind": "comment", "digest": "weekly" }),
            json!({ "kind": "comment", "channel": "sms" }),
        ] {
            assert_eq!(
                request
                    .put("/api/me/notification-prefs")
                    .add_header(ak.clone(), av.clone())
                    .json(&body)
                    .await
                    .status_code(),
                422,
                "{body}"
            );
        }
        assert_eq!(
            request
                .put("/api/me/notification-prefs")
                .add_header(ck, cv)
                .json(&json!({ "kind": "comment", "enabled": false }))
                .await
                .status_code(),
            422,
            "a non-UUID sub cannot be addressed, so its preference could never apply"
        );
        assert_eq!(request.get("/api/me/inbox?tz=UTC").await.status_code(), 401);
        assert_eq!(
            request
                .get("/api/me/inbox?tz=%2B02:00")
                .add_header(ak, av)
                .await
                .status_code(),
            422
        );
    })
    .await;
}
