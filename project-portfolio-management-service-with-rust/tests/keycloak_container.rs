//! A real Keycloak access token through the real blanket guard. Where
//! `tests/keycloak_guard.rs` stands a local OIDC provider in for Keycloak,
//! this starts an actual Keycloak in a container (Testcontainers), imports
//! `tests/fixtures/keycloak/mxi-realm.json` (realm `mxi`, public client
//! `mxi-api` with an audience mapper, users `edith`/`rhea`/`nora`), mints
//! tokens with the password grant, and checks authentication, the role map
//! and the ABAC decision.
//!
//! `#[ignore]`d: it needs a container runtime. This workspace uses Podman
//! (not Docker); point Testcontainers at its Docker-compatible socket:
//!
//! ```text
//! export DOCKER_HOST="unix://$(podman machine inspect --format '{{.ConnectionInfo.PodmanSocket.Path}}')"
//! export TESTCONTAINERS_RYUK_DISABLED=true
//! cargo test --features keycloak --test keycloak_container -- --ignored
//! ```
//!
//! The image name is fully qualified because Podman refuses short names.
//! One test function: the verifiers and policy live in process-wide
//! `OnceLock`s that read the environment on first use.
#![cfg(feature = "keycloak")]

use std::time::Duration;

use axum::http::header::AUTHORIZATION;
use axum::http::{HeaderMap, Method};
use project_portfolio_management_service::auth;
use serde_json::{Value, json};
use testcontainers::core::{IntoContainerPort, WaitFor};
use testcontainers::runners::AsyncRunner;
use testcontainers::{GenericImage, ImageExt};

const REALM_JSON: &str = include_str!("fixtures/keycloak/mxi-realm.json");

fn bearer(token: &str) -> HeaderMap {
    let mut h = HeaderMap::new();
    h.insert(AUTHORIZATION, format!("Bearer {token}").parse().unwrap());
    h
}

async fn status(method: &Method, path: &str, headers: &HeaderMap) -> u16 {
    match auth::enforce_request(true, method, path, headers, &auth::policy().current()).await {
        Ok(()) => 200,
        Err((code, _)) => code.as_u16(),
    }
}

/// Password-grant an access token for `user` from the imported realm.
async fn token(base: &str, user: &str, password: &str) -> String {
    let body: Value = reqwest::Client::new()
        .post(format!("{base}/realms/mxi/protocol/openid-connect/token"))
        .header("content-type", "application/x-www-form-urlencoded")
        // Fixture credentials are URL-safe, so no encoder is needed.
        .body(format!(
            "grant_type=password&client_id=mxi-api&username={user}&password={password}"
        ))
        .send()
        .await
        .expect("token request")
        .json()
        .await
        .expect("token response");
    body["access_token"]
        .as_str()
        .unwrap_or_else(|| panic!("no access_token: {body}"))
        .to_string()
}

#[tokio::test]
#[ignore = "requires a container runtime (Podman); see the module docs"]
async fn a_real_keycloak_token_is_authenticated_and_authorized_through_the_real_guard() {
    let dir = tempfile::tempdir().expect("tempdir");
    let realm_file = dir.path().join("mxi-realm.json");
    std::fs::write(&realm_file, REALM_JSON).expect("write realm");

    let container = GenericImage::new("quay.io/keycloak/keycloak", "26.0")
        .with_exposed_port(8080.tcp())
        .with_wait_for(WaitFor::message_on_stdout("Listening on:"))
        .with_cmd(["start-dev", "--import-realm"])
        .with_env_var("KC_BOOTSTRAP_ADMIN_USERNAME", "admin")
        .with_env_var("KC_BOOTSTRAP_ADMIN_PASSWORD", "admin")
        .with_mount(testcontainers::core::Mount::bind_mount(
            realm_file.to_string_lossy().to_string(),
            "/opt/keycloak/data/import/mxi-realm.json",
        ))
        .with_startup_timeout(Duration::from_secs(240))
        .start()
        .await
        .expect("keycloak container (is DOCKER_HOST set to the Podman socket?)");
    let port = container.get_host_port_ipv4(8080).await.expect("port");
    // Keycloak derives `iss` from the request host, so the token fetch and
    // the verifier must name the same origin.
    let base = format!("http://localhost:{port}");

    unsafe {
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_KEYCLOAK_URL", &base);
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_KEYCLOAK_REALM", "mxi");
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_KEYCLOAK_AUDIENCES", "mxi-api");
        std::env::set_var(
            "PROJECT_PORTFOLIO_MANAGEMENT_KEYCLOAK_ROLE_MAP",
            json!({ "editor": { "access": ["write"] }, "root": { "access": ["admin"] } })
                .to_string(),
        );
    }
    auth::init().await;
    assert!(auth::keycloak_verifier().is_some(), "keycloak configured");

    let list = "/api/plans";
    let merge = "/api/plans/merge";
    let editor = bearer(&token(&base, "edith", "edith-pw").await);
    let admin = bearer(&token(&base, "rhea", "rhea-pw").await);
    let nobody = bearer(&token(&base, "nora", "nora-pw").await);

    assert_eq!(status(&Method::GET, list, &HeaderMap::new()).await, 401);

    // `editor` → access=write: read and write, never delete or merge.
    assert_eq!(status(&Method::GET, list, &editor).await, 200);
    assert_eq!(status(&Method::POST, list, &editor).await, 200);
    assert_eq!(status(&Method::DELETE, list, &editor).await, 403);
    assert_eq!(status(&Method::POST, merge, &editor).await, 403);

    // `root` → access=admin: destructive allowed.
    assert_eq!(status(&Method::DELETE, list, &admin).await, 200);
    assert_eq!(status(&Method::POST, merge, &admin).await, 200);

    // No mapped role: authenticated, read-only.
    assert_eq!(status(&Method::GET, list, &nobody).await, 200);
    assert_eq!(status(&Method::POST, list, &nobody).await, 403);

    // A tampered token is 401, never 200/403.
    let mut forged = token(&base, "rhea", "rhea-pw").await;
    forged.push('x');
    assert_eq!(status(&Method::GET, list, &bearer(&forged)).await, 401);
}
