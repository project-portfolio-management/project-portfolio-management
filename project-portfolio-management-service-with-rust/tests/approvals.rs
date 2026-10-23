//! Approval requests (PL-1), end to end over the real router with real PASETO
//! identities: only the named approver decides, a decision is final, a
//! rejection must say why, only the requester withdraws.
//!
//! Its **own test binary**, for the same reason `enforcement.rs` is:
//! `verifier` is a process-wide `OnceLock`, read once on first use —
//! sharing the `tests/requests/mod.rs` binary would race whichever
//! test happens to touch an authenticated route first.
//!
//! `#[ignore]`d — boots the app, so it needs PostgreSQL via
//! `config/test.yaml` / `DATABASE_URL`. Run with
//! `cargo test --test approvals -- --ignored`.

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

/// PL-1 acceptance.
#[tokio::test]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test approvals -- --ignored`"]
async fn only_the_named_approver_decides_and_a_decision_is_final() {
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
        let line: Value = request
            .post(&format!("/api/plans/{p}/budget-lines"))
            .json(&json!({ "category": "capex", "description": "rig", "currency": "GBP", "planned_minor": 1000 }))
            .await
            .json();
        let bl = line["pid"].as_str().expect("budget line pid").to_string();
        let ask = |kind: &str, subject: &str| {
            let (request, ak, av) = (&request, ak.clone(), av.clone());
            let body = json!({ "subject_kind": kind, "subject_pid": subject,
                               "approver_ref": format!("person:{BOB}"), "message": "please" });
            async move {
                request.post("/api/approvals").add_header(ak, av).json(&body).await
            }
        };
        let inbox = |k: axum::http::HeaderName, v: axum::http::HeaderValue| {
            let request = &request;
            async move {
                let r: Value = request.get("/api/me/inbox?tz=UTC").add_header(k, v).await.json();
                r["items"].as_array().expect("items").len()
            }
        };

        // A budget line is a valid subject; the approver is notified.
        let r1: Value = ask("budget_line", &bl).await.json();
        let r1 = r1["pid"].as_str().expect("request pid").to_string();
        assert_eq!(inbox(bk.clone(), bv.clone()).await, 1);

        // Bad subject kind, unknown subject and a bad approver are refused.
        assert_eq!(ask("nope", &bl).await.status_code(), 422);
        assert_eq!(ask("budget_line", "3f2c6b7e-0000-4000-8000-000000000bad").await.status_code(), 404);
        assert_eq!(
            request
                .post("/api/approvals")
                .add_header(ak.clone(), av.clone())
                .json(&json!({ "subject_kind": "plan", "subject_pid": p, "approver_ref": "bob" }))
                .await
                .status_code(),
            422
        );
        assert_eq!(request.post("/api/approvals").json(&json!({})).await.status_code(), 401);

        // Someone who is not the named approver cannot decide.
        let decide = |k: axum::http::HeaderName, v: axum::http::HeaderValue, id: String, body: Value| {
            let request = &request;
            async move {
                request
                    .post(&format!("/api/approvals/{id}/decide"))
                    .add_header(k, v)
                    .json(&body)
                    .await
            }
        };
        assert_eq!(
            decide(ck.clone(), cv.clone(), r1.clone(), json!({ "decision": "approved" })).await.status_code(),
            403
        );
        // A rejection must say why; the approver may not decide with an unknown verb.
        assert_eq!(
            decide(bk.clone(), bv.clone(), r1.clone(), json!({ "decision": "rejected" })).await.status_code(),
            422
        );
        assert_eq!(
            decide(bk.clone(), bv.clone(), r1.clone(), json!({ "decision": "maybe" })).await.status_code(),
            422
        );
        // Still pending after the refusals.
        let pending: Value = request
            .get("/api/approvals?mine=approver&status=pending")
            .add_header(bk.clone(), bv.clone())
            .await
            .json();
        assert_eq!(pending.as_array().unwrap().len(), 1);

        // The named approver rejects with a comment; the requester is told.
        let done: Value = decide(
            bk.clone(), bv.clone(), r1.clone(),
            json!({ "decision": "rejected", "comment": "over budget" }),
        )
        .await
        .json();
        assert_eq!(done["status"], "rejected");
        assert_eq!(done["decision_comment"], "over budget");
        assert_eq!(inbox(ak.clone(), av.clone()).await, 1, "the requester learns the outcome");

        // A decision is final; a decided request cannot be withdrawn.
        assert_eq!(
            decide(bk.clone(), bv.clone(), r1.clone(), json!({ "decision": "approved" })).await.status_code(),
            409
        );
        assert_eq!(
            request
                .post(&format!("/api/approvals/{r1}/withdraw"))
                .add_header(ak.clone(), av.clone())
                .await
                .status_code(),
            409
        );

        // Withdrawal: only the requester, only while pending, and no decision is written.
        let r2: Value = ask("plan", &p).await.json();
        let r2 = r2["pid"].as_str().expect("request pid").to_string();
        assert_eq!(
            request
                .post(&format!("/api/approvals/{r2}/withdraw"))
                .add_header(bk.clone(), bv.clone())
                .await
                .status_code(),
            403,
            "the approver is not the requester"
        );
        let w: Value = request
            .post(&format!("/api/approvals/{r2}/withdraw"))
            .add_header(ak.clone(), av.clone())
            .await
            .json();
        assert_eq!(w["status"], "withdrawn");
        assert!(w["decided_by"].is_null(), "withdrawing writes no decision");
        assert_eq!(
            decide(bk.clone(), bv.clone(), r2, json!({ "decision": "approved" })).await.status_code(),
            409,
            "a withdrawn request cannot be decided"
        );

        // Listing by subject.
        let by_subject: Value = request
            .get(&format!("/api/approvals?subject_kind=budget_line&subject_pid={bl}"))
            .add_header(ak, av)
            .await
            .json();
        assert_eq!(by_subject.as_array().unwrap().len(), 1);
    })
    .await;
}
