//! Response caching on Loco's cache (Redis in production).
//!
//! Correctness first: the privacy gates (`privacy_mw`, handling marks, break-glass) run in middleware
//! **before** the handler, so a cache hit has already passed them. Entries are keyed by caller and by
//! a write **epoch**: any successful non-read request anywhere replaces the epoch, which orphans every
//! entry at once. A short lifetime is the backstop for changes that bypass the API (retention purge,
//! direct database edits, the passing of a date). The epoch is read before the data, so a result
//! computed across a write is stored under the old epoch and never found again. If the cache is down
//! the answer is simply computed: caching never fails a request.

use std::time::Duration;

use axum::{extract::Request, extract::State, http::Method, middleware::Next, response::Response};
use loco_rs::app::AppContext;
use uuid::Uuid;

const EPOCH_KEY: &str = "ppm:epoch";
/// How long an entry lives even if nothing is written.
pub const TTL: Duration = Duration::from_secs(30);

/// The current epoch (created on first use).
pub async fn epoch(ctx: &AppContext) -> String {
    if let Ok(Some(e)) = ctx.cache.get::<String>(EPOCH_KEY).await {
        return e;
    }
    let e = Uuid::new_v4().to_string();
    let _ = ctx
        .cache
        .insert_with_expiry(EPOCH_KEY, &e, Duration::from_hours(24))
        .await;
    e
}

/// Orphan every cached entry.
pub async fn bump(ctx: &AppContext) {
    let _ = ctx
        .cache
        .insert_with_expiry(
            EPOCH_KEY,
            &Uuid::new_v4().to_string(),
            Duration::from_hours(24),
        )
        .await;
}

/// The key for `what` asked by `caller` (their token subject, or `anonymous`).
#[must_use]
pub fn key(epoch: &str, caller: Option<&str>, what: &str) -> String {
    format!("ppm:r:{epoch}:{}:{what}", caller.unwrap_or("anonymous"))
}

/// Whether a request of this method can have changed data.
#[must_use]
pub fn is_write(m: &Method) -> bool {
    !matches!(*m, Method::GET | Method::HEAD | Method::OPTIONS)
}

/// Middleware: after any successful write, replace the epoch.
pub async fn epoch_mw(State(ctx): State<AppContext>, req: Request, next: Next) -> Response {
    let write = is_write(req.method());
    let res = next.run(req).await;
    if write && !res.status().is_client_error() && !res.status().is_server_error() {
        bump(&ctx).await;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keys_differ_by_epoch_and_by_caller() {
        let a = key("e1", Some("alice"), "cpm:p");
        assert_ne!(a, key("e2", Some("alice"), "cpm:p"));
        assert_ne!(a, key("e1", Some("bob"), "cpm:p"));
        assert_ne!(key("e1", None, "x"), key("e1", Some("anonymous2"), "x"));
        assert!(key("e1", None, "x").contains("anonymous"));
    }

    #[test]
    fn only_reads_are_not_writes() {
        assert!(!is_write(&Method::GET) && !is_write(&Method::HEAD) && !is_write(&Method::OPTIONS));
        assert!(
            is_write(&Method::POST)
                && is_write(&Method::PUT)
                && is_write(&Method::DELETE)
                && is_write(&Method::PATCH)
        );
    }
}
