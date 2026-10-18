//! Handling marks (RG-7), end to end with real PASETO identities: a plan marked above a caller's
//! clearance does not exist for them (administrators included), the mark travels on every response, and
//! you can mark only up to your own clearance.
//!
//! Its **own test binary**, for the same reason `enforcement.rs` is:
//! `verifier` is a process-wide `OnceLock`, read once on first use —
//! sharing the `tests/requests/mod.rs` binary would race whichever
//! test happens to touch an authenticated route first.
//!
//! `#[ignore]`d — boots the app, so it needs PostgreSQL via
//! `config/test.yaml` / `DATABASE_URL`. Run with
//! `cargo test --test handling_marks -- --ignored`.
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

const SEED: [u8; 32] = [47; 32];
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

#[tokio::test]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test handling_marks -- --ignored`"]
#[allow(clippy::too_many_lines)]
async fn a_plan_marked_above_your_clearance_does_not_exist_for_you() {
    let (keys, kid) = keys_and_kid();
    // SAFETY: this binary is single-test, so nothing races the process-wide verifier and the scheme.
    unsafe {
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS", keys.to_string());
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH", "1");
        std::env::set_var(
            "PROJECT_PORTFOLIO_MANAGEMENT_ABAC_POLICY",
            json!({ "rules": [{ "effect": "allow", "actions": ["*"], "when": { "access": ["write", "admin"] } }] }).to_string(),
        );
        std::env::set_var(
            "PROJECT_PORTFOLIO_MANAGEMENT_HANDLING_MARKS",
            r#"["Public","Internal","Confidential"]"#,
        );
    }
    let low = bearer(&kid, "low", &[("access", "write")]);
    let mid = bearer(
        &kid,
        "mid",
        &[("access", "write"), ("clearance", "Internal")],
    );
    let high = bearer(
        &kid,
        "high",
        &[("access", "write"), ("clearance", "Confidential")],
    );
    let admin = bearer(&kid, "admin", &[("access", "admin")]);
    let wrong_case = bearer(
        &kid,
        "wc",
        &[("access", "write"), ("clearance", "confidential")],
    );
    request::<App, _, _>(|request, ctx| async move {
        macro_rules! with { ($rb:expr, $who:expr) => { $rb.add_header($who.0.clone(), $who.1.clone()) }; }
        let mark = |plan: &str| format!("/api/plans/{plan}/handling-mark");

        let scheme: Value = with!(request.get("/api/handling-marks"), low).await.json();
        assert_eq!(scheme["scheme"], json!(["Public", "Internal", "Confidential"]));
        assert_eq!(scheme["on"], true);

        let plan: Value = with!(request.post("/api/plans"), high).json(&json!({ "kind": "Project", "name": "Cabinet paper" })).await.json();
        let p = plan["pid"].as_str().unwrap().to_string();
        with!(request.post(&format!("/api/plans/{p}/tasks")), high).json(&json!({ "title": "Draft" })).await.assert_status_ok();
        let open: Value = with!(request.post("/api/plans"), low).json(&json!({ "kind": "Project", "name": "Newsletter" })).await.json();
        let q = open["pid"].as_str().unwrap().to_string();

        // Unmarked means the lowest level, and the response says so.
        let r = with!(request.get(&format!("/api/plans/{q}")), low).await;
        assert_eq!(r.status_code(), 200);
        assert_eq!(r.header("x-handling-mark"), "Public");
        // Everyone can see the not-yet-marked plan P too, so far.
        assert_eq!(with!(request.get(&format!("/api/plans/{p}")), low).await.status_code(), 200);

        // Marking: only up to your own clearance.
        assert_eq!(with!(request.put(&mark(&p)), low).json(&json!({ "mark": "Internal" })).await.status_code(), 403, "no clearance");
        assert_eq!(with!(request.put(&mark(&p)), wrong_case).json(&json!({ "mark": "Internal" })).await.status_code(), 403, "clearance is matched exactly");
        assert_eq!(with!(request.put(&mark(&p)), mid).json(&json!({ "mark": "Confidential" })).await.status_code(), 403, "above your clearance");
        assert_eq!(with!(request.put(&mark(&p)), high).json(&json!({ "mark": "Secret" })).await.status_code(), 403, "not in the scheme");
        let set: Value = with!(request.put(&mark(&p)), high).json(&json!({ "mark": "Confidential" })).await.json();
        assert_eq!(set["effective"], "Confidential");

        // Now the plan does not exist for anyone below Confidential, administrators included.
        for (who, name) in [(&low, "low"), (&mid, "mid"), (&admin, "admin"), (&wrong_case, "wrong-case clearance")] {
            for path in [format!("/api/plans/{p}"), format!("/api/plans/{p}/tasks"), mark(&p), format!("/api/plans/{p}/cpm")] {
                assert_eq!(with!(request.get(&path), who).await.status_code(), 404, "{name} {path}");
            }
            let listed = with!(request.get("/api/plans"), who).await.text();
            assert!(!listed.contains(&p), "{name}: the plan is in the list");
            let glance = with!(request.get("/api/at-a-glance"), who).await.text();
            assert!(!glance.contains(&p), "{name}: the plan is in an aggregate");
        }
        // For the cleared caller it is all there, and labelled.
        let r = with!(request.get(&format!("/api/plans/{p}/tasks")), high).await;
        assert_eq!(r.status_code(), 200);
        assert_eq!(r.header("x-handling-mark"), "Confidential");
        assert!(with!(request.get("/api/plans"), high).await.text().contains(&p));
        // The unmarked plan is still everyone's.
        assert_eq!(with!(request.get(&format!("/api/plans/{q}")), mid).await.status_code(), 200);

        // A mark can be lowered by someone who can see the plan; others cannot touch it.
        assert_eq!(with!(request.put(&mark(&p)), mid).json(&json!({ "mark": null })).await.status_code(), 404, "cannot even see it");
        with!(request.put(&mark(&p)), high).json(&json!({ "mark": "Internal" })).await.assert_status_ok();
        assert_eq!(with!(request.get(&format!("/api/plans/{p}")), mid).await.status_code(), 200, "now Internal");
        assert_eq!(with!(request.get(&format!("/api/plans/{p}")), low).await.status_code(), 404);
        with!(request.put(&mark(&p)), high).json(&json!({ "mark": null })).await.assert_status_ok();
        assert_eq!(with!(request.get(&format!("/api/plans/{p}")), low).await.status_code(), 200, "reset to the lowest level");

        // A mark the scheme no longer knows hides from everyone below the highest clearance.
        ctx.db
            .execute_unprepared(&format!("INSERT INTO plan_handling_marks (pid, plan_pid, mark, created_at, updated_at) VALUES (gen_random_uuid(), '{p}', 'TOP SECRET', now(), now())"))
            .await
            .unwrap();
        assert_eq!(with!(request.get(&format!("/api/plans/{p}")), mid).await.status_code(), 404);
        let seen = with!(request.get(&format!("/api/plans/{p}")), high).await;
        assert_eq!(seen.status_code(), 200, "only the highest clearance reads it");
        assert_eq!(seen.header("x-handling-mark"), "Confidential");

        // The database refuses a mark that is empty, padded or long.
        for bad in ["", " x", "x ", &"y".repeat(41)] {
            assert!(ctx.db.execute_unprepared(&format!("INSERT INTO plan_handling_marks (pid, plan_pid, mark, created_at, updated_at) VALUES (gen_random_uuid(), gen_random_uuid(), '{bad}', now(), now())")).await.is_err(), "{bad:?}");
        }
        // Every set and reset was audited.
        let rows = ctx.db.query_one_raw(sea_orm::Statement::from_string(sea_orm::DatabaseBackend::Postgres, "SELECT COUNT(*) AS n FROM audit_logs WHERE action = 'handling_mark_set'".to_string())).await.unwrap().unwrap().try_get::<i64>("", "n").unwrap();
        assert_eq!(rows, 3);
    })
    .await;
}
