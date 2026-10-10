//! End-to-end proof that the real router carries the security headers, the
//! CORS allow-list and the rate limiter (HD-4, HD-5).
//!
//! The pure behaviour is pinned by the unit tests in `src/http_security.rs`
//! and `src/rate_limit.rs`; what they cannot show is that the layers are
//! *wired* onto the router loco builds, in the right order (so a `429` carries
//! the security headers, and health stays exempt). Its **own test binary**
//! because the CORS list and the limits are read once at boot from the
//! environment, before the first request.
//!
//! `#[ignore]`d — boots the app, so it needs `PostgreSQL` via
//! `config/test.yaml` / `DATABASE_URL`. Run with
//! `cargo test --test hardening -- --ignored`.

use loco_rs::testing::prelude::*;
use project_portfolio_management_service::app::App;
use serial_test::serial;

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test --test hardening -- --ignored`"]
async fn the_real_router_carries_headers_cors_and_rate_limits() {
    unsafe {
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_RATE_LIMIT", "on");
        std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_RATE_LIMIT_GENERAL_BURST", "6");
        std::env::set_var(
            "PROJECT_PORTFOLIO_MANAGEMENT_RATE_LIMIT_GENERAL_PER_MINUTE",
            "1",
        );
        std::env::set_var(
            "PROJECT_PORTFOLIO_MANAGEMENT_CORS_ORIGINS",
            "https://app.example.com",
        );
    }

    request::<App, _, _>(|request, _ctx| async move {
        // Security headers on an ordinary response.
        let response = request.get("/api/plans").await;
        assert_eq!(response.status_code(), 200);
        let headers = response.headers();
        assert_eq!(headers["x-content-type-options"], "nosniff");
        assert_eq!(headers["x-frame-options"], "DENY");
        assert_eq!(
            headers["content-security-policy"],
            "default-src 'none'; frame-ancestors 'none'"
        );
        assert_eq!(
            headers["referrer-policy"],
            "strict-origin-when-cross-origin"
        );
        assert!(headers.contains_key("permissions-policy"));
        assert_eq!(headers["cross-origin-opener-policy"], "same-origin");
        assert!(
            !headers.contains_key("strict-transport-security"),
            "HSTS is never sent over plain HTTP"
        );
        // ...and the rate-limit headers of the bucket that was charged.
        assert_eq!(headers["ratelimit-limit"], "6");
        assert_eq!(headers["ratelimit-remaining"], "5");

        // CORS: the listed origin is granted (and its preflight answered
        // without auth); another origin gets nothing.
        let allowed = request
            .get("/api/plans")
            .add_header("origin", "https://app.example.com")
            .await;
        assert_eq!(
            allowed.headers()["access-control-allow-origin"],
            "https://app.example.com"
        );
        let denied = request
            .get("/api/plans")
            .add_header("origin", "https://evil.example.com")
            .await;
        assert!(!denied.headers().contains_key("access-control-allow-origin"));

        // Health is exempt from limiting however often it is called.
        for _ in 0..10 {
            assert_eq!(request.get("/_health").await.status_code(), 200);
        }

        // Spend the rest of the burst (3 requests so far), then get refused
        // — with the security headers and a Retry-After on the 429.
        for _ in 0..3 {
            assert_eq!(request.get("/api/plans").await.status_code(), 200);
        }
        let refused = request.get("/api/plans").await;
        assert_eq!(refused.status_code(), 429);
        assert!(refused.headers().contains_key("retry-after"));
        assert_eq!(refused.headers()["ratelimit-remaining"], "0");
        assert_eq!(refused.headers()["x-content-type-options"], "nosniff");
    })
    .await;
}
