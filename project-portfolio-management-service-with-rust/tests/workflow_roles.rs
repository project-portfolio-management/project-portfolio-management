//! Per-role workflow transitions (RM-2), end to end over the real router
//! with real PASETO identities that carry a `role` attribute.
//!
//! Its **own test binary**, for the same reason `enforcement.rs` is:
//! `verifier` is a process-wide `OnceLock`, read once on first use —
//! sharing the `tests/requests/mod.rs` binary would race whichever
//! test happens to touch an authenticated route first.
//!
//! `#[ignore]`d — boots the app, so it needs PostgreSQL via
//! `config/test.yaml` / `DATABASE_URL`. Run with
//! `cargo test --test workflow_roles -- --ignored`.
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
fn mint(kid: &str, sub: &str, roles: &[&str]) -> String {
    let iat: i64 = 1_700_000_000;
    let payload = json!({
        "sub": sub,
        "email": format!("{sub}@example.com"), "name": sub,
        "iss": ISSUER, "aud": AUDIENCE,
        "exp": iat + 10_000_000_000_i64, "iat": iat,
        "sid": "test-sid", "attrs": { "role": roles },
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

/// RM-2 acceptance: a move restricted to `lead` is refused 403 for another
/// role and writes no transition row; the same move by a `lead` succeeds; an
/// unrestricted move is open to anyone; the refusal names the allowed roles.
#[tokio::test]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test workflow_roles -- --ignored`"]
async fn a_role_restricted_transition_admits_only_that_role() {
    let (keys, kid) = keys_and_kid();
    // SAFETY: this binary is single-test, so no other test races the
    // process-wide verifier `OnceLock` it sets.
    unsafe {
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS", keys.to_string());
    }
    let (lk, lv) = auth_header(&mint(&kid, ALICE, &["lead"]));
    let (dk, dv) = auth_header(&mint(&kid, BOB, &["dev"]));

    request::<App, _, _>(|request, _ctx| async move {
        let plan: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Apollo" }))
            .await
            .json();
        let p = plan["pid"].as_str().expect("plan pid").to_string();
        let state = |key: &str, cat: &str, initial: bool, terminal: bool| {
            json!({ "key": key, "label": key, "category": cat, "is_initial": initial, "is_terminal": terminal })
        };
        request
            .post("/api/workflows")
            .json(&json!({
                "name": "Gated", "applies_to": "task", "plan_pid": p,
                "states": [
                    state("todo", "todo", true, false),
                    state("review", "waiting", false, false),
                    state("done", "done", false, true),
                ],
                "transitions": [
                    { "from": "todo", "to": "review" },
                    { "from": "review", "to": "done", "roles": ["lead"] },
                ],
            }))
            .await
            .assert_status_ok();
        let task: Value = request
            .post(&format!("/api/plans/{p}/tasks"))
            .json(&json!({ "title": "Ship" }))
            .await
            .json();
        let t = task["pid"].as_str().expect("task pid").to_string();
        let mv = |status: &'static str, k: axum::http::HeaderName, v: axum::http::HeaderValue| {
            let (request, p, t) = (&request, p.clone(), t.clone());
            async move {
                request
                    .patch(&format!("/api/plans/{p}/tasks/{t}"))
                    .add_header(k, v)
                    .json(&json!({ "status": status }))
                    .await
            }
        };
        let transitions = || {
            let (request, p, t) = (&request, p.clone(), t.clone());
            async move {
                let r: Value = request
                    .get(&format!("/api/plans/{p}/tasks/{t}/transitions"))
                    .await
                    .json();
                format!("{r}").matches("\"to_status\"").count()
            }
        };

        // An unrestricted move is open to a dev.
        mv("review", dk.clone(), dv.clone()).await.assert_status_ok();
        let before = transitions().await;

        // The restricted move is refused for a dev, naming the role, and
        // writes no transition row.
        let denied = mv("done", dk.clone(), dv.clone()).await;
        assert_eq!(denied.status_code(), 403);
        assert!(denied.text().contains("lead"), "names the allowed role: {}", denied.text());
        assert_eq!(transitions().await, before, "a refused move writes no transition row");

        // The same move by a lead succeeds.
        mv("done", lk, lv).await.assert_status_ok();
        assert_eq!(transitions().await, before + 1);

        // The registered workflow exposes the roles.
        let wf: Value = request.get(&format!("/api/plans/{p}/workflow")).await.json();
        assert!(format!("{wf}").contains("\"roles\":[\"lead\"]"));
    })
    .await;
}
