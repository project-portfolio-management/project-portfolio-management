//! Personal planner (PL-4), end to end over the real router with real PASETO
//! identities: slots, clashes (overlap yes, touching no), the day in the
//! caller's own zone, privacy between users, and the calendar feed.
//!
//! Its **own test binary**, for the same reason `enforcement.rs` is:
//! `verifier` is a process-wide `OnceLock`, read once on first use —
//! sharing the `tests/requests/mod.rs` binary would race whichever
//! test happens to touch an authenticated route first.
//!
//! `#[ignore]`d — boots the app, so it needs PostgreSQL via
//! `config/test.yaml` / `DATABASE_URL`. Run with
//! `cargo test --test planner -- --ignored`.
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

/// PL-4 acceptance.
#[tokio::test]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test planner -- --ignored`"]
async fn slots_clash_when_they_overlap_render_in_the_callers_zone_and_stay_private() {
    let (keys, kid) = keys_and_kid();
    // SAFETY: this binary is single-test, so no other test races the
    // process-wide verifier `OnceLock` it sets.
    unsafe {
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS", keys.to_string());
    }
    let (ak, av) = auth_header(&mint(&kid, ALICE));
    let (bk, bv) = auth_header(&mint(&kid, BOB));

    request::<App, _, _>(|request, _ctx| async move {
        let plan: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Apollo" }))
            .await
            .json();
        let p = plan["pid"].as_str().expect("plan pid").to_string();
        let task: Value = request
            .post(&format!("/api/plans/{p}/tasks"))
            .json(&json!({ "title": "Write the spec", "due_date": "2026-10-03",
                           "assignee_ref": format!("person:{ALICE}") }))
            .await
            .json();
        let t = task["pid"].as_str().expect("task pid").to_string();

        let slot = |starts: &str, ends: &str| {
            let (request, ak, av, t) = (&request, ak.clone(), av.clone(), t.clone());
            let body = json!({ "task_pid": t, "starts_at": starts, "ends_at": ends });
            async move { request.post("/api/me/planned-slots").add_header(ak, av).json(&body).await }
        };
        let first: Value = slot("2026-10-03T09:00:00Z", "2026-10-03T10:30:00Z").await.json();
        slot("2026-10-03T10:00:00Z", "2026-10-03T11:00:00Z").await.assert_status_ok();
        slot("2026-10-03T13:00:00Z", "2026-10-03T14:00:00Z").await.assert_status_ok();
        slot("2026-10-03T14:00:00Z", "2026-10-03T15:00:00Z").await.assert_status_ok(); // touches the 13:00 one

        // Bad slots.
        assert_eq!(slot("2026-10-03T10:00:00Z", "2026-10-03T10:00:00Z").await.status_code(), 422);
        assert_eq!(slot("2026-10-03T00:00:00Z", "2026-10-03T13:00:00Z").await.status_code(), 422, "over 12 hours");
        let unknown = request
            .post("/api/me/planned-slots")
            .add_header(ak.clone(), av.clone())
            .json(&json!({ "task_pid": "3f2c6b7e-0000-4000-8000-000000000bad",
                           "starts_at": "2026-10-03T09:00:00Z", "ends_at": "2026-10-03T10:00:00Z" }))
            .await;
        assert_eq!(unknown.status_code(), 404);

        // A slot never moves the task's due date.
        let tasks: Value = request.get(&format!("/api/plans/{p}/tasks")).await.json();
        assert_eq!(tasks["tasks"][0]["due_date"], "2026-10-03");

        // The day, in the caller's zone. LA (UTC-7): the whole of 2026-10-03 UTC-ish.
        let day = |tz: &'static str| {
            let (request, ak, av) = (&request, ak.clone(), av.clone());
            async move {
                request
                    .get(&format!("/api/me/plan?day=2026-10-03&tz={tz}"))
                    .add_header(ak, av)
                    .await
                    .json::<Value>()
            }
        };
        let la = day("America/Los_Angeles").await;
        assert_eq!(la["slots"].as_array().unwrap().len(), 4);
        assert_eq!(la["clashes"].as_array().unwrap().len(), 1, "09:00-10:30 overlaps 10:00-11:00; touching slots do not clash");
        assert_eq!(la["clashes"][0].as_array().unwrap().len(), 2);
        assert!(la["slots"][0]["starts_local"].as_str().unwrap().ends_with("-07:00"), "{}", la["slots"][0]);
        assert_eq!(la["due_on_day"][0]["title"], "Write the spec");
        // Auckland (UTC+13): local 2026-10-03 is 2026-10-02 11:00Z .. 2026-10-03 11:00Z,
        // so the 13:00Z and 14:00Z slots fall on the *next* local day.
        let nz = day("Pacific/Auckland").await;
        assert_eq!(nz["slots"].as_array().unwrap().len(), 2, "only the two before 11:00Z");
        assert!(nz["slots"][0]["starts_local"].as_str().unwrap().ends_with("+13:00"));
        assert_eq!(
            request.get("/api/me/plan?day=2026-10-03&tz=%2B02:00").add_header(ak.clone(), av.clone()).await.status_code(),
            422
        );

        // Privacy: Bob sees none of Alice's slots and cannot delete one.
        let bob: Value = request
            .get("/api/me/plan?day=2026-10-03&tz=UTC")
            .add_header(bk.clone(), bv.clone())
            .await
            .json();
        assert!(bob["slots"].as_array().unwrap().is_empty());
        let sid = first["pid"].as_str().expect("slot pid").to_string();
        assert_eq!(
            request.post(&format!("/api/me/planned-slots/{sid}/delete")).add_header(bk, bv).await.status_code(),
            404
        );

        // The calendar feed carries the slots as timed UTC events.
        let issued: Value = request.post("/api/me/feed-token").add_header(ak.clone(), av.clone()).await.json();
        let token = issued["token"].as_str().unwrap().to_string();
        let ics = request.get(&format!("/api/feeds/calendar.ics?token={token}")).await.text();
        assert!(ics.contains("DTSTART:20261003T090000Z") && ics.contains("Focus: Write the spec"), "{ics}");

        // Remove one of your own (POST twin); the clash disappears with it.
        request
            .post(&format!("/api/me/planned-slots/{sid}/delete"))
            .add_header(ak.clone(), av.clone())
            .await
            .assert_status_ok();
        let after = day("UTC").await;
        assert_eq!(after["slots"].as_array().unwrap().len(), 3);
        assert!(after["clashes"].as_array().unwrap().is_empty());
        assert_eq!(after["window"]["hours"], 24);
    })
    .await;
}
