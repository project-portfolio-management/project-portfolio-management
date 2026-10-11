//! Organizations (EX-3), end to end with real PASETO identities: an organization is a record; a commitment
//! names one; a caller who belongs to an organization sees only its commitments; and a director pack can be
//! addressed to an organization, which alone (besides the pack's editors) can read it.
//!
//! Its **own test binary** (the verifier is a process-wide `OnceLock`). `#[ignore]`d: needs PostgreSQL.
//! Run with `cargo test --test organizations -- --ignored`.
#![cfg(feature = "paseto")]

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use ed25519_dalek::SigningKey;
use loco_rs::testing::prelude::*;
use project_portfolio_management_service::app::App;
use rusty_paseto::core::{Footer, Key, Paseto, PasetoAsymmetricPrivateKey, Payload, Public, V4};
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

const WRITER: &str = "3f2c6b7e-0000-4000-8000-0000000000a1";
const ISSUER_SUB: &str = "3f2c6b7e-0000-4000-8000-0000000000e2";
const ACME_USER: &str = "3f2c6b7e-0000-4000-8000-0000000000b1";
const OTHER_USER: &str = "3f2c6b7e-0000-4000-8000-0000000000b2";

#[tokio::test]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test organizations -- --ignored`"]
#[allow(clippy::too_many_lines)]
async fn organizations_are_records_and_a_partner_sees_its_own() {
    let (keys, kid) = keys_and_kid();
    // SAFETY: this binary is single-test, so nothing races the process-wide verifier or these settings.
    unsafe {
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS", keys.to_string());
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH", "1");
        std::env::set_var(
            "PROJECT_PORTFOLIO_MANAGEMENT_ABAC_POLICY",
            json!({ "rules": [{ "effect": "allow", "actions": ["*"], "when": { "access": ["write", "admin"] } }] }).to_string(),
        );
    }
    let writer = bearer(&kid, WRITER, &[("access", "write")]);
    let issuer = bearer(&kid, ISSUER_SUB, &[("access", "write"), ("directorpack", "issuer")]);
    let first = {
        use chrono::Datelike;
        chrono::Utc::now().date_naive().with_day(1).unwrap()
    };
    let month = first.format("%Y-%m").to_string();
    request::<App, _, _>(|request, _ctx| async move {
        macro_rules! with { ($rb:expr, $who:expr) => { $rb.add_header($who.0.clone(), $who.1.clone()) }; }

        // ── the records ──
        let mk = |name: &'static str, kind: &'static str| {
            let (request, writer) = (&request, writer.clone());
            async move {
                let r = request.post("/api/organizations").add_header(writer.0.clone(), writer.1.clone()).json(&json!({ "name": name, "kind": kind })).await;
                (r.status_code(), r.json::<Value>())
            }
        };
        let (c, acme) = mk("Acme Council", "partner").await;
        assert_eq!(c, 200);
        let acme_pid = acme["pid"].as_str().unwrap().to_string();
        assert_eq!(acme["audience"], format!("org:{acme_pid}"));
        let (c, other) = mk("Other Trust", "partner").await;
        assert_eq!(c, 200);
        let other_pid = other["pid"].as_str().unwrap().to_string();
        assert_eq!(mk("acme council", "supplier").await.0, 409, "names are unique ignoring case");
        assert_eq!(mk("Nobody", "friend").await.0, 422);
        assert_eq!(mk("  ", "partner").await.0, 422);
        let renamed: Value = with!(request.put(&format!("/api/organizations/{other_pid}")), writer).json(&json!({ "name": "Other Health Trust", "kind": "sponsor" })).await.json();
        assert_eq!((renamed["name"].as_str(), renamed["kind"].as_str()), (Some("Other Health Trust"), Some("sponsor")));
        let listed: Value = with!(request.get("/api/organizations"), writer).await.json();
        assert_eq!(listed["organizations"].as_array().unwrap().len(), 2);

        // ── commitments name an organization ──
        let prog: Value = with!(request.post("/api/plans"), writer).json(&json!({ "kind": "Program", "name": "Care" })).await.json();
        let prog = prog["pid"].as_str().unwrap().to_string();
        let pool: Value = with!(request.post("/api/skill-pools"), writer).json(&json!({ "name": "Engineering", "roles": ["Developer"] })).await.json();
        let pool = pool["pid"].as_str().unwrap().to_string();
        let commit = |body: Value| {
            let (request, writer) = (&request, writer.clone());
            async move { let r = request.put("/api/capacity-commitments").add_header(writer.0.clone(), writer.1.clone()).json(&body).await; (r.status_code(), r.json::<Value>()) }
        };
        let (c, a) = commit(json!({ "pool_pid": pool, "plan_pid": prog, "organization_pid": acme_pid, "kind": "supply", "month": month, "committed_fte": 2 })).await;
        assert_eq!(c, 200);
        assert_eq!((a["organization"].as_str(), a["organization_pid"].as_str()), (Some("Acme Council"), Some(acme_pid.as_str())), "the label is the name");
        let (_, b) = commit(json!({ "pool_pid": pool, "plan_pid": prog, "organization": "OTHER HEALTH TRUST", "kind": "supply", "month": month, "committed_fte": 1 })).await;
        assert_eq!(b["organization_pid"].as_str(), Some(other_pid.as_str()), "a label that matches a record is linked to it");
        let (_, own) = commit(json!({ "pool_pid": pool, "plan_pid": prog, "kind": "demand", "month": month, "committed_fte": 1 })).await;
        assert!(own["organization_pid"].is_null());
        let (c, _) = commit(json!({ "pool_pid": pool, "plan_pid": prog, "organization_pid": uuid::Uuid::new_v4(), "kind": "supply", "month": month, "committed_fte": 1 })).await;
        assert_eq!(c, 422, "an unknown organization");

        // ── a partner sees its own commitments, by the organization attribute ──
        let acme_user = bearer(&kid, ACME_USER, &[("access", "write"), ("organization", &acme_pid)]);
        let other_user = bearer(&kid, OTHER_USER, &[("access", "write"), ("organization", &other_pid)]);
        let n = |v: &Value| v["commitments"].as_array().unwrap().len();
        let all: Value = with!(request.get("/api/capacity-commitments"), writer).await.json();
        assert_eq!(n(&all), 3, "a caller of no organization sees all");
        let mine: Value = with!(request.get("/api/capacity-commitments"), acme_user).await.json();
        assert_eq!(n(&mine), 1);
        assert_eq!(mine["commitments"][0]["organization"], "Acme Council");
        let theirs: Value = with!(request.get("/api/capacity-commitments"), other_user).await.json();
        assert_eq!(n(&theirs), 1);
        assert_eq!(theirs["commitments"][0]["organization"], "Other Health Trust");

        // ── an organization is a pack audience ──
        let aud = format!("org:{acme_pid}");
        let bad = with!(request.post("/api/director-packs"), issuer).json(&json!({ "period": month, "audience": format!("org:{}", uuid::Uuid::new_v4()) })).await;
        assert_eq!(bad.status_code(), 422, "an organization that does not exist");
        let draft: Value = with!(request.post("/api/director-packs"), issuer).json(&json!({ "period": month, "audience": aud })).await.json();
        let pk = draft["pid"].as_str().unwrap().to_string();
        with!(request.post(&format!("/api/director-packs/{pk}/issue")), issuer).await.assert_status_ok();
        let seen: Value = with!(request.get("/api/director-packs"), acme_user).await.json();
        assert_eq!(seen["packs"].as_array().unwrap().len(), 1, "the organization's own pack");
        let not: Value = with!(request.get("/api/director-packs"), other_user).await.json();
        assert!(not["packs"].as_array().unwrap().is_empty(), "another organization sees none");
        with!(request.get(&format!("/api/director-packs/{pk}")), acme_user).await.assert_status_ok();
        assert_ne!(with!(request.get(&format!("/api/director-packs/{pk}")), other_user).await.status_code(), 200);

        // ── retired: it can no longer be named or addressed ──
        with!(request.put(&format!("/api/organizations/{acme_pid}")), writer).json(&json!({ "active": false })).await.assert_status_ok();
        let (c, _) = commit(json!({ "pool_pid": pool, "plan_pid": prog, "organization_pid": acme_pid, "kind": "demand", "month": month, "committed_fte": 1 })).await;
        assert_eq!(c, 422);
    })
    .await;
}
