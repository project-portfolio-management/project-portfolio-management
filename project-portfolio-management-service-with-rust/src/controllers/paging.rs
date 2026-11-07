//! One paging rule for every list (RM-12b, FR-46).
//!
//! A `GET` that answers `200` with a **bare JSON array** is a list, and every list
//! pages the same way (`agents/restful.md`): `?limit=` (1 to [`MAX_LIMIT`], which is
//! also the page when none is asked for), an opaque signed `?cursor=`, and
//! `Link: <…>; rel="next"` while more rows follow. There is no total, and no `?offset=`:
//! a `GET` that sends one is refused, so a client written for offsets cannot loop on
//! the first page for ever.
//!
//! Handlers that page in the database (and so mark their response
//! [`super::pagination::Paged`]) are left alone. For all the others this layer slices
//! the array they returned, so a list added tomorrow pages today without anyone
//! remembering to. Streams, files and objects (reports, aggregates) are not arrays and
//! pass through untouched.

use axum::body::Body;
use axum::extract::{Request, State};
use axum::http::{Method, StatusCode, header};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

use super::pagination::{MAX_LIMIT, Paged, page_with};
use crate::cursor::Cursors;
use crate::prelude::*;

/// The largest list body this layer will read to page (anything larger passes through).
const MAX_BODY: usize = 32 * 1024 * 1024;

/// Whether a raw query string names `key`.
fn names(query: Option<&str>, key: &str) -> bool {
    query.is_some_and(|q| {
        q.split('&')
            .any(|part| part.split_once('=').map_or(part, |(name, _)| name) == key)
    })
}

/// The paging layer. See the module docs.
pub async fn paging_mw(State(ctx): State<AppContext>, req: Request, next: Next) -> Response {
    if req.method() != Method::GET {
        return next.run(req).await;
    }
    if names(req.uri().query(), "offset") {
        return axa::Error::CustomError(
            StatusCode::BAD_REQUEST,
            axa::error::ErrorDetail::new(
                "offset_not_supported",
                "lists page by cursor: follow the Link header's next page instead of sending offset",
            ),
        )
        .into_response();
    }
    let uri = req.uri().clone();
    let response = next.run(req).await;
    if response.status() != StatusCode::OK || response.extensions().get::<Paged>().is_some() {
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
    let page = match page_with(&Cursors::of(&ctx).0, &uri, MAX_LIMIT, MAX_LIMIT) {
        Ok(page) => page,
        Err(error) => return error.into_response(),
    };
    let (rows, more) = page.split(page.slice(rows));
    let whole = page.offset() == 0 && !more;
    let body = serde_json::to_vec(&rows).unwrap_or_default();
    parts.headers.remove(header::CONTENT_LENGTH);
    // A tag describes the whole list; a page that leaves rows out is not it. A page
    // that is the whole list keeps it, so `If-Match` works on a list (CF-2).
    if !whole {
        parts.headers.remove(header::ETAG);
    }
    let mut response = Response::from_parts(parts, Body::from(body));
    page.stamp(&mut response, more);
    response
}

#[cfg(test)]
mod tests {
    use super::names;

    #[test]
    fn offset_is_found_by_name_only() {
        assert!(names(Some("a=1&offset=20"), "offset"));
        assert!(names(Some("offset"), "offset"));
        assert!(!names(Some("tz_offset=2&q=offset"), "offset"));
        assert!(!names(None, "offset"));
    }
}
