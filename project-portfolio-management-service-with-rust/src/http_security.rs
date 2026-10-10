//! Browser-facing HTTP hardening for every response: security headers and
//! an explicit CORS allow-list. Both are `axum` middleware layered in
//! `App::after_routes`, outside the auth guard so a `401`/`403`/`429` carries
//! them too.
//!
//! ## Security headers
//!
//! The API returns JSON and downloads, never pages, so the baseline is the
//! strictest one: `Content-Security-Policy: default-src 'none';
//! frame-ancestors 'none'`, `X-Content-Type-Options: nosniff`,
//! `X-Frame-Options: DENY`, `Referrer-Policy: strict-origin-when-cross-origin`,
//! a `Permissions-Policy` denying the powerful features, and
//! `Cross-Origin-Opener-Policy` / `Cross-Origin-Resource-Policy:
//! same-origin`. `/swagger-ui` is the one HTML page and loads its bundle from
//! a CDN, so it gets a relaxed CSP ([`SWAGGER_CSP`]) instead; it keeps the
//! other headers. A header a handler already set (a file download's
//! `Cache-Control`, `Content-Disposition`, `nosniff`) is never overwritten.
//!
//! `Strict-Transport-Security` is sent **only** when the request arrived over
//! TLS: `PROJECT_PORTFOLIO_MANAGEMENT_HSTS=proxy` (the default) believes
//! `X-Forwarded-Proto: https` from a peer in
//! `PROJECT_PORTFOLIO_MANAGEMENT_TRUSTED_PROXIES` only; `always` is the switch
//! for a deployment that terminates TLS in front of every request; `off`
//! never sends it. Plain HTTP never gets it.
//!
//! ## CORS
//!
//! `PROJECT_PORTFOLIO_MANAGEMENT_CORS_ORIGINS` is a comma-separated list of
//! exact origins (`https://app.example.com`). Unset or empty means no CORS
//! headers at all (same-origin only); that is the production default. A `*`
//! entry is refused (credentials are allowed for listed origins, and a
//! wildcard must never pair with credentials). A listed origin gets the grant
//! and its preflight (`OPTIONS` with `Access-Control-Request-Method`) is
//! answered `204` without reaching auth; any other origin gets no CORS header.

use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::{HeaderMap, HeaderName, HeaderValue, Method, StatusCode, header},
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::trusted_proxy::{TrustedProxies, peer_ip};

/// CSP for every response: nothing loads, nothing frames it.
pub const API_CSP: &str = "default-src 'none'; frame-ancestors 'none'";

/// CSP for `/swagger-ui`, which loads `swagger-ui-dist` from unpkg and runs
/// one inline bootstrap script; it may fetch only this origin's spec.
pub const SWAGGER_CSP: &str = "default-src 'none'; script-src 'self' 'unsafe-inline' https://unpkg.com; style-src 'self' 'unsafe-inline' https://unpkg.com; img-src 'self' data: https:; font-src 'self' https://unpkg.com data:; connect-src 'self'; frame-ancestors 'none'";

const HSTS_VALUE: &str = "max-age=31536000; includeSubDomains";
const PERMISSIONS_POLICY: &str = "accelerometer=(), camera=(), geolocation=(), gyroscope=(), magnetometer=(), microphone=(), payment=(), usb=()";

/// When `Strict-Transport-Security` is sent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HstsMode {
    /// Only for requests a trusted proxy says arrived over HTTPS.
    #[default]
    Proxy,
    /// On every response (TLS is terminated in front of all traffic).
    Always,
    /// Never.
    Off,
}

impl HstsMode {
    /// `proxy` (also the fallback for blank/unknown), `always`, `off`.
    #[must_use]
    pub fn parse(value: &str) -> Self {
        match value.trim().to_ascii_lowercase().as_str() {
            "always" | "on" | "true" => Self::Always,
            "off" | "never" | "false" | "none" => Self::Off,
            _ => Self::Proxy,
        }
    }
}

/// Settings for [`security_headers_mw`].
#[derive(Debug, Clone, Default)]
pub struct SecurityConfig {
    /// When HSTS is sent.
    pub hsts: HstsMode,
    /// Peers whose `X-Forwarded-Proto` is believed.
    pub proxies: TrustedProxies,
}

impl SecurityConfig {
    /// From `PROJECT_PORTFOLIO_MANAGEMENT_HSTS` and
    /// `PROJECT_PORTFOLIO_MANAGEMENT_TRUSTED_PROXIES`.
    #[must_use]
    pub fn from_env() -> Self {
        Self {
            hsts: HstsMode::parse(
                &std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_HSTS").unwrap_or_default(),
            ),
            proxies: TrustedProxies::from_env(),
        }
    }
}

/// Set `name` only when the handler did not.
fn set_default(headers: &mut HeaderMap, name: HeaderName, value: &'static str) {
    headers
        .entry(name)
        .or_insert_with(|| HeaderValue::from_static(value));
}

/// Add the security headers to every response (see the module docs).
pub async fn security_headers_mw(
    State(config): State<Arc<SecurityConfig>>,
    req: Request,
    next: Next,
) -> Response {
    let swagger = req.uri().path().starts_with("/swagger-ui");
    let tls = match config.hsts {
        HstsMode::Always => true,
        HstsMode::Off => false,
        HstsMode::Proxy => config
            .proxies
            .is_https(peer_ip(req.extensions()), req.headers()),
    };
    let mut response = next.run(req).await;
    let headers = response.headers_mut();
    set_default(headers, header::X_CONTENT_TYPE_OPTIONS, "nosniff");
    set_default(
        headers,
        header::CONTENT_SECURITY_POLICY,
        if swagger { SWAGGER_CSP } else { API_CSP },
    );
    set_default(headers, header::X_FRAME_OPTIONS, "DENY");
    set_default(
        headers,
        header::REFERRER_POLICY,
        "strict-origin-when-cross-origin",
    );
    set_default(
        headers,
        HeaderName::from_static("permissions-policy"),
        PERMISSIONS_POLICY,
    );
    set_default(
        headers,
        HeaderName::from_static("cross-origin-opener-policy"),
        "same-origin",
    );
    set_default(
        headers,
        HeaderName::from_static("cross-origin-resource-policy"),
        "same-origin",
    );
    if tls {
        set_default(headers, header::STRICT_TRANSPORT_SECURITY, HSTS_VALUE);
    }
    response
}

/// The CORS allow-list.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CorsConfig {
    origins: Vec<String>,
}

impl CorsConfig {
    /// Parse a comma-separated list of exact origins. Entries are trimmed,
    /// lower-cased and stripped of a trailing `/`; a `*` or anything that is
    /// not `http(s)://host[:port]` is dropped and returned in the second
    /// element so the caller can warn.
    #[must_use]
    pub fn parse(list: &str) -> (Self, Vec<String>) {
        let mut origins = Vec::new();
        let mut rejected = Vec::new();
        for entry in list.split(',').map(str::trim).filter(|e| !e.is_empty()) {
            let normalised = entry.trim_end_matches('/').to_ascii_lowercase();
            let valid = (normalised.starts_with("https://") || normalised.starts_with("http://"))
                && !normalised.contains('*')
                && !normalised
                    .split_once("://")
                    .is_some_and(|(_, rest)| rest.is_empty() || rest.contains(['/', '?', '#']));
            if valid {
                if !origins.contains(&normalised) {
                    origins.push(normalised);
                }
            } else {
                rejected.push(entry.to_string());
            }
        }
        (Self { origins }, rejected)
    }

    /// From `PROJECT_PORTFOLIO_MANAGEMENT_CORS_ORIGINS`; warns about rejects.
    #[must_use]
    pub fn from_env() -> Self {
        let raw = std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_CORS_ORIGINS").unwrap_or_default();
        let (config, rejected) = Self::parse(&raw);
        if !rejected.is_empty() {
            tracing::warn!(
                ?rejected,
                "PROJECT_PORTFOLIO_MANAGEMENT_CORS_ORIGINS: entries ignored (only exact http(s) origins are allowed; no wildcard)"
            );
        }
        config
    }

    /// Whether any origin is allowed at all.
    #[must_use]
    pub fn is_enabled(&self) -> bool {
        !self.origins.is_empty()
    }

    /// Whether `origin` is exactly one of the listed origins.
    #[must_use]
    pub fn allows(&self, origin: &str) -> bool {
        let origin = origin.to_ascii_lowercase();
        self.origins.contains(&origin)
    }
}

const ALLOWED_METHODS: &str = "GET, HEAD, POST, PUT, PATCH, DELETE, OPTIONS";
const ALLOWED_HEADERS: &str = "Authorization, Content-Type, If-Match, If-None-Match, Accepts-version, Idempotency-Key, X-Request-Id";
const EXPOSED_HEADERS: &str = "ETag, Content-Disposition, Retry-After, RateLimit-Limit, RateLimit-Remaining, RateLimit-Reset, X-Total-Count, X-Limit, X-Offset, Accepts-version";

fn append_vary(headers: &mut HeaderMap, value: &'static str) {
    headers.append(header::VARY, HeaderValue::from_static(value));
}

/// Apply the CORS allow-list (see the module docs). A no-op when no origin
/// is configured.
pub async fn cors_mw(State(config): State<Arc<CorsConfig>>, req: Request, next: Next) -> Response {
    if !config.is_enabled() {
        return next.run(req).await;
    }
    let origin = req
        .headers()
        .get(header::ORIGIN)
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned);
    let Some(origin) = origin.filter(|o| config.allows(o)) else {
        let mut response = next.run(req).await;
        append_vary(response.headers_mut(), "Origin");
        return response;
    };
    let Ok(origin_value) = HeaderValue::from_str(&origin) else {
        return next.run(req).await;
    };
    let preflight = req.method() == Method::OPTIONS
        && req
            .headers()
            .contains_key(header::ACCESS_CONTROL_REQUEST_METHOD);
    let mut response = if preflight {
        let mut r = StatusCode::NO_CONTENT.into_response();
        let h = r.headers_mut();
        h.insert(
            header::ACCESS_CONTROL_ALLOW_METHODS,
            HeaderValue::from_static(ALLOWED_METHODS),
        );
        h.insert(
            header::ACCESS_CONTROL_ALLOW_HEADERS,
            HeaderValue::from_static(ALLOWED_HEADERS),
        );
        h.insert(
            header::ACCESS_CONTROL_MAX_AGE,
            HeaderValue::from_static("600"),
        );
        append_vary(h, "Access-Control-Request-Method");
        append_vary(h, "Access-Control-Request-Headers");
        r
    } else {
        let mut r = next.run(req).await;
        r.headers_mut().insert(
            header::ACCESS_CONTROL_EXPOSE_HEADERS,
            HeaderValue::from_static(EXPOSED_HEADERS),
        );
        r
    };
    let h = response.headers_mut();
    h.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, origin_value);
    h.insert(
        header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
        HeaderValue::from_static("true"),
    );
    append_vary(h, "Origin");
    response
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{Router, body::Body, extract::ConnectInfo, routing::get};
    use std::net::SocketAddr;
    use tower::ServiceExt;

    fn app(security: SecurityConfig, cors: CorsConfig) -> Router {
        Router::new()
            .route("/api/x", get(|| async { "ok" }))
            .route("/swagger-ui", get(|| async { "<html>" }))
            .route(
                "/download",
                get(|| async {
                    (
                        [
                            (header::CACHE_CONTROL, "private, max-age=60"),
                            (header::CONTENT_SECURITY_POLICY, "sandbox"),
                        ],
                        "file",
                    )
                }),
            )
            .layer(axum::middleware::from_fn_with_state(
                Arc::new(cors),
                cors_mw,
            ))
            .layer(axum::middleware::from_fn_with_state(
                Arc::new(security),
                security_headers_mw,
            ))
    }

    fn get_req(path: &str, peer: Option<&str>) -> Request {
        let mut req = Request::builder().uri(path).body(Body::empty()).unwrap();
        if let Some(peer) = peer {
            let addr: SocketAddr = format!("{peer}:1234").parse().unwrap();
            req.extensions_mut().insert(ConnectInfo(addr));
        }
        req
    }

    #[tokio::test]
    async fn every_response_carries_the_baseline_headers() {
        let res = app(SecurityConfig::default(), CorsConfig::default())
            .oneshot(get_req("/api/x", None))
            .await
            .unwrap();
        let h = res.headers();
        assert_eq!(h["x-content-type-options"], "nosniff");
        assert_eq!(h["content-security-policy"], API_CSP);
        assert_eq!(h["x-frame-options"], "DENY");
        assert_eq!(h["referrer-policy"], "strict-origin-when-cross-origin");
        assert!(
            h["permissions-policy"]
                .to_str()
                .unwrap()
                .contains("camera=()")
        );
        assert_eq!(h["cross-origin-opener-policy"], "same-origin");
        assert_eq!(h["cross-origin-resource-policy"], "same-origin");
        assert!(
            !h.contains_key("strict-transport-security"),
            "never over plain HTTP"
        );
        assert!(!h.contains_key("access-control-allow-origin"));
    }

    #[tokio::test]
    async fn swagger_gets_a_relaxed_csp_and_handlers_headers_survive() {
        let a = app(SecurityConfig::default(), CorsConfig::default());
        let swagger = a
            .clone()
            .oneshot(get_req("/swagger-ui", None))
            .await
            .unwrap();
        assert_eq!(swagger.headers()["content-security-policy"], SWAGGER_CSP);
        assert_eq!(swagger.headers()["x-frame-options"], "DENY");
        let dl = a.oneshot(get_req("/download", None)).await.unwrap();
        assert_eq!(dl.headers()["cache-control"], "private, max-age=60");
        assert_eq!(dl.headers()["content-security-policy"], "sandbox");
        assert_eq!(dl.headers()["x-content-type-options"], "nosniff");
    }

    #[tokio::test]
    async fn hsts_only_over_tls() {
        let proxies = TrustedProxies::parse("10.0.0.1").unwrap();
        let cfg = SecurityConfig {
            hsts: HstsMode::Proxy,
            proxies,
        };
        let a = app(cfg, CorsConfig::default());
        let mk = |peer: &str| {
            let mut r = get_req("/api/x", Some(peer));
            r.headers_mut()
                .insert("x-forwarded-proto", "https".parse().unwrap());
            r
        };
        let via_proxy = a.clone().oneshot(mk("10.0.0.1")).await.unwrap();
        assert_eq!(via_proxy.headers()["strict-transport-security"], HSTS_VALUE);
        // A direct client claiming https is not believed.
        let spoof = a.clone().oneshot(mk("203.0.113.9")).await.unwrap();
        assert!(!spoof.headers().contains_key("strict-transport-security"));
        // A trusted proxy that forwarded plain HTTP.
        let plain = a
            .oneshot(get_req("/api/x", Some("10.0.0.1")))
            .await
            .unwrap();
        assert!(!plain.headers().contains_key("strict-transport-security"));
        // The explicit switch.
        let always = app(
            SecurityConfig {
                hsts: HstsMode::Always,
                proxies: TrustedProxies::none(),
            },
            CorsConfig::default(),
        )
        .oneshot(get_req("/api/x", None))
        .await
        .unwrap();
        assert!(always.headers().contains_key("strict-transport-security"));
        let off = app(
            SecurityConfig {
                hsts: HstsMode::Off,
                proxies: TrustedProxies::parse("10.0.0.1").unwrap(),
            },
            CorsConfig::default(),
        );
        let mut r = get_req("/api/x", Some("10.0.0.1"));
        r.headers_mut()
            .insert("x-forwarded-proto", "https".parse().unwrap());
        assert!(
            !off.oneshot(r)
                .await
                .unwrap()
                .headers()
                .contains_key("strict-transport-security")
        );
    }

    #[test]
    fn hsts_mode_parses() {
        assert_eq!(HstsMode::parse(""), HstsMode::Proxy);
        assert_eq!(HstsMode::parse("Always"), HstsMode::Always);
        assert_eq!(HstsMode::parse("off"), HstsMode::Off);
        assert_eq!(HstsMode::parse("nonsense"), HstsMode::Proxy);
    }

    #[test]
    fn cors_list_is_exact_and_refuses_wildcards() {
        let (c, rejected) = CorsConfig::parse(
            "https://App.example.com/, http://localhost:5173,*,https://*.example.com,example.com,https://x.com/path",
        );
        assert!(c.allows("https://app.example.com"));
        assert!(c.allows("HTTP://LOCALHOST:5173"));
        assert!(!c.allows("https://evil.example.com"));
        assert!(!c.allows("https://app.example.com.evil.net"));
        assert_eq!(rejected.len(), 4, "{rejected:?}");
        assert!(!CorsConfig::parse("").0.is_enabled());
    }

    fn cors_app() -> Router {
        app(
            SecurityConfig::default(),
            CorsConfig::parse("https://app.example.com").0,
        )
    }

    fn with_origin(mut req: Request, origin: &str) -> Request {
        req.headers_mut()
            .insert(header::ORIGIN, origin.parse().unwrap());
        req
    }

    #[tokio::test]
    async fn allowed_origin_gets_the_grant() {
        let res = cors_app()
            .oneshot(with_origin(
                get_req("/api/x", None),
                "https://app.example.com",
            ))
            .await
            .unwrap();
        let h = res.headers();
        assert_eq!(h["access-control-allow-origin"], "https://app.example.com");
        assert_eq!(h["access-control-allow-credentials"], "true");
        assert!(
            h["access-control-expose-headers"]
                .to_str()
                .unwrap()
                .contains("ETag")
        );
        assert!(h.get_all("vary").iter().any(|v| v == "Origin"));
    }

    #[tokio::test]
    async fn allowed_preflight_is_answered_without_the_handler() {
        let mut req = with_origin(get_req("/api/x", None), "https://app.example.com");
        *req.method_mut() = Method::OPTIONS;
        req.headers_mut().insert(
            header::ACCESS_CONTROL_REQUEST_METHOD,
            "PUT".parse().unwrap(),
        );
        let res = cors_app().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::NO_CONTENT);
        let h = res.headers();
        assert_eq!(h["access-control-allow-origin"], "https://app.example.com");
        assert!(
            h["access-control-allow-methods"]
                .to_str()
                .unwrap()
                .contains("PUT")
        );
        assert!(
            h["access-control-allow-headers"]
                .to_str()
                .unwrap()
                .contains("Authorization")
        );
        assert_eq!(h["access-control-max-age"], "600");
        assert_eq!(
            h["x-frame-options"], "DENY",
            "security headers wrap the preflight"
        );
    }

    #[tokio::test]
    async fn disallowed_origin_gets_no_cors_headers() {
        let res = cors_app()
            .oneshot(with_origin(
                get_req("/api/x", None),
                "https://evil.example.net",
            ))
            .await
            .unwrap();
        assert!(!res.headers().contains_key("access-control-allow-origin"));
        assert!(
            !res.headers()
                .contains_key("access-control-allow-credentials")
        );
        let mut pre = with_origin(get_req("/api/x", None), "https://evil.example.net");
        *pre.method_mut() = Method::OPTIONS;
        pre.headers_mut().insert(
            header::ACCESS_CONTROL_REQUEST_METHOD,
            "GET".parse().unwrap(),
        );
        let res = cors_app().oneshot(pre).await.unwrap();
        assert!(!res.headers().contains_key("access-control-allow-origin"));
    }

    #[tokio::test]
    async fn unset_list_adds_no_cors_headers_even_for_an_origin() {
        let res = app(SecurityConfig::default(), CorsConfig::default())
            .oneshot(with_origin(
                get_req("/api/x", None),
                "https://app.example.com",
            ))
            .await
            .unwrap();
        assert!(!res.headers().contains_key("access-control-allow-origin"));
    }
}
