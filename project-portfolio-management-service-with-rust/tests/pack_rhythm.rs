//! The monthly rhythm of the director pack (DP-12), end to end with real PASETO identities:
//! a schedule that drafts the month that just ended and never issues, a sweep that cannot draft twice, a
//! draft the schedule made that needs a review before it can be issued, a hand-made draft that stops short of
//! its period marked stale, and subscribers told with a link and never the body.
//!
//! Its **own test binary**, for the same reason `enforcement.rs` is:
//! `verifier` is a process-wide `OnceLock`, read once on first use —
//! sharing the `tests/requests/mod.rs` binary would race whichever
//! test happens to touch an authenticated route first.
//!
//! `#[ignore]`d — boots the app, so it needs PostgreSQL via
//! `config/test.yaml` / `DATABASE_URL`. Run with
//! `cargo test --test pack_rhythm -- --ignored`.
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

const WRITER: &str = "3f2c6b7e-0000-4000-8000-0000000000a1";
const EDITOR: &str = "3f2c6b7e-0000-4000-8000-0000000000e1";
const ISSUER_SUB: &str = "3f2c6b7e-0000-4000-8000-0000000000e2";
const READER: &str = "3f2c6b7e-0000-4000-8000-0000000000d1";
const PARTNER: &str = "3f2c6b7e-0000-4000-8000-0000000000d2";
const OTHER: &str = "3f2c6b7e-0000-4000-8000-0000000000d3";

#[tokio::test]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test pack_rhythm -- --ignored`"]
#[expect(clippy::too_many_lines)]
async fn a_schedule_drafts_an_office_reviews_a_person_issues_and_subscribers_are_told() {
    let (keys, kid) = keys_and_kid();
    // SAFETY: this binary is single-test, so nothing races the process-wide verifier.
    unsafe {
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS", keys.to_string());
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH", "1");
        std::env::set_var(
            "PROJECT_PORTFOLIO_MANAGEMENT_ABAC_POLICY",
            json!({ "rules": [{ "effect": "allow", "actions": ["*"], "when": { "access": ["write", "admin"] } }] }).to_string(),
        );
    }
    let writer = bearer(&kid, WRITER, &[("access", "write")]);
    let editor = bearer(
        &kid,
        EDITOR,
        &[("access", "write"), ("directorpack", "editor")],
    );
    let issuer = bearer(
        &kid,
        ISSUER_SUB,
        &[("access", "write"), ("directorpack", "issuer")],
    );
    let reader = bearer(
        &kid,
        READER,
        &[("access", "write"), ("pack_audience", "board")],
    );
    let other = bearer(
        &kid,
        OTHER,
        &[("access", "write"), ("pack_audience", "board")],
    );
    let today = chrono::Utc::now().date_naive();
    let prev = {
        use chrono::Datelike;
        let first = today.with_day(1).unwrap();
        let last = first.pred_opt().unwrap();
        format!("{:04}-{:02}", last.year(), last.month())
    };
    request::<App, _, _>(|request, ctx| async move {
        macro_rules! with { ($rb:expr, $who:expr) => { $rb.add_header($who.0.clone(), $who.1.clone()) }; }
        let url = "/api/director-pack-schedules";

        // ── who may schedule ──
        assert_eq!(with!(request.post(url), writer).json(&json!({ "audience": "board", "day_of_month": 5 })).await.status_code(), 403);
        for bad in [json!({ "audience": "board", "day_of_month": 0 }), json!({ "audience": "board", "day_of_month": 29 }), json!({ "audience": "club", "day_of_month": 5 })] {
            assert_eq!(with!(request.post(url), editor).json(&bad).await.status_code(), 422, "{bad}");
        }
        let s: Value = with!(request.post(url), editor).json(&json!({ "audience": "board", "day_of_month": 5 })).await.json();
        let sid = s["pid"].as_str().unwrap().to_string();
        assert!(s["next_run_date"].as_str().unwrap() > today.to_string().as_str(), "the first run is in the future: {s}");
        assert_eq!(with!(request.post(url), editor).json(&json!({ "audience": "board", "day_of_month": 6 })).await.status_code(), 409, "one schedule per audience");

        // ── nothing is due yet; then it is ──
        let none: Value = with!(request.post(&format!("{url}/sweep")), editor).await.json();
        assert_eq!((none["drafted"].as_array().unwrap().len(), none["issued"].as_i64()), (0, Some(0)));
        ctx.db.execute_unprepared("UPDATE director_pack_schedules SET next_run_date = current_date - 1").await.unwrap();
        assert_eq!(with!(request.post(&format!("{url}/sweep")), writer).await.status_code(), 403);
        let swept: Value = with!(request.post(&format!("{url}/sweep")), editor).await.json();
        assert_eq!(swept["drafted"].as_array().unwrap().len(), 1, "{swept}");
        assert_eq!(swept["issued"], 0, "a schedule never issues");
        let again: Value = with!(request.post(&format!("{url}/sweep")), editor).await.json();
        assert_eq!(again["drafted"].as_array().unwrap().len(), 0, "the date moved first, so it does not draft twice");
        let pid = swept["drafted"][0].as_str().unwrap().to_string();

        // The draft: the month that just ended, covering all of it, made by the schedule, not yet reviewed.
        let got: Value = with!(request.get(&format!("/api/director-packs/{pid}")), editor).await.json();
        let m = &got["pack"];
        assert_eq!((m["period"].as_str(), m["status"].as_str(), m["origin"].as_str(), m["reviewed"].as_bool()), (Some(prev.as_str()), Some("draft"), Some("scheduled"), Some(false)), "{m}");
        assert_eq!((m["covers_whole_period"].as_bool(), m["stale"].as_bool()), (Some(true), Some(false)));
        let packs: Value = with!(request.get("/api/director-packs"), issuer).await.json();
        assert!(packs["packs"].as_array().unwrap().iter().all(|p| p["status"] != "issued"), "still nothing issued");
        // A due schedule with the month already drafted skips it.
        ctx.db.execute_unprepared("UPDATE director_pack_schedules SET next_run_date = current_date - 1").await.unwrap();
        let skipped: Value = with!(request.post(&format!("{url}/sweep")), editor).await.json();
        assert_eq!((skipped["drafted"].as_array().unwrap().len(), skipped["skipped"].as_i64()), (0, Some(1)));
        let listed: Value = with!(request.get(url), editor).await.json();
        assert!(listed["schedules"][0]["last_result"].as_str().unwrap().starts_with("skipped"), "{listed}");

        // ── subscribers ──
        let subs = "/api/director-pack-subscriptions";
        assert_eq!(with!(request.put(subs), reader).json(&json!({ "audience": "executive", "subscribed": true })).await.status_code(), 403, "not an audience you may read");
        assert_eq!(with!(request.put(subs), writer).json(&json!({ "audience": "board", "subscribed": true })).await.status_code(), 403);
        with!(request.put(subs), reader).json(&json!({ "audience": "board", "subscribed": true })).await.assert_status_ok();
        with!(request.put(subs), reader).json(&json!({ "audience": "board", "subscribed": true })).await.assert_status_ok(); // idempotent
        with!(request.put(subs), other).json(&json!({ "audience": "board", "subscribed": true })).await.assert_status_ok();
        with!(request.put(subs), other).json(&json!({ "audience": "board", "subscribed": false })).await.assert_status_ok(); // and left again
        assert_eq!(with!(request.put(subs), reader).json(&json!({ "audience": "board", "subscribed": true, "person": PARTNER })).await.status_code(), 403, "only an editor subscribes someone else");
        with!(request.put(subs), editor).json(&json!({ "audience": "board", "subscribed": true, "person": PARTNER })).await.assert_status_ok();
        let mine: Value = with!(request.get(subs), reader).await.json();
        assert_eq!(mine["subscribed_to"], json!(["board"]));
        let rows = ctx.db.query_all_raw(sea_orm::Statement::from_string(sea_orm::DatabaseBackend::Postgres, "SELECT count(*) AS n FROM director_pack_subscriptions".to_string())).await.unwrap();
        assert_eq!(rows[0].try_get::<i64>("", "n").unwrap(), 2, "the reader and the partner, not the one who left");

        // ── the review comes before the issue ──
        assert_eq!(with!(request.post(&format!("/api/director-packs/{pid}/issue")), issuer).await.status_code(), 409, "not reviewed");
        assert_eq!(with!(request.post(&format!("/api/director-packs/{pid}/review")), writer).await.status_code(), 403);
        with!(request.post(&format!("/api/director-packs/{pid}/review")), editor).await.assert_status_ok();
        assert_eq!(with!(request.post(&format!("/api/director-packs/{pid}/review")), editor).await.status_code(), 409, "once");
        assert_eq!(with!(request.post(&format!("/api/director-packs/{pid}/issue")), editor).await.status_code(), 403, "an editor reviews; a person with the right issues");
        with!(request.post(&format!("/api/director-packs/{pid}/issue")), issuer).await.assert_status_ok();
        assert_eq!(with!(request.post(&format!("/api/director-packs/{pid}/review")), editor).await.status_code(), 409, "an issued pack is frozen");

        // ── the notices: a link, never the body ──
        let notes = ctx.db.query_all_raw(sea_orm::Statement::from_string(sea_orm::DatabaseBackend::Postgres,
            "SELECT recipient_ref, message FROM notifications WHERE kind = 'director_pack_issued' ORDER BY id".to_string())).await.unwrap();
        let told: Vec<(String, String)> = notes.iter().map(|r| (r.try_get("", "recipient_ref").unwrap(), r.try_get("", "message").unwrap())).collect();
        assert_eq!(told.len(), 2, "{told:?}");
        assert!(told.iter().any(|(r, _)| r == &format!("person:{READER}")) && told.iter().any(|(r, _)| r == &format!("person:{PARTNER}")));
        assert!(!told.iter().any(|(r, _)| r.contains(&OTHER.to_string())), "the one who left is not told");
        for (_, msg) in &told {
            assert!(msg.contains(&prev) && msg.contains("board") && msg.contains(&format!("/director-packs/{pid}")), "{msg}");
            assert!(msg.len() < 220 && !msg.contains("plans_included") && !msg.contains("portfolio_map"), "a link, not the body: {msg}");
        }
        // Issuing the same pack again is refused, so nobody is told twice.
        assert_eq!(with!(request.post(&format!("/api/director-packs/{pid}/issue")), issuer).await.status_code(), 409);

        // ── a hand-made draft that stops short of its period is stale ──
        let early = {
            use chrono::Datelike;
            let first = today.with_day(1).unwrap();
            let two_back = first.checked_sub_months(chrono::Months::new(2)).unwrap();
            two_back + chrono::Duration::days(2)
        };
        let d: Value = with!(request.post("/api/director-packs"), editor)
            .json(&json!({ "period": early.format("%Y-%m").to_string(), "audience": "executive", "cut_off": format!("{early}T12:00:00Z") })).await.json();
        assert_eq!((d["stale"].as_bool(), d["covers_whole_period"].as_bool(), d["origin"].as_str()), (Some(true), Some(false), Some("manual")), "{d}");
        assert_eq!(d["reviewed"], false);
        // A hand-made draft needs no review to be issued (the review gates only what the schedule drafts).
        with!(request.post(&format!("/api/director-packs/{}/issue", d["pid"].as_str().unwrap())), issuer).await.assert_status_ok();

        // ── off means off ──
        with!(request.put(&format!("{url}/{sid}")), editor).json(&json!({ "enabled": false })).await.assert_status_ok();
        ctx.db.execute_unprepared("UPDATE director_pack_schedules SET next_run_date = current_date - 1").await.unwrap();
        let off: Value = with!(request.post(&format!("{url}/sweep")), editor).await.json();
        assert_eq!((off["drafted"].as_array().unwrap().len(), off["skipped"].as_i64()), (0, Some(0)));
        assert_eq!(with!(request.put(&format!("{url}/{sid}")), editor).json(&json!({ "day_of_month": 31 })).await.status_code(), 422);

        // ── the database ──
        for (what, sql) in [
            ("a day of 30", "UPDATE director_pack_schedules SET day_of_month = 30".to_string()),
            ("a change to an issued pack's review", format!("UPDATE director_packs SET reviewed_by = 'x' WHERE pid = '{pid}'")),
        ] {
            assert!(ctx.db.execute_unprepared(&sql).await.is_err(), "{what}");
        }
        let trail = ctx.db.query_all_raw(sea_orm::Statement::from_string(sea_orm::DatabaseBackend::Postgres,
            "SELECT action, actor FROM audit_logs WHERE action IN ('director_pack_drafted_by_schedule','director_pack_reviewed','director_pack_issued') ORDER BY id".to_string())).await.unwrap();
        let trail: Vec<(String, String)> = trail.iter().map(|r| (r.try_get("", "action").unwrap(), r.try_get("", "actor").unwrap())).collect();
        assert!(trail.contains(&("director_pack_drafted_by_schedule".to_string(), "schedule".to_string())), "{trail:?}");
        assert!(trail.contains(&("director_pack_reviewed".to_string(), EDITOR.to_string())));
        assert!(trail.contains(&("director_pack_issued".to_string(), ISSUER_SUB.to_string())));
    })
    .await;
}
