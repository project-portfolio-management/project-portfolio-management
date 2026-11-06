//! API documentation endpoints: the `OpenAPI` spec + a Swagger UI page.
//!
//! Swagger UI assets are loaded from a CDN (swagger-ui-dist), pinned by `axa::openapi`, to keep the
//! crate dependency-light; the spec itself is served from this service.

use axa::prelude::*;

/// The `OpenAPI` 3 document.
#[debug_handler]
async fn openapi_json() -> Result<Response> {
    format::json(crate::openapi::spec())
}

/// A Swagger UI page wired to `/api-docs/openapi.json`. One exact version with integrity hashes, so the browser refuses a
/// script that has changed. `lang="en"` is the truth, not a default (WW-1.13): Swagger UI's own labels and every description
/// in the `OpenAPI` document are English. Translating the API reference is a stated limit (service spec).
#[debug_handler]
async fn swagger_ui() -> Result<Response> {
    format::html(&axa::openapi::swagger_html(
        "Portfolio Service API",
        axa::openapi::SPEC_PATH,
    ))
}

/// Routes for the API documentation: the raw `OpenAPI` JSON and the
/// Swagger UI page that renders it.
pub fn routes() -> Routes {
    Routes::new()
        .add("/api-docs/openapi.json", get(openapi_json))
        .add("/swagger-ui", get(swagger_ui))
}
