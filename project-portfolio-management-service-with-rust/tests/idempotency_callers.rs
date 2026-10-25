//! BR-1: idempotency keys belong to the caller, and a replay is decided after the guards. Two writers using the same
//! key do not collide and each replays only their own answer; a caller the guards refuse (a read-only token, no token
//! at all) is refused even when the request is an exact replay of someone else's.
//!
//! Its **own test binary** (the verifier is a process-wide `OnceLock`).
//! `#[ignore]`d — needs PostgreSQL. `cargo test --test idempotency_callers -- --ignored`.

#![cfg(feature = "paseto")]

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use ed25519_dalek::SigningKey;
use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use rusty_paseto::core::{Footer, Key, Paseto, PasetoAsymmetricPrivateKey, Payload, Public, V4};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

const SEED: [u8; 32] = [71; 32];
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
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test idempotency_callers -- --ignored`"]
async fn keys_are_per_caller_and_a_replay_never_skips_the_guards() {
    let (keys, kid) = keys_and_kid();
    // SAFETY: this binary is single-test, so nothing races the process-wide verifier.
    unsafe {
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS", keys.to_string());
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH", "1");
    }
    let ana = auth_header(&mint(
        &kid,
        "3f2c6b7e-0000-4000-8000-0000000000a1",
        &json!({ "access": ["write"] }),
    ));
    let bo = auth_header(&mint(
        &kid,
        "3f2c6b7e-0000-4000-8000-0000000000b2",
        &json!({ "access": ["write"] }),
    ));
    let ana_read_only = auth_header(&mint(
        &kid,
        "3f2c6b7e-0000-4000-8000-0000000000a1",
        &json!({ "access": ["write"], "scope": ["readonly"] }),
    ));
    request::<App, _, _>(|request, _ctx| async move {
        let post = |who: &(axum::http::HeaderName, axum::http::HeaderValue), name: &str| {
            request
                .post("/api/plans")
                .add_header(who.0.clone(), who.1.clone())
                .add_header("Idempotency-Key", "shared-key")
                .json(&json!({ "kind": "Project", "name": name }))
        };
        // Two callers, one key, different requests: both run.
        let a: Value = post(&ana, "Ana's plan").await.json();
        let b: Value = post(&bo, "Bo's plan").await.json();
        assert_ne!(a["pid"], b["pid"]);
        // Each replays only their own.
        let a2 = post(&ana, "Ana's plan").await;
        assert_eq!(a2.header("idempotent-replayed"), "true");
        assert_eq!(a2.json::<Value>()["pid"], a["pid"]);
        let b2 = post(&bo, "Bo's plan").await;
        assert_eq!(b2.json::<Value>()["pid"], b["pid"]);
        // Ana's key is not Bo's: Bo cannot replay Ana's answer by sending her request under it.
        assert_eq!(post(&bo, "Ana's plan").await.status_code(), 409);

        // An exact replay by a caller the guards refuse is refused, not answered from the store.
        assert_eq!(post(&ana_read_only, "Ana's plan").await.status_code(), 403);
        let anonymous = request
            .post("/api/plans")
            .add_header("Idempotency-Key", "shared-key")
            .json(&json!({ "kind": "Project", "name": "Ana's plan" }))
            .await;
        assert_eq!(anonymous.status_code(), 401);
    })
    .await;
}
