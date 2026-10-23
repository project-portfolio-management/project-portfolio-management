//! Change requests (IR-4), end to end with real PASETO identities: the separation of duties. The requester
//! drafts and submits, only the named approver decides (not the requester, not an administrator), the
//! approver cannot be the requester, and the database itself refuses a decision without a decider and a reason.
//!
//! Its **own test binary**, for the same reason `enforcement.rs` is:
//! `verifier` is a process-wide `OnceLock`, read once on first use —
//! sharing the `tests/requests/mod.rs` binary would race whichever
//! test happens to touch an authenticated route first.
//!
//! `#[ignore]`d — boots the app, so it needs PostgreSQL via
//! `config/test.yaml` / `DATABASE_URL`. Run with
//! `cargo test --test change_requests -- --ignored`.
#![cfg(feature = "paseto")]

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use ed25519_dalek::SigningKey;
use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use rusty_paseto::core::{Footer, Key, Paseto, PasetoAsymmetricPrivateKey, Payload, Public, V4};
use sea_orm::ConnectionTrait;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

const SEED: [u8; 32] = [53; 32];
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
fn mint(kid: &str, sub: &str, attrs: &[(&str, &str)]) -> String {
    let iat: i64 = 1_700_000_000;
    let payload = json!({
        "sub": sub,
        "email": format!("{sub}@example.com"), "name": sub,
        "iss": ISSUER, "aud": AUDIENCE,
        "exp": iat + 10_000_000_000_i64, "iat": iat,
        "sid": "test-sid", "attrs": attrs.iter().fold(serde_json::Map::new(), |mut m, (k, v)| { m.insert((*k).to_string(), json!([v])); m }),
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

fn bearer(
    kid: &str,
    sub: &str,
    attrs: &[(&str, &str)],
) -> (axum::http::HeaderName, axum::http::HeaderValue) {
    auth_header(&mint(kid, sub, attrs))
}

const ALICE: &str = "3f2c6b7e-0000-4000-8000-0000000000a1"; // the requester
const BOB: &str = "3f2c6b7e-0000-4000-8000-0000000000b1"; // the named approver
const CAROL: &str = "3f2c6b7e-0000-4000-8000-0000000000c1"; // another writer
const ADMIN: &str = "3f2c6b7e-0000-4000-8000-0000000000ad";

#[tokio::test]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test change_requests -- --ignored`"]
async fn only_the_named_approver_decides_and_never_the_requester() {
    let (keys, kid) = keys_and_kid();
    // SAFETY: this binary is single-test, so nothing races the process-wide verifier.
    unsafe {
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS", keys.to_string());
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH", "1");
        // A deployment policy that lets every writer do everything: only the service's own rules hold.
        std::env::set_var(
            "PROJECT_PORTFOLIO_MANAGEMENT_ABAC_POLICY",
            json!({ "rules": [{ "effect": "allow", "actions": ["*"], "when": { "access": ["write", "admin"] } }] }).to_string(),
        );
    }
    let alice = bearer(&kid, ALICE, &[("access", "write")]);
    let bob = bearer(&kid, BOB, &[("access", "write")]);
    let carol = bearer(&kid, CAROL, &[("access", "write")]);
    let admin = bearer(&kid, ADMIN, &[("access", "admin")]);
    request::<App, _, _>(|request, ctx| async move {
        macro_rules! with { ($rb:expr, $who:expr) => { $rb.add_header($who.0.clone(), $who.1.clone()) }; }
        let plan: Value = with!(request.post("/api/plans"), alice).json(&json!({ "kind": "Project", "name": "Apollo" })).await.json();
        let p = plan["pid"].as_str().unwrap().to_string();
        let base = format!("/api/plans/{p}/change-requests");
        let draft = json!({ "title": "Move go-live", "description": "Two weeks later", "reason": "Supplier slipped", "kind": "schedule" });
        let cr: Value = with!(request.post(&base), alice).json(&draft).await.json();
        let one = format!("{base}/{}", cr["pid"].as_str().unwrap());
        assert_eq!(cr["requested_by"], ALICE);

        // Only the requester edits, submits or (with an administrator) withdraws a draft.
        for (who, name) in [(&bob, "bob"), (&carol, "carol"), (&admin, "admin")] {
            assert_eq!(with!(request.put(&one), who).json(&draft).await.status_code(), 403, "{name} edits");
            assert_eq!(with!(request.post(&format!("{one}/submit")), who).json(&json!({ "approver_ref": format!("person:{BOB}") })).await.status_code(), 403, "{name} submits");
        }
        assert_eq!(with!(request.post(&format!("{one}/withdraw")), carol).await.status_code(), 403, "a stranger withdraws");

        // The approver cannot be the requester, in any form.
        for me in [format!("person:{ALICE}"), format!("worker:{ALICE}"), ALICE.to_uppercase()] {
            let r = with!(request.post(&format!("{one}/submit")), alice).json(&json!({ "approver_ref": me })).await;
            assert_eq!(r.status_code(), 422, "{me}");
        }
        let sub: Value = with!(request.post(&format!("{one}/submit")), alice).json(&json!({ "approver_ref": format!("person:{BOB}") })).await.json();
        assert_eq!(sub["status"], "submitted");

        // Nobody but the named approver reviews or decides: not the requester, not another writer,
        // and not an administrator.
        let decide = json!({ "decision": "approve", "reason": "Within contingency; PMO agrees" });
        for (who, name) in [(&alice, "the requester"), (&carol, "another writer"), (&admin, "an administrator")] {
            assert_eq!(with!(request.post(&format!("{one}/review")), who).await.status_code(), 403, "{name} reviews");
            assert_eq!(with!(request.post(&format!("{one}/decision")), who).json(&decide).await.status_code(), 403, "{name} decides");
        }
        let status: Value = with!(request.get(&one), alice).await.json();
        assert_eq!(status["status"], "submitted", "nothing moved");
        assert_eq!(with!(request.post(&format!("{one}/decision")), bob).json(&json!({ "decision": "approve", "reason": "ok" })).await.status_code(), 422, "a thin reason");
        let approved: Value = with!(request.post(&format!("{one}/decision")), bob).json(&decide).await.json();
        assert_eq!(approved["status"], "approved");
        assert_eq!(approved["decided_by"], BOB);

        // The audit trail names who did each step.
        let actors = ctx.db.query_all_raw(sea_orm::Statement::from_string(sea_orm::DatabaseBackend::Postgres,
            "SELECT action, actor FROM audit_logs WHERE action LIKE 'change_request_%' ORDER BY id".to_string())).await.unwrap();
        let trail: Vec<(String, String)> = actors.iter().map(|r| (r.try_get("", "action").unwrap(), r.try_get("", "actor").unwrap())).collect();
        assert_eq!(trail, [
            ("change_request_drafted".to_string(), ALICE.to_string()),
            ("change_request_submitted".to_string(), ALICE.to_string()),
            ("change_request_approved".to_string(), BOB.to_string()),
        ]);

        // And the database does not rely on the service being right.
        for (what, sql) in [
            ("approved with no decider", "INSERT INTO change_requests (pid, plan_pid, title, description, reason, kind, status, requested_by, requested_on, approver_ref, created_at, updated_at) VALUES (gen_random_uuid(), gen_random_uuid(), 't', 'd', 'r', 'other', 'approved', 'x', current_date, 'person:y', now(), now())"),
            ("approved with a thin reason", "INSERT INTO change_requests (pid, plan_pid, title, description, reason, kind, status, requested_by, requested_on, approver_ref, decided_by, decided_on, decision_reason, created_at, updated_at) VALUES (gen_random_uuid(), gen_random_uuid(), 't', 'd', 'r', 'other', 'approved', 'x', current_date, 'person:y', 'y', current_date, 'ok', now(), now())"),
            ("submitted with no approver", "INSERT INTO change_requests (pid, plan_pid, title, description, reason, kind, status, requested_by, requested_on, created_at, updated_at) VALUES (gen_random_uuid(), gen_random_uuid(), 't', 'd', 'r', 'other', 'submitted', 'x', current_date, now(), now())"),
            ("a cost without a currency", "INSERT INTO change_requests (pid, plan_pid, title, description, reason, kind, status, requested_by, requested_on, impact_cost_minor, created_at, updated_at) VALUES (gen_random_uuid(), gen_random_uuid(), 't', 'd', 'r', 'other', 'draft', 'x', current_date, 5, now(), now())"),
            ("an unknown status", "INSERT INTO change_requests (pid, plan_pid, title, description, reason, kind, status, requested_by, requested_on, created_at, updated_at) VALUES (gen_random_uuid(), gen_random_uuid(), 't', 'd', 'r', 'other', 'done', 'x', current_date, now(), now())"),
        ] {
            assert!(ctx.db.execute_unprepared(sql).await.is_err(), "{what}");
        }
    })
    .await;
}
