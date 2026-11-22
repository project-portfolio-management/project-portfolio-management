//! Prometheus metrics endpoint.
//!
//! Exposes the process-wide registry from [`crate::metrics`] at
//! `GET /metrics.prom` (mounted at the **root**, not under `/api`) in
//! Prometheus text-exposition format. With login enforcement on (the
//! default) the route needs a bearer token like any other: it is not on
//! the public list (`crate::auth::is_public_path`), so give the scraper a
//! token. Configure it with `metrics_path: /metrics.prom`.

use crate::prelude::*;
use axum::http::{HeaderValue, StatusCode, header::CONTENT_TYPE};
use axum::response::{IntoResponse, Response};

use crate::metrics::{self, Metrics};

/// Render the process-wide registry in Prometheus text-exposition format.
///
/// Responds `200` with `Content-Type: text/plain; version=0.0.4` (see
/// [`metrics::CONTENT_TYPE`]) and the gathered metric families as the body.
#[debug_handler]
async fn metrics_prom() -> Response {
    // No metric set, or an encoder error, is a stated 500 for the scrape, not a dropped connection.
    match Metrics::global().map(Metrics::render) {
        Some(Ok(body)) => (
            StatusCode::OK,
            [(
                CONTENT_TYPE,
                HeaderValue::from_static(metrics::CONTENT_TYPE),
            )],
            body,
        )
            .into_response(),
        Some(Err(error)) => {
            tracing::error!(%error, "the metrics could not be rendered");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
        None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// The `GET /metrics.prom` route, mounted at the root (no `/api` prefix) to
/// mirror the conventional Prometheus scrape path.
pub fn routes() -> Routes {
    Routes::new().add("/metrics.prom", get(metrics_prom))
}
