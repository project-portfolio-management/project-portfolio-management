//! Feeds (LT-7 iCalendar, RM-6 Atom), end to end with real PASETO
//! identities and enforcement ON: the feed paths authenticate by token alone
//! while every other route still needs a bearer; revocation is immediate.
//!
//! Its **own test binary**, for the same reason `enforcement.rs` is:
//! `verifier` is a process-wide `OnceLock`, read once on first use —
//! sharing the `tests/requests/mod.rs` binary would race whichever
//! test happens to touch an authenticated route first.
//!
//! `#[ignore]`d — boots the app, so it needs PostgreSQL via
//! `config/test.yaml` / `DATABASE_URL`. Run with
//! `cargo test --test feeds -- --ignored`.
#![cfg(feature = "paseto")]

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use ed25519_dalek::SigningKey;
use loco_rs::testing::prelude::*;
use project_portfolio_management_service::app::App;
use rusty_paseto::core::{Footer, Key, Paseto, PasetoAsymmetricPrivateKey, Payload, Public, V4};
use sea_orm::EntityTrait;
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
        "sid": "test-sid", "attrs": { "access": ["write"] },
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

/// LT-7 / RM-6 acceptance.
#[tokio::test]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test feeds -- --ignored`"]
async fn feeds_authenticate_by_token_are_stable_and_revocable() {
    let (keys, kid) = keys_and_kid();
    // SAFETY: this binary is single-test, so no other test races these
    // process-wide `OnceLock`s (verifier + enforcement flag).
    unsafe {
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS", keys.to_string());
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH", "true");
    }
    let (ak, av) = auth_header(&mint(&kid, ALICE));
    let (bk, bv) = auth_header(&mint(&kid, BOB));

    request::<App, _, _>(|request, ctx| async move {
        // With enforcement on, an ordinary route needs a bearer ...
        assert_eq!(request.get("/api/plans").await.status_code(), 401);
        // ... and a feed path does not get that far without a token either.
        assert_eq!(
            request
                .get("/api/feeds/calendar.ics?token=nope")
                .await
                .status_code(),
            401,
            "a malformed token is rejected by the feed, not by the bearer guard"
        );

        let plan: Value = request
            .post("/api/plans")
            .add_header(ak.clone(), av.clone())
            .json(&json!({ "kind": "Project", "name": "Apollo" }))
            .await
            .json();
        let p = plan["pid"].as_str().expect("plan pid").to_string();
        let mk = |title: &str, who: &str, due: Option<&str>| {
            let mut body = json!({ "title": title, "assignee_ref": who });
            if let Some(d) = due {
                body["due_date"] = json!(d);
            }
            body
        };
        let alice = format!("person:{ALICE}");
        let bob = format!("person:{BOB}");
        let mut alice_task = String::new();
        for (body, mine) in [
            (mk("Ship, the thing", &alice, Some("2026-10-10")), true),
            (mk("Undated", &alice, None), false),
            (mk("Bob's secret", &bob, Some("2026-10-11")), false),
        ] {
            let t: Value = request
                .post(&format!("/api/plans/{p}/tasks"))
                .add_header(ak.clone(), av.clone())
                .json(&body)
                .await
                .json();
            if mine {
                alice_task = t["pid"].as_str().expect("task pid").to_string();
            }
        }
        request
            .post(&format!("/api/plans/{p}/milestones"))
            .add_header(ak.clone(), av.clone())
            .json(&json!({ "name": "Beta", "due": "2026-10-20" }))
            .await
            .assert_status_ok();
        request
            .post(&format!("/api/plans/{p}/sprints"))
            .add_header(ak.clone(), av.clone())
            .json(&json!({ "name": "S1", "starts_on": "2026-10-05", "ends_on": "2026-10-16" }))
            .await
            .assert_status_ok();

        // Mint a token; it is shown once and only its hash is stored.
        let issued: Value = request
            .post("/api/me/feed-token")
            .add_header(ak.clone(), av.clone())
            .await
            .json();
        let token = issued["token"].as_str().expect("token").to_string();
        assert_eq!(token.len(), 64);
        let stored =
            project_portfolio_management_service::models::_entities::feed_tokens::Entity::find()
                .all(&ctx.db)
                .await
                .expect("tokens");
        assert_eq!(stored.len(), 1);
        assert_ne!(stored[0].token_hash, token, "never stored in the clear");

        // iCalendar: no bearer needed; the owner's own items only.
        let cal = request
            .get(&format!("/api/feeds/calendar.ics?token={token}"))
            .await;
        cal.assert_status_ok();
        assert!(
            cal.headers()
                .get("content-type")
                .is_some_and(|v| v.to_str().unwrap().starts_with("text/calendar"))
        );
        let text = cal.text();
        assert!(text.starts_with("BEGIN:VCALENDAR\r\n"));
        assert!(text.contains(&format!("UID:task-{alice_task}@ppm")));
        assert!(text.contains("DTSTART;VALUE=DATE:20261010"));
        assert!(
            text.contains("SUMMARY:Ship\\, the thing"),
            "escaped: {text}"
        );
        assert!(text.contains("Milestone: Beta") && text.contains("Sprint starts: S1"));
        assert!(
            !text.contains("Bob's secret"),
            "another person's task never appears"
        );
        assert!(
            !text.contains("Undated"),
            "an undated task has no day to show"
        );
        assert!(
            text.replace("\r\n", "").find('\n').is_none(),
            "CRLF line ends only"
        );
        // Stable UIDs across fetches.
        let uids = |t: &str| {
            t.lines()
                .filter(|l| l.starts_with("UID:"))
                .map(str::to_string)
                .collect::<Vec<_>>()
        };
        let again = request
            .get(&format!("/api/feeds/calendar.ics?token={token}"))
            .await
            .text();
        assert_eq!(uids(&text), uids(&again));

        // Atom + conditional GET.
        let a1 = request
            .get(&format!("/api/feeds/me.atom?token={token}"))
            .await;
        a1.assert_status_ok();
        let etag = a1.headers().get("etag").expect("etag").clone();
        let cond = request
            .get(&format!("/api/feeds/me.atom?token={token}"))
            .add_header(axum::http::header::IF_NONE_MATCH, etag.clone())
            .await;
        assert_eq!(cond.status_code(), 304, "unchanged feed ⇒ 304");
        // A new notification changes the feed.
        request
            .post(&format!("/api/plans/{p}/tasks/{alice_task}/assign"))
            .add_header(bk.clone(), bv.clone())
            .json(&json!({ "assignee_ref": alice }))
            .await
            .assert_status_ok();
        let a2 = request
            .get(&format!("/api/feeds/me.atom?token={token}"))
            .add_header(axum::http::header::IF_NONE_MATCH, etag)
            .await;
        a2.assert_status_ok();
        assert!(a2.text().contains("You were assigned"));

        // Revocation is immediate, and one user cannot revoke another's.
        request
            .post("/api/me/feed-token/revoke")
            .add_header(bk, bv)
            .await
            .assert_status_ok();
        request
            .get(&format!("/api/feeds/calendar.ics?token={token}"))
            .await
            .assert_status_ok();
        request
            .post("/api/me/feed-token/revoke")
            .add_header(ak, av)
            .await
            .assert_status_ok();
        assert_eq!(
            request
                .get(&format!("/api/feeds/calendar.ics?token={token}"))
                .await
                .status_code(),
            401
        );
        assert_eq!(
            request
                .get(&format!("/api/feeds/me.atom?token={token}"))
                .await
                .status_code(),
            401
        );
    })
    .await;
}
