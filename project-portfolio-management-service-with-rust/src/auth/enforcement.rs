//! Enforcement: whether authentication is required, the startup checks, which paths are public, the middleware that authenticates and authorises each request, and the action a request derives.

#[allow(
    clippy::wildcard_imports,
    reason = "the parent module holds the shared imports and the other parts"
)]
use super::*;

/// The cached enforcement posture (see [`require_auth`]).
pub(super) static REQUIRE_AUTH: OnceLock<bool> = OnceLock::new();

/// Resolve the enforcement posture from the two explicit sources. Pure.
///
/// **Secure by default**: enforcement is on unless somebody explicitly
/// turned it off. The environment variable
/// `PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH` wins when set and non-blank
/// (`0`/`false`/`no`/`off`, case-insensitive, is off; anything else,
/// including a typo, is on); otherwise the `configured` value (the loco
/// `settings.require_auth` key of the active `config/<env>.yaml`) applies;
/// otherwise it is on.
#[must_use]
pub fn resolve_require_auth(env: Option<&str>, configured: Option<bool>) -> bool {
    match env.map(str::trim).filter(|v| !v.is_empty()) {
        Some(value) => !is_falsy(value),
        None => configured.unwrap_or(true),
    }
}

/// `0`/`false`/`no`/`off` (case-insensitive, surrounding whitespace ignored).
pub(super) fn is_falsy(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "0" | "false" | "no" | "off"
    )
}

/// Read `settings.require_auth` from a loco config `settings` block.
#[must_use]
pub fn configured_require_auth(settings: Option<&serde_json::Value>) -> Option<bool> {
    settings?.get("require_auth")?.as_bool()
}

/// Fix the process-wide enforcement posture from the environment plus the
/// config file's `settings.require_auth` (called once at boot, from
/// `App::after_routes`). First caller wins: the value is cached for the life
/// of the process, and every later call just reports it. Returns the
/// effective posture.
pub fn configure_require_auth(configured: Option<bool>) -> bool {
    *REQUIRE_AUTH.get_or_init(|| {
        resolve_require_auth(
            std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH")
                .ok()
                .as_deref(),
            configured,
        )
    })
}

/// Whether blanket `/api/*` enforcement is on, cached for the process. On
/// by default — see [`resolve_require_auth`] and the module docs. When
/// nothing has called [`configure_require_auth`] yet (a unit test, a CLI
/// task) it is resolved from the environment alone, so the answer is still
/// "on" unless explicitly turned off.
#[must_use]
pub fn require_auth() -> bool {
    configure_require_auth(None)
}

/// Whether any token verifier this build accepts has keys to verify with.
pub(super) fn verifier_configured() -> bool {
    #[cfg(feature = "paseto")]
    if verifier().current().key_count() > 0 {
        return true;
    }
    #[cfg(feature = "keycloak")]
    if keycloak_verifier().is_some() {
        return true;
    }
    // WW-1.12: a configured OIDC provider counts even before its keys arrive (they are fetched again on the
    // first token), as a configured PASETO key URL does.
    super::oidc::oidc_verifier().is_some()
}

/// The startup gate, pure. `Err(message)` means the service must refuse to
/// start:
///
/// - the loco environment is `production` and enforcement is off;
/// - enforcement is on but no token verifier has keys, *and* no
///   `PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS_URL` is set that the rotation
///   loop could still fill in (with a URL configured the service boots and
///   answers `401` until the key set arrives, rather than crash-looping while
///   the auth service is briefly down).
///
/// # Errors
///
/// See above; the message names the variable to fix.
#[expect(clippy::fn_params_excessive_bools)] // four independent facts, named at every call
pub fn startup_verdict(
    production: bool,
    require: bool,
    verifier_has_keys: bool,
    keys_url_configured: bool,
) -> Result<(), String> {
    if !require && production {
        return Err(
            "refusing to start: authentication enforcement is off in the production \
             environment. Remove PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH=false (or the \
             config's settings.require_auth: false); production never runs anonymously."
                .to_string(),
        );
    }
    if require && !verifier_has_keys && !keys_url_configured {
        return Err(
            "refusing to start: authentication enforcement is on but no token verifier is \
             configured, so every request would be 401. Set \
             PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS (or ..._PASETO_KEYS_URL, ..._OIDC_ISSUER \
             or the Keycloak settings), or explicitly opt out for development with \
             PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH=false."
                .to_string(),
        );
    }
    Ok(())
}

/// Fix the enforcement posture and apply [`startup_verdict`] (called from
/// `App::after_routes` after [`init`] seeded the verifiers). Logs a loud
/// warning when enforcement is off.
///
/// # Errors
///
/// The refusal message from [`startup_verdict`].
pub fn check_startup(production: bool, settings: Option<&serde_json::Value>) -> Result<(), String> {
    let require = configure_require_auth(configured_require_auth(settings));
    let keys_url = std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS_URL")
        .is_ok_and(|u| !u.trim().is_empty());
    startup_verdict(production, require, verifier_configured(), keys_url)?;
    if !require {
        tracing::warn!(
            "AUTHENTICATION ENFORCEMENT IS OFF: every route is open to anonymous callers and \
             every ABAC check is skipped. This is for local development and tests only; never \
             expose this process to a network you do not control."
        );
    } else if !verifier_configured() {
        tracing::warn!(
            "authentication enforcement is on and no token key set is loaded yet; requests \
             answer 401 until PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS_URL delivers keys"
        );
    }
    Ok(())
}

/// Derive the request's ABAC action from its HTTP method and path (per
/// `authorization-attributes.md` §2): `GET`/`HEAD`/`OPTIONS` ⇒ `Read`;
/// `DELETE` ⇒ `Delete`; a `POST` whose path ends with a
/// [`DESTRUCTIVE_POST_SUFFIXES`] entry ⇒ `Destructive`; every other
/// `POST`/`PUT`/`PATCH` (and any unrecognized method) ⇒ `Write`.
/// (`DELETE /…/links/{id}` needs no suffix entry — the DELETE method
/// rule already covers it.)
#[must_use]
pub fn derive_action(method: &Method, path: &str) -> Action {
    // SEC-G6: normalize a trailing slash before the destructive-suffix
    // check, so `POST …/merge/` stays `Destructive` rather than being
    // downgraded to `Write` (which an `access=write` non-admin caller
    // could exploit to reach a destructive op).
    let path = path.trim_end_matches('/');
    match *method {
        Method::GET | Method::HEAD | Method::OPTIONS => Action::Read,
        Method::DELETE => Action::Delete,
        Method::POST
            if DESTRUCTIVE_POST_SUFFIXES
                .iter()
                .any(|suffix| path.ends_with(suffix)) =>
        {
            Action::Destructive
        }
        _ => Action::Write,
    }
}

/// Lenient boolean parse: `1`/`true`/`yes`/`on` (case-insensitive,
/// surrounding whitespace ignored) ⇒ `true`; everything else
/// (incl. empty) ⇒ `false`.
#[must_use]
pub fn parse_bool(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "1" | "true" | "yes" | "on"
    )
}

/// Paths that stay public even when enforcement is on: health/ping, the
/// `OpenAPI` doc + Swagger UI, and the calendar feeds (which carry their own
/// token). The Prometheus endpoint is deliberately **not** here: it exposes
/// route and traffic shape, so a scraper presents a bearer token like any
/// other client. Everything else requires a valid bearer token.
pub(super) fn is_public_path(path: &str) -> bool {
    path == "/_health"
        || path == "/_ping"
        // Readiness pings the database, the queue and the cache and answers only
        // ok / not ok (200 / 503): what a load balancer or orchestrator needs.
        || path == "/_readiness"
        || path == "/api-docs/openapi.json"
        || path.starts_with("/swagger-ui")
        // Feeds authenticate themselves with a per-user revocable token in the
        // query string (a calendar app cannot send a bearer header); see
        // `controllers::feeds`.
        || path.starts_with("/api/feeds/")
}

/// The blanket-enforcement decision: authentication, then ABAC
/// authorization. `Ok(())` ⇒ let the request through; `Err((401|403,
/// msg))` ⇒ reject. Pure: the caller passes the flag, method, path,
/// headers, verifier and policy, so it is fully unit-testable without
/// booting the app or a database.
///
/// # Errors
///
/// `401` when enforcement is on, the path is not public, and the
/// request carries no valid bearer token
/// (missing/malformed/expired/tampered). `403` when the token is valid
/// but the ABAC policy denies the derived action (the message names
/// the deciding rule, per `authorization-attributes.md` §5).
#[cfg(feature = "paseto")]
pub fn enforce(
    require_auth: bool,
    method: &Method,
    path: &str,
    headers: &HeaderMap,
    verifier: &Verifier,
    policy: &Policy,
) -> Result<(), (StatusCode, String)> {
    if !require_auth || is_public_path(path) {
        return Ok(());
    }
    let claims = bearer_claims(headers, verifier)?;
    decide(&claims, method, path, policy)
}

/// The ABAC half of [`enforce`]: `Ok` when the policy allows the request's
/// derived action for `claims`, else `403` carrying the deciding rule.
pub(super) fn decide(
    claims: &Claims,
    method: &Method,
    path: &str,
    policy: &Policy,
) -> Result<(), (StatusCode, String)> {
    // A capture token (BR-12) is narrowed before the policy is asked; the policy can only narrow it further.
    if has_attr(claims, CAPTURE_SCOPE.0, CAPTURE_SCOPE.1) && !capture_allows(method, path) {
        return Err((
            StatusCode::FORBIDDEN,
            "a token with scope=capture may only post status updates, comments and lessons"
                .to_string(),
        ));
    }
    let decision = policy.evaluate(claims, derive_action(method, path), ENTITY);
    if decision.allowed {
        Ok(())
    } else {
        Err((StatusCode::FORBIDDEN, decision.reason))
    }
}

/// The blanket guard the middleware runs: like [`enforce`], but the caller
/// is authenticated through every credential type this build accepts
/// (PASETO with `paseto`, a Keycloak access token with `keycloak`) via
/// [`request_claims`], against the process-wide verifiers.
///
/// # Errors
///
/// `401` when enforcement is on and no accepted credential is presented;
/// `403` when the ABAC policy denies the derived action.
pub async fn enforce_request(
    require_auth: bool,
    method: &Method,
    path: &str,
    headers: &HeaderMap,
    policy: &Policy,
) -> Result<(), (StatusCode, String)> {
    if !require_auth || is_public_path(path) {
        return Ok(());
    }
    let claims = request_claims(headers).await?;
    decide(&claims, method, path, policy)
}
