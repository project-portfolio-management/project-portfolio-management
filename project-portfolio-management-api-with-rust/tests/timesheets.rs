//! Timesheets (LT-11) and time reports (RM-10), end to end over the real
//! router with real PASETO identities: an owner, an approver and a bystander.
//!
//! Its **own test binary**, for the same reason `enforcement.rs` is:
//! `verifier` is a process-wide `OnceLock`, read once on first use —
//! sharing the `tests/requests/mod.rs` binary would race whichever
//! test happens to touch an authenticated route first.
//!
//! `#[ignore]`d — boots the app, so it needs PostgreSQL via
//! `config/test.yaml` / `DATABASE_URL`. Run with
//! `cargo test --test timesheets -- --ignored`.
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
const CAROL: &str = "3f2c6b7e-0000-4000-8000-00000000000c";

/// LT-11 / RM-10 acceptance.
#[tokio::test]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test timesheets -- --ignored`"]
async fn timesheets_report_submit_approve_and_lock() {
    let (keys, kid) = keys_and_kid();
    // SAFETY: this binary is single-test, so no other test races the
    // process-wide verifier `OnceLock` it sets.
    unsafe {
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS", keys.to_string());
    }
    let (ak, av) = auth_header(&mint(&kid, ALICE, &[]));
    let (bk, bv) = auth_header(&mint(&kid, BOB, &[]));
    let (ck, cv) = auth_header(&mint(&kid, CAROL, &["timesheet_approver"]));

    request::<App, _, _>(|request, _ctx| async move {
        let alice = format!("person:{ALICE}");
        let mut plans = Vec::new();
        for name in ["Apollo", "Borealis"] {
            let p: Value = request
                .post("/api/plans")
                .json(&json!({ "kind": "Project", "name": name }))
                .await
                .json();
            plans.push(p["pid"].as_str().expect("plan pid").to_string());
        }
        let log = |plan: &str, day: &str, minutes: i64, billable: bool| {
            let (request, body) = (&request, json!({ "actor_ref": alice, "spent_on": day, "minutes": minutes, "billable": billable }));
            let url = format!("/api/plans/{plan}/time-entries");
            async move { request.post(&url).json(&body).await.status_code().as_u16() }
        };
        // ISO week 2026-W41 is Mon 5 Oct .. Sun 11 Oct.
        assert_eq!(log(&plans[0], "2026-10-05", 60, true).await, 200);
        assert_eq!(log(&plans[0], "2026-10-05", 30, false).await, 200);
        assert_eq!(log(&plans[1], "2026-10-07", 45, false).await, 200);
        assert_eq!(log(&plans[1], "2026-10-13", 99, true).await, 200, "the following week");

        let week = |k: axum::http::HeaderName, v: axum::http::HeaderValue, w: &'static str, person: Option<&'static str>| {
            let request = &request;
            let q = match person {
                Some(p) => format!("/api/timesheets?week={w}&person={p}"),
                None => format!("/api/timesheets?week={w}"),
            };
            async move { request.get(&q).add_header(k, v).await }
        };
        let own: Value = week(ak.clone(), av.clone(), "2026-W41", None).await.json();
        assert_eq!(own["status"], "open");
        assert_eq!(own["grid"]["total"], 135, "the next week's entry is not in this week");
        assert_eq!(own["grid"]["split"], json!({ "billable": 60, "non_billable": 75 }), "never merged");
        assert_eq!(own["grid"]["day_totals"][0], 90);
        assert_eq!(own["grid"]["rows"].as_array().unwrap().len(), 2);

        // An empty week reports null + a reason, not zero.
        let empty: Value = week(ak.clone(), av.clone(), "2026-W30", None).await.json();
        assert!(empty["grid"]["total"].is_null());
        assert_eq!(empty["grid"]["reason"], "no_entries");

        // A bystander cannot read it; an approver can.
        let person = Box::leak(alice.clone().into_boxed_str());
        assert_eq!(week(bk.clone(), bv.clone(), "2026-W41", Some(person)).await.status_code(), 403);
        assert_eq!(week(ck.clone(), cv.clone(), "2026-W41", Some(person)).await.status_code(), 200);

        // Submit: an empty week is refused; your own only; once.
        let post = |k: axum::http::HeaderName, v: axum::http::HeaderValue, path: &'static str, body: Value| {
            let request = &request;
            async move { request.post(path).add_header(k, v).json(&body).await }
        };
        assert_eq!(post(ak.clone(), av.clone(), "/api/timesheets/submit", json!({ "week": "2026-W30" })).await.status_code(), 422);
        assert_eq!(post(bk.clone(), bv.clone(), "/api/timesheets/submit", json!({ "week": "2026-W41", "person": alice })).await.status_code(), 403, "not their week");
        assert_eq!(post(ak.clone(), av.clone(), "/api/timesheets/submit", json!({ "week": "2026-W41" })).await.status_code(), 200);
        assert_eq!(post(ak.clone(), av.clone(), "/api/timesheets/submit", json!({ "week": "2026-W41" })).await.status_code(), 409);

        // Deciding: needs the approver right; a rejection needs a comment.
        let decide = |k: axum::http::HeaderName, v: axum::http::HeaderValue, body: Value| {
            post(k, v, "/api/timesheets/decide", body)
        };
        assert_eq!(decide(bk.clone(), bv.clone(), json!({ "person": alice, "week": "2026-W41", "decision": "approved" })).await.status_code(), 403);
        assert_eq!(decide(ck.clone(), cv.clone(), json!({ "person": alice, "week": "2026-W41", "decision": "rejected" })).await.status_code(), 422);
        let rej = decide(ck.clone(), cv.clone(), json!({ "person": alice, "week": "2026-W41", "decision": "rejected", "comment": "fix Monday" })).await;
        assert_eq!(rej.status_code(), 200);
        let after: Value = week(ak.clone(), av.clone(), "2026-W41", None).await.json();
        assert_eq!(after["status"], "rejected");
        assert_eq!(after["decision_comment"], "fix Monday");

        // Resubmit, then approve; approved is final and locks the week.
        assert_eq!(post(ak.clone(), av.clone(), "/api/timesheets/submit", json!({ "week": "2026-W41" })).await.status_code(), 200);
        assert_eq!(decide(ck.clone(), cv.clone(), json!({ "person": alice, "week": "2026-W41", "decision": "approved" })).await.status_code(), 200);
        assert_eq!(decide(ck.clone(), cv.clone(), json!({ "person": alice, "week": "2026-W41", "decision": "rejected", "comment": "too late" })).await.status_code(), 409, "approved is final");
        assert_eq!(log(&plans[0], "2026-10-06", 10, true).await, 409, "an approved week is locked");
        assert_eq!(log(&plans[0], "2026-10-14", 10, true).await, 200, "a different week is not");

        // Time report: both halves always; totals reconcile; CSV; empty ⇒ reason.
        let rep: Value = request
            .get("/api/time-reports?group_by=plan&from=2026-10-01&to=2026-10-31")
            .await
            .json();
        let groups = rep["groups"].as_array().unwrap();
        assert_eq!(groups.len(), 2);
        let sum: i64 = groups.iter().map(|g| g["minutes"].as_i64().unwrap()).sum();
        assert_eq!(rep["total_minutes"].as_i64().unwrap(), sum);
        assert_eq!(rep["split"]["billable"].as_i64().unwrap() + rep["split"]["non_billable"].as_i64().unwrap(), sum);
        let by_billable: Value = request.get("/api/time-reports?group_by=billable&from=2026-10-01&to=2026-10-31").await.json();
        assert_eq!(by_billable["groups"].as_array().unwrap().len(), 2, "billable and non-billable are separate groups");
        let csv = request.get("/api/time-reports?group_by=month&from=2026-10-01&to=2026-10-31&format=csv").await;
        assert!(csv.text().starts_with("group,entries,billable_minutes,non_billable_minutes,minutes\n2026-10,"));
        let none: Value = request.get("/api/time-reports?group_by=person&from=2020-01-01&to=2020-01-31").await.json();
        assert!(none["total_minutes"].is_null());
        assert_eq!(none["reason"], "no_entries");
        assert_eq!(request.get("/api/time-reports?group_by=nope").await.status_code(), 422);
    })
    .await;
}
