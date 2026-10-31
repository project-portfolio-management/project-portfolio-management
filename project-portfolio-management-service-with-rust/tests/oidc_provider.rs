//! Any OIDC provider (WW-1.12), end to end: a stand-in provider on loopback publishes discovery and a
//! key set, the service is configured with nothing but its issuer, audience and role map, and its ES256 tokens
//! are accepted, mapped, and refused when they should be.
//!
//! Its **own test binary**: the verifier is a process-wide `OnceLock` set at boot from the environment.
//! `#[ignore]`d — boots the app, so it needs PostgreSQL via `config/test.yaml` / `DATABASE_URL`. Run with
//! `cargo test --test oidc_provider -- --ignored`.

use axum::{Json, Router, routing::get};
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use serde_json::{Value, json};

/// A P-256 key made for this test only (the same as the unit tests' in `src/auth/oidc.rs`).
const TEST_KEY: &str = "-----BEGIN PRIVATE KEY-----
MIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQgm6upgKprEdospMx5
FUMtF2vxdlsV3cFGfnn+fNvCoP2hRANCAARifB4LKgA87OiIXxmUbZhJz7YuCcCm
Gc/gtIhTyl+CZJRNvIA6LLb3PfKHFbXDtr3foVOqWsbuncgcMXm+/vd7
-----END PRIVATE KEY-----
";

fn sign(claims: &Value) -> String {
    let mut header = Header::new(Algorithm::ES256);
    header.kid = Some("k1".to_string());
    jsonwebtoken::encode(
        &header,
        claims,
        &EncodingKey::from_ec_pem(TEST_KEY.as_bytes()).unwrap(),
    )
    .unwrap()
}

fn auth(token: &str) -> (axum::http::HeaderName, axum::http::HeaderValue) {
    (
        axum::http::header::AUTHORIZATION,
        format!("Bearer {token}").parse().unwrap(),
    )
}

#[tokio::test]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test oidc_provider -- --ignored`"]
async fn a_token_from_a_plain_oidc_provider_is_accepted_and_mapped() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let issuer = format!("http://127.0.0.1:{}", listener.local_addr().unwrap().port());
    let discovery = json!({ "issuer": issuer, "jwks_uri": format!("{issuer}/jwks") });
    let jwks = json!({ "keys": [{
        "kty": "EC", "crv": "P-256", "kid": "k1", "use": "sig", "alg": "ES256",
        "x": "YnweCyoAPOzoiF8ZlG2YSc-2LgnAphnP4LSIU8pfgmQ",
        "y": "lE28gDostvc98ocVtcO2vd-hU6paxu6dyBwxeb7-93s",
    }]});
    let provider = Router::new()
        .route(
            "/.well-known/openid-configuration",
            get(move || async move { Json(discovery) }),
        )
        .route("/jwks", get(move || async move { Json(jwks) }));
    tokio::spawn(async move { axum::serve(listener, provider).await.unwrap() });

    // SAFETY: this binary has one test, so nothing races the process-wide settings.
    unsafe {
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH", "1");
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_OIDC_ISSUER", &issuer);
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_OIDC_AUDIENCES", "ppm");
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_OIDC_ROLES_CLAIM", "groups");
        std::env::set_var(
            "PROJECT_PORTFOLIO_MANAGEMENT_OIDC_ROLE_MAP",
            r#"{"planners": {"access": ["write"]}, "pm-admins": {"access": ["admin"]}}"#,
        );
    }
    let now = chrono::Utc::now().timestamp();
    let claims = |sub: &str, groups: &[&str], aud: &str| json!({ "iss": issuer, "aud": aud, "sub": sub, "exp": now + 600, "iat": now, "name": sub, "groups": groups });
    let writer = auth(&sign(&claims("ana", &["planners"], "ppm")));
    let admin = auth(&sign(&claims("ola", &["pm-admins"], "ppm")));
    let wrong_audience = auth(&sign(&claims("ana", &["planners"], "elsewhere")));
    let mut expired = claims("ana", &["planners"], "ppm");
    expired["exp"] = json!(now - 3600);
    let expired = auth(&sign(&expired));

    request::<App, _, _>(|request, _ctx| async move {
        macro_rules! with {
            ($rb:expr, $who:expr) => {
                $rb.add_header($who.0.clone(), $who.1.clone())
            };
        }
        assert_eq!(
            request.get("/api/plans").await.status_code(),
            401,
            "no token"
        );
        assert_eq!(
            with!(request.get("/api/plans"), writer).await.status_code(),
            200
        );
        let made = with!(request.post("/api/plans"), writer)
            .json(&json!({ "kind": "Project", "name": "Federated" }))
            .await;
        assert_eq!(
            made.status_code(),
            200,
            "a mapped role writes: {}",
            made.text()
        );
        assert_eq!(
            with!(request.get("/api/access-review"), writer)
                .await
                .status_code(),
            403
        );
        assert_eq!(
            with!(request.get("/api/access-review"), admin)
                .await
                .status_code(),
            200
        );
        assert_eq!(
            with!(request.get("/api/plans"), wrong_audience)
                .await
                .status_code(),
            401
        );
        assert_eq!(
            with!(request.get("/api/plans"), expired)
                .await
                .status_code(),
            401
        );
    })
    .await;
}
