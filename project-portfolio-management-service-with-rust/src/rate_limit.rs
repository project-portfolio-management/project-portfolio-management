//! In-process request rate limiting (HD-5), counted by `axa-ratelimit`: fixed windows, no external store.
//!
//! Three independent limits, each keyed separately:
//!
//! - **general** — every request, keyed by the caller's credential (the bearer token, held only as `axa-ratelimit`'s
//!   keyed hash, so no extra verification) else the client address;
//! - **expensive** — the routes that do real work per call (simulations, CPM,
//!   levelling, forecasts, imports, exports, bulk jobs, PDF), a tighter limit
//!   charged on top of the general one;
//! - **lockout** — a client address that has earned `LOCKOUT_CLIENT_ADDRESS_AFTER_COUNT` authentication failures
//!   (`401`) is refused until its window ends, so a password-guessing or token-guessing client is stopped before the
//!   auth guard does any work. Only a `401` counts, so a client that signs in correctly is never slowed. It is
//!   `axa-ratelimit`'s [`axa::ratelimit::lock_out_failures`], which reads the address [`client_address_mw`] records.
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
//! The client address is axa's: [`axa::client::resolve`] with the `TRUSTED_PROXIES` list axa read, so the socket peer
//! unless that peer is a trusted proxy, in which case the rightmost untrusted `X-Forwarded-For` hop (see
//! [`crate::trusted_proxy`]). A spoofed header from any other peer is ignored. An IPv6 address counts by its `/64`
//! ([`axa::ratelimit::client_key`]), which is what one subscriber is usually given. This layer sits outside axa's
//! stack, which has not yet recorded the address when it runs, so it resolves the address itself the same way.
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
//! | `LOCKOUT_CLIENT_ADDRESS_AFTER_COUNT` | 20 | authentication failures one client address may earn before it is locked out |
//! | `…_RATE_LIMIT_FAILURES_PER_MINUTE` | 6 | sets how long a lockout lasts: as long as this rate takes to earn the count (200 s by default) |
//!
//! `…_RATE_LIMIT_FAILURES_BURST`, the name this service read for the count before, still works alone, with a warning.
//! Set to a different count from `LOCKOUT_CLIENT_ADDRESS_AFTER_COUNT`, the service refuses to start, as there would
//! be no telling which was meant.

use std::{net::IpAddr, num::NonZeroU32, sync::Arc};

use axa::{
    client::ProxyNet,
    ratelimit::{Limiter, Verdict},
};
use axum::{
    extract::{Request, State},
    http::{Extensions, HeaderValue, StatusCode, header},
    middleware::Next,
    response::{IntoResponse, Response},
};

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

/// The number of authentication failures one client address may earn before it is locked out.
pub const LOCKOUT_CLIENT_ADDRESS_AFTER_COUNT: &str = "LOCKOUT_CLIENT_ADDRESS_AFTER_COUNT";

/// The name this service read for that count before.
pub const OLD_LOCKOUT_COUNT: &str = "PROJECT_PORTFOLIO_MANAGEMENT_RATE_LIMIT_FAILURES_BURST";

/// The lockout count to use, from the two names, as `lookup` reads them: the new name, else the old one with a
/// warning, else `None` for the default. A number that is not a positive whole number, under either, is refused; so
/// are two different counts.
///
/// # Errors
///
/// A message naming the variable that cannot be used.
pub fn lockout_count(
    lookup: impl Fn(&str) -> Option<String>,
) -> Result<Option<NonZeroU32>, String> {
    let read = |name: &str| -> Result<Option<NonZeroU32>, String> {
        match lookup(name)
            .as_deref()
            .map(str::trim)
            .filter(|v| !v.is_empty())
        {
            None => Ok(None),
            Some(value) => value
                .parse::<NonZeroU32>()
                .map(Some)
                .map_err(|_| format!("{name} must be a whole number above zero, not {value:?}")),
        }
    };
    match (
        read(LOCKOUT_CLIENT_ADDRESS_AFTER_COUNT)?,
        read(OLD_LOCKOUT_COUNT)?,
    ) {
        (Some(new), Some(old)) if new != old => Err(format!(
            "{LOCKOUT_CLIENT_ADDRESS_AFTER_COUNT} is {new} and {OLD_LOCKOUT_COUNT} is {old}; set only \
             {LOCKOUT_CLIENT_ADDRESS_AFTER_COUNT}"
        )),
        (Some(new), _) => Ok(Some(new)),
        (None, Some(old)) => {
            tracing::warn!(
                "{OLD_LOCKOUT_COUNT} is read for now; rename it to {LOCKOUT_CLIENT_ADDRESS_AFTER_COUNT}"
            );
            Ok(Some(old))
        }
        (None, None) => Ok(None),
    }
}

/// The rate limiter's settings and state.
#[derive(Debug)]
pub struct RateLimiter {
    /// Master switch; when false every request passes unchecked.
    enabled: bool,
    /// The proxies axa trusts, from its configuration.
    proxies: Vec<ProxyNet>,
    /// Counts for the general limit, per credential or client.
    general: Limiter,
    /// Counts for the expensive-route limit, per credential or client.
    expensive: Limiter,
    /// Counts of authentication failures, per client address, for `axa-ratelimit`'s lockout.
    lockout: Limiter,
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
    ///
    /// # Errors
    ///
    /// When the lockout count cannot be used ([`lockout_count`]).
    pub fn from_env(configured: Option<bool>, proxies: Vec<ProxyNet>) -> Result<Self, String> {
        let env = std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_RATE_LIMIT").ok();
        let enabled = match env.as_deref().map(str::trim).filter(|v| !v.is_empty()) {
            Some(v) => !matches!(
                v.to_ascii_lowercase().as_str(),
                "0" | "false" | "no" | "off"
            ),
            None => configured.unwrap_or(true),
        };
        let pm = "PROJECT_PORTFOLIO_MANAGEMENT_RATE_LIMIT";
        let after = lockout_count(|name| std::env::var(name).ok())?.map_or(20, NonZeroU32::get);
        Ok(Self::new(
            enabled,
            Limit::per_minute(
                env_u32(&format!("{pm}_GENERAL_PER_MINUTE"), 600),
                env_u32(&format!("{pm}_GENERAL_BURST"), 200),
            ),
            Limit::per_minute(
                env_u32(&format!("{pm}_EXPENSIVE_PER_MINUTE"), 30),
                env_u32(&format!("{pm}_EXPENSIVE_BURST"), 10),
            ),
            Limit::per_minute(env_u32(&format!("{pm}_FAILURES_PER_MINUTE"), 6), after),
            proxies,
        ))
    }

    /// Build with explicit settings (tests, and [`Self::from_env`]).
    #[must_use]
    pub fn new(
        enabled: bool,
        general: Limit,
        expensive: Limit,
        lockout: Limit,
        proxies: Vec<ProxyNet>,
    ) -> Self {
        Self {
            enabled,
            proxies,
            general: general.limiter(),
            expensive: expensive.limiter(),
            lockout: lockout.limiter(),
        }
    }

    /// The client address: the one axa recorded if it has, else axa's resolution of the socket peer and
    /// `X-Forwarded-For`. `None` with no peer (an in-process transport).
    fn address(&self, req: &Request) -> Option<IpAddr> {
        req.extensions().get::<IpAddr>().copied().or_else(|| {
            let peer = crate::trusted_proxy::peer_ip(req.extensions())?;
            Some(axa::client::resolve(peer, req.headers(), &self.proxies))
        })
    }

    /// The client a request is counted against: its address, reduced as axa reduces it.
    fn client(&self, req: &Request) -> Option<IpAddr> {
        let mut resolved = Extensions::new();
        resolved.insert(self.address(req)?);
        axa::ratelimit::client_key(&resolved)
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

/// Record the client address on the request, as axa would, for the lockout inside. Outermost of the three limit
/// layers: `client_address_mw`, then [`lockout_mw`], then [`rate_limit_mw`].
pub async fn client_address_mw(
    State(limiter): State<Arc<RateLimiter>>,
    mut req: Request,
    next: Next,
) -> Response {
    if req.extensions().get::<IpAddr>().is_none()
        && let Some(address) = limiter.address(&req)
    {
        req.extensions_mut().insert(address);
    }
    next.run(req).await
}

/// `axa-ratelimit`'s failed-login lockout ([`axa::ratelimit::lock_out_failures`]), per client address, when limiting
/// is on. A locked-out client is refused before it is charged to the general limit. Each lockout is counted once, on
/// its first refusal, in `client_address_lockouts_total`, and logged without the address (which is personal data).
pub async fn lockout_mw(
    State(limiter): State<Arc<RateLimiter>>,
    req: Request,
    next: Next,
) -> Response {
    if !limiter.enabled {
        return next.run(req).await;
    }
    let response =
        axa::ratelimit::lock_out_failures(State(limiter.lockout.clone()), req, next).await;
    if is_new_lockout(&response) {
        crate::metrics::Metrics::global()
            .client_address_lockouts_total
            .inc();
        tracing::warn!("a client address was locked out for failed authentications");
    }
    response
}

/// Whether `response` is the first refusal of a lockout: a `429` whose [`Verdict`] says it is the first this window.
fn is_new_lockout(response: &Response) -> bool {
    response.status() == StatusCode::TOO_MANY_REQUESTS
        && response
            .extensions()
            .get::<Verdict>()
            .is_some_and(|verdict| verdict.first_refusal)
}

/// The general and expensive limits (see the module docs). Layer the three limit layers outside the auth guard and
/// inside the security-headers layer, so a `429` carries the security headers.
pub async fn rate_limit_mw(
    State(limiter): State<Arc<RateLimiter>>,
    req: Request,
    next: Next,
) -> Response {
    if !limiter.enabled || is_exempt(req.uri().path()) {
        return next.run(req).await;
    }
    let client = limiter.client(&req);

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

    /// The limiter as production layers it: outside axa's stack, with the proxy list axa was given.
    fn app(limiter: RateLimiter, proxies: &str) -> Router {
        let nets = crate::trusted_proxy::TrustedProxies::parse(proxies)
            .unwrap()
            .into_nets();
        let limiter = Arc::new(RateLimiter {
            proxies: nets.clone(),
            ..limiter
        });
        let routes = Router::new()
            .route("/api/x", get(|| async { "ok" }))
            .route("/api/plans/p/cpm", get(|| async { "ok" }))
            .route("/_health", get(|| async { "ok" }))
            .route(
                "/api/private",
                get(|| async { (StatusCode::UNAUTHORIZED, "no") }),
            );
        let app = axa::middleware::apply(
            routes,
            &axa::config::Http {
                body_limit: 1 << 20,
                compression: false,
                timeout_seconds: 0,
                timeout_overrides: Vec::new(),
                cors_origins: Vec::new(),
                hsts: false,
                hsts_max_age: 0,
                csp: None,
                trusted_proxies: nets,
            },
        )
        .unwrap();
        app.layer(axum::middleware::from_fn_with_state(
            limiter.clone(),
            rate_limit_mw,
        ))
        .layer(axum::middleware::from_fn_with_state(
            limiter.clone(),
            lockout_mw,
        ))
        .layer(axum::middleware::from_fn_with_state(
            limiter,
            client_address_mw,
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
        let addr: SocketAddr = match peer.parse::<std::net::IpAddr>().unwrap() {
            std::net::IpAddr::V4(v4) => SocketAddr::from((v4, 4000)),
            std::net::IpAddr::V6(v6) => SocketAddr::from((v6, 4000)),
        };
        req.extensions_mut().insert(ConnectInfo(addr));
        req
    }

    fn tight(general: Limit, expensive: Limit, lockout: Limit) -> RateLimiter {
        RateLimiter::new(true, general, expensive, lockout, Vec::new())
    }

    async fn status(app: &Router, req: HttpRequest<Body>) -> (StatusCode, axum::http::HeaderMap) {
        let response = app.clone().oneshot(req).await.unwrap();
        (response.status(), response.headers().clone())
    }

    #[tokio::test]
    async fn one_subject_is_limited_while_another_is_not() {
        let app = app(tight(limit(2, 60), limit(5, 60), limit(5, 60)), "");
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
        let app = app(tight(limit(5, 60), limit(5, 60), limit(5, 60)), "");
        let (_, h) = status(&app, request("/api/x", "10.0.0.1", None, None)).await;
        assert_eq!(h["ratelimit-limit"], "5");
        assert_eq!(h["ratelimit-remaining"], "4");
    }

    #[tokio::test]
    async fn the_expensive_bucket_trips_before_the_general_one() {
        let app = app(tight(limit(50, 600), limit(2, 60), limit(50, 600)), "");
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
        let app = app(tight(limit(1, 60), limit(5, 60), limit(5, 60)), "192.0.2.1");
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
        let app = app(tight(limit(1, 60), limit(5, 60), limit(5, 60)), "192.0.2.1");
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
        let app = app(tight(limit(50, 600), limit(5, 60), limit(2, 60)), "");
        for _ in 0..2 {
            let (s, _) = status(&app, request("/api/private", "10.0.0.1", Some("bad"), None)).await;
            assert_eq!(s, StatusCode::UNAUTHORIZED);
        }
        // Locked out: even a valid-looking request is refused, with when to come back.
        let (s, h) = status(&app, request("/api/x", "10.0.0.1", Some("good"), None)).await;
        assert_eq!(s, StatusCode::TOO_MANY_REQUESTS);
        assert!(h.contains_key("retry-after"));
        // Another client is unaffected.
        let (s, _) = status(&app, request("/api/x", "10.0.0.2", Some("good"), None)).await;
        assert_eq!(s, StatusCode::OK);
    }

    #[tokio::test]
    async fn a_lockout_is_counted_once_not_per_refused_request() {
        let app = app(tight(limit(50, 600), limit(5, 60), limit(1, 60)), "");
        let (s, _) = status(&app, request("/api/private", "10.0.0.7", Some("bad"), None)).await;
        assert_eq!(s, StatusCode::UNAUTHORIZED);
        let mut firsts = Vec::new();
        for _ in 0..3 {
            let response = app
                .clone()
                .oneshot(request("/api/x", "10.0.0.7", None, None))
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);
            firsts.push(is_new_lockout(&response));
        }
        assert_eq!(firsts, [true, false, false]);
        let ok = app
            .clone()
            .oneshot(request("/api/x", "10.0.0.8", None, None))
            .await
            .unwrap();
        assert!(!is_new_lockout(&ok), "an allowed response is no lockout");
    }

    #[tokio::test]
    async fn only_failures_count_towards_a_lockout() {
        let app = app(tight(limit(50, 600), limit(5, 60), limit(2, 60)), "");
        for _ in 0..5 {
            let (s, _) = status(&app, request("/api/x", "10.0.0.1", Some("good"), None)).await;
            assert_eq!(s, StatusCode::OK);
        }
        let (s, _) = status(&app, request("/api/private", "10.0.0.1", Some("bad"), None)).await;
        assert_eq!(
            s,
            StatusCode::UNAUTHORIZED,
            "one failure is under the count"
        );
    }

    #[tokio::test]
    async fn a_lockout_behind_a_trusted_proxy_is_per_client_not_per_proxy() {
        let app = app(
            tight(limit(50, 600), limit(5, 60), limit(1, 60)),
            "192.0.2.1",
        );
        let (s, _) = status(
            &app,
            request(
                "/api/private",
                "192.0.2.1",
                Some("bad"),
                Some("203.0.113.1"),
            ),
        )
        .await;
        assert_eq!(s, StatusCode::UNAUTHORIZED);
        let (s, _) = status(
            &app,
            request("/api/x", "192.0.2.1", None, Some("203.0.113.1")),
        )
        .await;
        assert_eq!(
            s,
            StatusCode::TOO_MANY_REQUESTS,
            "the guessing client is locked out"
        );
        let (s, _) = status(
            &app,
            request("/api/x", "192.0.2.1", None, Some("203.0.113.2")),
        )
        .await;
        assert_eq!(
            s,
            StatusCode::OK,
            "another client through the same proxy is not"
        );
    }

    #[test]
    fn the_lockout_count_is_read_from_the_new_name_and_the_old_one() {
        let env = |pairs: &'static [(&'static str, &'static str)]| {
            move |name: &str| {
                pairs
                    .iter()
                    .find(|(n, _)| *n == name)
                    .map(|(_, v)| (*v).to_string())
            }
        };
        let n = |v: u32| Ok(NonZeroU32::new(v));
        assert_eq!(lockout_count(env(&[])), Ok(None));
        assert_eq!(
            lockout_count(env(&[(LOCKOUT_CLIENT_ADDRESS_AFTER_COUNT, "5")])),
            n(5)
        );
        assert_eq!(lockout_count(env(&[(OLD_LOCKOUT_COUNT, "7")])), n(7));
        assert_eq!(
            lockout_count(env(&[
                (LOCKOUT_CLIENT_ADDRESS_AFTER_COUNT, "5"),
                (OLD_LOCKOUT_COUNT, "5")
            ])),
            n(5)
        );
        let two = lockout_count(env(&[
            (LOCKOUT_CLIENT_ADDRESS_AFTER_COUNT, "5"),
            (OLD_LOCKOUT_COUNT, "7"),
        ]))
        .unwrap_err();
        assert!(
            two.contains(LOCKOUT_CLIENT_ADDRESS_AFTER_COUNT) && two.contains(OLD_LOCKOUT_COUNT),
            "{two}"
        );
        for bad in ["0", "-1", "many"] {
            let refused = lockout_count(move |name: &str| {
                (name == LOCKOUT_CLIENT_ADDRESS_AFTER_COUNT).then(|| bad.to_string())
            })
            .unwrap_err();
            assert!(
                refused.contains(LOCKOUT_CLIENT_ADDRESS_AFTER_COUNT),
                "{refused}"
            );
        }
    }

    #[tokio::test]
    async fn an_ipv6_client_counts_by_its_slash_64() {
        let app = app(tight(limit(1, 60), limit(5, 60), limit(5, 60)), "");
        let (s, _) = status(&app, request("/api/x", "2001:db8:1:2::1", None, None)).await;
        assert_eq!(s, StatusCode::OK);
        // Another address in the same /64 is the same subscriber.
        let (s, _) = status(&app, request("/api/x", "2001:db8:1:2:ffff::9", None, None)).await;
        assert_eq!(s, StatusCode::TOO_MANY_REQUESTS);
        let (s, _) = status(&app, request("/api/x", "2001:db8:1:3::1", None, None)).await;
        assert_eq!(s, StatusCode::OK);
    }

    #[tokio::test]
    async fn health_is_never_limited() {
        let app = app(tight(limit(1, 60), limit(1, 60), limit(1, 60)), "");
        for _ in 0..5 {
            let (s, _) = status(&app, request("/_health", "10.0.0.1", None, None)).await;
            assert_eq!(s, StatusCode::OK);
        }
    }

    #[tokio::test]
    async fn a_disabled_limiter_passes_everything() {
        let off = RateLimiter::new(false, limit(1, 60), limit(1, 60), limit(1, 60), Vec::new());
        let app = app(off, "");
        for _ in 0..5 {
            let (s, h) = status(&app, request("/api/x", "10.0.0.1", None, None)).await;
            assert_eq!(s, StatusCode::OK);
            assert!(!h.contains_key("ratelimit-limit"));
        }
    }

    #[test]
    fn the_setting_resolves_from_config_and_environment() {
        // No environment variable is set by the test suite for this name.
        let on = RateLimiter::from_env(None, Vec::new()).unwrap();
        assert!(on.enabled());
        assert!(
            RateLimiter::from_env(Some(true), Vec::new())
                .unwrap()
                .enabled()
        );
        assert!(
            !RateLimiter::from_env(Some(false), Vec::new())
                .unwrap()
                .enabled()
        );
    }
}
