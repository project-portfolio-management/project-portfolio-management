//! RE-1: a private plan's public product roadmap is a `404` to a non-member, read or written, and the member sees
//! it. Real signed tokens through the real guard.
//!
//! Its **own test binary** (the verifier is a process-wide `OnceLock`).
//! `#[ignore]`d — needs PostgreSQL. `cargo test --test public_roadmap_privacy -- --ignored`.

#![cfg(feature = "paseto")]

use axa_test::prelude::*;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use ed25519_dalek::SigningKey;
use project_portfolio_management::app::App;
use rusty_paseto::core::{Footer, Key, Paseto, PasetoAsymmetricPrivateKey, Payload, Public, V4};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

const SEED: [u8; 32] = [79; 32];
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
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test --test public_roadmap_privacy -- --ignored`"]
async fn a_private_plans_roadmap_is_not_found_for_a_non_member() {
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
        let tag = uuid::Uuid::new_v4().simple().to_string()[..6].to_string();
        let plan: Value = request
            .post("/api/plans")
            .add_header(alice.0.clone(), alice.1.clone())
            .json(&json!({ "kind": "Product", "name": format!("Secret product {tag}") }))
            .await
            .json();
        let plan = plan["pid"].as_str().unwrap().to_string();
        let url = format!("/api/plans/{plan}/roadmap");
        request
            .put(&url)
            .add_header(alice.0.clone(), alice.1.clone())
            .json(&json!({ "title": format!("Hidden roadmap {tag}") }))
            .await
            .assert_status_ok();
        let item: Value = request
            .post(&format!("{url}/items"))
            .add_header(alice.0.clone(), alice.1.clone())
            .json(&json!({ "title": "Hidden item", "placement": "now" }))
            .await
            .json();
        let item = item["pid"].as_str().unwrap().to_string();
        // Public: Bob reads it. Then private: Bob gets nothing, read or write.
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
        for (method, path, body) in [
            ("put", url.clone(), json!({ "published": true })),
            (
                "post",
                format!("{url}/items"),
                json!({ "title": "Planted" }),
            ),
            (
                "post",
                format!("{url}/items/{item}/move"),
                json!({ "placement": "later" }),
            ),
            (
                "put",
                format!("{url}/items/{item}/translations/cy"),
                json!({ "title": "Cudd" }),
            ),
        ] {
            let r = if method == "put" {
                request
                    .put(&path)
                    .add_header(bob.0.clone(), bob.1.clone())
                    .json(&body)
                    .await
            } else {
                request
                    .post(&path)
                    .add_header(bob.0.clone(), bob.1.clone())
                    .json(&body)
                    .await
            };
            assert_eq!(r.status_code(), 404, "{method} {path}");
        }
        let mine: Value = request
            .get(&url)
            .add_header(alice.0.clone(), alice.1.clone())
            .await
            .json();
        assert_eq!(mine["published"], false, "Bob's write did not land");
        assert_eq!(mine["items"].as_array().unwrap().len(), 1);
        assert_eq!(mine["items"][0]["placement"], "now");
        // RE-3: the list of stale roadmaps leaves the private plan out for Bob, and keeps it for Alice.
        let listed = |v: &Value| {
            v["roadmaps"]
                .as_array()
                .unwrap()
                .iter()
                .any(|r| r["plan_pid"] == plan.as_str())
        };
        let bobs: Value = request
            .get("/api/public-roadmaps/stale")
            .add_header(bob.0.clone(), bob.1.clone())
            .await
            .json();
        assert!(!listed(&bobs), "{bobs}");
        let alices: Value = request
            .get("/api/public-roadmaps/stale")
            .add_header(alice.0.clone(), alice.1.clone())
            .await
            .json();
        assert!(listed(&alices), "never reviewed, so listed: {alices}");
    })
    .await;
}
