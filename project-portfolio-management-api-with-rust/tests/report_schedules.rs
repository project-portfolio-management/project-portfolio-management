//! Scheduled report runs (T-28e), end to end with real PASETO identities:
//! claim-based sweeps, one run for a missed schedule, audited even at zero
//! rows, owner-only downloads, a failure recorded and not retried in a loop.
//!
//! Its **own test binary**, for the same reason `enforcement.rs` is:
//! `verifier` is a process-wide `OnceLock`, read once on first use —
//! sharing the `tests/requests/mod.rs` binary would race whichever
//! test happens to touch an authenticated route first.
//!
//! `#[ignore]`d — boots the app, so it needs PostgreSQL via
//! `config/test.yaml` / `DATABASE_URL`. Run with
//! `cargo test --test report_schedules -- --ignored`.
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
const BOB: &str = "3f2c6b7e-0000-4000-8000-00000000000b";

/// T-28e acceptance.
#[tokio::test]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test report_schedules -- --ignored`"]
async fn scheduled_runs_are_claimed_once_audited_and_owner_only() {
    let (keys, kid) = keys_and_kid();
    let dir = tempfile::tempdir().expect("tempdir");
    // SAFETY: this binary is single-test, so nothing races these variables.
    unsafe {
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS", keys.to_string());
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_ARTIFACT_DIR", dir.path());
    }
    let (ak, av) = auth_header(&mint(&kid, ALICE, &["write"]));
    let (bk, bv) = auth_header(&mint(&kid, BOB, &["write"]));
    let (mk, mv) = auth_header(&mint(&kid, "ops", &["admin"]));

    request::<App, _, _>(|request, ctx| async move {
        for name in ["Apollo", "Orion"] {
            request.post("/api/plans").json(&json!({ "kind": "Project", "name": name })).await.assert_status_ok();
        }
        let mk_report = |name: &str, like: &str| {
            let (request, body) = (&request, json!({ "name": name, "collection": "", "filters": { "name_like": like }, "fields": ["name"] }));
            async move { request.post("/api/reports").json(&body).await.json::<Value>() }
        };
        let some = mk_report("Space", "o").await;
        let none = mk_report("Nothing", "zzz-no-match").await;
        let (some, none) = (some["pid"].as_str().unwrap().to_string(), none["pid"].as_str().unwrap().to_string());
        let schedule = |k: axum::http::HeaderName, v: axum::http::HeaderValue, report: &str, body: Value| {
            let (request, url) = (&request, format!("/api/reports/{report}/schedule"));
            async move { request.post(&url).add_header(k, v).json(&body).await }
        };
        let sweep = |k: axum::http::HeaderName, v: axum::http::HeaderValue| {
            let request = &request;
            async move { request.post("/api/report-schedules/sweep").add_header(k, v).await }
        };

        // Validation.
        for bad in [json!({ "cadence": "hourly" }), json!({ "cadence": "daily", "format": "pdf" }), json!({ "cadence": "daily", "start": "tomorrow" })] {
            assert_eq!(schedule(ak.clone(), av.clone(), &some, bad.clone()).await.status_code(), 422, "{bad}");
        }
        assert_eq!(schedule(ak.clone(), av.clone(), "3f2c6b7e-0000-4000-8000-000000000bad", json!({ "cadence": "daily" })).await.status_code(), 404);

        // Daily, due since 20 days ago; a weekly one in the future; a zero-row one.
        let missed: Value = schedule(ak.clone(), av.clone(), &some, json!({ "cadence": "daily", "start": "2026-09-01" })).await.json();
        let future: Value = schedule(ak.clone(), av.clone(), &some, json!({ "cadence": "weekly", "start": "2999-01-01" })).await.json();
        let empty: Value = schedule(ak.clone(), av.clone(), &none, json!({ "cadence": "daily" })).await.json();
        let _ = future;

        // Not an admin: refused.
        assert_eq!(sweep(ak.clone(), av.clone()).await.status_code(), 403);
        // The sweep runs the two due ones, once each.
        let r: Value = sweep(mk.clone(), mv.clone()).await.json();
        assert_eq!((r["ran"].as_i64(), r["failed"].as_i64()), (Some(2), Some(0)), "{r}");
        // A second sweep finds nothing due: the missed daily did NOT replay.
        let r2: Value = sweep(mk.clone(), mv.clone()).await.json();
        assert_eq!(r2["ran"], 0, "{r2}");

        let mine: Value = request.get("/api/report-schedules").add_header(ak.clone(), av.clone()).await.json();
        let by = |pid: &Value| mine.as_array().unwrap().iter().find(|s| s["pid"] == *pid).cloned().unwrap();
        let (m, e, f) = (by(&missed["pid"]), by(&empty["pid"]), by(&future["pid"]));
        assert_eq!(m["last_status"], "ok");
        assert_eq!(m["last_rows"], 2, "both plans match name_like=o");
        assert!(m["next_run_date"].as_str().unwrap() > "2026-10-03", "moved to the future: {m}");
        assert_eq!(e["last_rows"], 0, "a real zero, not null: {e}");
        assert_eq!(e["has_output"], true, "a zero-row run still produced a file");
        assert!(f["last_status"].is_null(), "not due, not run");

        // Audited like an export, even the zero-row run.
        {
            use project_portfolio_management_service::models::_entities::audit_logs;
            use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
            let rows = audit_logs::Entity::find().filter(audit_logs::Column::Action.eq("report_run_scheduled")).all(&ctx.db).await.unwrap();
            assert_eq!(rows.len(), 2);
            assert!(rows.iter().any(|a| a.snapshot.as_ref().is_some_and(|d| d["rows"] == 0)), "the zero-row run is in the audit log");
        }

        // The owner downloads it; another person cannot see or fetch it.
        let id = m["pid"].as_str().unwrap();
        let dl = request.get(&format!("/api/report-schedules/{id}/latest")).add_header(ak.clone(), av.clone()).await;
        assert_eq!(dl.status_code(), 200);
        assert!(dl.text().starts_with("name\n"), "{}", dl.text());
        assert!(dl.text().contains("Apollo") && dl.text().contains("Orion"));
        assert_eq!(request.get(&format!("/api/report-schedules/{id}/latest")).add_header(bk.clone(), bv.clone()).await.status_code(), 404);
        let bobs: Value = request.get("/api/report-schedules").add_header(bk.clone(), bv.clone()).await.json();
        assert!(bobs.as_array().unwrap().is_empty());
        assert_eq!(request.post(&format!("/api/report-schedules/{id}/delete")).add_header(bk.clone(), bv.clone()).await.status_code(), 404);
        assert_eq!(request.get(&format!("/api/report-schedules/{}/latest", f["pid"].as_str().unwrap())).add_header(ak.clone(), av.clone()).await.status_code(), 404, "before its first run");

        // A deleted report fails the run, records why, and does not loop.
        request.delete(&format!("/api/reports/{none}")).add_header(mk.clone(), mv.clone()).await.assert_status_ok();
        {
            use project_portfolio_management_service::models::_entities::report_schedules;
            use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, sea_query::Expr};
            report_schedules::Entity::update_many()
                .col_expr(report_schedules::Column::NextRunDate, Expr::value(chrono::NaiveDate::from_ymd_opt(2026, 9, 1).unwrap()))
                .filter(report_schedules::Column::Pid.eq(uuid::Uuid::parse_str(e["pid"].as_str().unwrap()).unwrap()))
                .exec(&ctx.db)
                .await
                .unwrap();
        }
        let r3: Value = sweep(mk.clone(), mv.clone()).await.json();
        assert_eq!((r3["ran"].as_i64(), r3["failed"].as_i64()), (Some(0), Some(1)), "{r3}");
        let again: Value = sweep(mk.clone(), mv.clone()).await.json();
        assert_eq!((again["ran"].as_i64(), again["failed"].as_i64()), (Some(0), Some(0)), "the date advanced");
        let mine: Value = request.get("/api/report-schedules").add_header(ak.clone(), av.clone()).await.json();
        let e = mine.as_array().unwrap().iter().find(|s| s["pid"] == empty["pid"]).unwrap();
        assert_eq!(e["last_status"], "failed");
        assert_eq!(e["last_error"], "the report no longer exists");

        // Stopping a schedule (POST twin).
        request.post(&format!("/api/report-schedules/{id}/delete")).add_header(ak.clone(), av.clone()).await.assert_status_ok();
        let mine: Value = request.get("/api/report-schedules").add_header(ak.clone(), av.clone()).await.json();
        assert!(mine.as_array().unwrap().iter().all(|s| s["pid"] != missed["pid"]));
    })
    .await;
}
