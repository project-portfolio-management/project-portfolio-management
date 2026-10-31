//! Any OIDC identity provider's access tokens (WW-1.12).
//!
//! The Keycloak path ([`super::keycloak_verifier`], the `keycloak` feature) accepts one product's tokens. This
//! module accepts a signed JWT access token from **any** provider that publishes standard OIDC discovery
//! (`{issuer}/.well-known/openid-configuration`) and a JSON Web Key Set: a national or sector federation, a cloud
//! directory, a university's or a ministry's own server. Nothing in it names a product or a country. It is off until
//! `PROJECT_PORTFOLIO_MANAGEMENT_OIDC_ISSUER` is set, and needs no build feature.
//!
//! Fail-closed rules, the same as the Keycloak path's where they overlap:
//!
//! - **HTTPS only** for the issuer and the key-set address (loopback excepted, for tests and local providers).
//! - **The discovery document's `issuer` must equal the configured one**, exactly (OIDC Discovery §4.3),
//!   and so must every token's `iss`.
//! - **At least one audience** is required; a token must carry one of them in `aud`.
//! - **Asymmetric signatures only** (RS, PS, ES and `EdDSA` families): `none` and the shared-secret HS family are
//!   refused, and so is a symmetric (`oct`) key in the set.
//! - **`exp` is required**; `nbf` is honoured; 60 seconds of clock leeway.
//! - **Verified email**, when the token carries an `email` (unless turned off): `$email` policy templates would
//!   otherwise be spoofable.
//! - **An explicit role map.** The provider's roles or groups (from a claim the deployment names, `roles` by
//!   default, a dotted path such as `realm_access.roles` allowed) become ABAC attributes only through
//!   `PROJECT_PORTFOLIO_MANAGEMENT_OIDC_ROLE_MAP`; an unmapped role grants nothing.
//!
//! Keys are fetched at boot and again when a token names a key id the cached set lacks (at most once a minute), so
//! a provider's key rotation is followed and a provider unreachable at boot is retried. One issuer per deployment.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;
use std::time::{Duration, Instant};

use authentication_verifier::Claims;
use jsonwebtoken::jwk::{AlgorithmParameters, JwkSet};
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use serde_json::Value;
use tokio::sync::Mutex;

/// The environment prefix of every setting.
const PREFIX: &str = "PROJECT_PORTFOLIO_MANAGEMENT_OIDC_";

/// The shortest wait between two key-set fetches.
const REFETCH_AFTER: Duration = Duration::from_secs(60);

/// Seconds of clock skew allowed on `exp` and `nbf`.
const LEEWAY_SECONDS: u64 = 60;

/// The signature algorithms accepted: the asymmetric families only.
const ALGORITHMS: &[Algorithm] = &[
    Algorithm::RS256,
    Algorithm::RS384,
    Algorithm::RS512,
    Algorithm::PS256,
    Algorithm::PS384,
    Algorithm::PS512,
    Algorithm::ES256,
    Algorithm::ES384,
    Algorithm::EdDSA,
];

/// A provider's role or group → ABAC attributes (`{"editor": {"access": ["write"]}}`).
pub type RoleMap = BTreeMap<String, BTreeMap<String, Vec<String>>>;

/// The deployment's OIDC settings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OidcSettings {
    /// The issuer identifier, exactly as the provider states it (no trailing slash is added or removed).
    pub issuer: String,
    /// Accepted `aud` values; never empty.
    pub audiences: Vec<String>,
    /// The claim holding the caller's roles or groups: a name, or a dotted path into nested objects.
    pub roles_claim: String,
    /// Roles → attributes.
    pub role_map: RoleMap,
    /// Require `email_verified: true` when an `email` is present.
    pub require_verified_email: bool,
}

/// Whether `url` is HTTPS, or HTTP to a loopback host.
fn secure_url(url: &str) -> bool {
    if url.starts_with("https://") {
        return true;
    }
    url.strip_prefix("http://").is_some_and(|rest| {
        let host = rest.split(['/', ':']).next().unwrap_or("");
        matches!(host, "localhost" | "127.0.0.1") || rest.starts_with("[::1]")
    })
}

/// Whether `s` is a short lower-case token (an attribute key or value).
fn plain_token(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 64
        && s.bytes().all(|b| {
            b.is_ascii_lowercase() || b.is_ascii_digit() || matches!(b, b'_' | b'-' | b'.')
        })
}

/// The settings from `lookup` (the environment, or a map in tests). `Ok(None)` when the issuer is unset or blank.
///
/// # Errors
///
/// A message naming the setting when the issuer is not HTTPS, no audience is given, the role map is not a JSON
/// object of objects of string arrays with plain lower-case keys and values, or the email switch is not a boolean.
pub fn settings_from_lookup(
    lookup: impl Fn(&str) -> Option<String>,
) -> Result<Option<OidcSettings>, String> {
    let var = |name: &str| {
        lookup(&format!("{PREFIX}{name}"))
            .map(|v| v.trim().to_string())
            .filter(|v| !v.is_empty())
    };
    let Some(issuer) = var("ISSUER") else {
        return Ok(None);
    };
    if !secure_url(&issuer) {
        return Err(format!(
            "{PREFIX}ISSUER must be an https URL (got {issuer:?})"
        ));
    }
    let audiences: Vec<String> = var("AUDIENCES")
        .unwrap_or_default()
        .split(',')
        .map(|a| a.trim().to_string())
        .filter(|a| !a.is_empty())
        .collect();
    if audiences.is_empty() {
        return Err(format!(
            "{PREFIX}AUDIENCES is required: without one a token minted for any client of the provider would pass"
        ));
    }
    let role_map: RoleMap = match var("ROLE_MAP") {
        None => RoleMap::new(),
        Some(raw) => serde_json::from_str(&raw).map_err(|_| {
            format!(r#"{PREFIX}ROLE_MAP must be a JSON object such as {{"editor": {{"access": ["write"]}}}}"#)
        })?,
    };
    for attrs in role_map.values() {
        for (key, values) in attrs {
            if !plain_token(key) || !values.iter().all(|v| plain_token(v)) {
                return Err(format!(
                    "{PREFIX}ROLE_MAP attribute keys and values must be short lower-case tokens (got {key:?})"
                ));
            }
        }
    }
    let require_verified_email = match var("REQUIRE_VERIFIED_EMAIL").as_deref() {
        None => true,
        Some(v) if v.eq_ignore_ascii_case("true") => true,
        Some(v) if v.eq_ignore_ascii_case("false") => false,
        Some(v) => {
            return Err(format!(
                "{PREFIX}REQUIRE_VERIFIED_EMAIL must be true or false (got {v:?})"
            ));
        }
    };
    Ok(Some(OidcSettings {
        issuer,
        audiences,
        roles_claim: var("ROLES_CLAIM").unwrap_or_else(|| "roles".to_string()),
        role_map,
        require_verified_email,
    }))
}

/// The value at a dotted path (`realm_access.roles`) in a claims object.
fn at_path<'a>(claims: &'a Value, path: &str) -> Option<&'a Value> {
    path.split('.').try_fold(claims, |v, key| v.get(key))
}

/// The roles a token carries at `path`: an array of strings, or one space-separated string.
fn roles_at(claims: &Value, path: &str) -> Vec<String> {
    match at_path(claims, path) {
        Some(Value::Array(items)) => items
            .iter()
            .filter_map(Value::as_str)
            .map(str::to_string)
            .collect(),
        Some(Value::String(s)) => s.split_whitespace().map(str::to_string).collect(),
        _ => Vec::new(),
    }
}

/// Map a token's **already verified** claims onto [`Claims`]: the verified-email check and the role map. Pure.
///
/// # Errors
///
/// When there is no `sub`, or an `email` that is not verified while that is required.
pub fn claims_from(claims: &Value, settings: &OidcSettings) -> Result<Claims, String> {
    let text = |k: &str| {
        claims
            .get(k)
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string()
    };
    let sub = text("sub");
    if sub.is_empty() {
        return Err("the token has no sub".to_string());
    }
    let email = text("email");
    let verified = claims
        .get("email_verified")
        .is_some_and(|v| v.as_bool() == Some(true) || v.as_str() == Some("true"));
    if settings.require_verified_email && !email.is_empty() && !verified {
        return Err("email is not verified".to_string());
    }
    let mut attrs: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for role in roles_at(claims, &settings.roles_claim) {
        if let Some(mapped) = settings.role_map.get(&role) {
            for (attr, values) in mapped {
                attrs
                    .entry(attr.clone())
                    .or_default()
                    .extend(values.iter().cloned());
            }
        }
    }
    let carried: Vec<String> = match claims.get("aud") {
        Some(Value::String(a)) => vec![a.clone()],
        Some(Value::Array(a)) => a
            .iter()
            .filter_map(Value::as_str)
            .map(str::to_string)
            .collect(),
        _ => Vec::new(),
    };
    let aud = settings
        .audiences
        .iter()
        .find(|a| carried.contains(a))
        .or_else(|| settings.audiences.first())
        .cloned()
        .unwrap_or_default();
    let number = |k: &str| claims.get(k).and_then(Value::as_i64);
    let name = [text("name"), text("preferred_username")]
        .into_iter()
        .find(|n| !n.is_empty())
        .unwrap_or_default();
    let sid = [text("sid"), text("jti")]
        .into_iter()
        .find(|n| !n.is_empty())
        .unwrap_or_default();
    Ok(Claims {
        sub,
        email,
        name,
        iss: text("iss"),
        aud,
        exp: number("exp").unwrap_or(0),
        iat: number("iat").unwrap_or(0),
        nbf: number("nbf"),
        sid,
        scope: Vec::new(),
        roles: Vec::new(),
        attrs: attrs
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().collect()))
            .collect(),
    })
}

/// The cached key set and when it was last fetched.
#[derive(Default)]
struct KeyState {
    keys: Option<JwkSet>,
    fetched: Option<Instant>,
}

/// Verifies one provider's access tokens.
pub struct OidcVerifier {
    settings: OidcSettings,
    state: Mutex<KeyState>,
    /// `None` in tests built with fixed keys: no fetch is ever made.
    http: Option<reqwest::Client>,
}

impl std::fmt::Debug for OidcVerifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OidcVerifier")
            .field("issuer", &self.settings.issuer)
            .finish_non_exhaustive()
    }
}

impl OidcVerifier {
    /// A verifier that fetches the provider's keys over HTTPS.
    #[must_use]
    pub fn new(settings: OidcSettings) -> Self {
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .ok();
        Self {
            settings,
            state: Mutex::new(KeyState::default()),
            http,
        }
    }

    /// A verifier with a fixed key set and no fetching (tests).
    #[must_use]
    pub fn with_keys(settings: OidcSettings, keys: JwkSet) -> Self {
        Self {
            settings,
            state: Mutex::new(KeyState {
                keys: Some(keys),
                fetched: Some(Instant::now()),
            }),
            http: None,
        }
    }

    /// The configured issuer.
    #[must_use]
    pub fn issuer(&self) -> &str {
        &self.settings.issuer
    }

    /// Discovery, then the key set. The discovery document must name this issuer and a secure `jwks_uri`.
    async fn fetch_keys(&self, http: &reqwest::Client) -> Result<JwkSet, String> {
        let discovery = format!(
            "{}/.well-known/openid-configuration",
            self.settings.issuer.trim_end_matches('/')
        );
        let doc: Value = http
            .get(&discovery)
            .send()
            .await
            .and_then(reqwest::Response::error_for_status)
            .map_err(|e| format!("discovery: {e}"))?
            .json()
            .await
            .map_err(|e| format!("discovery: {e}"))?;
        if doc.get("issuer").and_then(Value::as_str) != Some(self.settings.issuer.as_str()) {
            return Err("discovery names another issuer".to_string());
        }
        let jwks_uri = doc
            .get("jwks_uri")
            .and_then(Value::as_str)
            .filter(|u| secure_url(u))
            .ok_or("discovery has no https jwks_uri")?;
        http.get(jwks_uri)
            .send()
            .await
            .and_then(reqwest::Response::error_for_status)
            .map_err(|e| format!("key set: {e}"))?
            .json::<JwkSet>()
            .await
            .map_err(|e| format!("key set: {e}"))
    }

    /// Refresh the key set if it is missing or lacks `kid`, at most once per [`REFETCH_AFTER`].
    async fn refresh_if_needed(&self, kid: Option<&str>) {
        let Some(http) = &self.http else { return };
        let mut state = self.state.lock().await;
        let has = state
            .keys
            .as_ref()
            .is_some_and(|k| kid.is_none_or(|id| k.find(id).is_some()));
        let recent = state.fetched.is_some_and(|t| t.elapsed() < REFETCH_AFTER);
        if has || recent {
            return;
        }
        state.fetched = Some(Instant::now());
        match self.fetch_keys(http).await {
            Ok(keys) => state.keys = Some(keys),
            Err(error) => {
                tracing::error!(issuer = %self.settings.issuer, %error, "auth: OIDC keys could not be fetched");
            }
        }
    }

    /// Verify a token's signature and registered claims, then map it ([`claims_from`]).
    ///
    /// # Errors
    ///
    /// A message for any failure: malformed, an algorithm not accepted, no matching key, a bad signature, a wrong
    /// issuer or audience, expired, not yet valid, or a claim [`claims_from`] refuses.
    pub async fn verify(&self, token: &str) -> Result<Claims, String> {
        let header =
            jsonwebtoken::decode_header(token).map_err(|e| format!("malformed token: {e}"))?;
        if !ALGORITHMS.contains(&header.alg) {
            return Err(format!("algorithm {:?} is not accepted", header.alg));
        }
        self.refresh_if_needed(header.kid.as_deref()).await;
        let key = {
            let state = self.state.lock().await;
            let keys = state
                .keys
                .as_ref()
                .ok_or("the provider's keys are not available")?;
            let jwk = match header.kid.as_deref() {
                Some(kid) => keys.find(kid),
                // No key id: only a set of exactly one key is unambiguous.
                None if keys.keys.len() == 1 => keys.keys.first(),
                None => None,
            }
            .ok_or("no key matches the token")?;
            if matches!(jwk.algorithm, AlgorithmParameters::OctetKey(_)) {
                return Err("a symmetric key is not accepted".to_string());
            }
            DecodingKey::from_jwk(jwk).map_err(|e| format!("unusable key: {e}"))?
        };
        let mut validation = Validation::new(header.alg);
        validation.set_issuer(&[&self.settings.issuer]);
        validation.set_audience(&self.settings.audiences);
        validation.set_required_spec_claims(&["exp", "iss", "aud", "sub"]);
        validation.validate_nbf = true;
        validation.leeway = LEEWAY_SECONDS;
        let data = jsonwebtoken::decode::<Value>(token, &key, &validation)
            .map_err(|e| format!("token rejected: {e}"))?;
        claims_from(&data.claims, &self.settings)
    }
}

/// The process-wide verifier, present when `PROJECT_PORTFOLIO_MANAGEMENT_OIDC_ISSUER` is set and valid.
static OIDC: OnceLock<OidcVerifier> = OnceLock::new();

/// The OIDC verifier, or `None` when it is not configured (or was configured badly).
#[must_use]
pub fn oidc_verifier() -> Option<&'static OidcVerifier> {
    OIDC.get()
}

/// Build the process-wide verifier from the environment and fetch its keys once. Unset: off. A bad setting logs
/// an error and leaves it off (its tokens are then refused); the service still boots.
pub async fn init_oidc() {
    if OIDC.get().is_some() {
        return;
    }
    match settings_from_lookup(|k| std::env::var(k).ok()) {
        Ok(None) => {}
        Ok(Some(settings)) => {
            let issuer = settings.issuer.clone();
            let verifier = OIDC.get_or_init(|| OidcVerifier::new(settings));
            verifier.refresh_if_needed(None).await;
            tracing::info!(%issuer, "auth: OpenID Connect access tokens accepted");
        }
        Err(error) => {
            tracing::error!(%error, "auth: PROJECT_PORTFOLIO_MANAGEMENT_OIDC_* is invalid; OIDC tokens will be refused");
        }
    }
}

/// The `iss` a JWT claims, read **without** verifying it: only to choose which verifier checks it.
#[must_use]
pub fn unverified_issuer(token: &str) -> Option<String> {
    let mut validation = Validation::new(Algorithm::ES256);
    validation.insecure_disable_signature_validation();
    validation.validate_exp = false;
    validation.validate_aud = false;
    validation.required_spec_claims.clear();
    validation.algorithms = ALGORITHMS.to_vec();
    let data =
        jsonwebtoken::decode::<Value>(token, &DecodingKey::from_secret(&[]), &validation).ok()?;
    data.claims
        .get("iss")
        .and_then(Value::as_str)
        .map(str::to_string)
}

#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::{EncodingKey, Header};
    use serde_json::json;

    /// A P-256 key made for these tests only.
    const TEST_KEY: &str = "-----BEGIN PRIVATE KEY-----
MIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQgm6upgKprEdospMx5
FUMtF2vxdlsV3cFGfnn+fNvCoP2hRANCAARifB4LKgA87OiIXxmUbZhJz7YuCcCm
Gc/gtIhTyl+CZJRNvIA6LLb3PfKHFbXDtr3foVOqWsbuncgcMXm+/vd7
-----END PRIVATE KEY-----
";
    const ISS: &str = "https://id.example.org/federation";

    fn keys() -> JwkSet {
        serde_json::from_value(json!({ "keys": [{
            "kty": "EC", "crv": "P-256", "kid": "k1", "use": "sig", "alg": "ES256",
            "x": "YnweCyoAPOzoiF8ZlG2YSc-2LgnAphnP4LSIU8pfgmQ",
            "y": "lE28gDostvc98ocVtcO2vd-hU6paxu6dyBwxeb7-93s",
        }]}))
        .unwrap()
    }

    fn settings() -> OidcSettings {
        settings_from_lookup(|k| match k {
            "PROJECT_PORTFOLIO_MANAGEMENT_OIDC_ISSUER" => Some(ISS.to_string()),
            "PROJECT_PORTFOLIO_MANAGEMENT_OIDC_AUDIENCES" => Some("ppm, other".to_string()),
            "PROJECT_PORTFOLIO_MANAGEMENT_OIDC_ROLES_CLAIM" => Some("groups".to_string()),
            "PROJECT_PORTFOLIO_MANAGEMENT_OIDC_ROLE_MAP" => {
                Some(r#"{"planners": {"access": ["write"]}}"#.to_string())
            }
            _ => None,
        })
        .unwrap()
        .unwrap()
    }

    fn sign(claims: &Value, kid: Option<&str>) -> String {
        let mut header = Header::new(Algorithm::ES256);
        header.kid = kid.map(str::to_string);
        jsonwebtoken::encode(
            &header,
            claims,
            &EncodingKey::from_ec_pem(TEST_KEY.as_bytes()).unwrap(),
        )
        .unwrap()
    }

    fn now() -> i64 {
        chrono::Utc::now().timestamp()
    }

    fn good() -> Value {
        json!({
            "iss": ISS, "aud": "ppm", "sub": "a1b2", "exp": now() + 300, "iat": now(),
            "email": "ana@example.org", "email_verified": true, "name": "Ana",
            "groups": ["planners", "unmapped"],
        })
    }

    #[test]
    fn settings_are_off_unless_an_issuer_is_set_and_fail_closed_when_half_set() {
        assert_eq!(settings_from_lookup(|_| None), Ok(None));
        let only_issuer =
            |k: &str| (k == "PROJECT_PORTFOLIO_MANAGEMENT_OIDC_ISSUER").then(|| ISS.to_string());
        assert!(
            settings_from_lookup(only_issuer)
                .unwrap_err()
                .contains("AUDIENCES")
        );
        let plain_http = |k: &str| match k {
            "PROJECT_PORTFOLIO_MANAGEMENT_OIDC_ISSUER" => Some("http://id.example.org".to_string()),
            "PROJECT_PORTFOLIO_MANAGEMENT_OIDC_AUDIENCES" => Some("ppm".to_string()),
            _ => None,
        };
        assert!(
            settings_from_lookup(plain_http)
                .unwrap_err()
                .contains("https")
        );
        let s = settings();
        assert_eq!(s.audiences, ["ppm", "other"]);
        assert!(s.require_verified_email);
        assert!(secure_url("http://localhost:8080/realms/x"));
        assert!(!secure_url("http://localhost.evil.example"));
    }

    #[tokio::test]
    async fn a_token_from_any_provider_verifies_and_maps_only_listed_roles() {
        let v = OidcVerifier::with_keys(settings(), keys());
        let claims = v.verify(&sign(&good(), Some("k1"))).await.unwrap();
        assert_eq!(claims.sub, "a1b2");
        assert_eq!(claims.name, "Ana");
        assert_eq!(claims.aud, "ppm");
        assert_eq!(claims.attrs.get("access"), Some(&vec!["write".to_string()]));
        assert_eq!(claims.attrs.len(), 1, "an unmapped group grants nothing");
        // With one key in the set, a token without a key id is unambiguous.
        assert!(v.verify(&sign(&good(), None)).await.is_ok());
        assert_eq!(
            unverified_issuer(&sign(&good(), None)).as_deref(),
            Some(ISS)
        );
    }

    #[tokio::test]
    async fn every_fail_closed_rule_holds() {
        let v = OidcVerifier::with_keys(settings(), keys());
        let with = |k: &str, val: Value| {
            let mut c = good();
            c[k] = val;
            c
        };
        for (why, claims) in [
            ("issuer", with("iss", json!("https://other.example.org"))),
            ("audience", with("aud", json!("someone-else"))),
            ("expired", with("exp", json!(now() - 3600))),
            ("not yet", with("nbf", json!(now() + 3600))),
            ("unverified email", with("email_verified", json!(false))),
            ("no subject", with("sub", json!(""))),
        ] {
            assert!(v.verify(&sign(&claims, Some("k1"))).await.is_err(), "{why}");
        }
        assert!(
            v.verify(&sign(&good(), Some("k2"))).await.is_err(),
            "unknown kid"
        );
        // An HS256 token signed with anything, even the public key's bytes, is refused before any key is used.
        let hs = jsonwebtoken::encode(
            &Header::new(Algorithm::HS256),
            &good(),
            &EncodingKey::from_secret(b"YnweCyoAPOzoiF8ZlG2YSc-2LgnAphnP4LSIU8pfgmQ"),
        )
        .unwrap();
        assert!(v.verify(&hs).await.unwrap_err().contains("not accepted"));
        // A tampered payload breaks the signature.
        let token = sign(&good(), Some("k1"));
        let mut parts: Vec<&str> = token.split('.').collect();
        let forged = sign(&with("sub", json!("admin")), Some("k1"));
        let forged_payload = forged.split('.').nth(1).unwrap().to_string();
        parts[1] = &forged_payload;
        assert!(v.verify(&parts.join(".")).await.is_err());
        assert!(v.verify("not-a-token").await.is_err());
    }

    #[test]
    fn roles_come_from_a_named_claim_or_a_dotted_path() {
        let c = json!({ "realm_access": { "roles": ["a", "b"] }, "scope": "x y" });
        assert_eq!(roles_at(&c, "realm_access.roles"), ["a", "b"]);
        assert_eq!(roles_at(&c, "scope"), ["x", "y"]);
        assert!(roles_at(&c, "missing.path").is_empty());
    }
}
