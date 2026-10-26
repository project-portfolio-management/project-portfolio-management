//! BR-12: a token that carries `scope=capture` reads, searches and posts a status update, a comment and a lesson, and
//! is refused every other write, whatever else it holds; the same identity without the scope is unchanged. Real signed tokens through the real guard.
//!
//! Its **own test binary** (the verifier is a process-wide `OnceLock`).
//! `#[ignore]`d — needs PostgreSQL. `cargo test --test capture_scope -- --ignored`.

#![cfg(feature = "paseto")]

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use ed25519_dalek::SigningKey;
use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use rusty_paseto::core::{Footer, Key, Paseto, PasetoAsymmetricPrivateKey, Payload, Public, V4};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

const SEED: [u8; 32] = [72; 32];
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
fn mint(kid: &str, sub: &str, attrs: &serde_json::Value) -> String {
    let iat: i64 = 1_700_000_000;
    let payload = json!({
        "sub": sub,
        "email": format!("{sub}@example.com"), "name": sub,
        "iss": ISSUER, "aud": AUDIENCE,
        "exp": iat + 10_000_000_000_i64, "iat": iat,
        "sid": "test-sid", "attrs": attrs,
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

const SUB: &str = "3f2c6b7e-0000-4000-8000-0000000000ad";

#[tokio::test]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test capture_scope -- --ignored`"]
async fn a_capture_token_posts_three_things_and_is_refused_the_rest() {
    let (keys, kid) = keys_and_kid();
    // SAFETY: this binary is single-test, so nothing races the process-wide verifier.
    unsafe {
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS", keys.to_string());
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH", "1");
    }
    let admin = auth_header(&mint(&kid, SUB, &json!({ "access": ["admin"] })));
    let capture = auth_header(&mint(
        &kid,
        SUB,
        &json!({ "access": ["admin"], "scope": ["capture"] }),
    ));
    request::<App, _, _>(|request, _ctx| async move {
        let made: Value = request
            .post("/api/plans")
            .add_header(admin.0.clone(), admin.1.clone())
            .json(&json!({ "kind": "Project", "name": "Capture me" }))
            .await
            .json();
        let pid = made["pid"].as_str().expect("pid").to_string();
        let c = &capture;

        // Reads and search work.
        request.get("/api/plans").add_header(c.0.clone(), c.1.clone()).await.assert_status_ok();
        request.get(&format!("/api/plans/{pid}")).add_header(c.0.clone(), c.1.clone()).await.assert_status_ok();

        // The three posts work.
        request
            .post(&format!("/api/plans/{pid}/status-updates"))
            .add_header(c.0.clone(), c.1.clone())
            .json(&json!({ "status": "on_track", "narrative": "Captured" }))
            .await
            .assert_status_ok();
        request
            .post(&format!("/api/plans/{pid}/lessons"))
            .add_header(c.0.clone(), c.1.clone())
            .json(&json!({ "occasion": "closure", "category": "went_well", "summary": "Captured lesson" }))
            .await
            .assert_status_ok();
        request
            .post("/api/comments")
            .add_header(c.0.clone(), c.1.clone())
            .json(&json!({ "subject_kind": "plan", "subject_pid": pid, "body": "Captured comment" }))
            .await
            .assert_status_ok();

        // Everything else is refused: create, edit, delete, a task, a template route.
        request.post("/api/plans").add_header(c.0.clone(), c.1.clone())
            .json(&json!({ "kind": "Project", "name": "Nope" })).await.assert_status_forbidden();
        request.put(&format!("/api/plans/{pid}")).add_header(c.0.clone(), c.1.clone())
            .json(&json!({ "kind": "Project", "name": "Renamed" })).await.assert_status_forbidden();
        request.delete(&format!("/api/plans/{pid}")).add_header(c.0.clone(), c.1.clone())
            .await.assert_status_forbidden();
        request.post(&format!("/api/plans/{pid}/tasks")).add_header(c.0.clone(), c.1.clone())
            .json(&json!({ "title": "Nope" })).await.assert_status_forbidden();
        request.post(&format!("/api/plan-templates/{pid}/revise")).add_header(c.0.clone(), c.1.clone())
            .json(&json!({})).await.assert_status_forbidden();

        // The plan is as it was, and the same identity without the scope is unchanged.
        let after: Value = request.get(&format!("/api/plans/{pid}"))
            .add_header(admin.0.clone(), admin.1.clone()).await.json();
        assert_eq!(after["name"], "Capture me");
        request.post("/api/plans").add_header(admin.0.clone(), admin.1.clone())
            .json(&json!({ "kind": "Project", "name": "Allowed" })).await.assert_status_ok();
    })
    .await;
}
