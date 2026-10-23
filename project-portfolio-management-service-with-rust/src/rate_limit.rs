//! In-process request rate limiting (HD-5): token buckets, no external store.
//!
//! Three independent bucket families, each keyed separately:
//!
//! - **general** — every request, keyed by the caller's credential (a hash of
//!   the bearer token, so no extra verification) else the client address;
//! - **expensive** — the routes that do real work per call (simulations, CPM,
//!   levelling, forecasts, imports, exports, bulk jobs, PDF), a tighter bucket
//!   charged on top of the general one;
//! - **failures** — authentication failures (`401`) per client address, so a
//!   password-guessing or token-guessing client is slowed before the auth
//!   guard does any work.
//!
//! `/_health`, `/_ping` and `/_readiness` are exempt. A refused request is `429` with
//! `Retry-After` and the `RateLimit-Limit` / `RateLimit-Remaining` /
//! `RateLimit-Reset` headers (whole seconds); an allowed one carries the
//! `RateLimit-*` headers of the tightest bucket it was charged to.
//!
//! The client address is the socket peer unless that peer is listed in
//! `PROJECT_PORTFOLIO_MANAGEMENT_TRUSTED_PROXIES`, in which case the
//! rightmost untrusted `X-Forwarded-For` hop is used (see
//! [`crate::trusted_proxy`]). A spoofed header from any other peer is
//! ignored.
//!
//! Memory is bounded: when a family holds [`MAX_KEYS`] keys, idle (full)
//! buckets are dropped, and if it is still full new keys share one overflow
//! bucket. State is per process; with several replicas each enforces its own
//! share (a shared store is a later step, `plan.md` §47).
//!
//! ## Configuration
//!
//! | Variable | Default | Meaning |
//! |---|---|---|
//! | `PROJECT_PORTFOLIO_MANAGEMENT_RATE_LIMIT` | on | `off`/`false`/`0` disables (also `settings.rate_limit: false` in the loco config; the shipped development and test configs do) |
//! | `…_RATE_LIMIT_GENERAL_PER_MINUTE` / `…_GENERAL_BURST` | 600 / 200 | sustained requests per minute, and the burst allowed |
//! | `…_RATE_LIMIT_EXPENSIVE_PER_MINUTE` / `…_EXPENSIVE_BURST` | 30 / 10 | the same for expensive routes |
//! | `…_RATE_LIMIT_FAILURES_PER_MINUTE` / `…_FAILURES_BURST` | 6 / 20 | authentication failures per client address |

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Instant,
};

use axum::{
    body::Body,
    extract::{Request, State},
    http::{HeaderName, HeaderValue, StatusCode, header},
    middleware::Next,
    response::Response,
};

use crate::trusted_proxy::{TrustedProxies, peer_ip};

/// Keys one bucket family may hold before it sheds idle ones.
pub const MAX_KEYS: usize = 100_000;

/// The shared key for callers that arrive once the family is full.
const OVERFLOW_KEY: &str = "overflow";

/// Paths that are never limited.
const EXEMPT: &[&str] = &["/_health", "/_ping", "/_readiness"];

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

/// One bucket's shape: how many requests it holds, and how fast it refills.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Limit {
    /// Burst size: the most requests available at once.
    pub burst: f64,
    /// Refill rate in requests per second.
    pub per_second: f64,
}

impl Limit {
    /// A limit of `per_minute` sustained requests with a `burst` allowance.
    #[must_use]
    pub fn per_minute(per_minute: u32, burst: u32) -> Self {
        Self {
            burst: f64::from(burst.max(1)),
            per_second: f64::from(per_minute.max(1)) / 60.0,
        }
    }
}

/// The outcome of charging a bucket.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Verdict {
    /// Whether the request may proceed.
    pub allowed: bool,
    /// The bucket's burst size.
    pub limit: u32,
    /// Whole requests left after this one.
    pub remaining: u32,
    /// Seconds until the bucket is full again.
    pub reset_secs: u64,
    /// Seconds to wait before retrying (0 when allowed).
    pub retry_after_secs: u64,
}

#[derive(Debug, Clone, Copy)]
struct Bucket {
    tokens: f64,
    last: Instant,
}

impl Bucket {
    fn refilled(self, limit: &Limit, now: Instant) -> f64 {
        let elapsed = now.saturating_duration_since(self.last).as_secs_f64();
        (self.tokens + elapsed * limit.per_second).min(limit.burst)
    }
}

/// A keyed family of token buckets. The clock is passed in, so tests drive
/// time by hand.
#[derive(Debug, Default)]
pub struct Limiter {
    buckets: Mutex<HashMap<String, Bucket>>,
}

fn secs_ceil(seconds: f64) -> u64 {
    if seconds <= 0.0 {
        0
    } else {
        #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let whole = seconds.ceil() as u64;
        whole
    }
}

impl Limiter {
    /// A new, empty family.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    fn slot<'a>(
        map: &'a mut HashMap<String, Bucket>,
        key: &str,
        limit: &Limit,
        now: Instant,
        max_keys: usize,
    ) -> (&'a mut Bucket, String) {
        let mut key = key.to_string();
        if !map.contains_key(&key) && map.len() >= max_keys {
            map.retain(|_, b| b.refilled(limit, now) < limit.burst);
            if map.len() >= max_keys {
                key = OVERFLOW_KEY.to_string();
            }
        }
        let bucket = map.entry(key.clone()).or_insert(Bucket {
            tokens: limit.burst,
            last: now,
        });
        (bucket, key)
    }

    /// Charge `cost` requests to `key`'s bucket.
    ///
    /// # Panics
    ///
    /// Only if another thread panicked while holding the lock.
    pub fn take(&self, key: &str, limit: &Limit, now: Instant, cost: f64) -> Verdict {
        self.take_bounded(key, limit, now, cost, MAX_KEYS)
    }

    fn take_bounded(
        &self,
        key: &str,
        limit: &Limit,
        now: Instant,
        cost: f64,
        max_keys: usize,
    ) -> Verdict {
        let mut map = self.buckets.lock().expect("the limiter lock");
        let (bucket, _) = Self::slot(&mut map, key, limit, now, max_keys);
        let have = bucket.refilled(limit, now);
        let allowed = have >= cost;
        let left = if allowed { have - cost } else { have };
        bucket.tokens = left;
        bucket.last = now;
        #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let remaining = left.floor().max(0.0) as u32;
        #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let burst = limit.burst as u32;
        Verdict {
            allowed,
            limit: burst,
            remaining,
            reset_secs: secs_ceil((limit.burst - left) / limit.per_second),
            retry_after_secs: if allowed {
                0
            } else {
                secs_ceil((cost - left) / limit.per_second).max(1)
            },
        }
    }

    /// Whether `key` has at least one request available, without charging it.
    ///
    /// # Panics
    ///
    /// Only if another thread panicked while holding the lock.
    pub fn has_capacity(&self, key: &str, limit: &Limit, now: Instant) -> Option<Verdict> {
        let map = self.buckets.lock().expect("the limiter lock");
        let have = map.get(key).map_or(limit.burst, |b| b.refilled(limit, now));
        if have >= 1.0 {
            return None;
        }
        #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let burst = limit.burst as u32;
        Some(Verdict {
            allowed: false,
            limit: burst,
            remaining: 0,
            reset_secs: secs_ceil((limit.burst - have) / limit.per_second),
            retry_after_secs: secs_ceil((1.0 - have) / limit.per_second).max(1),
        })
    }

    /// Keys currently held.
    ///
    /// # Panics
    ///
    /// Only if another thread panicked while holding the lock.
    #[must_use]
    pub fn len(&self) -> usize {
        self.buckets.lock().expect("the limiter lock").len()
    }

    /// Whether no key is held.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
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
    EXEMPT.contains(&path)
}

/// A 64-bit FNV-1a hash of `text`, hex-encoded. Not a security boundary: it
/// only turns a bearer token into a bucket key so the token itself is never
/// held in memory longer than the request.
fn key_hash(text: &str) -> String {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in text.bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0100_0000_01b3);
    }
    format!("{hash:016x}")
}

/// The rate limiter's settings and state.
#[derive(Debug)]
pub struct RateLimiter {
    enabled: bool,
    general_limit: Limit,
    expensive_limit: Limit,
    failure_limit: Limit,
    proxies: TrustedProxies,
    general: Limiter,
    expensive: Limiter,
    failures: Limiter,
}

fn env_u32(name: &str, default: u32) -> u32 {
    std::env::var(name)
        .ok()
        .and_then(|v| v.trim().parse().ok())
        .filter(|n| *n > 0)
        .unwrap_or(default)
}

impl RateLimiter {
    /// Build from the environment; `configured` is the loco
    /// `settings.rate_limit` value (the environment wins when set).
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
            general_limit: general,
            expensive_limit: expensive,
            failure_limit: failures,
            proxies,
            general: Limiter::new(),
            expensive: Limiter::new(),
            failures: Limiter::new(),
        }
    }

    /// Whether limiting is on.
    #[must_use]
    pub fn enabled(&self) -> bool {
        self.enabled
    }
}

fn set_ratelimit_headers(response: &mut Response, verdict: &Verdict) {
    let h = response.headers_mut();
    for (name, value) in [
        ("ratelimit-limit", verdict.limit.to_string()),
        ("ratelimit-remaining", verdict.remaining.to_string()),
        ("ratelimit-reset", verdict.reset_secs.to_string()),
    ] {
        if let Ok(v) = HeaderValue::from_str(&value) {
            h.insert(HeaderName::from_static(name), v);
        }
    }
}

fn too_many(verdict: &Verdict, what: &str) -> Response {
    let body = serde_json::json!({
        "error": "too_many_requests",
        "description": format!("{what}; retry after {} seconds", verdict.retry_after_secs),
    })
    .to_string();
    let mut response = Response::new(Body::from(body));
    *response.status_mut() = StatusCode::TOO_MANY_REQUESTS;
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );
    if let Ok(v) = HeaderValue::from_str(&verdict.retry_after_secs.to_string()) {
        response.headers_mut().insert(header::RETRY_AFTER, v);
    }
    set_ratelimit_headers(&mut response, verdict);
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
    let path = req.uri().path().to_string();
    if !limiter.enabled || is_exempt(&path) {
        return next.run(req).await;
    }
    let now = Instant::now();
    let client = limiter
        .proxies
        .client_ip(peer_ip(req.extensions()), req.headers())
        .map_or_else(|| "unknown".to_string(), |ip| ip.to_string());

    if let Some(verdict) = limiter
        .failures
        .has_capacity(&client, &limiter.failure_limit, now)
    {
        return too_many(&verdict, "too many failed authentication attempts");
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
    let key = credential.map_or_else(
        || format!("ip:{client}"),
        |t| format!("tok:{}", key_hash(t)),
    );

    let mut tightest = limiter.general.take(&key, &limiter.general_limit, now, 1.0);
    if !tightest.allowed {
        return too_many(&tightest, "rate limit exceeded");
    }
    if is_expensive(&path) {
        let verdict = limiter
            .expensive
            .take(&key, &limiter.expensive_limit, now, 1.0);
        if !verdict.allowed {
            return too_many(&verdict, "rate limit exceeded for this kind of request");
        }
        if verdict.remaining < tightest.remaining {
            tightest = verdict;
        }
    }

    let mut response = next.run(req).await;
    if response.status() == StatusCode::UNAUTHORIZED {
        limiter
            .failures
            .take(&client, &limiter.failure_limit, Instant::now(), 1.0);
    }
    set_ratelimit_headers(&mut response, &tightest);
    response
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{Router, extract::ConnectInfo, http::Request as HttpRequest, routing::get};
    use std::{net::SocketAddr, time::Duration};
    use tower::ServiceExt;

    fn limit(burst: u32, per_minute: u32) -> Limit {
        Limit::per_minute(per_minute, burst)
    }

    #[test]
    fn a_bucket_allows_its_burst_then_refuses_then_refills() {
        let l = Limiter::new();
        let lim = limit(3, 60); // one per second
        let t0 = Instant::now();
        for left in [2, 1, 0] {
            let v = l.take("k", &lim, t0, 1.0);
            assert!(v.allowed);
            assert_eq!(v.remaining, left);
        }
        let refused = l.take("k", &lim, t0, 1.0);
        assert!(!refused.allowed);
        assert_eq!(refused.retry_after_secs, 1);
        assert_eq!(refused.limit, 3);
        // Half a second later there is still not a whole request.
        assert!(
            !l.take("k", &lim, t0 + Duration::from_millis(500), 1.0)
                .allowed
        );
        // A second after the start one has refilled.
        assert!(l.take("k", &lim, t0 + Duration::from_secs(1), 1.0).allowed);
    }

    #[test]
    fn the_bucket_never_holds_more_than_its_burst() {
        let l = Limiter::new();
        let lim = limit(2, 60);
        let t0 = Instant::now();
        assert!(l.take("k", &lim, t0, 1.0).allowed);
        let later = l.take("k", &lim, t0 + Duration::from_hours(1), 1.0);
        assert!(later.allowed);
        assert_eq!(later.remaining, 1);
    }

    #[test]
    fn keys_are_independent() {
        let l = Limiter::new();
        let lim = limit(1, 1);
        let t0 = Instant::now();
        assert!(l.take("a", &lim, t0, 1.0).allowed);
        assert!(!l.take("a", &lim, t0, 1.0).allowed);
        assert!(l.take("b", &lim, t0, 1.0).allowed);
    }

    #[test]
    fn memory_is_bounded_by_an_overflow_bucket() {
        let l = Limiter::new();
        let lim = limit(1, 1);
        let t0 = Instant::now();
        for i in 0..4 {
            l.take_bounded(&format!("k{i}"), &lim, t0, 1.0, 3);
        }
        // Three real keys, the fourth caller landed in the shared overflow key.
        assert_eq!(l.len(), 4);
        for i in 4..40 {
            l.take_bounded(&format!("k{i}"), &lim, t0, 1.0, 3);
        }
        assert_eq!(l.len(), 4, "new callers share the overflow bucket");
    }

    #[test]
    fn idle_buckets_are_shed_before_overflow() {
        let l = Limiter::new();
        let lim = limit(2, 120);
        let t0 = Instant::now();
        l.take_bounded("a", &lim, t0, 1.0, 1);
        // Long idle: "a" is full again, so "b" displaces it instead of overflowing.
        l.take_bounded("b", &lim, t0 + Duration::from_mins(10), 1.0, 1);
        assert_eq!(l.len(), 1);
        assert!(l.buckets.lock().unwrap().contains_key("b"));
    }

    #[test]
    fn has_capacity_does_not_charge() {
        let l = Limiter::new();
        let lim = limit(1, 60);
        let t0 = Instant::now();
        assert!(l.has_capacity("k", &lim, t0).is_none());
        assert!(l.has_capacity("k", &lim, t0).is_none());
        l.take("k", &lim, t0, 1.0);
        let refused = l.has_capacity("k", &lim, t0).expect("empty bucket");
        assert!(!refused.allowed);
        assert_eq!(refused.retry_after_secs, 1);
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
    fn the_credential_key_is_stable_and_not_the_token() {
        assert_eq!(key_hash("abc"), key_hash("abc"));
        assert_ne!(key_hash("abc"), key_hash("abd"));
        assert!(!key_hash("secret-token").contains("secret"));
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
        assert_eq!(h["retry-after"], "1");
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
