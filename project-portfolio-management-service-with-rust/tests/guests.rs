//! Guest access (PL-7), end to end over the real router with real PASETO
//! identities: a guest sees only what is granted, masked, and nothing else.
//!
//! Its **own test binary**, for the same reason `enforcement.rs` is:
//! `verifier` is a process-wide `OnceLock`, read once on first use —
//! sharing the `tests/requests/mod.rs` binary would race whichever
//! test happens to touch an authenticated route first.
//!
//! `#[ignore]`d — boots the app, so it needs PostgreSQL via
//! `config/test.yaml` / `DATABASE_URL`. Run with
//! `cargo test --test guests -- --ignored`.
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
const GUEST: &str = "3f2c6b7e-0000-4000-8000-0000000000e1";
const OTHER: &str = "3f2c6b7e-0000-4000-8000-0000000000e2";

/// PL-7 acceptance.
#[tokio::test]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test guests -- --ignored`"]
async fn a_guest_sees_only_what_is_granted_masked_and_nothing_else() {
    let (keys, kid) = keys_and_kid();
    // SAFETY: this binary is single-test, so no other test races the
    // process-wide verifier `OnceLock` it sets.
    unsafe {
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS", keys.to_string());
    }
    let (ak, av) = auth_header(&mint(&kid, ALICE, &[]));
    let (gk, gv) = auth_header(&mint(&kid, GUEST, &["guest"]));
    let (ok, ov) = auth_header(&mint(&kid, OTHER, &["guest"]));

    request::<App, _, _>(|request, ctx| async move {
        let plan = |name: &str| {
            let (request, ak, av, name) = (&request, ak.clone(), av.clone(), name.to_string());
            async move {
                let p: Value = request
                    .post("/api/plans")
                    .add_header(ak, av)
                    .json(&json!({ "kind": "Project", "name": name,
                                   "lead_ref": "person:3f2c6b7e-0000-4000-8000-000000000001" }))
                    .await
                    .json();
                p["pid"].as_str().expect("pid").to_string()
            }
        };
        let p = plan("Granted plan").await;
        let q = plan("Not granted").await;
        let task: Value = request
            .post(&format!("/api/plans/{p}/tasks"))
            .add_header(ak.clone(), av.clone())
            .json(&json!({ "title": "Internal task" }))
            .await
            .json();
        let t = task["pid"].as_str().unwrap().to_string();
        request
            .post(&format!("/api/plans/{p}/milestones"))
            .add_header(ak.clone(), av.clone())
            .json(&json!({ "name": "Beta", "due": "2026-10-20" }))
            .await
            .assert_status_ok();

        let get = |path: String, k: axum::http::HeaderName, v: axum::http::HeaderValue| {
            let request = &request;
            async move { request.get(&path).add_header(k, v).await }
        };
        let status = |path: String, k: axum::http::HeaderName, v: axum::http::HeaderValue| {
            let r = get(path, k, v);
            async move { r.await.status_code().as_u16() }
        };

        // Before any grant a guest sees nothing — and cannot enumerate.
        for path in [format!("/api/plans/{p}"), "/api/plans".to_string(), "/api/plans/search?q=plan".to_string(),
                     "/api/reports".to_string(), "/api/at-a-glance".to_string()] {
            assert_eq!(status(path.clone(), gk.clone(), gv.clone()).await, 404, "{path}");
        }
        // A non-guest is untouched by the guard.
        assert_eq!(status("/api/plans".to_string(), ak.clone(), av.clone()).await, 200);

        // Grant read.
        let granted: Value = request
            .post(&format!("/api/plans/{p}/guests"))
            .add_header(ak.clone(), av.clone())
            .json(&json!({ "guest_sub": GUEST, "access": "read" }))
            .await
            .json();
        let grant = granted["pid"].as_str().unwrap().to_string();
        let rec: Value = get(format!("/api/plans/{p}"), gk.clone(), gv.clone()).await.json();
        assert_eq!(rec["name"], "Granted plan");
        assert!(rec["lead_ref"].is_null(), "a guest never sees people references: {rec}");
        assert_eq!(status(format!("/api/plans/{p}/milestones"), gk.clone(), gv.clone()).await, 200);
        // Not granted, not safe, not allowed: all 404.
        for path in [format!("/api/plans/{q}"), format!("/api/plans/{p}/tasks"), format!("/api/plans/{p}/risks"),
                     "/api/plans".to_string()] {
            assert_eq!(status(path.clone(), gk.clone(), gv.clone()).await, 404, "{path}");
        }
        // A guest cannot write.
        assert_eq!(
            request.put(&format!("/api/plans/{p}")).add_header(gk.clone(), gv.clone())
                .json(&json!({ "name": "hijack" })).await.status_code(),
            404
        );
        assert_eq!(
            request.post("/api/plans").add_header(gk.clone(), gv.clone())
                .json(&json!({ "name": "new" })).await.status_code(),
            404
        );
        // Another guest has no grant.
        assert_eq!(status(format!("/api/plans/{p}"), ok.clone(), ov.clone()).await, 404);
        // Their own harmless endpoints work.
        assert_eq!(status("/api/me/tasks?tz=UTC".to_string(), gk.clone(), gv.clone()).await, 200);

        // Read access does not allow commenting; comment access does.
        let comment = |k: axum::http::HeaderName, v: axum::http::HeaderValue, body: &str| {
            let (request, body) = (&request, json!({ "subject_kind": "plan", "subject_pid": p, "body": body }));
            async move { request.post("/api/comments").add_header(k, v).json(&body).await.status_code().as_u16() }
        };
        assert_eq!(comment(gk.clone(), gv.clone(), "may I?").await, 403);
        request
            .post(&format!("/api/plans/{p}/guests"))
            .add_header(ak.clone(), av.clone())
            .json(&json!({ "guest_sub": GUEST, "access": "comment" }))
            .await
            .assert_status_ok();
        assert_eq!(comment(gk.clone(), gv.clone(), "thanks for the update").await, 200);
        let thread: Value = get(format!("/api/comments?subject_kind=plan&subject_pid={p}"), gk.clone(), gv.clone()).await.json();
        assert_eq!(thread.as_array().unwrap().len(), 1);
        // Comments on a subject outside the grant are 404, and so are tasks'.
        assert_eq!(status(format!("/api/comments?subject_kind=plan&subject_pid={q}"), gk.clone(), gv.clone()).await, 404);
        assert_eq!(
            request.post("/api/comments").add_header(gk.clone(), gv.clone())
                .json(&json!({ "subject_kind": "plan", "subject_pid": q, "body": "x" })).await.status_code(),
            404
        );
        let _ = t;

        // Revocation is immediate.
        let list: Value = get(format!("/api/plans/{p}/guests"), ak.clone(), av.clone()).await.json();
        assert_eq!(list.as_array().unwrap().len(), 2);
        for g in list.as_array().unwrap() {
            let id = g["pid"].as_str().unwrap();
            request
                .post(&format!("/api/plans/{p}/guests/{id}/revoke"))
                .add_header(ak.clone(), av.clone())
                .await
                .assert_status_ok();
        }
        assert_eq!(status(format!("/api/plans/{p}"), gk.clone(), gv.clone()).await, 404, "revoked: the very next request");
        let _ = grant;

        // Expiry is immediate too: a grant whose time has passed stops working.
        let fresh: Value = request
            .post(&format!("/api/plans/{p}/guests"))
            .add_header(ak.clone(), av.clone())
            .json(&json!({ "guest_sub": GUEST, "access": "read", "expires_at": (chrono::Utc::now() + chrono::Duration::hours(1)).to_rfc3339() }))
            .await
            .json();
        assert_eq!(status(format!("/api/plans/{p}"), gk.clone(), gv.clone()).await, 200);
        {
            use project_portfolio_management_service::models::_entities::guest_grants;
            use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter};
            let row = guest_grants::Entity::find()
                .filter(guest_grants::Column::Pid.eq(uuid::Uuid::parse_str(fresh["pid"].as_str().unwrap()).unwrap()))
                .one(&ctx.db).await.unwrap().unwrap();
            let mut a: guest_grants::ActiveModel = row.into();
            a.expires_at = ActiveValue::set(Some((chrono::Utc::now() - chrono::Duration::minutes(1)).into()));
            a.update(&ctx.db).await.unwrap();
        }
        assert_eq!(status(format!("/api/plans/{p}"), gk.clone(), gv.clone()).await, 404, "expired");

        // Bad grants are refused.
        for body in [json!({ "guest_sub": GUEST, "access": "admin" }),
                     json!({ "guest_sub": GUEST, "access": "read", "expires_at": "2020-01-01T00:00:00Z" })] {
            assert_eq!(
                request.post(&format!("/api/plans/{p}/guests")).add_header(ak.clone(), av.clone()).json(&body).await.status_code(),
                422
            );
        }
    })
    .await;
}
