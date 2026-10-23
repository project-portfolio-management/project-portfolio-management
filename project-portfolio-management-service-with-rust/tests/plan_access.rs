//! Private plans (RM-8), end to end over the real router with real PASETO
//! identities: a non-member gets 404 (never 403) everywhere a plan is
//! addressed, lists omit it, an admin and a member see it, and it fails
//! closed for an anonymous caller.
//!
//! Its **own test binary**, for the same reason `enforcement.rs` is:
//! `verifier` is a process-wide `OnceLock`, read once on first use —
//! sharing the `tests/requests/mod.rs` binary would race whichever
//! test happens to touch an authenticated route first.
//!
//! `#[ignore]`d — boots the app, so it needs PostgreSQL via
//! `config/test.yaml` / `DATABASE_URL`. Run with
//! `cargo test --test plan_access -- --ignored`.

#![expect(
    clippy::too_many_lines,
    reason = "request-level test: long, linear, short names; recorded under CQ-14"
)]
#![cfg(feature = "paseto")]

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use ed25519_dalek::SigningKey;
use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
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
const CAROL: &str = "3f2c6b7e-0000-4000-8000-00000000000c";

/// RM-8 acceptance.
#[tokio::test]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test plan_access -- --ignored`"]
async fn a_private_plan_is_404_to_non_members_and_visible_to_members_and_admins() {
    let (keys, kid) = keys_and_kid();
    // SAFETY: this binary is single-test, so no other test races the
    // process-wide verifier `OnceLock` it sets.
    unsafe {
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS", keys.to_string());
    }
    let (ak, av) = auth_header(&mint(&kid, ALICE, &["write"]));
    let (bk, bv) = auth_header(&mint(&kid, BOB, &["write"]));
    let (ck, cv) = auth_header(&mint(&kid, CAROL, &["admin"]));

    request::<App, _, _>(|request, _ctx| async move {
        let plan: Value = request
            .post("/api/plans")
            .add_header(ak.clone(), av.clone())
            .json(&json!({ "kind": "Project", "name": "Secret Apollo" }))
            .await
            .json();
        let p = plan["pid"].as_str().expect("plan pid").to_string();
        let task: Value = request
            .post(&format!("/api/plans/{p}/tasks"))
            .add_header(ak.clone(), av.clone())
            .json(&json!({ "title": "Hidden task" }))
            .await
            .json();
        let t = task["pid"].as_str().expect("task pid").to_string();

        let status =
            |path: String, k: Option<(axum::http::HeaderName, axum::http::HeaderValue)>| {
                let request = &request;
                async move {
                    let mut r = request.get(&path);
                    if let Some((k, v)) = k {
                        r = r.add_header(k, v);
                    }
                    r.await.status_code().as_u16()
                }
            };
        let bob = || Some((bk.clone(), bv.clone()));
        let alice = || Some((ak.clone(), av.clone()));
        let carol = || Some((ck.clone(), cv.clone()));

        // Public by default: everyone, including anonymous, sees it.
        assert_eq!(status(format!("/api/plans/{p}"), bob()).await, 200);
        assert_eq!(status(format!("/api/plans/{p}"), None).await, 200);

        // Make it private.
        let put: Value = request
            .put(&format!("/api/plans/{p}/access"))
            .add_header(ak.clone(), av.clone())
            .json(&json!({ "private": true, "members": [] }))
            .await
            .json();
        assert_eq!(put["private"], true);
        assert_eq!(
            put["members"],
            json!([ALICE]),
            "the caller is always a member"
        );

        // A non-member, and an anonymous caller, get 404 — never 403 — on the
        // record and on everything beneath it.
        for path in [
            format!("/api/plans/{p}"),
            format!("/api/plans/{p}/tasks"),
            format!("/api/plans/{p}/access"),
        ] {
            assert_eq!(status(path.clone(), bob()).await, 404, "bob {path}");
            assert_eq!(status(path.clone(), None).await, 404, "anonymous {path}");
        }
        // A member and an admin see it.
        assert_eq!(status(format!("/api/plans/{p}"), alice()).await, 200);
        assert_eq!(
            status(format!("/api/plans/{p}/tasks"), carol()).await,
            200,
            "an admin who is not a member"
        );

        // Lists omit it for a non-member and include it for a member.
        let names = |v: Value| -> Vec<String> {
            v.as_array()
                .expect("list")
                .iter()
                .map(|p| p["name"].as_str().unwrap_or_default().to_string())
                .collect()
        };
        let bob_list: Value = request
            .get("/api/plans")
            .add_header(bk.clone(), bv.clone())
            .await
            .json();
        assert!(!names(bob_list).contains(&"Secret Apollo".to_string()));
        let alice_list: Value = request
            .get("/api/plans")
            .add_header(ak.clone(), av.clone())
            .await
            .json();
        assert!(names(alice_list).contains(&"Secret Apollo".to_string()));

        // Discussion resolves its subject through the same check.
        assert_eq!(
            request
                .post("/api/comments")
                .add_header(bk.clone(), bv.clone())
                .json(&json!({ "subject_kind": "task", "subject_pid": t, "body": "peek" }))
                .await
                .status_code(),
            404
        );
        // A non-member cannot change the access either.
        assert_eq!(
            request
                .put(&format!("/api/plans/{p}/access"))
                .add_header(bk.clone(), bv.clone())
                .json(&json!({ "private": false, "members": [] }))
                .await
                .status_code(),
            404
        );

        // "My work" omits a private plan's tasks for a non-member, and says so.
        request
            .post(&format!("/api/plans/{p}/tasks/{t}/assign"))
            .add_header(ak.clone(), av.clone())
            .json(&json!({ "assignee_ref": format!("person:{BOB}") }))
            .await
            .assert_status_ok();
        let mine: Value = request
            .get("/api/me/tasks?tz=UTC")
            .add_header(bk.clone(), bv.clone())
            .await
            .json();
        assert_eq!(mine["total"], 0);
        assert_eq!(mine["omitted_unreadable"], 1);

        // Adding Bob as a member opens it to him.
        request
            .put(&format!("/api/plans/{p}/access"))
            .add_header(ak.clone(), av.clone())
            .json(&json!({ "private": true, "members": [BOB] }))
            .await
            .assert_status_ok();
        assert_eq!(status(format!("/api/plans/{p}"), bob()).await, 200);
        let mine: Value = request
            .get("/api/me/tasks?tz=UTC")
            .add_header(bk.clone(), bv.clone())
            .await
            .json();
        assert_eq!(mine["total"], 1);

        // Bad members are refused; public again opens it to all.
        assert_eq!(
            request
                .put(&format!("/api/plans/{p}/access"))
                .add_header(ak.clone(), av.clone())
                .json(&json!({ "private": true, "members": ["alice"] }))
                .await
                .status_code(),
            422
        );
        request
            .put(&format!("/api/plans/{p}/access"))
            .add_header(ak, av)
            .json(&json!({ "private": false, "members": [] }))
            .await
            .assert_status_ok();
        assert_eq!(status(format!("/api/plans/{p}"), None).await, 200);
    })
    .await;
}
