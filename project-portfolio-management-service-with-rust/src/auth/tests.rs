use super::*;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use ed25519_dalek::SigningKey;
use rusty_paseto::core::{Footer, Key, Paseto, PasetoAsymmetricPrivateKey, Payload, Public, V4};
use sha2::{Digest, Sha256};

/// Issuer the test tokens and verifier agree on.
const ISSUER: &str = "authentication-service";
/// Audience the test tokens and verifier agree on.
const AUDIENCE: &str = "main-x-service";
/// A throwaway Ed25519 seed, used only to mint test tokens and a
/// matching key set in-process. Not a secret — never used in production.
const SEED: [u8; 32] = [
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
];

/// Build a key set + matching `kid` from the test public key, the same
/// way the auth service publishes it.
fn test_keys_and_kid() -> (serde_json::Value, String) {
    let public = SigningKey::from_bytes(&SEED).verifying_key().to_bytes();
    let kid = URL_SAFE_NO_PAD.encode(Sha256::digest(public));
    let keys = serde_json::json!({
        "keys": [{
            "kty": "OKP", "crv": "Ed25519", "use": "sig",
            "kid": kid, "x": URL_SAFE_NO_PAD.encode(public),
        }]
    });
    (keys, kid)
}

/// Mint a signed PASETO `v4.public` token for the test identity with
/// `kid` in the footer and `exp` set `exp_offset_secs` from a fixed
/// `iat` (negative offsets produce an already-expired token).
fn sign(kid: &str, exp_offset_secs: i64) -> String {
    sign_with_attrs(kid, exp_offset_secs, &[])
}

/// Like [`sign`], with the given ABAC subject attributes minted into
/// the token's `attrs` claim (e.g. `&[("access", &["write"])]`).
fn sign_with_attrs(kid: &str, exp_offset_secs: i64, attrs: &[(&str, &[&str])]) -> String {
    let iat: i64 = 1_700_000_000;
    let claims = Claims {
        sub: "11111111-1111-1111-1111-111111111111".into(),
        email: "alice@example.com".into(),
        name: "Alice".into(),
        iss: ISSUER.into(),
        aud: AUDIENCE.into(),
        exp: iat + exp_offset_secs,
        iat,
        nbf: None,
        sid: "test-sid".into(),
        scope: Vec::new(),
        roles: Vec::new(),
        attrs: attrs
            .iter()
            .map(|(key, values)| {
                (
                    (*key).to_string(),
                    values.iter().map(ToString::to_string).collect(),
                )
            })
            .collect(),
    };
    let keypair = SigningKey::from_bytes(&SEED).to_keypair_bytes();
    let key = Key::<64>::from(keypair);
    let private = PasetoAsymmetricPrivateKey::<V4, Public>::from(&key);
    let payload = serde_json::to_string(&claims).expect("serialize claims");
    let footer = format!(r#"{{"kid":"{kid}"}}"#);
    let mut builder = Paseto::<V4, Public>::builder();
    builder.set_payload(Payload::from(payload.as_str()));
    builder.set_footer(Footer::from(footer.as_str()));
    builder.try_sign(&private).expect("sign")
}

/// Wrap a token in a `HeaderMap` with `Authorization: Bearer <token>`.
fn bearer(token: &str) -> HeaderMap {
    let mut h = HeaderMap::new();
    h.insert(AUTHORIZATION, format!("Bearer {token}").parse().unwrap());
    h
}

/// A well-formed, in-date, correctly-signed token verifies and yields
/// the expected claims.
#[test]
fn valid_token_yields_claims() {
    let (keys, kid) = test_keys_and_kid();
    let verifier = Verifier::from_paseto_keys_value(&keys, ISSUER, AUDIENCE).unwrap();
    let token = sign(&kid, 10_000_000_000);
    let claims = bearer_claims(&bearer(&token), &verifier).expect("valid token verifies");
    assert_eq!(claims.sub, "11111111-1111-1111-1111-111111111111");
    assert_eq!(claims.email, "alice@example.com");
}

/// No `Authorization` header ⇒ `401`.
#[test]
fn missing_header_is_401() {
    let (keys, _) = test_keys_and_kid();
    let verifier = Verifier::from_paseto_keys_value(&keys, ISSUER, AUDIENCE).unwrap();
    let err = bearer_claims(&HeaderMap::new(), &verifier).unwrap_err();
    assert_eq!(err.0, StatusCode::UNAUTHORIZED);
}

/// A non-bearer scheme (e.g. `Basic`) ⇒ `401`.
#[test]
fn non_bearer_header_is_401() {
    let (keys, _) = test_keys_and_kid();
    let verifier = Verifier::from_paseto_keys_value(&keys, ISSUER, AUDIENCE).unwrap();
    let mut h = HeaderMap::new();
    h.insert(AUTHORIZATION, "Basic abc123".parse().unwrap());
    assert_eq!(
        bearer_claims(&h, &verifier).unwrap_err().0,
        StatusCode::UNAUTHORIZED
    );
}

/// A token whose `exp` is in the past ⇒ `401`.
#[test]
fn expired_token_is_401() {
    let (keys, kid) = test_keys_and_kid();
    let verifier = Verifier::from_paseto_keys_value(&keys, ISSUER, AUDIENCE).unwrap();
    let token = sign(&kid, -60);
    assert_eq!(
        bearer_claims(&bearer(&token), &verifier).unwrap_err().0,
        StatusCode::UNAUTHORIZED
    );
}

/// Flipping a token character breaks PASETO verification ⇒ `401`.
#[test]
fn tampered_token_is_401() {
    let (keys, kid) = test_keys_and_kid();
    let verifier = Verifier::from_paseto_keys_value(&keys, ISSUER, AUDIENCE).unwrap();
    let mut token = sign(&kid, 10_000_000_000);
    let last = token.pop().unwrap();
    token.push(if last == 'a' { 'b' } else { 'a' });
    assert_eq!(
        bearer_claims(&bearer(&token), &verifier).unwrap_err().0,
        StatusCode::UNAUTHORIZED
    );
}

/// A no-key verifier (the boot fallback) rejects even a valid token.
#[test]
fn empty_verifier_rejects_everything() {
    let verifier = empty_verifier(ISSUER, AUDIENCE);
    let (_, kid) = test_keys_and_kid();
    let token = sign(&kid, 10_000_000_000);
    assert_eq!(
        bearer_claims(&bearer(&token), &verifier).unwrap_err().0,
        StatusCode::UNAUTHORIZED
    );
}

/// `parse_bool` accepts the documented truthy set and rejects the
/// rest (including empty, `0`, and junk).
#[test]
fn parse_bool_truthy_and_falsy() {
    for t in ["1", "true", "TRUE", "Yes", "on", " on ", "ON"] {
        assert!(parse_bool(t), "{t:?} should parse true");
    }
    for f in ["", " ", "0", "false", "no", "off", "junk", "2"] {
        assert!(!parse_bool(f), "{f:?} should parse false");
    }
}

/// The default policy the enforcement tests share (what an
/// unconfigured service uses).
fn policy() -> Policy {
    Policy::default_policy()
}

/// Enforcement off ⇒ a protected path passes with no token — for
/// reads and mutations alike (no authn and no authz when the flag
/// is off; behaviour-neutral).
#[test]
fn enforce_off_allows_without_token() {
    let (keys, _) = test_keys_and_kid();
    let verifier = Verifier::from_paseto_keys_value(&keys, ISSUER, AUDIENCE).unwrap();
    let policy = policy();
    for method in [Method::GET, Method::POST, Method::DELETE] {
        assert!(
            enforce(
                false,
                &method,
                "/api/projects",
                &HeaderMap::new(),
                &verifier,
                &policy
            )
            .is_ok(),
            "{method} should pass with enforcement off"
        );
    }
}

/// Enforcement on ⇒ the public paths (health/ping, `OpenAPI`,
/// Swagger UI, calendar feeds) still pass without a token.
#[test]
fn enforce_on_allows_public_paths() {
    let (keys, _) = test_keys_and_kid();
    let verifier = Verifier::from_paseto_keys_value(&keys, ISSUER, AUDIENCE).unwrap();
    let policy = policy();
    for path in [
        "/_health",
        "/_ping",
        "/_readiness",
        "/api-docs/openapi.json",
        "/swagger-ui",
        "/swagger-ui/index.html",
        "/api/feeds/calendar.ics",
        "/api/feeds/me.atom",
    ] {
        assert!(
            enforce(
                true,
                &Method::GET,
                path,
                &HeaderMap::new(),
                &verifier,
                &policy
            )
            .is_ok(),
            "{path} should be public"
        );
    }
}

/// The Prometheus endpoint is behind login: with enforcement on and no
/// token it is `401`, like any other route.
#[test]
fn metrics_requires_a_token() {
    let (keys, _) = test_keys_and_kid();
    let verifier = Verifier::from_paseto_keys_value(&keys, ISSUER, AUDIENCE).unwrap();
    let err = enforce(
        true,
        &Method::GET,
        "/metrics.prom",
        &HeaderMap::new(),
        &verifier,
        &policy(),
    )
    .unwrap_err();
    assert_eq!(err.0, StatusCode::UNAUTHORIZED);
}

/// Secure by default: only an explicit false-like value turns it off; a
/// typo keeps it on; the environment beats the config file.
#[test]
fn resolve_require_auth_is_on_unless_explicitly_off() {
    assert!(resolve_require_auth(None, None));
    assert!(resolve_require_auth(Some(""), None));
    assert!(resolve_require_auth(Some("  "), None));
    assert!(resolve_require_auth(Some("1"), None));
    assert!(resolve_require_auth(Some("true"), None));
    assert!(resolve_require_auth(Some("flase"), None), "a typo stays on");
    for off in ["0", "false", "FALSE", "no", "Off", " off "] {
        assert!(!resolve_require_auth(Some(off), None), "{off}");
    }
    assert!(!resolve_require_auth(None, Some(false)), "config opt-out");
    assert!(resolve_require_auth(None, Some(true)));
    assert!(resolve_require_auth(Some("1"), Some(false)), "env wins");
    assert!(!resolve_require_auth(Some("false"), Some(true)), "env wins");
}

#[test]
fn configured_require_auth_reads_the_settings_block() {
    let on = serde_json::json!({ "require_auth": true });
    let off = serde_json::json!({ "require_auth": false });
    let other = serde_json::json!({ "x": 1 });
    assert_eq!(configured_require_auth(Some(&on)), Some(true));
    assert_eq!(configured_require_auth(Some(&off)), Some(false));
    assert_eq!(configured_require_auth(Some(&other)), None);
    assert_eq!(configured_require_auth(None), None);
}

/// The startup gate: production never runs anonymously, and enforcement
/// with nothing to verify against fails fast (unless a key URL may still
/// deliver keys).
#[test]
fn startup_verdict_refuses_the_unsafe_postures() {
    assert!(startup_verdict(true, false, true, false).is_err());
    assert!(startup_verdict(true, false, false, false).is_err());
    assert!(startup_verdict(false, false, false, false).is_ok());
    assert!(startup_verdict(false, true, true, false).is_ok());
    assert!(startup_verdict(true, true, true, false).is_ok());
    let err = startup_verdict(false, true, false, false).unwrap_err();
    assert!(err.contains("PASETO_KEYS"), "{err}");
    assert!(startup_verdict(false, true, false, true).is_ok());
}

/// Enforcement on, protected path, no token ⇒ `401`.
#[test]
fn enforce_on_protected_without_token_is_401() {
    let (keys, _) = test_keys_and_kid();
    let verifier = Verifier::from_paseto_keys_value(&keys, ISSUER, AUDIENCE).unwrap();
    let err = enforce(
        true,
        &Method::GET,
        "/api/projects",
        &HeaderMap::new(),
        &verifier,
        &policy(),
    )
    .unwrap_err();
    assert_eq!(err.0, StatusCode::UNAUTHORIZED);
}

/// Enforcement on, protected path, valid token ⇒ a read passes.
#[test]
fn enforce_on_protected_with_valid_token_is_ok() {
    let (keys, kid) = test_keys_and_kid();
    let verifier = Verifier::from_paseto_keys_value(&keys, ISSUER, AUDIENCE).unwrap();
    let token = sign(&kid, 10_000_000_000);
    assert!(
        enforce(
            true,
            &Method::GET,
            "/api/projects",
            &bearer(&token),
            &verifier,
            &policy()
        )
        .is_ok()
    );
}

/// Enforcement on, protected path, expired token ⇒ `401`.
#[test]
fn enforce_on_protected_with_expired_token_is_401() {
    let (keys, kid) = test_keys_and_kid();
    let verifier = Verifier::from_paseto_keys_value(&keys, ISSUER, AUDIENCE).unwrap();
    let token = sign(&kid, -60);
    let err = enforce(
        true,
        &Method::GET,
        "/api/projects",
        &bearer(&token),
        &verifier,
        &policy(),
    )
    .unwrap_err();
    assert_eq!(err.0, StatusCode::UNAUTHORIZED);
}

/// Enforcement on, protected path, tampered token ⇒ `401`.
#[test]
fn enforce_on_protected_with_tampered_token_is_401() {
    let (keys, kid) = test_keys_and_kid();
    let verifier = Verifier::from_paseto_keys_value(&keys, ISSUER, AUDIENCE).unwrap();
    let mut token = sign(&kid, 10_000_000_000);
    let last = token.pop().unwrap();
    token.push(if last == 'a' { 'b' } else { 'a' });
    let err = enforce(
        true,
        &Method::GET,
        "/api/projects",
        &bearer(&token),
        &verifier,
        &policy(),
    )
    .unwrap_err();
    assert_eq!(err.0, StatusCode::UNAUTHORIZED);
}

/// Action derivation (`authorization-attributes.md` §2): safe
/// methods read; DELETE deletes; the crate's destructive named
/// POSTs (merge today; deduplicate / bulk import when they land,
/// their §9.8 routes ending with the same suffixes) are
/// destructive, not write; every other POST/PUT/PATCH writes —
/// across all four collections.
#[test]
fn derive_action_matrix() {
    for method in [Method::GET, Method::HEAD, Method::OPTIONS] {
        assert_eq!(derive_action(&method, "/api/projects"), Action::Read);
    }
    assert_eq!(
        derive_action(&Method::DELETE, "/api/projects/123"),
        Action::Delete
    );
    // /links/{id} removal is covered by the DELETE method rule —
    // no suffix entry needed.
    assert_eq!(
        derive_action(&Method::DELETE, "/api/portfolios/123/links/456"),
        Action::Delete
    );
    for path in [
        "/api/portfolios/merge",
        "/api/projects/merge",
        "/api/products/merge",
        "/api/programs/merge",
        "/api/projects/deduplicate",
        "/api/portfolios/import", // §9.8 bulk-import route shape
    ] {
        assert_eq!(derive_action(&Method::POST, path), Action::Destructive);
    }
    // SEC-G6: a trailing slash must not downgrade a destructive POST.
    for path in [
        "/api/x/merge/",
        "/api/x/merge//",
        "/api/x/deduplicate/",
        "/api/x/import/",
    ] {
        assert_eq!(
            derive_action(&Method::POST, path),
            Action::Destructive,
            "{path}"
        );
    }
    assert_eq!(derive_action(&Method::POST, "/api/projects"), Action::Write);
    assert_eq!(
        derive_action(&Method::POST, "/api/projects/check-duplicates"),
        Action::Write
    );
    assert_eq!(
        derive_action(&Method::POST, "/api/portfolios/123/links"),
        Action::Write
    );
    assert_eq!(
        derive_action(&Method::PUT, "/api/projects/123"),
        Action::Write
    );
    assert_eq!(
        derive_action(&Method::PATCH, "/api/projects/123"),
        Action::Write
    );
    // GET on destructive-suffixed paths is still a read — only
    // POST consults the suffix list (e.g. the import-job status
    // read `GET …/import/{id}` and hypothetical `GET …/merge`).
    assert_eq!(
        derive_action(&Method::GET, "/api/portfolios/merge"),
        Action::Read
    );
}

/// ABAC default policy, empty `attrs` ⇒ GET allowed, POST `403`
/// (default allow-read / deny-mutation).
#[test]
fn abac_empty_attrs_reads_but_cannot_write() {
    let (keys, kid) = test_keys_and_kid();
    let verifier = Verifier::from_paseto_keys_value(&keys, ISSUER, AUDIENCE).unwrap();
    let policy = policy();
    let token = sign_with_attrs(&kid, 10_000_000_000, &[]);
    assert!(
        enforce(
            true,
            &Method::GET,
            "/api/projects",
            &bearer(&token),
            &verifier,
            &policy
        )
        .is_ok()
    );
    let err = enforce(
        true,
        &Method::POST,
        "/api/projects",
        &bearer(&token),
        &verifier,
        &policy,
    )
    .unwrap_err();
    assert_eq!(err.0, StatusCode::FORBIDDEN);
}

/// ABAC `access=write` ⇒ POST/PUT allowed; DELETE, the merge POST,
/// and the (future) import POST still `403` (write is not
/// destructive).
#[test]
fn abac_access_write_writes_but_not_destructive() {
    let (keys, kid) = test_keys_and_kid();
    let verifier = Verifier::from_paseto_keys_value(&keys, ISSUER, AUDIENCE).unwrap();
    let policy = policy();
    let token = sign_with_attrs(&kid, 10_000_000_000, &[("access", &["write"])]);
    for method in [Method::POST, Method::PUT] {
        assert!(
            enforce(
                true,
                &method,
                "/api/projects",
                &bearer(&token),
                &verifier,
                &policy
            )
            .is_ok(),
            "{method} should be allowed for access=write"
        );
    }
    let delete = enforce(
        true,
        &Method::DELETE,
        "/api/projects/123",
        &bearer(&token),
        &verifier,
        &policy,
    )
    .unwrap_err();
    assert_eq!(delete.0, StatusCode::FORBIDDEN);
    for path in ["/api/projects/merge", "/api/portfolios/import"] {
        let destructive = enforce(
            true,
            &Method::POST,
            path,
            &bearer(&token),
            &verifier,
            &policy,
        )
        .unwrap_err();
        assert_eq!(destructive.0, StatusCode::FORBIDDEN, "{path} should 403");
    }
}

/// ABAC `access=admin` ⇒ DELETE and the destructive named POSTs
/// are allowed (destructive covers delete).
#[test]
fn abac_access_admin_allows_destructive() {
    let (keys, kid) = test_keys_and_kid();
    let verifier = Verifier::from_paseto_keys_value(&keys, ISSUER, AUDIENCE).unwrap();
    let policy = policy();
    let token = sign_with_attrs(&kid, 10_000_000_000, &[("access", &["admin"])]);
    assert!(
        enforce(
            true,
            &Method::DELETE,
            "/api/projects/123",
            &bearer(&token),
            &verifier,
            &policy
        )
        .is_ok()
    );
    for path in [
        "/api/projects/merge",
        "/api/projects/deduplicate",
        "/api/portfolios/import",
    ] {
        assert!(
            enforce(
                true,
                &Method::POST,
                path,
                &bearer(&token),
                &verifier,
                &policy
            )
            .is_ok(),
            "{path} should be allowed for access=admin"
        );
    }
}

/// ABAC `svc=true` (machine peer) ⇒ everything is allowed.
#[test]
fn abac_svc_true_allows_everything() {
    let (keys, kid) = test_keys_and_kid();
    let verifier = Verifier::from_paseto_keys_value(&keys, ISSUER, AUDIENCE).unwrap();
    let policy = policy();
    let token = sign_with_attrs(&kid, 10_000_000_000, &[("svc", &["true"])]);
    for (method, path) in [
        (Method::GET, "/api/projects"),
        (Method::POST, "/api/projects"),
        (Method::PUT, "/api/projects/123"),
        (Method::DELETE, "/api/projects/123"),
        (Method::POST, "/api/projects/merge"),
        (Method::POST, "/api/portfolios/import"),
    ] {
        assert!(
            enforce(true, &method, path, &bearer(&token), &verifier, &policy).is_ok(),
            "{method} {path} should be allowed for svc=true"
        );
    }
}

/// A configured deny rule ahead of an allow rule wins
/// (first-match-wins pin, through the guard).
#[test]
fn abac_configured_deny_beats_later_allow() {
    let (keys, kid) = test_keys_and_kid();
    let verifier = Verifier::from_paseto_keys_value(&keys, ISSUER, AUDIENCE).unwrap();
    let policy = Policy::from_json(
        r#"{ "rules": [
            { "effect": "deny",  "actions": ["write"], "when": { "dept": ["pmo"] } },
            { "effect": "allow", "actions": ["write"], "when": { "access": ["write"] } }
        ] }"#,
    )
    .expect("policy parses");
    let denied = sign_with_attrs(
        &kid,
        10_000_000_000,
        &[("access", &["write"]), ("dept", &["pmo"])],
    );
    let err = enforce(
        true,
        &Method::POST,
        "/api/projects",
        &bearer(&denied),
        &verifier,
        &policy,
    )
    .unwrap_err();
    assert_eq!(err.0, StatusCode::FORBIDDEN);
    let allowed = sign_with_attrs(&kid, 10_000_000_000, &[("access", &["write"])]);
    assert!(
        enforce(
            true,
            &Method::POST,
            "/api/projects",
            &bearer(&allowed),
            &verifier,
            &policy
        )
        .is_ok()
    );
}

/// 401 vs 403: missing/bad credential is `401`; a valid credential
/// the policy denies is `403` with the deciding-rule reason
/// (`"default deny"` when no rule matched).
#[test]
fn abac_401_versus_403_distinction() {
    let (keys, kid) = test_keys_and_kid();
    let verifier = Verifier::from_paseto_keys_value(&keys, ISSUER, AUDIENCE).unwrap();
    let policy = policy();
    let no_token = enforce(
        true,
        &Method::POST,
        "/api/projects",
        &HeaderMap::new(),
        &verifier,
        &policy,
    )
    .unwrap_err();
    assert_eq!(no_token.0, StatusCode::UNAUTHORIZED);
    let token = sign_with_attrs(&kid, 10_000_000_000, &[]);
    let denied = enforce(
        true,
        &Method::POST,
        "/api/projects",
        &bearer(&token),
        &verifier,
        &policy,
    )
    .unwrap_err();
    assert_eq!(denied.0, StatusCode::FORBIDDEN);
    assert_eq!(denied.1, "default deny");
}

/// `policy_from_env` never breaks boot: bad policy JSON falls back
/// to the built-in default policy. (Pinned on the pure fallback
/// expression rather than by mutating the process-global env,
/// which other tests share.)
#[test]
fn policy_bad_json_falls_back_to_default() {
    assert!(Policy::from_json("{ not json").is_err());
    assert_eq!(
        Policy::from_json("{ not json").unwrap_or_else(|_| Policy::default_policy()),
        Policy::default_policy()
    );
}

/// Serve `keys` as the key-set JSON from a local ephemeral-port HTTP
/// listener (the auth service's `/.well-known/paseto-keys` stand-in)
/// and return the URL to fetch it from.
async fn serve_keys(keys: serde_json::Value) -> String {
    let app = axum::Router::new().route(
        "/.well-known/paseto-keys",
        axum::routing::get(move || {
            let keys = keys.clone();
            async move { axum::Json(keys) }
        }),
    );
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind ephemeral port");
    let addr = listener.local_addr().expect("local addr");
    tokio::spawn(async move {
        axum::serve(listener, app).await.expect("serve key set");
    });
    format!("http://{addr}/.well-known/paseto-keys")
}

/// Boot-time fetch happy path: `fetch_or` against a local listener
/// serving a valid key set builds the verifier from the **fetched**
/// keys — a token signed by the served key verifies even though the
/// fallback verifier has no keys (the fetched key set wins).
#[tokio::test]
async fn fetch_or_fetched_key_set_wins() {
    let (keys, kid) = test_keys_and_kid();
    let url = serve_keys(keys).await;
    let verifier = fetch_or(&url, ISSUER, AUDIENCE, empty_verifier(ISSUER, AUDIENCE)).await;
    assert_eq!(verifier.key_count(), 1);
    let token = sign(&kid, 10_000_000_000);
    let claims = bearer_claims(&bearer(&token), &verifier).expect("token signed by fetched key");
    assert_eq!(claims.sub, "11111111-1111-1111-1111-111111111111");
}

/// Boot-time fetch fallback: a fast-failing URL (nothing listens on
/// port 1) makes `fetch_or` return the env-style fallback verifier —
/// no panic, and tokens signed by the fallback key set still verify,
/// so the service always boots.
#[tokio::test]
async fn fetch_or_unreachable_url_falls_back() {
    let (keys, kid) = test_keys_and_kid();
    let fallback = Verifier::from_paseto_keys_value(&keys, ISSUER, AUDIENCE).unwrap();
    let verifier = fetch_or("http://127.0.0.1:1/", ISSUER, AUDIENCE, fallback).await;
    assert_eq!(verifier.key_count(), 1);
    let token = sign(&kid, 10_000_000_000);
    assert!(bearer_claims(&bearer(&token), &verifier).is_ok());
}
