//! Availability access (PJ-3), end to end with real PASETO identities: a
//! person reads and sets their own ranges; a resource manager or an admin may for
//! anyone; everyone else gets 404.
//!
//! Its **own test binary**, for the same reason `enforcement.rs` is:
//! `verifier` is a process-wide `OnceLock`, read once on first use —
//! sharing the `tests/requests/mod.rs` binary would race whichever
//! test happens to touch an authenticated route first.
//!
//! `#[ignore]`d — boots the app, so it needs PostgreSQL via
//! `config/test.yaml` / `DATABASE_URL`. Run with
//! `cargo test --test availability_acl -- --ignored`.
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
fn mint(kid: &str, sub: &str, access: &[&str], role: &[&str]) -> String {
    let iat: i64 = 1_700_000_000;
    let payload = json!({
        "sub": sub,
        "email": format!("{sub}@example.com"), "name": sub,
        "iss": ISSUER, "aud": AUDIENCE,
        "exp": iat + 10_000_000_000_i64, "iat": iat,
        "sid": "test-sid", "attrs": { "access": access, "role": role },
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

#[tokio::test]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test availability_acl -- --ignored`"]
async fn availability_is_private_to_the_person_and_their_managers() {
    let (keys, kid) = keys_and_kid();
    // SAFETY: this binary is single-test, so nothing races the process-wide verifier.
    unsafe {
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS", keys.to_string());
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH", "1");
    }
    let (ak, av) = auth_header(&mint(&kid, ALICE, &["write"], &[]));
    let (bk, bv) = auth_header(&mint(&kid, BOB, &["write"], &[]));
    let (rk, rv) = auth_header(&mint(&kid, "manager", &["write"], &["resource_manager"]));
    let alice = format!("person:{ALICE}");
    request::<App, _, _>(|request, _ctx| async move {
        let body = json!({ "person_ref": alice, "from": "2026-10-12", "to": "2026-10-18", "max_percent": 40 });
        // Bob cannot set Alice's; Alice can; a manager can set another range for her.
        assert_eq!(request.post("/api/availability").add_header(bk.clone(), bv.clone()).json(&body).await.status_code(), 404);
        let mine: Value = request.post("/api/availability").add_header(ak.clone(), av.clone()).json(&body).await.json();
        request
            .post("/api/availability")
            .add_header(rk.clone(), rv.clone())
            .json(&json!({ "person_ref": alice, "from": "2026-11-02", "to": "2026-11-06", "max_percent": 0, "note": "leave" }))
            .await
            .assert_status_ok();

        // Reading: Alice her own (the default), the manager hers, Bob gets 404 either way.
        let own: Value = request.get("/api/availability").add_header(ak.clone(), av.clone()).await.json();
        assert_eq!(own.as_array().unwrap().len(), 2, "her own default, including the one a manager set");
        let seen: Value = request.get(&format!("/api/availability?person_ref={alice}")).add_header(rk.clone(), rv.clone()).await.json();
        assert_eq!(seen.as_array().unwrap().len(), 2);
        assert_eq!(request.get(&format!("/api/availability?person_ref={alice}")).add_header(bk.clone(), bv.clone()).await.status_code(), 404);
        let bobs: Value = request.get("/api/availability").add_header(bk.clone(), bv.clone()).await.json();
        assert!(bobs.as_array().unwrap().is_empty(), "Bob's default is Bob's own, which is empty");

        // Removal: not by Bob (404, identically to a missing one); by Alice.
        let pid = mine["pid"].as_str().unwrap();
        assert_eq!(request.post(&format!("/api/availability/{pid}/delete")).add_header(bk.clone(), bv.clone()).await.status_code(), 404);
        request.post(&format!("/api/availability/{pid}/delete")).add_header(ak.clone(), av.clone()).await.assert_status_ok();

        // Committing a proposed booking (PJ-4): the named person or a manager may; Bob may not.
        let plan: Value = request.post("/api/plans").add_header(ak.clone(), av.clone()).json(&json!({ "kind": "Project", "name": "Apollo" })).await.json();
        let p = plan["pid"].as_str().unwrap().to_string();
        let propose = |_n: u8| {
            let (request, url, who) = (&request, format!("/api/plans/{p}/allocations"), alice.clone());
            let (k, v) = (ak.clone(), av.clone());
            async move {
                let r: Value = request
                    .post(&url)
                    .add_header(k, v)
                    .json(&json!({ "person_ref": who, "percent": 30, "booking": "proposed", "start_date": "2026-10-05", "end_date": "2026-10-09" }))
                    .await
                    .json();
                r["pid"].as_str().unwrap().to_string()
            }
        };
        let (one, two) = (propose(1).await, propose(2).await);
        assert_eq!(request.post(&format!("/api/plans/{p}/allocations/{one}/commit")).add_header(bk.clone(), bv.clone()).await.status_code(), 404, "Bob may not commit Alice's time");
        request.post(&format!("/api/plans/{p}/allocations/{one}/commit")).add_header(ak.clone(), av.clone()).await.assert_status_ok();
        request.post(&format!("/api/plans/{p}/allocations/{two}/commit")).add_header(rk.clone(), rv.clone()).await.assert_status_ok();
    })
    .await;
}
