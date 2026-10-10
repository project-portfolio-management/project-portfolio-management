//! Sign in with Microsoft Entra ID, brokered by a real Keycloak, through the real guard
//! (HD-12, `operations/sign-in-with-entra.md`).
//!
//! The chain, all real except the Entra stand-in:
//!
//! ```text
//! browser-like client --> Keycloak (realm mxi-entra) --> mock Entra (v2.0 shapes) --> Keycloak
//!        --> authorization code --> Keycloak token endpoint --> access token --> this crate's guard
//! ```
//!
//! The mock provider follows Entra's v2.0 endpoints and token shape: a tenant-scoped issuer
//! (`https://login.microsoftonline.com/<tenant>/v2.0`), RS256 ID tokens with `tid`, `oid`,
//! `roles`, `preferred_username`, `nonce`, an exact redirect URI, PKCE at the token endpoint,
//! single-use codes. Keycloak is configured exactly as the operations guide says
//! (`tests/fixtures/keycloak/entra-broker-realm.json`): an OIDC identity provider with
//! `trustEmail`, signature validation against the provider's JWKS, and role mappers that turn
//! Entra app roles into realm roles. The access token Keycloak then issues is verified by the
//! service's Keycloak verifier and mapped to ABAC by the explicit role map.
//!
//! `#[ignore]`d: it needs a container runtime (Podman via its Docker-compatible socket):
//!
//! ```text
//! export DOCKER_HOST="unix://$(podman machine inspect --format '{{.ConnectionInfo.PodmanSocket.Path}}')"
//! export TESTCONTAINERS_RYUK_DISABLED=true
//! cargo test --features keycloak --test keycloak_entra_broker -- --ignored
//! ```
//!
//! The container reaches the mock on the host as `host.containers.internal` (Podman); the
//! browser-facing authorization address uses `localhost`. One test function: the verifier
//! and policy are process-wide `OnceLock`s that read the environment on first use.
#![cfg(feature = "keycloak")]

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use axum::extract::{Form, Path, Query, State};
use axum::http::header::{AUTHORIZATION, LOCATION};
use axum::http::{HeaderMap, Method, StatusCode};
use axum::response::{IntoResponse, Redirect};
use axum::routing::{get, post};
use axum::{Json, Router};
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use project_portfolio_management_service::auth;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use testcontainers::core::{IntoContainerPort, WaitFor};
use testcontainers::runners::AsyncRunner;
use testcontainers::{GenericImage, ImageExt};

const REALM_JSON: &str = include_str!("fixtures/keycloak/entra-broker-realm.json");
const TENANT: &str = "11111111-2222-3333-4444-555555555555";
const CLIENT_ID: &str = "mock-client-id";
const CLIENT_SECRET: &str = "mock-secret";
/// What Entra v2.0 puts in `iss` for a tenant.
const ENTRA_ISSUER: &str =
    "https://login.microsoftonline.com/11111111-2222-3333-4444-555555555555/v2.0";

/// A throwaway RSA key for the mock provider. It protects nothing.
const MOCK_KEY_PEM: &str = r"-----BEGIN PRIVATE KEY-----
MIIEvgIBADANBgkqhkiG9w0BAQEFAASCBKgwggSkAgEAAoIBAQC22QJLBuz98hk8
627CFGFiQ5sJtYW0FSTEOJk6dCxI8VcvFGZo0XaY7TsATVbGD33PfWrU7X5xDykn
Rnty2Y4TQ+haKvUv3uBD3sP7Z/CujZ/pSoZX5rXKdhR47XVoO/hJc+Ivdf+p9KrO
hcWGxK0aN9hUpuT1PVB96m8MBVIIE5OsVyax2F/8v9LeipOHOA99gNybXMUgzoxY
/8fMFS+QOcBW9foQG5UfawJ11d0Lz4K6ZsLWYd6V7tqZod/FBLSedT2ruIDgchSb
LWe5mYORPFnJEo/k4bb7C1tk+MLsI8drnElRMII4XyABoZkgfaS2vf2o+VyB9Om0
WR2Gqw+xAgMBAAECggEAJRJvoBWZ5Ez7J/pV/PyT2A/79aZ7oszspOoSyyK1vd5M
vmtk7hiV8qCap0+G6eZHLFPx3pT+X17jCacFLX6Fi0tn5ywxhRCrqKcYVjc3ObTW
OL8W6OiKcC+rq+79MHKEPyRDNjs26FcGcJP3vgDPVfL/pf4misBxOu5WsHm9rYwt
z/81KTwhNKrJRBQLD4u9kTto+3G0p90o/kSQNvCkUIAcbyem7Aonm4OMd9Vt6xfo
Iit9obGeyMx/NJ0Hix3adibJfNSweeO2O2ce+zi0uTS0wrp7VSDUhsdB/1H5Krft
44omTCxrlzR/Fbjjv8Brw9LdjadzK5CE7xHPvuSv7wKBgQD8JyW15FnwXMzvH/+p
R2gte0rK8puUyVfBy7oYVdCznCg962w8f3ElIEcg192AjWD2M13F5TGVga+ot2Ts
rAiOX0QUy4dzU1PlMZbp2y0G8ikIpfesmchXgQZDvNFaR5UaBDwaEqRC0uUjglb9
cJLPLPH74Zy5VXHJV4tlPlWPbwKBgQC5oyvF+F2C3bTzMsx5r0Lo4KjVcgZ6V4Z+
9lH6Q1cK1DkGl0VOLHhkAArsGaRdhjCaiFVLt9trDAi1uuRU4WncA5PZJlsV/3qG
oOJ/yfMmL2ZunZ+6MkV9WMows5Ha2+NZsVGaaA/3O3l15i7nZB7mmnqcCI8IFxC0
TmDKyx/C3wKBgE2OxPyjlj5mbOqITx0ER7jlmWW/JbeoDoQfY8gkbSMxvvb8uLgL
AwRGX1psy0bFNbPErO03x/6SezNL5ZEegSs5hjlXOPxx/vYaQK8BqWHfjluIoRvH
2WWA3Cpto38/UU9FSi9C2J9tOumnJ5agz6dFzdCJ3qjq7Sre8JNdjvelAoGBAKA7
b3UZw91JcP7aQhRmauKHyDM0kNLY6YZ9JpVVurPmKrK9r1yilef7AXJ5T55KTXmV
Y8pPkFz9ribuglehmbYGgH9W1BhQxHmgNOEId4/mKgwgkdYtvj1Lfno7BRJb6glp
PH6ChAVqmJRjcujXSrTRcjNsT8kz339Ch9mDKo8RAoGBAKg16Qz4KhfVv3wvxqXb
Mhc5X1pz1hx/b9w9pdWET3r7HgNGusSLS4iHdSgDljXvX9OVB5THxXFBib6wvTHb
CHarFVcS0TRflbYiVu9T8ZWmz+pgbkYVK2xmHE9TaMrDT5HEPHX03hAKNNfECEzi
YGbTVi3l7hD6rjL4msVxGUpF
-----END PRIVATE KEY-----";
/// Its public modulus, base64url.
const MOCK_KEY_N: &str = "ttkCSwbs_fIZPOtuwhRhYkObCbWFtBUkxDiZOnQsSPFXLxRmaNF2mO07AE1Wxg99z31q1O1-cQ8pJ0Z7ctmOE0PoWir1L97gQ97D-2fwro2f6UqGV-a1ynYUeO11aDv4SXPiL3X_qfSqzoXFhsStGjfYVKbk9T1QfepvDAVSCBOTrFcmsdhf_L_S3oqThzgPfYDcm1zFIM6MWP_HzBUvkDnAVvX6EBuVH2sCddXdC8-CumbC1mHele7amaHfxQS0nnU9q7iA4HIUmy1nuZmDkTxZyRKP5OG2-wtbZPjC7CPHa5xJUTCCOF8gAaGZIH2ktr39qPlcgfTptFkdhqsPsQ";

#[derive(Clone)]
struct Mock {
    state: Arc<Mutex<MockState>>,
}

#[derive(Default)]
struct MockState {
    /// Who the next authorization signs in as: `alice` (Portfolio.Editor) or `bob` (no roles).
    user: String,
    codes: HashMap<String, Pending>,
    pkce_checked: usize,
}

struct Pending {
    nonce: String,
    challenge: String,
    user: String,
}

fn jwks() -> Value {
    json!({ "keys": [{ "kty": "RSA", "use": "sig", "alg": "RS256", "kid": "entra-mock-1", "n": MOCK_KEY_N, "e": "AQAB" }] })
}

async fn mock_keys(Path(_tenant): Path<String>) -> Json<Value> {
    Json(jwks())
}

async fn mock_authorize(
    State(m): State<Mock>,
    Path(_tenant): Path<String>,
    Query(q): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let bad = |why: &str| (StatusCode::BAD_REQUEST, why.to_string()).into_response();
    if q.get("client_id").map(String::as_str) != Some(CLIENT_ID) {
        return bad("AADSTS700016: unknown application");
    }
    if q.get("response_type").map(String::as_str) != Some("code")
        || q.get("code_challenge_method").map(String::as_str) != Some("S256")
    {
        return bad("PKCE (S256) with the code flow is required");
    }
    let (Some(redirect), Some(state), Some(challenge)) = (
        q.get("redirect_uri"),
        q.get("state"),
        q.get("code_challenge"),
    ) else {
        return bad("missing parameter");
    };
    let mut s = m.state.lock().unwrap();
    let code = format!("code-{}", s.codes.len() + 1);
    let user = s.user.clone();
    s.codes.insert(
        code.clone(),
        Pending {
            nonce: q.get("nonce").cloned().unwrap_or_default(),
            challenge: challenge.clone(),
            user,
        },
    );
    Redirect::to(&format!("{redirect}?code={code}&state={state}")).into_response()
}

async fn mock_token(
    State(m): State<Mock>,
    Path(_tenant): Path<String>,
    Form(f): Form<HashMap<String, String>>,
) -> impl IntoResponse {
    let err = |status: StatusCode, e: &str| (status, Json(json!({ "error": e }))).into_response();
    if f.get("client_secret").map(String::as_str) != Some(CLIENT_SECRET) {
        return err(StatusCode::UNAUTHORIZED, "invalid_client");
    }
    let mut s = m.state.lock().unwrap();
    // Single use, like a real provider.
    let Some(p) = f.get("code").and_then(|c| s.codes.remove(c)) else {
        return err(StatusCode::BAD_REQUEST, "invalid_grant");
    };
    let verifier = f.get("code_verifier").cloned().unwrap_or_default();
    let challenge = URL_SAFE_NO_PAD.encode(Sha256::digest(verifier.as_bytes()));
    if verifier.is_empty() || challenge != p.challenge {
        return err(StatusCode::BAD_REQUEST, "invalid_grant");
    }
    s.pkce_checked += 1;
    let (name, given, family, roles): (&str, &str, &str, Vec<&str>) = match p.user.as_str() {
        "alice" => (
            "Alice Example",
            "Alice",
            "Example",
            vec!["Portfolio.Editor"],
        ),
        _ => ("Bob Example", "Bob", "Example", vec![]),
    };
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let claims = json!({
        "ver": "2.0", "iss": ENTRA_ISSUER, "aud": CLIENT_ID,
        "sub": format!("entra-sub-{}", p.user), "oid": format!("oid-{}", p.user), "tid": TENANT,
        "name": name, "given_name": given, "family_name": family,
        "preferred_username": format!("{}@contoso.example", p.user),
        "email": format!("{}@contoso.example", p.user),
        "roles": roles, "nonce": p.nonce,
        "iat": now - 5, "nbf": now - 5, "exp": now + 3600,
    });
    let mut header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::RS256);
    header.kid = Some("entra-mock-1".into());
    let id_token = jsonwebtoken::encode(
        &header,
        &claims,
        &jsonwebtoken::EncodingKey::from_rsa_pem(MOCK_KEY_PEM.as_bytes()).unwrap(),
    )
    .unwrap();
    Json(json!({
        "token_type": "Bearer", "expires_in": 3599, "scope": "openid profile email",
        "access_token": "opaque-entra-access-token", "id_token": id_token,
    }))
    .into_response()
}

/// Start the mock on all interfaces (the Keycloak container reaches it from outside).
async fn start_mock() -> (u16, Mock) {
    let mock = Mock {
        state: Arc::new(Mutex::new(MockState::default())),
    };
    let app = Router::new()
        .route("/{tenant}/discovery/v2.0/keys", get(mock_keys))
        .route("/{tenant}/oauth2/v2.0/authorize", get(mock_authorize))
        .route("/{tenant}/oauth2/v2.0/token", post(mock_token))
        .with_state(mock.clone());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:0")
        .await
        .expect("bind");
    let port = listener.local_addr().unwrap().port();
    tokio::spawn(async move {
        axum::serve(listener, app).await.ok();
    });
    (port, mock)
}

/// A tiny cookie jar (reqwest's `cookies` feature is not enabled in this crate).
#[derive(Default)]
struct Jar(Vec<(String, String)>);

impl Jar {
    fn store(&mut self, headers: &reqwest::header::HeaderMap) {
        for v in headers.get_all(reqwest::header::SET_COOKIE) {
            let Some(pair) = v.to_str().ok().and_then(|s| s.split(';').next()) else {
                continue;
            };
            let Some((k, val)) = pair.split_once('=') else {
                continue;
            };
            self.0.retain(|(name, _)| name != k);
            if !val.is_empty() {
                self.0.push((k.to_string(), val.to_string()));
            }
        }
    }
    fn header(&self) -> String {
        self.0
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join("; ")
    }
}

/// Sign in as `user` the way a browser would: follow every redirect from Keycloak to the
/// provider and back, carrying Keycloak's cookies, until Keycloak redirects to the client.
/// Returns the Keycloak access token.
async fn sign_in(kc: &str, mock: &Mock, user: &str) -> String {
    mock.state.lock().unwrap().user = user.to_string();
    let http = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();
    let verifier = format!("verifier-{user}-0123456789-0123456789-0123456789-0123456789");
    let challenge = URL_SAFE_NO_PAD.encode(Sha256::digest(verifier.as_bytes()));
    let redirect_uri = "http://localhost/callback";
    let mut url = format!(
        "{kc}/realms/mxi-entra/protocol/openid-connect/auth?client_id=mxi-api&response_type=code\
         &scope=openid&redirect_uri={redirect_uri}&state=client-state&nonce=client-nonce\
         &code_challenge={challenge}&code_challenge_method=S256&kc_idp_hint=entra"
    );
    let mut jar = Jar::default();
    for _ in 0..20 {
        let mut req = http.get(&url);
        if url.starts_with(kc) && !jar.0.is_empty() {
            req = req.header(reqwest::header::COOKIE, jar.header());
        }
        let resp = req.send().await.expect("redirect hop");
        if url.starts_with(kc) {
            jar.store(resp.headers());
        }
        let status = resp.status();
        let location = resp
            .headers()
            .get(LOCATION)
            .and_then(|v| v.to_str().ok())
            .map(str::to_string);
        let Some(loc) = location else {
            let body = resp.text().await.unwrap_or_default();
            panic!(
                "sign-in stopped at {url} with {status} and no redirect; body starts: {}",
                &body[..body.len().min(400)]
            );
        };
        let next = if loc.starts_with('/') {
            let origin = url.split('/').take(3).collect::<Vec<_>>().join("/");
            format!("{origin}{loc}")
        } else {
            loc
        };
        if next.starts_with(redirect_uri) {
            assert!(
                next.contains("state=client-state"),
                "the client's state comes back: {next}"
            );
            let code = next
                .split(['?', '&'])
                .find_map(|p| p.strip_prefix("code="))
                .unwrap_or_else(|| panic!("no code in {next}"))
                .to_string();
            // The values are URL-safe except the redirect URI, which is encoded by hand
            // (reqwest's `form` feature is not enabled in this crate).
            let tokens: Value = http
                .post(format!("{kc}/realms/mxi-entra/protocol/openid-connect/token"))
                .header("content-type", "application/x-www-form-urlencoded")
                .body(format!(
                    "grant_type=authorization_code&code={code}&redirect_uri=http%3A%2F%2Flocalhost%2Fcallback\
                     &client_id=mxi-api&code_verifier={verifier}"
                ))
                .send()
                .await
                .expect("token request")
                .json()
                .await
                .expect("token response");
            return tokens["access_token"]
                .as_str()
                .unwrap_or_else(|| panic!("no access_token: {tokens}"))
                .to_string();
        }
        url = next;
    }
    panic!("sign-in did not finish within 20 redirects; last url {url}");
}

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

#[tokio::test]
#[ignore = "requires a container runtime (Podman); see the module docs"]
async fn an_entra_sign_in_brokered_by_a_real_keycloak_passes_the_real_guard() {
    let (port, mock) = start_mock().await;
    let browser_facing = format!("http://localhost:{port}/{TENANT}");
    let from_container = format!("http://host.containers.internal:{port}/{TENANT}");
    let realm = REALM_JSON
        .replace(
            "__IDP_AUTHORIZE__",
            &format!("{browser_facing}/oauth2/v2.0/authorize"),
        )
        .replace(
            "__IDP_TOKEN__",
            &format!("{from_container}/oauth2/v2.0/token"),
        )
        .replace(
            "__IDP_JWKS__",
            &format!("{from_container}/discovery/v2.0/keys"),
        )
        .replace("__IDP_ISSUER__", ENTRA_ISSUER);
    let dir = tempfile::tempdir().expect("tempdir");
    let realm_file = dir.path().join("entra-broker-realm.json");
    std::fs::write(&realm_file, realm).expect("write realm");

    let container = GenericImage::new("quay.io/keycloak/keycloak", "26.0")
        .with_exposed_port(8080.tcp())
        .with_wait_for(WaitFor::message_on_stdout("Listening on:"))
        .with_cmd(["start-dev", "--import-realm"])
        .with_env_var("KC_BOOTSTRAP_ADMIN_USERNAME", "admin")
        .with_env_var("KC_BOOTSTRAP_ADMIN_PASSWORD", "admin")
        .with_mount(testcontainers::core::Mount::bind_mount(
            realm_file.to_string_lossy().to_string(),
            "/opt/keycloak/data/import/entra-broker-realm.json",
        ))
        .with_startup_timeout(Duration::from_secs(240))
        .start()
        .await
        .expect("keycloak container (is DOCKER_HOST set to the Podman socket?)");
    let kc_port = container.get_host_port_ipv4(8080).await.expect("port");
    // Keycloak derives `iss` from the request host: the sign-in, the token fetch and the
    // verifier must all name this origin.
    let kc = format!("http://localhost:{kc_port}");

    unsafe {
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_KEYCLOAK_URL", &kc);
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_KEYCLOAK_REALM", "mxi-entra");
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
    assert_eq!(
        status(&Method::GET, list, &HeaderMap::new()).await,
        401,
        "no token, no entry"
    );

    // Alice signs in with Entra. Her Entra app role `Portfolio.Editor` becomes the realm role
    // `editor` (identity-provider mapper), which the explicit role map turns into access=write.
    let alice = bearer(&sign_in(&kc, &mock, "alice").await);
    assert_eq!(
        status(&Method::GET, list, &alice).await,
        200,
        "authenticated and may read"
    );
    assert_eq!(
        status(&Method::POST, list, &alice).await,
        200,
        "editor may write"
    );
    assert_eq!(
        status(&Method::DELETE, list, &alice).await,
        403,
        "editor may not delete"
    );

    // Bob signs in with Entra but holds no mapped role: authenticated, read-only. Entra proving who
    // he is grants him nothing by itself.
    let bob = bearer(&sign_in(&kc, &mock, "bob").await);
    assert_eq!(status(&Method::GET, list, &bob).await, 200);
    assert_eq!(status(&Method::POST, list, &bob).await, 403);

    // The provider was held to PKCE on both legs.
    assert!(
        mock.state.lock().unwrap().pkce_checked >= 2,
        "Keycloak used PKCE with the provider"
    );

    // A tampered token is refused.
    let mut forged = sign_in(&kc, &mock, "alice").await;
    forged.push('x');
    assert_eq!(status(&Method::GET, list, &bearer(&forged)).await, 401);
}
