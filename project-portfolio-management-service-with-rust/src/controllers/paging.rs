//! One paging rule for every list (RM-12b).
//!
//! A `GET` that answers `200` with a **bare JSON array** is a list, and every list
//! pages the same way (`agents/share/restful.md`): `?limit=` (default and ceiling
//! [`MAX_LIMIT`]), `?offset=` (a request past [`MAX_OFFSET`] is `400`), and
//! `X-Total-Count` / `X-Limit` / `X-Offset` headers; the body stays the bare array.
//!
//! Handlers that page in the database (and so already set `X-Total-Count`) are
//! left alone. For all the others this layer slices the array they returned, so a
//! list added tomorrow pages today without anyone remembering to. Streams, files
//! and objects (reports, aggregates) are not arrays and pass through untouched.

use axum::body::Body;
use axum::extract::Request;
use axum::http::{HeaderValue, Method, StatusCode, header};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

use super::pagination::{MAX_LIMIT, MAX_OFFSET};

/// The largest list body this layer will read to page (anything larger passes through).
const MAX_BODY: usize = 32 * 1024 * 1024;

fn query_value(query: Option<&str>, key: &str) -> Option<String> {
    query?
        .split('&')
        .find_map(|kv| kv.strip_prefix(&format!("{key}=")))
        .map(str::to_string)
}

fn bad_request(code: &str, message: &str) -> Response {
    (
        StatusCode::BAD_REQUEST,
        axum::Json(serde_json::json!({ "error": code, "description": message })),
    )
        .into_response()
}

/// `?limit=` / `?offset=` for a handler that pages in the database.
#[derive(Debug, Default, serde::Deserialize)]
pub struct PageQuery {
    /// Page size.
    #[serde(default)]
    pub limit: Option<u64>,
    /// Rows to skip.
    #[serde(default)]
    pub offset: Option<u64>,
}

/// The paging layer. See the module docs.
#[expect(clippy::result_large_err)]
pub async fn paging_mw(req: Request, next: Next) -> Response {
    if req.method() != Method::GET {
        return next.run(req).await;
    }
    let query = req.uri().query().map(str::to_string);
    let parse = |key: &str| -> Result<Option<u64>, Response> {
        match query_value(query.as_deref(), key) {
            None => Ok(None),
            Some(v) => v.parse::<u64>().map(Some).map_err(|_| {
                bad_request("invalid_paging", &format!("{key} must be a whole number"))
            }),
        }
    };
    let (limit, offset) = match (parse("limit"), parse("offset")) {
        (Ok(l), Ok(o)) => (l, o),
        (Err(r), _) | (_, Err(r)) => return r,
    };
    let response = next.run(req).await;
    if response.status() != StatusCode::OK || response.headers().contains_key("x-total-count") {
        return response;
    }
    let is_json = response
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .is_some_and(|v| v.starts_with("application/json"));
    if !is_json {
        return response;
    }
    let (mut parts, body) = response.into_parts();
    let Ok(bytes) = axum::body::to_bytes(body, MAX_BODY).await else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "response too large to page",
        )
            .into_response();
    };
    let rebuilt = |parts: axum::http::response::Parts, bytes: axum::body::Bytes| {
        Response::from_parts(parts, Body::from(bytes))
    };
    // Only a bare array is a list; look at the first byte before parsing anything.
    if bytes.iter().find(|b| !b.is_ascii_whitespace()) != Some(&b'[') {
        return rebuilt(parts, bytes);
    }
    let Ok(serde_json::Value::Array(rows)) = serde_json::from_slice::<serde_json::Value>(&bytes)
    else {
        return rebuilt(parts, bytes);
    };
    if offset.unwrap_or(0) > MAX_OFFSET {
        return bad_request(
            "offset_too_large",
            &format!("offset must not exceed {MAX_OFFSET}; narrow the query instead"),
        );
    }
    let total = rows.len() as u64;
    let limit = limit.filter(|l| *l > 0).unwrap_or(MAX_LIMIT).min(MAX_LIMIT);
    let offset = offset.unwrap_or(0);
    let page: Vec<serde_json::Value> = rows
        .into_iter()
        .skip(usize::try_from(offset).unwrap_or(usize::MAX))
        .take(usize::try_from(limit).unwrap_or(usize::MAX))
        .collect();
    let whole = offset == 0 && total <= limit;
    let body = serde_json::to_vec(&page).unwrap_or_default();
    parts.headers.remove(header::CONTENT_LENGTH);
    // A tag describes the whole list; a page that leaves rows out is not it. A page
    // that is the whole list keeps it, so `If-Match` works on a list (CF-2).
    if !whole {
        parts.headers.remove(header::ETAG);
    }
    for (name, value) in [
        ("x-total-count", total),
        ("x-limit", limit),
        ("x-offset", offset),
    ] {
        if let Ok(v) = HeaderValue::from_str(&value.to_string()) {
            parts.headers.insert(name, v);
        }
    }
    Response::from_parts(parts, Body::from(body))
}
