//! Hard schedule constraints (SC-2), end to end with real PASETO identities:
//! a constraint that overrides the schedule's logic needs the ABAC-style
//! attribute and a reason; ordinary constraints do not.
//!
//! Its **own test binary**, for the same reason `enforcement.rs` is:
//! `verifier` is a process-wide `OnceLock`, read once on first use —
//! sharing the `tests/requests/mod.rs` binary would race whichever
//! test happens to touch an authenticated route first.
//!
//! `#[ignore]`d — boots the app, so it needs PostgreSQL via
//! `config/test.yaml` / `DATABASE_URL`. Run with
//! `cargo test --test hard_constraint -- --ignored`.
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

const ALICE: &str = "3f2c6b7e-0000-4000-8000-00000000000a";

/// SC-2 acceptance: the hard-constraint permission.
#[tokio::test]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test hard_constraint -- --ignored`"]
async fn a_hard_constraint_needs_the_permission_and_a_reason() {
    let (keys, kid) = keys_and_kid();
    // SAFETY: this binary is single-test, so nothing races the process-wide verifier.
    unsafe {
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS", keys.to_string());
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH", "1");
    }
    let (ak, av) = auth_header(&mint(&kid, ALICE, &["write"]));
    let (mk, mv) = auth_header(&mint(&kid, "ops", &["admin"]));
    request::<App, _, _>(|request, _ctx| async move {
        let plan: Value = request.post("/api/plans").add_header(ak.clone(), av.clone()).json(&json!({ "kind": "Project", "name": "Apollo" })).await.json();
        let p = plan["pid"].as_str().unwrap().to_string();
        let t: Value = request
            .post(&format!("/api/plans/{p}/tasks"))
            .add_header(ak.clone(), av.clone())
            .json(&json!({ "title": "Ship", "start_date": "2026-10-05", "duration_days": 3 }))
            .await
            .json();
        let t = t["pid"].as_str().unwrap().to_string();
        let url = format!("/api/plans/{p}/tasks/{t}/schedule");
        let hard = json!({ "start_date": "2026-10-05", "duration_days": 3, "constraint_kind": "mso", "constraint_date": "2026-10-07", "reason": "contractual" });
        let soft = json!({ "start_date": "2026-10-05", "duration_days": 3, "constraint_kind": "snet", "constraint_date": "2026-10-07" });

        assert_eq!(request.put(&url).add_header(ak.clone(), av.clone()).json(&hard).await.status_code(), 403, "a writer may not override logic");
        request.put(&url).add_header(ak.clone(), av.clone()).json(&soft).await.assert_status_ok();
        assert_eq!(request.put(&url).add_header(mk.clone(), mv.clone()).json(&json!({ "constraint_kind": "mfo", "constraint_date": "2026-10-09", "duration_days": 3 })).await.status_code(), 422, "an admin still owes a reason");
        request.put(&url).add_header(mk.clone(), mv.clone()).json(&hard).await.assert_status_ok();
        let cpm: Value = request.get(&format!("/api/plans/{p}/cpm")).add_header(ak.clone(), av.clone()).await.json();
        assert_eq!(cpm["nodes"][0]["constraint"], "mso");

        // The MS Project import honours the same permission: without it a hard
        // constraint is dropped *and reported*; with it, it is kept.
        let xml = r#"<Project xmlns="http://schemas.microsoft.com/project"><Tasks>
            <Task><UID>1</UID><Name>Pinned</Name><Start>2026-10-05T08:00:00</Start><Duration>PT8H0M0S</Duration>
            <ConstraintType>2</ConstraintType><ConstraintDate>2026-10-07T08:00:00</ConstraintDate></Task></Tasks></Project>"#;
        let import = |k: axum::http::HeaderName, v: axum::http::HeaderValue, plan: String| {
            let request = &request;
            async move {
                request
                    .post(&format!("/api/plans/{plan}/import/mspdi?dry_run=false"))
                    .add_header(k, v)
                    .content_type("application/xml")
                    .text(xml)
                    .await
                    .json::<Value>()
            }
        };
        let p2: Value = request.post("/api/plans").add_header(ak.clone(), av.clone()).json(&json!({ "kind": "Project", "name": "Two" })).await.json();
        let p2 = p2["pid"].as_str().unwrap().to_string();
        let r = import(ak.clone(), av.clone(), p2.clone()).await;
        assert!(r["unmapped"].to_string().contains("needs access=admin"), "{r}");
        let cpm: Value = request.get(&format!("/api/plans/{p2}/cpm")).add_header(ak.clone(), av.clone()).await.json();
        assert!(cpm["nodes"][0]["constraint"].is_null(), "imported without the hard constraint");
        let p3: Value = request.post("/api/plans").add_header(ak.clone(), av.clone()).json(&json!({ "kind": "Project", "name": "Three" })).await.json();
        let p3 = p3["pid"].as_str().unwrap().to_string();
        import(mk.clone(), mv.clone(), p3.clone()).await;
        let cpm: Value = request.get(&format!("/api/plans/{p3}/cpm")).add_header(ak.clone(), av.clone()).await.json();
        assert_eq!(cpm["nodes"][0]["constraint"], "mso", "an admin's import keeps it");
    })
    .await;
}
