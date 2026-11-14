//! axa 0.13's cross-site write guard, through the whole stack: a browser's cross-site write is refused, a write from
//! an origin `PROJECT_PORTFOLIO_MANAGEMENT_CORS_ORIGINS` lists passes, and a request with neither header (a server's,
//! as the UI's proxy sends) passes.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use axa_test::prelude::*;
use axum::http::{HeaderName, HeaderValue, StatusCode};
use project_portfolio_management::app::App;
use serde_json::{Value, json};
use serial_test::serial;

const CORS: &str = "PROJECT_PORTFOLIO_MANAGEMENT_CORS_ORIGINS";
const EDITOR: &str = "https://editor.example.org";

fn h(name: &'static str, value: &str) -> (HeaderName, HeaderValue) {
    (HeaderName::from_static(name), value.parse().unwrap())
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
async fn a_cross_site_write_is_refused_unless_its_origin_is_listed() {
    // SAFETY: serial; the setting is read when the app is built, and cleared at the end.
    unsafe { std::env::set_var(CORS, EDITOR) };
    request::<App, _, _>(|request, _ctx| async move {
        let body = json!({ "kind": "Project", "name": "Cross-site check" });
        // A server's request, with neither header: passes.
        request
            .post("/api/plans")
            .json(&body)
            .await
            .assert_status_ok();
        // A browser on another site, not listed: refused, by either header.
        let (n, v) = h("sec-fetch-site", "cross-site");
        let (on, ov) = h("origin", "https://evil.example");
        let r = request
            .post("/api/plans")
            .add_header(n.clone(), v.clone())
            .add_header(on.clone(), ov.clone())
            .json(&body)
            .await;
        assert_eq!(r.status_code(), StatusCode::FORBIDDEN);
        let refusal: Value = r.json();
        assert!(
            refusal.to_string().contains("cross_site_write_refused"),
            "{refusal}"
        );
        let r = request
            .post("/api/plans")
            .add_header(on, ov)
            .json(&body)
            .await;
        assert_eq!(r.status_code(), StatusCode::FORBIDDEN);
        assert!(r.text().contains("origin_mismatch"), "{}", r.text());
        // The same browser's read passes.
        let (n2, v2) = h("sec-fetch-site", "cross-site");
        request
            .get("/api/plans")
            .add_header(n2, v2)
            .await
            .assert_status_ok();
        // A listed origin may write, and gets its CORS grant.
        let (lo, lv) = h("origin", EDITOR);
        let r = request
            .post("/api/plans")
            .add_header(n, v)
            .add_header(lo, lv)
            .json(&body)
            .await;
        r.assert_status_ok();
        assert_eq!(r.header("access-control-allow-origin"), EDITOR);
        // A same-origin browser write passes.
        let (s, sv) = h("sec-fetch-site", "same-origin");
        request
            .post("/api/plans")
            .add_header(s, sv)
            .json(&body)
            .await
            .assert_status_ok();
    })
    .await;
    // SAFETY: as above.
    unsafe { std::env::remove_var(CORS) };
}
