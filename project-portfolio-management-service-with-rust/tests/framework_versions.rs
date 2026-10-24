//! Framework versions and who may change a shared framework (CF-4), end to end with real PASETO identities.
//! An organisation-wide framework can be created, edited and deleted only by an administrator or a caller with
//! `framework=edit`; a framework a plan owns follows that plan's write access. An edit appends a version and
//! never changes an earlier one: a plan that follows the framework keeps the version it pinned (and the
//! escalation path that version declares) until it is moved, and `behind` says it is not on the latest.
//!
//! Its **own test binary** (the verifier is a process-wide `OnceLock`). `#[ignore]`d: needs PostgreSQL.
//! Run with `cargo test --test framework_versions -- --ignored`.

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

const WRITER: &str = "3f2c6b7e-0000-4000-8000-0000000000b1";
const EDITOR: &str = "3f2c6b7e-0000-4000-8000-0000000000b2";
const ADMIN: &str = "3f2c6b7e-0000-4000-8000-0000000000bd";

#[tokio::test]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test framework_versions -- --ignored`"]
async fn an_edit_is_a_new_version_and_following_plans_keep_theirs() {
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
    let editor = bearer(&kid, EDITOR, &[("access", "write"), ("framework", "edit")]);
    let admin = bearer(&kid, ADMIN, &[("access", "admin")]);
    request::<App, _, _>(|request, ctx| async move {
        let path = |level: u32, body: &str| json!([{ "level": level, "body": body, "trigger": "a tolerance is forecast to be exceeded" }]);
        let framework = |name: &str, body: &str| json!({ "name": name, "decision_model": "steering_committee", "escalation_path": path(1, body) });
        let name = format!("Standard {}", uuid::Uuid::new_v4());

        // Only an administrator or a framework editor may define an organisation-wide framework.
        let r = request.post("/api/governance-frameworks").add_header(writer.0.clone(), writer.1.clone()).json(&framework(&name, "Team lead")).await;
        assert_eq!(r.status_code(), 403, "{}", r.text());
        let r = request.post("/api/governance-frameworks").add_header(editor.0.clone(), editor.1.clone()).json(&framework(&name, "Team lead")).await;
        assert_eq!(r.status_code(), 200, "{}", r.text());
        let created: Value = r.json();
        let fw = created["pid"].as_str().unwrap().to_string();
        assert_eq!(created["version"], 1);

        // Two plans follow it.
        let mut plans = Vec::new();
        for n in ["Plan A", "Plan B"] {
            let plan: Value = request.post("/api/plans").add_header(writer.0.clone(), writer.1.clone()).json(&json!({ "kind": "Project", "name": n })).await.json();
            let pid = plan["pid"].as_str().unwrap().to_string();
            let r = request.put(&format!("/api/plans/{pid}/governance-framework")).add_header(writer.0.clone(), writer.1.clone()).json(&json!({ "framework_pid": fw })).await;
            assert_eq!(r.status_code(), 200, "{}", r.text());
            plans.push(pid);
        }
        let (a, b) = (plans[0].clone(), plans[1].clone());
        let in_force = |plan: &str| {
            let (request, url, who) = (&request, format!("/api/plans/{plan}/governance-framework"), writer.clone());
            async move { request.get(&url).add_header(who.0, who.1).await.json::<Value>() }
        };
        let path_body = |v: &Value| v["governance_framework"]["escalation_path"][0]["body"].as_str().unwrap().to_string();

        // A writer without the right cannot edit it; an editor can, and that is a new version.
        let r = request.put(&format!("/api/governance-frameworks/{fw}")).add_header(writer.0.clone(), writer.1.clone()).json(&framework(&name, "Board")).await;
        assert_eq!(r.status_code(), 403, "{}", r.text());
        let r = request.put(&format!("/api/governance-frameworks/{fw}")).add_header(editor.0.clone(), editor.1.clone()).json(&framework(&name, "Board")).await;
        assert_eq!(r.status_code(), 200, "{}", r.text());
        assert_eq!(r.json::<Value>()["version"], 2);

        // The plans that follow it keep version 1, and say they are behind.
        for plan in [&a, &b] {
            let v = in_force(plan).await;
            assert_eq!((v["governance_framework"]["version"].as_i64(), v["latest_version"].as_i64(), v["behind"].as_bool()), (Some(1), Some(2), Some(true)), "{v}");
            assert_eq!(path_body(&v), "Team lead", "still the version it pinned");
        }
        // The record itself shows the latest.
        let current: Value = request.get(&format!("/api/governance-frameworks/{fw}")).add_header(writer.0.clone(), writer.1.clone()).await.json();
        assert_eq!((current["version"].as_i64(), current["escalation_path"][0]["body"].as_str()), (Some(2), Some("Board")));
        // An escalation on a following plan goes to the body its pinned version names.
        let e: Value = request.post(&format!("/api/plans/{b}/escalations")).add_header(writer.0.clone(), writer.1.clone()).json(&json!({ "title": "Slip" })).await.json();
        assert_eq!(e["escalated_to"], "Team lead");

        // The history lists both versions and who follows each.
        let vs: Value = request.get(&format!("/api/governance-frameworks/{fw}/versions")).add_header(writer.0.clone(), writer.1.clone()).await.json();
        assert_eq!(vs["current_version"], 2);
        let rows = vs["versions"].as_array().unwrap();
        assert_eq!((rows[0]["version"].as_i64(), rows[0]["plans_following"].as_i64(), rows[0]["current"].as_bool()), (Some(2), Some(0), Some(true)));
        assert_eq!((rows[1]["version"].as_i64(), rows[1]["plans_following"].as_i64(), rows[1]["escalation_path"][0]["body"].as_str()), (Some(1), Some(2), Some("Team lead")));

        // Moving a plan is its own act: A moves to the current version, B does not.
        let r = request.put(&format!("/api/plans/{a}/governance-framework")).add_header(writer.0.clone(), writer.1.clone()).json(&json!({ "framework_pid": fw })).await;
        assert_eq!(r.status_code(), 200, "{}", r.text());
        let va = in_force(&a).await;
        assert_eq!((va["governance_framework"]["version"].as_i64(), va["behind"].as_bool()), (Some(2), Some(false)));
        assert_eq!(path_body(&va), "Board");
        assert_eq!(in_force(&b).await["governance_framework"]["version"], 1);
        // A plan can be pinned to an earlier version on purpose, but not to one that never existed.
        let r = request.put(&format!("/api/plans/{a}/governance-framework")).add_header(writer.0.clone(), writer.1.clone()).json(&json!({ "framework_pid": fw, "version": 1 })).await;
        assert_eq!(r.status_code(), 200, "{}", r.text());
        assert_eq!(in_force(&a).await["governance_framework"]["version"], 1);
        let r = request.put(&format!("/api/plans/{a}/governance-framework")).add_header(writer.0.clone(), writer.1.clone()).json(&json!({ "framework_pid": fw, "version": 9 })).await;
        assert_eq!(r.status_code(), 422, "{}", r.text());

        // The edit is audited with both versions.
        let row = ctx.db.query_one_raw(sea_orm::Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            "SELECT actor, snapshot::text AS s FROM audit_logs WHERE action = 'framework_updated' ORDER BY id DESC LIMIT 1",
        )).await.unwrap().expect("an audit row");
        let snap: Value = serde_json::from_str(&row.try_get::<String>("", "s").unwrap()).unwrap();
        assert_eq!((row.try_get::<Option<String>>("", "actor").unwrap().as_deref(), snap["version"].as_i64(), snap["previous_version"].as_i64()), (Some(EDITOR), Some(2), Some(1)));

        // A version is never changed or removed: the database refuses it.
        assert!(ctx.db.execute_unprepared("UPDATE framework_versions SET name = 'x'").await.is_err());
        assert!(ctx.db.execute_unprepared("DELETE FROM framework_versions").await.is_err());

        // A framework a plan owns follows that plan's write access, not the shared right.
        let own = json!({ "name": format!("Own {}", uuid::Uuid::new_v4()), "plan_pid": a, "decision_model": "sponsor" });
        let r = request.post("/api/governance-frameworks").add_header(writer.0.clone(), writer.1.clone()).json(&own).await;
        assert_eq!(r.status_code(), 200, "{}", r.text());
        // Deleting the shared one needs the right too (and is refused while plans follow it).
        let r = request.post(&format!("/api/governance-frameworks/{fw}/delete")).add_header(writer.0.clone(), writer.1.clone()).await;
        assert_eq!(r.status_code(), 403, "{}", r.text());
        let r = request.post(&format!("/api/governance-frameworks/{fw}/delete")).add_header(admin.0.clone(), admin.1.clone()).await;
        assert_eq!(r.status_code(), 409, "{}", r.text());
    })
    .await;
}
