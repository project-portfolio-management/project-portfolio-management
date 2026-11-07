//! In-process request rate limiting (HD-5), counted by `axa-ratelimit`: fixed windows, no external store.
//!
//! Three independent limits, each keyed separately:
//!
//! - **general** — every request, keyed by the caller's credential (the bearer token, held only as `axa-ratelimit`'s
//!   keyed hash, so no extra verification) else the client address;
//! - **expensive** — the routes that do real work per call (simulations, CPM,
//!   levelling, forecasts, imports, exports, bulk jobs, PDF), a tighter limit
//!   charged on top of the general one;
//! - **failures** — authentication failures (`401`) per client address, so a
//!   password-guessing or token-guessing client is slowed before the auth
//!   guard does any work.
//!
//! `/_health`, `/_ping` and `/_readiness` are exempt. A refused request is `429` with
//! `Retry-After` and the `RateLimit-Limit` / `RateLimit-Remaining` /
//! `RateLimit-Reset` headers (whole seconds); an allowed one carries the
//! `RateLimit-*` headers of the tightest limit it was charged to.
//!
//! Each limit allows its *burst* in one window, and the window is as long as the sustained rate takes to earn that
//! burst: 10 at 30 a minute is 10 per 20 seconds. As with any fixed window, a client can send up to twice the burst
//! across the end of one.
//!
//! The client address is the socket peer unless that peer is listed in
//! `PROJECT_PORTFOLIO_MANAGEMENT_TRUSTED_PROXIES`, in which case the
//! rightmost untrusted `X-Forwarded-For` hop is used (see
//! [`crate::trusted_proxy`]). A spoofed header from any other peer is
//! ignored. An IPv6 address counts by its `/64`, which is what one subscriber is usually given.
//!
//! Memory is bounded by `axa-ratelimit`: a key is dropped two windows after its last request, and past
//! [`axa::ratelimit::MAX_KEYS`] new keys share one count. State is per process; with several replicas each enforces
//! its own share (a shared store is a later step, `plan.md` §47).
//!
//! ## Configuration
//!
//! | Variable | Default | Meaning |
//! |---|---|---|
//! | `PROJECT_PORTFOLIO_MANAGEMENT_RATE_LIMIT` | on | `off`/`false`/`0` disables (also off in the `development`, `test` and `container-dev` profiles) |
//! | `…_RATE_LIMIT_GENERAL_PER_MINUTE` / `…_GENERAL_BURST` | 600 / 200 | sustained requests per minute, and the burst allowed |
//! | `…_RATE_LIMIT_EXPENSIVE_PER_MINUTE` / `…_EXPENSIVE_BURST` | 30 / 10 | the same for expensive routes |
//! | `…_RATE_LIMIT_FAILURES_PER_MINUTE` / `…_FAILURES_BURST` | 6 / 20 | authentication failures per client address |

use std::{
    net::{IpAddr, Ipv6Addr},
    num::NonZeroU32,
    sync::Arc,
};

use axa::ratelimit::{Limiter, Verdict};
use axum::{
    extract::{Request, State},
    http::{HeaderValue, StatusCode, header},
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::trusted_proxy::{TrustedProxies, peer_ip};

/// Path segments that mark a route as expensive.
const EXPENSIVE_SEGMENTS: &[&str] = &[
    "cost-risk",
    "joint-risk",
    "schedule-risk",
    "cpm",
    "leveling",
    "forecast",
    "export",
    "import",
    "bulk-jobs",
    "evidence-pack",
    "compare",
    "generate",
    "evaluate",
    "dependency-risk",
];

/// One limit's shape: the burst allowed in a window, and the sustained rate that sets the window's length.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limit {
    /// The most requests in one window.
    pub burst: NonZeroU32,
    /// The window, in whole seconds: as long as `per_minute` takes to earn `burst`, rounded up.
    pub window_seconds: NonZeroU32,
}

impl Limit {
    /// A limit of `per_minute` sustained requests with a `burst` allowance. A `0` counts as `1`.
    #[must_use]
    pub fn per_minute(per_minute: u32, burst: u32) -> Self {
        let burst = NonZeroU32::new(burst).unwrap_or(NonZeroU32::MIN);
        let per_minute = per_minute.max(1);
        let seconds = (u64::from(burst.get()) * 60).div_ceil(u64::from(per_minute));
        Self {
            burst,
            window_seconds: u32::try_from(seconds)
                .ok()
                .and_then(NonZeroU32::new)
                .unwrap_or(NonZeroU32::MAX),
        }
    }

    /// A fresh limiter of this shape.
    fn limiter(self) -> Limiter {
        Limiter::new(self.burst, self.window_seconds)
    }
}

/// What a request is counted against for the general and expensive limits.
#[derive(Hash)]
enum Key<'a> {
    /// The bearer token. `axa-ratelimit` keeps only a keyed hash of it.
    Credential(&'a str),
    /// The client address, or `None` when there is none.
    Client(Option<IpAddr>),
}

/// The client address a request is counted against, with an IPv6 address reduced to its `/64`.
fn client(proxies: &TrustedProxies, req: &Request) -> Option<IpAddr> {
    let address = proxies
        .client_ip(peer_ip(req.extensions()), req.headers())?
        .to_canonical();
    Some(match address {
        IpAddr::V4(_) => address,
        IpAddr::V6(v6) => IpAddr::V6(Ipv6Addr::from_bits(v6.to_bits() & (!0u128 << 64))),
    })
}

/// Whether `path` is one of the routes that does real work per call.
#[must_use]
pub fn is_expensive(path: &str) -> bool {
    std::path::Path::new(path)
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("pdf"))
        || path
            .split('/')
            .any(|segment| EXPENSIVE_SEGMENTS.contains(&segment))
}

/// Whether `path` is never limited.
#[must_use]
pub fn is_exempt(path: &str) -> bool {
    axa::ratelimit::EXEMPT.contains(&path)
}

/// The rate limiter's settings and state.
#[derive(Debug)]
pub struct RateLimiter {
    /// Master switch; when false every request passes unchecked.
    enabled: bool,
    /// Peers trusted to supply the forwarded client address.
    proxies: TrustedProxies,
    /// Counts for the general limit, per credential or client.
    general: Limiter,
    /// Counts for the expensive-route limit, per credential or client.
    expensive: Limiter,
    /// Counts of authentication failures, per client address.
    failures: Limiter,
}

/// Read an unsigned integer from the environment variable `name`, falling back to `default` when it is unset or not a number.
fn env_u32(name: &str, default: u32) -> u32 {
    std::env::var(name)
        .ok()
        .and_then(|v| v.trim().parse().ok())
        .filter(|n| *n > 0)
        .unwrap_or(default)
}

impl RateLimiter {
    /// Build from the environment; `configured` is the
    /// profile's value (the environment wins when set).
    #[must_use]
    pub fn from_env(configured: Option<bool>) -> Self {
        let env = std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_RATE_LIMIT").ok();
        let enabled = match env.as_deref().map(str::trim).filter(|v| !v.is_empty()) {
            Some(v) => !matches!(
                v.to_ascii_lowercase().as_str(),
                "0" | "false" | "no" | "off"
            ),
            None => configured.unwrap_or(true),
        };
        let pm = "PROJECT_PORTFOLIO_MANAGEMENT_RATE_LIMIT";
        Self::new(
            enabled,
            Limit::per_minute(
                env_u32(&format!("{pm}_GENERAL_PER_MINUTE"), 600),
                env_u32(&format!("{pm}_GENERAL_BURST"), 200),
            ),
            Limit::per_minute(
                env_u32(&format!("{pm}_EXPENSIVE_PER_MINUTE"), 30),
                env_u32(&format!("{pm}_EXPENSIVE_BURST"), 10),
            ),
            Limit::per_minute(
                env_u32(&format!("{pm}_FAILURES_PER_MINUTE"), 6),
                env_u32(&format!("{pm}_FAILURES_BURST"), 20),
            ),
            TrustedProxies::from_env(),
        )
    }

    /// Build with explicit settings (tests, and [`Self::from_env`]).
    #[must_use]
    pub fn new(
        enabled: bool,
        general: Limit,
        expensive: Limit,
        failures: Limit,
        proxies: TrustedProxies,
    ) -> Self {
        Self {
            enabled,
            proxies,
            general: general.limiter(),
            expensive: expensive.limiter(),
            failures: failures.limiter(),
        }
    }

    /// Whether limiting is on.
    #[must_use]
    pub fn enabled(&self) -> bool {
        self.enabled
    }
}

/// The `429` problem for a refused request: `axa-ratelimit`'s headers, with `detail` naming the limit that was hit.
fn too_many(verdict: &Verdict, what: &str) -> Response {
    let mut response = crate::controllers::problem(
        StatusCode::TOO_MANY_REQUESTS,
        "too_many_requests",
        format!("{what}; retry after {} seconds", verdict.reset_seconds),
    )
    .into_response();
    verdict.stamp(&mut response);
    response.headers_mut().insert(
        header::RETRY_AFTER,
        HeaderValue::from(verdict.reset_seconds),
    );
    response
}

/// The rate-limiting middleware (see the module docs). Layer it outside the
/// auth guard and inside the security-headers layer, so a `429` carries the
/// security headers.
pub async fn rate_limit_mw(
    State(limiter): State<Arc<RateLimiter>>,
    req: Request,
    next: Next,
) -> Response {
    if !limiter.enabled || is_exempt(req.uri().path()) {
        return next.run(req).await;
    }
    let client = client(&limiter.proxies, &req);

    // Peek, so a client under the failure limit is not charged for asking.
    let failures = limiter.failures.peek(client);
    if !failures.allowed {
        return too_many(&failures, "too many failed authentication attempts");
    }

    let credential = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| {
            v.strip_prefix("Bearer ")
                .or_else(|| v.strip_prefix("bearer "))
        })
        .map(str::trim)
        .filter(|t| !t.is_empty());
    let (mut tightest, expensive) = {
        let key = credential.map_or(Key::Client(client), Key::Credential);
        let general = limiter.general.check(&key);
        let expensive = (general.allowed && is_expensive(req.uri().path()))
            .then(|| limiter.expensive.check(&key));
        (general, expensive)
    };
    if !tightest.allowed {
        return too_many(&tightest, "rate limit exceeded");
    }
    if let Some(verdict) = expensive {
        if !verdict.allowed {
            return too_many(&verdict, "rate limit exceeded for this kind of request");
        }
        if verdict.remaining < tightest.remaining {
            tightest = verdict;
        }
    }

    let mut response = next.run(req).await;
    if response.status() == StatusCode::UNAUTHORIZED {
        limiter.failures.check(client);
    }
    tightest.stamp(&mut response);
    response
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        Router, body::Body, extract::ConnectInfo, http::Request as HttpRequest, routing::get,
    };
    use std::net::SocketAddr;
    use tower::ServiceExt;

    fn limit(burst: u32, per_minute: u32) -> Limit {
        Limit::per_minute(per_minute, burst)
    }

    #[test]
    fn a_window_is_as_long_as_the_rate_takes_to_earn_the_burst() {
        let window = |per_minute, burst| Limit::per_minute(per_minute, burst).window_seconds.get();
        // The defaults: general 200 per 20 s, expensive 10 per 20 s, failures 20 per 200 s.
        assert_eq!(window(600, 200), 20);
        assert_eq!(window(30, 10), 20);
        assert_eq!(window(6, 20), 200);
        // Rounded up, never to zero, and a zero setting counts as one.
        assert_eq!(window(7, 1), 9);
        assert_eq!(window(6000, 1), 1);
        assert_eq!(Limit::per_minute(0, 0), Limit::per_minute(1, 1));
        assert_eq!(window(1, 1), 60);
    }

    #[test]
    fn expensive_routes_are_recognized_by_segment() {
        for path in [
            "/api/plans/x/cpm",
            "/api/plans/x/schedule-risk",
            "/api/plans/x/cost-risk",
            "/api/plans/x/joint-risk",
            "/api/plans/x/leveling",
            "/api/plans/x/leveling/apply",
            "/api/plans/export",
            "/api/plans/export/7/download",
            "/api/plans/x/import/mspdi",
            "/api/plans/bulk-jobs",
            "/api/board/pack.pdf",
            "/api/plans/x/summary.pdf",
            "/api/scenarios/compare",
            "/api/auditor/evidence-pack",
            "/api/plans/x/earned-value/forecast",
        ] {
            assert!(is_expensive(path), "{path}");
        }
        for path in [
            "/api/plans",
            "/api/plans/x/risks",
            "/api/plans/x/risks/y/escalate",
            "/api/risk/heatmap",
            "/api/plans/x/tasks",
        ] {
            assert!(!is_expensive(path), "{path}");
        }
    }

    #[test]
    fn health_and_ping_are_exempt() {
        assert!(is_exempt("/_health"));
        assert!(is_exempt("/_ping"));
        assert!(is_exempt("/_readiness"));
        assert!(!is_exempt("/api/plans"));
    }

    #[test]
    fn an_ipv6_client_counts_by_its_slash_64() {
        let at = |peer: &str| {
            let mut req = Request::new(Body::empty());
            let addr: SocketAddr = format!("[{peer}]:4000").parse().unwrap();
            req.extensions_mut().insert(ConnectInfo(addr));
            client(&TrustedProxies::none(), &req)
        };
        assert_eq!(at("2001:db8:1:2::1"), at("2001:db8:1:2:ffff::9"));
        assert_ne!(at("2001:db8:1:2::1"), at("2001:db8:1:3::1"));
        // An IPv4 address seen over IPv6 is the IPv4 address.
        assert_eq!(at("::ffff:10.0.0.1"), Some("10.0.0.1".parse().unwrap()));
    }

    fn app(limiter: RateLimiter) -> Router {
        Router::new()
            .route("/api/x", get(|| async { "ok" }))
            .route("/api/plans/p/cpm", get(|| async { "ok" }))
            .route("/_health", get(|| async { "ok" }))
            .route(
                "/api/private",
                get(|| async { (StatusCode::UNAUTHORIZED, "no") }),
            )
            .layer(axum::middleware::from_fn_with_state(
                Arc::new(limiter),
                rate_limit_mw,
            ))
    }

    fn request(
        path: &str,
        peer: &str,
        bearer: Option<&str>,
        xff: Option<&str>,
    ) -> HttpRequest<Body> {
        let mut builder = HttpRequest::builder().uri(path);
        if let Some(token) = bearer {
            builder = builder.header(header::AUTHORIZATION, format!("Bearer {token}"));
        }
        if let Some(value) = xff {
            builder = builder.header("x-forwarded-for", value);
        }
        let mut req = builder.body(Body::empty()).unwrap();
        let addr: SocketAddr = format!("{peer}:4000").parse().unwrap();
        req.extensions_mut().insert(ConnectInfo(addr));
        req
    }

    fn tight(general: Limit, expensive: Limit, failures: Limit, proxies: &str) -> RateLimiter {
        RateLimiter::new(
            true,
            general,
            expensive,
            failures,
            TrustedProxies::parse(proxies).unwrap(),
        )
    }

    async fn status(app: &Router, req: HttpRequest<Body>) -> (StatusCode, axum::http::HeaderMap) {
        let response = app.clone().oneshot(req).await.unwrap();
        (response.status(), response.headers().clone())
    }

    #[tokio::test]
    async fn one_subject_is_limited_while_another_is_not() {
        let app = app(tight(limit(2, 60), limit(5, 60), limit(5, 60), ""));
        for _ in 0..2 {
            let (s, h) = status(&app, request("/api/x", "10.0.0.1", Some("alice"), None)).await;
            assert_eq!(s, StatusCode::OK);
            assert!(h.contains_key("ratelimit-limit"));
        }
        let (s, h) = status(&app, request("/api/x", "10.0.0.1", Some("alice"), None)).await;
        assert_eq!(s, StatusCode::TOO_MANY_REQUESTS);
        assert_eq!(
            h["retry-after"], "2",
            "2 per minute with a burst of 2 is 2 per 2 s"
        );
        assert_eq!(h["ratelimit-remaining"], "0");
        assert_eq!(h["ratelimit-limit"], "2");
        // Same address, different credential: unaffected.
        let (s, _) = status(&app, request("/api/x", "10.0.0.1", Some("bob"), None)).await;
        assert_eq!(s, StatusCode::OK);
    }

    #[tokio::test]
    async fn the_allowed_response_reports_what_is_left() {
        let app = app(tight(limit(5, 60), limit(5, 60), limit(5, 60), ""));
        let (_, h) = status(&app, request("/api/x", "10.0.0.1", None, None)).await;
        assert_eq!(h["ratelimit-limit"], "5");
        assert_eq!(h["ratelimit-remaining"], "4");
    }

    #[tokio::test]
    async fn the_expensive_bucket_trips_before_the_general_one() {
        let app = app(tight(limit(50, 600), limit(2, 60), limit(50, 600), ""));
        for _ in 0..2 {
            let (s, _) = status(
                &app,
                request("/api/plans/p/cpm", "10.0.0.1", Some("a"), None),
            )
            .await;
            assert_eq!(s, StatusCode::OK);
        }
        let (s, _) = status(
            &app,
            request("/api/plans/p/cpm", "10.0.0.1", Some("a"), None),
        )
        .await;
        assert_eq!(s, StatusCode::TOO_MANY_REQUESTS);
        // Ordinary routes for the same caller still pass.
        let (s, _) = status(&app, request("/api/x", "10.0.0.1", Some("a"), None)).await;
        assert_eq!(s, StatusCode::OK);
    }

    #[tokio::test]
    async fn a_spoofed_forwarded_for_from_an_untrusted_peer_is_ignored() {
        let app = app(tight(limit(1, 60), limit(5, 60), limit(5, 60), "192.0.2.1"));
        let (s, _) = status(
            &app,
            request("/api/x", "10.0.0.9", None, Some("203.0.113.1")),
        )
        .await;
        assert_eq!(s, StatusCode::OK);
        // Same untrusted peer claiming a different client: still one bucket.
        let (s, _) = status(
            &app,
            request("/api/x", "10.0.0.9", None, Some("203.0.113.2")),
        )
        .await;
        assert_eq!(s, StatusCode::TOO_MANY_REQUESTS);
    }

    #[tokio::test]
    async fn a_trusted_proxy_lets_clients_be_told_apart() {
        let app = app(tight(limit(1, 60), limit(5, 60), limit(5, 60), "192.0.2.1"));
        let (s, _) = status(
            &app,
            request("/api/x", "192.0.2.1", None, Some("203.0.113.1")),
        )
        .await;
        assert_eq!(s, StatusCode::OK);
        let (s, _) = status(
            &app,
            request("/api/x", "192.0.2.1", None, Some("203.0.113.2")),
        )
        .await;
        assert_eq!(s, StatusCode::OK);
        let (s, _) = status(
            &app,
            request("/api/x", "192.0.2.1", None, Some("203.0.113.1")),
        )
        .await;
        assert_eq!(s, StatusCode::TOO_MANY_REQUESTS);
    }

    #[tokio::test]
    async fn repeated_authentication_failures_are_cut_off_for_that_client() {
        let app = app(tight(limit(50, 600), limit(5, 60), limit(2, 60), ""));
        for _ in 0..2 {
            let (s, _) = status(&app, request("/api/private", "10.0.0.1", Some("bad"), None)).await;
            assert_eq!(s, StatusCode::UNAUTHORIZED);
        }
        // The failure bucket is empty: even a valid-looking request is refused.
        let (s, _) = status(&app, request("/api/x", "10.0.0.1", Some("good"), None)).await;
        assert_eq!(s, StatusCode::TOO_MANY_REQUESTS);
        // Another client is unaffected.
        let (s, _) = status(&app, request("/api/x", "10.0.0.2", Some("good"), None)).await;
        assert_eq!(s, StatusCode::OK);
    }

    #[tokio::test]
    async fn health_is_never_limited() {
        let app = app(tight(limit(1, 60), limit(1, 60), limit(1, 60), ""));
        for _ in 0..5 {
            let (s, _) = status(&app, request("/_health", "10.0.0.1", None, None)).await;
            assert_eq!(s, StatusCode::OK);
        }
    }

    #[tokio::test]
    async fn a_disabled_limiter_passes_everything() {
        let off = RateLimiter::new(
            false,
            limit(1, 60),
            limit(1, 60),
            limit(1, 60),
            TrustedProxies::none(),
        );
        let app = app(off);
        for _ in 0..5 {
            let (s, h) = status(&app, request("/api/x", "10.0.0.1", None, None)).await;
            assert_eq!(s, StatusCode::OK);
            assert!(!h.contains_key("ratelimit-limit"));
        }
    }

    #[test]
    fn the_setting_resolves_from_config_and_environment() {
        // No environment variable is set by the test suite for this name.
        let on = RateLimiter::from_env(None);
        assert!(on.enabled());
        assert!(RateLimiter::from_env(Some(true)).enabled());
        assert!(!RateLimiter::from_env(Some(false)).enabled());
    }
}
