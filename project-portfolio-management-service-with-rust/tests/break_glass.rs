//! Break-glass access (RG-10), end to end with real PASETO identities: who may open a private plan,
//! what a grant does and does not open, that it ends, and that it cannot be made without telling a
//! second person.
//!
//! Its **own test binary**, for the same reason `enforcement.rs` is:
//! `verifier` is a process-wide `OnceLock`, read once on first use —
//! sharing the `tests/requests/mod.rs` binary would race whichever
//! test happens to touch an authenticated route first.
//!
//! `#[ignore]`d — boots the app, so it needs PostgreSQL via
//! `config/test.yaml` / `DATABASE_URL`. Run with
//! `cargo test --test break_glass -- --ignored`.
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

const SEED: [u8; 32] = [43; 32];
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

const ALICE: &str = "3f2c6b7e-0000-4000-8000-0000000000a1"; // owns the private plan
const CAROL: &str = "3f2c6b7e-0000-4000-8000-0000000000c1"; // eligible for break-glass
const DAVE: &str = "3f2c6b7e-0000-4000-8000-0000000000d1"; // a writer, not eligible
const ERIN: &str = "3f2c6b7e-0000-4000-8000-0000000000e1"; // the person who is told
const ADMIN: &str = "3f2c6b7e-0000-4000-8000-0000000000ad";
const REASON: &str = "Incident 4412: the plan owner is unreachable and the release is blocked";

#[tokio::test]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test break_glass -- --ignored`"]
#[allow(clippy::too_many_lines)]
async fn a_private_plan_opens_only_for_the_eligible_for_a_while_and_with_a_witness() {
    let (keys, kid) = keys_and_kid();
    // SAFETY: this binary is single-test, so nothing races the process-wide verifier.
    unsafe {
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS", keys.to_string());
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH", "1");
        // A deployment policy that would, by itself, let every writer do everything.
        std::env::set_var(
            "PROJECT_PORTFOLIO_MANAGEMENT_ABAC_POLICY",
            json!({ "rules": [{ "effect": "allow", "actions": ["*"], "when": { "access": ["write", "admin"] } }] }).to_string(),
        );
    }
    let alice = bearer(&kid, ALICE, &[("access", "write")]);
    let carol = bearer(
        &kid,
        CAROL,
        &[("access", "write"), ("breakglass", "eligible")],
    );
    let dave = bearer(&kid, DAVE, &[("access", "write")]);
    let admin = bearer(&kid, ADMIN, &[("access", "admin")]);
    request::<App, _, _>(|request, ctx| async move {
        macro_rules! with { ($rb:expr, $who:expr) => { $rb.add_header($who.0.clone(), $who.1.clone()) }; }
        let db = &ctx.db;
        let count = |sql: String| {
            let db = db.clone();
            async move {
                db.query_one_raw(sea_orm::Statement::from_string(sea_orm::DatabaseBackend::Postgres, sql))
                    .await.unwrap().unwrap().try_get::<i64>("", "n").unwrap()
            }
        };

        // Alice makes a plan private, with only herself in it.
        let plan: Value = with!(request.post("/api/plans"), alice).json(&json!({ "kind": "Project", "name": "Secret" })).await.json();
        let p = plan["pid"].as_str().unwrap().to_string();
        with!(request.put(&format!("/api/plans/{p}/access")), alice)
            .json(&json!({ "private": true, "members": [ALICE] })).await.assert_status_ok();
        let public: Value = with!(request.post("/api/plans"), alice).json(&json!({ "kind": "Project", "name": "Open" })).await.json();
        let open_plan = public["pid"].as_str().unwrap().to_string();

        // Before: nobody else sees it.
        for (who, name) in [(&carol, "carol"), (&dave, "dave")] {
            assert_eq!(with!(request.get(&format!("/api/plans/{p}")), who).await.status_code(), 404, "{name}");
        }
        let body = |plan: &str, reason: &str, minutes: Value, notify: String| json!({ "plan_pid": plan, "reason": reason, "minutes": minutes, "notify_ref": notify });
        let told = format!("person:{ERIN}");

        // Who may ask.
        assert_eq!(with!(request.post("/api/break-glass"), dave).json(&body(&p, REASON, json!(30), told.clone())).await.status_code(), 403, "a writer who is not eligible");
        assert_eq!(request.post("/api/break-glass").json(&body(&p, REASON, json!(30), told.clone())).await.status_code(), 401);

        // What is refused.
        let bad = |b: Value| { let (r, c) = (&request, &carol); async move { with!(r.post("/api/break-glass"), c).json(&b).await.status_code() } };
        assert_eq!(bad(body(&p, "because", json!(30), told.clone())).await, 422, "a thin reason");
        assert_eq!(bad(body(&p, REASON, json!(241), told.clone())).await, 422, "over four hours");
        assert_eq!(bad(body(&p, REASON, json!(0), told.clone())).await, 422, "no time");
        assert_eq!(bad(body(&p, REASON, json!(30), format!("person:{CAROL}"))).await, 422, "telling yourself");
        assert_eq!(bad(body(&p, REASON, json!(30), "erin".to_string())).await, 422, "not a person reference");
        assert_eq!(bad(body(&open_plan, REASON, json!(30), told.clone())).await, 422, "a plan that is not private");
        assert_eq!(bad(body("00000000-0000-4000-8000-000000000000", REASON, json!(30), told.clone())).await, 404, "no such plan");
        assert_eq!(count("SELECT COUNT(*) AS n FROM break_glass_grants".into()).await, 0, "nothing was made");

        // The second person cannot be told: no grant. (Not reachable through the preferences API,
        // which does not list this kind; forced here to prove the guard.)
        db.execute_unprepared(&format!(
            "INSERT INTO notification_prefs (pid, sub, kind, channel, enabled, digest, created_at, updated_at) VALUES (gen_random_uuid(), '{ERIN}', 'break_glass', 'in_app', false, 'instant', now(), now())"
        )).await.unwrap();
        assert_eq!(bad(body(&p, REASON, json!(30), told.clone())).await, 422, "a witness who has switched the notice off");
        assert_eq!(count("SELECT COUNT(*) AS n FROM break_glass_grants".into()).await, 0, "no witness, no grant");
        db.execute_unprepared("DELETE FROM notification_prefs WHERE kind = 'break_glass'").await.unwrap();

        // The good request.
        let opened: Value = with!(request.post("/api/break-glass"), carol).json(&body(&p, REASON, json!(45), told.clone())).await.json();
        assert_eq!(opened["notified"], true, "{opened}");
        let grant = opened["pid"].as_str().unwrap().to_string();
        assert_eq!(
            count(format!("SELECT COUNT(*) AS n FROM notifications WHERE recipient_ref = '{told}' AND kind = 'break_glass' AND subject_pid = '{p}'")).await,
            1, "the witness was told"
        );

        // Carol can now see the plan; Dave still cannot; Alice is unaffected.
        assert_eq!(with!(request.get(&format!("/api/plans/{p}")), carol).await.status_code(), 200);
        assert_eq!(with!(request.get(&format!("/api/plans/{p}/tasks")), carol).await.status_code(), 200);
        assert_eq!(with!(request.get(&format!("/api/plans/{p}")), dave).await.status_code(), 404);
        let listed: Value = with!(request.get("/api/plans"), carol).await.json::<Value>();
        assert!(listed.to_string().contains(&p), "the plan is in Carol's list while the grant lives");
        let listed_dave: Value = with!(request.get("/api/plans"), dave).await.json::<Value>();
        assert!(!listed_dave.to_string().contains(&p));
        // The use is audited, request by request.
        assert!(count(format!("SELECT COUNT(*) AS n FROM audit_logs WHERE action = 'break_glass_access' AND entity_pid = '{p}' AND actor = '{CAROL}'")).await >= 2);
        assert_eq!(count("SELECT COUNT(*) AS n FROM audit_logs WHERE action = 'break_glass_opened'".into()).await, 1);

        // One live grant at a time.
        assert_eq!(bad(body(&p, REASON, json!(30), told.clone())).await, 409);

        // Someone else cannot end it, or see it; its holder can.
        assert_eq!(with!(request.post(&format!("/api/break-glass/{grant}/revoke")), dave).await.status_code(), 404);
        let mine: Value = with!(request.get("/api/break-glass"), carol).await.json::<Value>();
        assert_eq!(mine["grants"].as_array().unwrap().len(), 1);
        let theirs: Value = with!(request.get("/api/break-glass"), dave).await.json::<Value>();
        assert!(theirs["grants"].as_array().unwrap().is_empty());
        let everything: Value = with!(request.get("/api/break-glass"), admin).await.json::<Value>();
        assert_eq!(everything["grants"].as_array().unwrap().len(), 1);
        with!(request.post(&format!("/api/break-glass/{grant}/revoke")), carol).await.assert_status_ok();
        assert_eq!(with!(request.get(&format!("/api/plans/{p}")), carol).await.status_code(), 404, "revoked");

        // It ends by itself: a fresh grant, then its expiry moves into the past.
        let again: Value = with!(request.post("/api/break-glass"), carol).json(&body(&p, REASON, json!(30), told.clone())).await.json();
        let g2 = again["pid"].as_str().unwrap().to_string();
        assert_eq!(with!(request.get(&format!("/api/plans/{p}")), carol).await.status_code(), 200);
        db.execute_unprepared(&format!("UPDATE break_glass_grants SET created_at = now() - interval '2 hours', expires_at = now() - interval '1 second' WHERE pid = '{g2}'")).await.unwrap();
        assert_eq!(with!(request.get(&format!("/api/plans/{p}")), carol).await.status_code(), 404, "expired");

        // A day's limit.
        for _ in 0..3 {
            db.execute_unprepared(&format!(
                "INSERT INTO break_glass_grants (pid, plan_pid, sub, reason, notify_ref, expires_at, revoked_at, created_at, updated_at) VALUES (gen_random_uuid(), '{p}', '{CAROL}', '{REASON}', '{told}', now() + interval '1 minute', now(), now(), now())"
            )).await.unwrap();
        }
        assert_eq!(bad(body(&p, REASON, json!(30), told.clone())).await, 429, "the sixth in a day");

        // The database itself refuses a window over four hours and a thin reason.
        assert!(db.execute_unprepared(&format!(
            "INSERT INTO break_glass_grants (pid, plan_pid, sub, reason, notify_ref, expires_at, created_at, updated_at) VALUES (gen_random_uuid(), '{p}', 'x', '{REASON}', '{told}', now() + interval '5 hours', now(), now())"
        )).await.is_err());
        assert!(db.execute_unprepared(&format!(
            "INSERT INTO break_glass_grants (pid, plan_pid, sub, reason, notify_ref, expires_at, created_at, updated_at) VALUES (gen_random_uuid(), '{p}', 'x', 'short', '{told}', now() + interval '1 hour', now(), now())"
        )).await.is_err());
    })
    .await;
}
