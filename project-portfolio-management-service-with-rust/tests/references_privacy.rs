//! BR-4: the references of a record in a private plan are a `404` for a non-member, however the request is phrased,
//! and the member sees them. Real signed tokens through the real guard.
//!
//! Its **own test binary** (the verifier is a process-wide `OnceLock`).
//! `#[ignore]`d — needs PostgreSQL. `cargo test --test references_privacy -- --ignored`.

#![cfg(feature = "paseto")]

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use ed25519_dalek::SigningKey;
use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use rusty_paseto::core::{Footer, Key, Paseto, PasetoAsymmetricPrivateKey, Payload, Public, V4};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

const SEED: [u8; 32] = [73; 32];
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

#[tokio::test]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test references_privacy -- --ignored`"]
async fn a_private_plans_references_are_not_found_for_a_non_member() {
    let (keys, kid) = keys_and_kid();
    // SAFETY: this binary is single-test, so nothing races the process-wide verifier.
    unsafe {
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS", keys.to_string());
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH", "1");
    }
    let alice = auth_header(&mint(
        &kid,
        "3f2c6b7e-0000-4000-8000-0000000000a1",
        &json!({ "access": ["write"] }),
    ));
    let bob = auth_header(&mint(
        &kid,
        "3f2c6b7e-0000-4000-8000-0000000000b2",
        &json!({ "access": ["write"] }),
    ));
    request::<App, _, _>(|request, _ctx| async move {
        let plan: Value = request
            .post("/api/plans")
            .add_header(alice.0.clone(), alice.1.clone())
            .json(&json!({ "kind": "Project", "name": "Secret" }))
            .await
            .json();
        let plan = plan["pid"].as_str().unwrap().to_string();
        let task: Value = request
            .post(&format!("/api/plans/{plan}/tasks"))
            .add_header(alice.0.clone(), alice.1.clone())
            .json(&json!({ "title": "Hidden task" }))
            .await
            .json();
        let task = task["pid"].as_str().unwrap().to_string();
        request
            .post(&format!("/api/plans/{plan}/lessons"))
            .add_header(alice.0.clone(), alice.1.clone())
            .json(
                &json!({ "occasion": "closure", "category": "went_well", "summary": "Hidden lesson",
                "link": { "kind": "task", "pid": task } }),
            )
            .await
            .assert_status_ok();
        let url = format!("/api/plans/{plan}/references?kind=task&pid={task}");
        // Public: Bob sees it. Then private: Bob sees nothing, not even that the record exists.
        request
            .get(&url)
            .add_header(bob.0.clone(), bob.1.clone())
            .await
            .assert_status_ok();
        request
            .put(&format!("/api/plans/{plan}/access"))
            .add_header(alice.0.clone(), alice.1.clone())
            .json(&json!({ "private": true, "members": [] }))
            .await
            .assert_status_ok();
        let seen = request
            .get(&url)
            .add_header(bob.0.clone(), bob.1.clone())
            .await;
        assert_eq!(seen.status_code(), 404);
        assert!(!seen.text().contains("Hidden"), "{}", seen.text());
        let mine: Value = request
            .get(&url)
            .add_header(alice.0.clone(), alice.1.clone())
            .await
            .json();
        assert_eq!(mine["count"], 1);
    })
    .await;
}
