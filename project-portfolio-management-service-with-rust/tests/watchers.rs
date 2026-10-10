//! Watchers (RM-3), end to end over the real router with real PASETO
//! identities: following, notification fan-out, auto-watch and its opt-out.
//!
//! Its **own test binary**, for the same reason `enforcement.rs` is:
//! `verifier` is a process-wide `OnceLock`, read once on first use —
//! sharing the `tests/requests/mod.rs` binary would race whichever
//! test happens to touch an authenticated route first.
//!
//! `#[ignore]`d — boots the app, so it needs PostgreSQL via
//! `config/test.yaml` / `DATABASE_URL`. Run with
//! `cargo test --test watchers -- --ignored`.
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
const CAROL: &str = "3f2c6b7e-0000-4000-8000-00000000000c";

/// RM-3 acceptance: watching is idempotent; a watcher is told once of a
/// comment and of a status move, never the actor; auto-watch follows what a
/// user creates, is assigned or comments on, and `auto_watch` off stops it;
/// a user's watch list and other watchers stay private.
#[tokio::test]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test watchers -- --ignored`"]
async fn watchers_are_told_once_and_the_actor_never() {
    let (keys, kid) = keys_and_kid();
    // SAFETY: this binary is single-test, so no other test races the
    // process-wide verifier `OnceLock` it sets.
    unsafe {
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS", keys.to_string());
    }
    let (ak, av) = auth_header(&mint(&kid, ALICE));
    let (bk, bv) = auth_header(&mint(&kid, BOB));
    let (ck, cv) = auth_header(&mint(&kid, CAROL));

    request::<App, _, _>(|request, _ctx| async move {
        let plan: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Apollo" }))
            .await
            .json();
        let p = plan["pid"].as_str().expect("plan pid").to_string();

        let inbox = |k: axum::http::HeaderName, v: axum::http::HeaderValue| {
            let request = &request;
            async move {
                let r: Value = request
                    .get("/api/me/inbox?tz=UTC")
                    .add_header(k, v)
                    .await
                    .json();
                r["items"].as_array().expect("items").len()
            }
        };
        let watching = |k: axum::http::HeaderName, v: axum::http::HeaderValue| {
            let request = &request;
            async move {
                let r: Value = request
                    .get("/api/me/watching")
                    .add_header(k, v)
                    .await
                    .json();
                r["count"].as_u64().expect("count")
            }
        };

        // Bob creates a task: auto-watch.
        let task: Value = request
            .post(&format!("/api/plans/{p}/tasks"))
            .add_header(bk.clone(), bv.clone())
            .json(&json!({ "title": "Ship" }))
            .await
            .json();
        let t = task["pid"].as_str().expect("task pid").to_string();
        assert_eq!(
            watching(bk.clone(), bv.clone()).await,
            1,
            "the creator follows it"
        );

        // Alice follows (idempotent) and sees counts without names.
        let w = json!({ "subject_kind": "task", "subject_pid": t });
        let first: Value = request
            .post("/api/watchers")
            .add_header(ak.clone(), av.clone())
            .json(&w)
            .await
            .json();
        let again: Value = request
            .post("/api/watchers")
            .add_header(ak.clone(), av.clone())
            .json(&w)
            .await
            .json();
        assert_eq!(
            (first["created"].as_bool(), again["created"].as_bool()),
            (Some(true), Some(false))
        );
        let st: Value = request
            .get(&format!("/api/watchers?subject_kind=task&subject_pid={t}"))
            .add_header(ak.clone(), av.clone())
            .await
            .json();
        assert_eq!(st["count"], 2);
        assert_eq!(st["mine"], true);
        assert!(
            st.get("watchers").is_none(),
            "other watchers are never named"
        );

        // Bob moves the task: Alice is told once, Bob never.
        request
            .patch(&format!("/api/plans/{p}/tasks/{t}"))
            .add_header(bk.clone(), bv.clone())
            .json(&json!({ "status": "in_progress" }))
            .await
            .assert_status_ok();
        assert_eq!(inbox(ak.clone(), av.clone()).await, 1);
        assert_eq!(
            inbox(bk.clone(), bv.clone()).await,
            0,
            "the actor is never told"
        );

        // Alice comments: Bob (a watcher) is told, Alice is not.
        request
            .post("/api/comments")
            .add_header(ak.clone(), av.clone())
            .json(&json!({ "subject_kind": "task", "subject_pid": t, "body": "looks good" }))
            .await
            .assert_status_ok();
        assert_eq!(inbox(bk.clone(), bv.clone()).await, 1);
        assert_eq!(
            inbox(ak.clone(), av.clone()).await,
            1,
            "no self-notification"
        );

        // Bob replies: Alice is both a prior commenter AND a watcher, yet is
        // told exactly once.
        request
            .post("/api/comments")
            .add_header(bk.clone(), bv.clone())
            .json(&json!({ "subject_kind": "task", "subject_pid": t, "body": "thanks" }))
            .await
            .assert_status_ok();
        assert_eq!(
            inbox(ak.clone(), av.clone()).await,
            2,
            "exactly one more, not two"
        );

        // Unfollow: a later move tells Alice nothing.
        // The POST twin: a write-level user cannot DELETE under the default policy.
        request
            .post("/api/watchers/unwatch")
            .add_header(ak.clone(), av.clone())
            .json(&w)
            .await
            .assert_status_ok();
        let before = inbox(ak.clone(), av.clone()).await;
        request
            .patch(&format!("/api/plans/{p}/tasks/{t}"))
            .add_header(bk.clone(), bv.clone())
            .json(&json!({ "status": "in_review" }))
            .await
            .assert_status_ok();
        assert_eq!(inbox(ak.clone(), av.clone()).await, before);

        // Auto-watch opt-out: Carol switches it off, then creates a task.
        request
            .put("/api/me/notification-prefs")
            .add_header(ck.clone(), cv.clone())
            .json(&json!({ "kind": "auto_watch", "enabled": false }))
            .await
            .assert_status_ok();
        let t2: Value = request
            .post(&format!("/api/plans/{p}/tasks"))
            .add_header(ck.clone(), cv.clone())
            .json(&json!({ "title": "Carol's" }))
            .await
            .json();
        let t2 = t2["pid"].as_str().expect("task pid").to_string();
        assert_eq!(
            watching(ck.clone(), cv.clone()).await,
            0,
            "opted out of auto-watch"
        );

        // Being assigned follows (Alice has not opted out).
        request
            .post(&format!("/api/plans/{p}/tasks/{t2}/assign"))
            .json(&json!({ "assignee_ref": format!("person:{ALICE}") }))
            .await
            .assert_status_ok();
        assert_eq!(
            watching(ak.clone(), av.clone()).await,
            1,
            "assignment follows the task"
        );

        // Refusals.
        assert_eq!(
            request.post("/api/watchers").json(&w).await.status_code(),
            401
        );
        assert_eq!(
            request
                .post("/api/watchers")
                .add_header(ak, av)
                .json(&json!({ "subject_kind": "nope", "subject_pid": t }))
                .await
                .status_code(),
            422
        );
    })
    .await;
}
