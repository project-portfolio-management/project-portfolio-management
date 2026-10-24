//! Token verification: the process-wide PASETO verifier (and the Keycloak one), its key sources and refresh, and the bearer-token extraction.

#[allow(
    clippy::wildcard_imports,
    reason = "the parent module holds the shared imports and the other parts"
)]
use super::*;

/// Default issuer expected in tokens (`iss`).
#[cfg(feature = "paseto")]
pub(super) const DEFAULT_ISSUER: &str = "authentication-service";

/// Default audience expected in tokens (`aud`).
#[cfg(feature = "paseto")]
pub(super) const DEFAULT_AUDIENCE: &str = "main-x-service";

/// The process-wide token verifier, seeded by [`init`] at boot (or, if
/// [`init`] never ran — e.g. in unit tests — built lazily from the
/// environment on first use). Shared behind an `Arc` and read-only.
#[cfg(feature = "paseto")]
pub(super) static VERIFIER: OnceLock<ReloadableVerifier> = OnceLock::new();

/// The process-wide token verifier (see [`VERIFIER`] and the module
/// docs). Shared behind an `Arc` and read-only.
#[cfg(feature = "paseto")]
#[must_use]
pub fn verifier() -> &'static ReloadableVerifier {
    VERIFIER.get_or_init(|| ReloadableVerifier::new(build_from_env()))
}

/// Seed the process-wide [`verifier`] before the app serves traffic
/// (called from `App::after_routes`). When `PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS_URL` is
/// set (non-blank) the published key set is fetched over HTTP **once** —
/// no refresh loop — and, on success, wins over the
/// `PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS` env key set; on fetch failure, or with the URL
/// unset/blank, the verifier is built from the environment exactly as
/// before, so the service always boots. Idempotent: a no-op when the
/// verifier is already built.
// `async` in every feature shape so `app.rs` has one call site; only the
// `paseto` half awaits (the key fetch).
pub async fn init() {
    #[cfg(feature = "paseto")]
    init_paseto().await;
    #[cfg(feature = "keycloak")]
    init_keycloak();
}

/// Seed the PASETO [`verifier`] (the `paseto` half of [`init`]).
#[cfg(feature = "paseto")]
pub(super) async fn init_paseto() {
    let built = match std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS_URL")
        .ok()
        .filter(|s| !s.trim().is_empty())
    {
        Some(url) => {
            let issuer = env_or("PROJECT_PORTFOLIO_MANAGEMENT_TOKEN_ISSUER", DEFAULT_ISSUER);
            let audience = env_or(
                "PROJECT_PORTFOLIO_MANAGEMENT_TOKEN_AUDIENCE",
                DEFAULT_AUDIENCE,
            );
            fetch_or(url.trim(), &issuer, &audience, build_from_env()).await
        }
        None => build_from_env(),
    };
    // `store` rather than `set`: the holder may already exist (a lazy
    // `verifier()` call beat us here), and the boot-fetched key set
    // should win either way.
    verifier().store(built);
}

/// The process-wide Keycloak verifier, present only when
/// `PROJECT_PORTFOLIO_MANAGEMENT_KEYCLOAK_URL` is set and valid.
#[cfg(feature = "keycloak")]
pub(super) static KEYCLOAK: OnceLock<authentication_verifier::keycloak::KeycloakVerifier> =
    OnceLock::new();

/// The Keycloak verifier, or `None` when Keycloak is not configured (or was
/// configured badly — see [`init_keycloak`]).
#[cfg(feature = "keycloak")]
#[must_use]
pub fn keycloak_verifier() -> Option<&'static authentication_verifier::keycloak::KeycloakVerifier> {
    KEYCLOAK.get()
}

/// Build the Keycloak verifier from `PROJECT_PORTFOLIO_MANAGEMENT_KEYCLOAK_*`.
/// Fail-closed and non-fatal, like the PASETO key loaders: a missing URL
/// leaves Keycloak off; a malformed configuration logs an error and leaves
/// it off (Keycloak tokens are then rejected), and the service still boots.
/// Must run inside the Tokio runtime (discovery starts in the background).
#[cfg(feature = "keycloak")]
pub(super) fn init_keycloak() {
    use authentication_verifier::keycloak::{KeycloakSettings, KeycloakVerifier};
    if KEYCLOAK.get().is_some() {
        return;
    }
    match KeycloakSettings::from_env("PROJECT_PORTFOLIO_MANAGEMENT") {
        Ok(None) => {}
        Ok(Some(settings)) => {
            let issuer = settings.expected_issuer();
            match KeycloakVerifier::new(settings) {
                Ok(verifier) => {
                    let _ = KEYCLOAK.set(verifier);
                    tracing::info!(%issuer, "auth: Keycloak access tokens accepted");
                }
                Err(error) => {
                    tracing::error!(%error, "auth: Keycloak configuration rejected; Keycloak tokens will be refused");
                }
            }
        }
        Err(error) => {
            tracing::error!(%error, "auth: PROJECT_PORTFOLIO_MANAGEMENT_KEYCLOAK_* is invalid; Keycloak tokens will be refused");
        }
    }
}

/// Default key-set refresh interval (seconds) when
/// `PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS_REFRESH_SECS` is unset. One
/// hour: rotation is infrequent, so a slow poll suffices.
#[cfg(feature = "paseto")]
pub(super) const KEY_REFRESH_DEFAULT_SECS: u64 = 3600;

/// Spawn the background loop that re-fetches the published key set from
/// `PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS_URL` and swaps it into the live
/// [`verifier`], so **a key rotation needs no restart**.
///
/// A failed fetch **keeps the current key set** — a transient
/// auth-service outage must not lock every caller out. Interval from
/// `PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS_REFRESH_SECS` (default
/// [`KEY_REFRESH_DEFAULT_SECS`]); `0` disables the loop. A no-op when
/// the URL is unset, since an env-supplied key set has nothing to
/// re-fetch.
pub fn spawn_key_refresh() {
    #[cfg(feature = "paseto")]
    spawn_paseto_key_refresh();
}

/// The `paseto` half of [`spawn_key_refresh`].
#[cfg(feature = "paseto")]
pub(super) fn spawn_paseto_key_refresh() {
    let Some(url) = std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS_URL")
        .ok()
        .map(|u| u.trim().to_string())
        .filter(|u| !u.is_empty())
    else {
        return;
    };
    let secs = std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS_REFRESH_SECS")
        .ok()
        .and_then(|v| v.trim().parse::<u64>().ok())
        .unwrap_or(KEY_REFRESH_DEFAULT_SECS);
    if secs == 0 {
        return;
    }
    let issuer = env_or("PROJECT_PORTFOLIO_MANAGEMENT_TOKEN_ISSUER", DEFAULT_ISSUER);
    let audience = env_or(
        "PROJECT_PORTFOLIO_MANAGEMENT_TOKEN_AUDIENCE",
        DEFAULT_AUDIENCE,
    );
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(std::time::Duration::from_secs(secs));
        ticker.tick().await; // the first tick is immediate — skip it
        loop {
            ticker.tick().await;
            match Verifier::from_paseto_keys_url(&url, &issuer, &audience).await {
                Ok(fetched) => {
                    tracing::info!(keys = fetched.key_count(), "refreshed PASETO key set");
                    verifier().store(fetched);
                }
                Err(error) => {
                    tracing::warn!(%error, "PASETO key-set refresh failed; keeping current keys");
                }
            }
        }
    });
    tracing::info!(secs, "polling the published key set for rotation");
}

/// Build a verifier by fetching the published key set from `url`
/// ([`Verifier::from_paseto_keys_url`]); on success the fetched key set
/// wins (`tracing::info!`), on any fetch/parse failure the given
/// `fallback` verifier is returned after a `tracing::warn!` — never a
/// panic, so the caller always boots. Pure dependency injection (URL,
/// issuer, audience and fallback are all passed in), so it is testable
/// against a local HTTP listener without touching the process global.
#[cfg(feature = "paseto")]
pub async fn fetch_or(url: &str, issuer: &str, audience: &str, fallback: Verifier) -> Verifier {
    match Verifier::from_paseto_keys_url(url, issuer, audience).await {
        Ok(fetched) => {
            tracing::info!(
                url,
                keys = fetched.key_count(),
                "PASETO key set fetched over HTTP; fetched key set wins over the env key set"
            );
            fetched
        }
        Err(error) => {
            tracing::warn!(
                url,
                %error,
                "PASETO key set fetch failed; falling back to the env-configured key set"
            );
            fallback
        }
    }
}

/// Read env var `name`, treating unset/blank as absent and falling back
/// to `default`. Used for the issuer/audience so a blank value doesn't
/// override the sensible default.
#[cfg(feature = "paseto")]
pub(super) fn env_or(name: &str, default: &str) -> String {
    std::env::var(name)
        .ok()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| default.to_string())
}

/// Build the process-wide [`Verifier`] from the environment: issuer,
/// audience, and the published key set. A missing/blank/unparseable key
/// set yields an empty key set (every token rejected) so the service still
/// boots without credentials configured.
#[cfg(feature = "paseto")]
pub(super) fn build_from_env() -> Verifier {
    let issuer = env_or("PROJECT_PORTFOLIO_MANAGEMENT_TOKEN_ISSUER", DEFAULT_ISSUER);
    let audience = env_or(
        "PROJECT_PORTFOLIO_MANAGEMENT_TOKEN_AUDIENCE",
        DEFAULT_AUDIENCE,
    );
    let keys = std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .and_then(|raw| serde_json::from_str::<serde_json::Value>(&raw).ok())
        .unwrap_or_else(|| serde_json::json!({ "keys": [] }));
    Verifier::from_paseto_keys_value(&keys, &issuer, &audience)
        .unwrap_or_else(|_| empty_verifier(&issuer, &audience))
}

/// A verifier with no keys: rejects every token until a real key set is
/// configured. Infallible — an empty `keys` array always parses.
#[cfg(feature = "paseto")]
pub(super) fn empty_verifier(issuer: &str, audience: &str) -> Verifier {
    let empty = serde_json::json!({ "keys": [] });
    Verifier::from_paseto_keys_value(&empty, issuer, audience).expect("empty key set always builds")
}

/// Extract and verify the bearer token from request headers. Pure (the
/// verifier is passed in), so it is unit-testable without the global.
///
/// # Errors
///
/// `401` when the `Authorization` header is missing, is not a bearer
/// token, or the token fails PASETO signature / issuer / audience / expiry
/// verification.
#[cfg(feature = "paseto")]
pub fn bearer_claims(
    headers: &HeaderMap,
    verifier: &Verifier,
) -> Result<Claims, (StatusCode, String)> {
    let token = bearer_token(headers)?;
    verifier
        .verify(token)
        .map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))
}

/// The bearer token from the `Authorization` header, trimmed.
///
/// # Errors
///
/// `401` when the header is missing or is not a bearer token.
pub(super) fn bearer_token(headers: &HeaderMap) -> Result<&str, (StatusCode, String)> {
    let header = headers
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .ok_or((
            StatusCode::UNAUTHORIZED,
            "missing authorization header".to_string(),
        ))?;
    header
        .strip_prefix("Bearer ")
        .or_else(|| header.strip_prefix("bearer "))
        .map(str::trim)
        .ok_or((
            StatusCode::UNAUTHORIZED,
            "expected bearer token".to_string(),
        ))
}

/// Authenticate a request through every credential type this build
/// accepts, against the process-wide verifiers. A token beginning `v4.` is
/// a PASETO; anything else is offered to Keycloak when configured. Each
/// path fails closed with `401`.
///
/// # Errors
///
/// `401` when the header is missing or not a bearer token, the token fails
/// verification, or no verifier in this build accepts its type.
// `async` for the Keycloak path (JWKS refresh); a PASETO-only build never awaits. An `allow`, not an
// `expect`: the lint fires in that build and not in an all-features one.
#[allow(clippy::unused_async)]
pub async fn request_claims(headers: &HeaderMap) -> Result<Claims, (StatusCode, String)> {
    let token = bearer_token(headers)?;
    #[cfg(feature = "keycloak")]
    if !token.starts_with("v4.")
        && let Some(keycloak) = keycloak_verifier()
    {
        return keycloak
            .verify(token)
            .await
            .map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()));
    }
    #[cfg(feature = "paseto")]
    {
        verifier()
            .current()
            .verify(token)
            .map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))
    }
    #[cfg(not(feature = "paseto"))]
    {
        let _ = token;
        Err((
            StatusCode::UNAUTHORIZED,
            "no verifier accepts this credential".to_string(),
        ))
    }
}
