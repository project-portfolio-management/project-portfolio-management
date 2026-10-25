//! Idempotency keys on writes (BR-1): `Idempotency-Key: <opaque>` on a `POST`.
//!
//! * The same key and the same request returns the **first answer** again (same status and body, header
//!   `Idempotent-Replayed: true`), and nothing is written twice.
//! * The same key with a different request (method, path, query or body) is `409` `idempotency_key_reused`.
//! * The same key while the first request is still running is `409` `idempotency_in_progress` with `Retry-After`.
//! * Keys are **per caller**: two callers using one key do not collide, and a replay is decided after the auth guards
//!   (this layer sits inside them), so a replay never answers a caller the guards would refuse.
//! * Only a `2xx` answer is kept. A refusal or a failure releases the key, so the caller can correct and retry.
//! * A key lives one day. A run that died holding the key is taken over after five minutes.
//! * Bodies over 1 MiB (the request, or the answer) are not supported: the request is refused `422`, the answer is
//!   returned but not kept and the key is released. The bulk import has its own key and is left alone.
//!
//! The decision is pure ([`decide`]) and unit-tested; the layer ([`idempotency_mw`]) is the thin edge around it.

use axum::body::Body;
use axum::extract::{Request, State};
use axum::http::{HeaderMap, HeaderValue, Method, StatusCode, header};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use chrono::{DateTime, Duration, Utc};
use loco_rs::app::AppContext;
use sea_orm::{ConnectionTrait, DatabaseBackend, Statement};

/// The header a caller sends.
pub const HEADER: &str = "idempotency-key";
/// The longest key kept.
pub const MAX_KEY: usize = 200;
/// The largest request or answer body this layer will hold.
pub const MAX_BODY: usize = 1024 * 1024;
/// How long a key is remembered.
pub const TTL: Duration = Duration::hours(24);
/// How long a run may hold a key before another request may take it over.
pub const STALE_AFTER: Duration = Duration::minutes(5);
/// Paths that keep their own idempotency (the bulk import returns the original job).
const NATIVE: &[&str] = &["/api/plans/import"];

/// What a stored row says.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stored {
    /// Hash of the request that took the key.
    pub request_hash: String,
    /// `in_progress` or `done`.
    pub state: String,
    /// When the key was taken.
    pub created_at: DateTime<Utc>,
}

/// What to do with a request that carries a key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision {
    /// No usable row: take the key and run.
    Run,
    /// The row is expired or stale: delete it, then take the key and run.
    TakeOver,
    /// The first request finished: return its stored answer.
    Replay,
    /// The key was used for a different request.
    Reused,
    /// The first request is still running.
    InProgress,
}

/// The key a request carries, trimmed and cut to [`MAX_KEY`]; `None` when absent or blank.
#[must_use]
pub fn key_of(headers: &HeaderMap) -> Option<String> {
    headers
        .get(HEADER)
        .and_then(|v| v.to_str().ok())
        .map(str::trim)
        .filter(|k| !k.is_empty())
        .map(|k| k.chars().take(MAX_KEY).collect())
}

/// A hash of what makes a request the same request: method, path, query and body.
#[must_use]
pub fn request_hash(method: &str, path_and_query: &str, body: &[u8]) -> String {
    let mut data = Vec::with_capacity(body.len() + path_and_query.len() + 16);
    data.extend_from_slice(method.as_bytes());
    data.push(b' ');
    data.extend_from_slice(path_and_query.as_bytes());
    data.push(b'\n');
    data.extend_from_slice(body);
    crate::artifacts::sha256_hex(&data)
}

/// Whether this request can carry a key at all: a `POST` outside the paths with their own.
#[must_use]
pub fn applies(method: &Method, path: &str) -> bool {
    method == Method::POST && !NATIVE.contains(&path)
}

/// Decide what to do with a request, given the row stored under its key (if any).
#[must_use]
pub fn decide(stored: Option<&Stored>, hash: &str, now: DateTime<Utc>) -> Decision {
    let Some(row) = stored else {
        return Decision::Run;
    };
    if now - row.created_at >= TTL {
        return Decision::TakeOver;
    }
    if row.request_hash != hash {
        return Decision::Reused;
    }
    match row.state.as_str() {
        "done" => Decision::Replay,
        _ if now - row.created_at >= STALE_AFTER => Decision::TakeOver,
        _ => Decision::InProgress,
    }
}

fn refuse(status: StatusCode, code: &str, message: &str) -> Response {
    let mut r = (
        status,
        axum::Json(serde_json::json!({ "error": code, "description": message })),
    )
        .into_response();
    if code == "idempotency_in_progress" {
        r.headers_mut()
            .insert(header::RETRY_AFTER, HeaderValue::from_static("1"));
    }
    r
}

fn unavailable() -> Response {
    refuse(
        StatusCode::SERVICE_UNAVAILABLE,
        "idempotency_unavailable",
        "the idempotency store could not be reached, so the request was not run; retry it",
    )
}

fn sql(text: &str, values: Vec<sea_orm::Value>) -> Statement {
    Statement::from_sql_and_values(DatabaseBackend::Postgres, text.to_string(), values)
}

fn replay_response(r: &sea_orm::QueryResult) -> Response {
    let status = r
        .try_get::<i32>("", "status")
        .ok()
        .and_then(|s| u16::try_from(s).ok())
        .and_then(|s| StatusCode::from_u16(s).ok())
        .unwrap_or(StatusCode::OK);
    let body = r.try_get::<Vec<u8>>("", "body").unwrap_or_default();
    let mut response = Response::new(Body::from(body));
    *response.status_mut() = status;
    if let Ok(Some(ct)) = r.try_get::<Option<String>>("", "content_type")
        && let Ok(v) = HeaderValue::from_str(&ct)
    {
        response.headers_mut().insert(header::CONTENT_TYPE, v);
    }
    response
        .headers_mut()
        .insert("idempotent-replayed", HeaderValue::from_static("true"));
    response
}

/// The request as it identifies itself to the store.
struct Ident {
    caller: String,
    key: String,
    method: String,
    path: String,
    hash: String,
}

/// Take the key, or say what to answer instead: `Ok(())` means this request holds the key and should run.
#[expect(clippy::result_large_err)] // the Err is the answer to send, as in the other layers
async fn claim(db: &sea_orm::DatabaseConnection, id: &Ident) -> Result<(), Response> {
    for _ in 0..3 {
        let inserted = db
            .execute_raw(sql(
                "INSERT INTO idempotency_keys (caller, key, method, path, request_hash) \
                 VALUES ($1, $2, $3, $4, $5) ON CONFLICT (caller, key) DO NOTHING",
                vec![
                    id.caller.clone().into(),
                    id.key.clone().into(),
                    id.method.clone().into(),
                    id.path.clone().into(),
                    id.hash.clone().into(),
                ],
            ))
            .await
            .map_err(|_| unavailable())?;
        if inserted.rows_affected() == 1 {
            return Ok(());
        }
        let row = db
            .query_one_raw(sql(
                "SELECT request_hash, state, created_at, status, content_type, body \
                 FROM idempotency_keys WHERE caller = $1 AND key = $2",
                vec![id.caller.clone().into(), id.key.clone().into()],
            ))
            .await
            .map_err(|_| unavailable())?;
        let stored = row.as_ref().and_then(|r| {
            Some(Stored {
                request_hash: r.try_get("", "request_hash").ok()?,
                state: r.try_get("", "state").ok()?,
                created_at: r
                    .try_get::<chrono::DateTime<chrono::FixedOffset>>("", "created_at")
                    .ok()?
                    .to_utc(),
            })
        });
        match decide(stored.as_ref(), &id.hash, Utc::now()) {
            Decision::Run => {}
            Decision::TakeOver => release(db, id).await.map_err(|()| unavailable())?,
            Decision::Reused => {
                return Err(refuse(
                    StatusCode::CONFLICT,
                    "idempotency_key_reused",
                    "this Idempotency-Key was used for a different request",
                ));
            }
            Decision::InProgress => {
                return Err(refuse(
                    StatusCode::CONFLICT,
                    "idempotency_in_progress",
                    "the first request with this Idempotency-Key is still running; retry shortly",
                ));
            }
            Decision::Replay => {
                return Err(row.as_ref().map_or_else(unavailable, replay_response));
            }
        }
    }
    Err(unavailable())
}

async fn release(db: &sea_orm::DatabaseConnection, id: &Ident) -> Result<(), ()> {
    db.execute_raw(sql(
        "DELETE FROM idempotency_keys WHERE caller = $1 AND key = $2",
        vec![id.caller.clone().into(), id.key.clone().into()],
    ))
    .await
    .map(|_| ())
    .map_err(|_| ())
}

/// The layer. See the module docs.
pub async fn idempotency_mw(State(ctx): State<AppContext>, req: Request, next: Next) -> Response {
    let Some(key) = key_of(req.headers()).filter(|_| applies(req.method(), req.uri().path()))
    else {
        return next.run(req).await;
    };
    let caller = crate::auth::request_claims(req.headers())
        .await
        .map_or_else(|_| "-".to_string(), |c| c.sub);
    let (parts, body) = req.into_parts();
    let Ok(bytes) = axum::body::to_bytes(body, MAX_BODY).await else {
        return refuse(
            StatusCode::UNPROCESSABLE_ENTITY,
            "idempotency_body_too_large",
            "an Idempotency-Key is supported for request bodies up to 1 MiB",
        );
    };
    let path_and_query = parts
        .uri
        .path_and_query()
        .map_or_else(|| parts.uri.path().to_string(), ToString::to_string);
    let id = Ident {
        caller,
        key,
        method: parts.method.to_string(),
        path: parts.uri.path().to_string(),
        hash: request_hash(parts.method.as_str(), &path_and_query, &bytes),
    };
    if let Err(response) = claim(&ctx.db, &id).await {
        return response;
    }

    // We hold the key: run the request, then keep a 2xx answer or release the key.
    let (head, body) = next
        .run(Request::from_parts(parts, Body::from(bytes)))
        .await
        .into_parts();
    let answer = axum::body::to_bytes(body, MAX_BODY).await;
    let kept = match (&answer, head.status.is_success()) {
        (Ok(bytes), true) => ctx
            .db
            .execute_raw(sql(
                "UPDATE idempotency_keys SET state = 'done', status = $3, content_type = $4, body = $5, \
                 updated_at = now() WHERE caller = $1 AND key = $2",
                vec![
                    id.caller.clone().into(),
                    id.key.clone().into(),
                    i32::from(head.status.as_u16()).into(),
                    head.headers
                        .get(header::CONTENT_TYPE)
                        .and_then(|v| v.to_str().ok())
                        .map(str::to_string)
                        .into(),
                    bytes.to_vec().into(),
                ],
            ))
            .await
            .is_ok(),
        _ => false,
    };
    if !kept {
        // A refusal, a failure or an answer too large to keep: release the key so the caller can retry.
        let _ = release(&ctx.db, &id).await;
    }
    match answer {
        Ok(bytes) => Response::from_parts(head, Body::from(bytes)),
        Err(_) => refuse(
            StatusCode::INTERNAL_SERVER_ERROR,
            "idempotency_answer_too_large",
            "the answer was too large to read back; the request ran once and its key was released",
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(hash: &str, state: &str, age: Duration, now: DateTime<Utc>) -> Stored {
        Stored {
            request_hash: hash.into(),
            state: state.into(),
            created_at: now - age,
        }
    }

    #[test]
    fn the_key_is_trimmed_cut_and_blank_means_none() {
        let mut h = HeaderMap::new();
        assert_eq!(key_of(&h), None);
        h.insert(HEADER, HeaderValue::from_static("  abc  "));
        assert_eq!(key_of(&h).as_deref(), Some("abc"));
        h.insert(HEADER, HeaderValue::from_static("   "));
        assert_eq!(key_of(&h), None);
        h.insert(HEADER, HeaderValue::from_str(&"k".repeat(300)).unwrap());
        assert_eq!(key_of(&h).unwrap().len(), MAX_KEY);
    }

    #[test]
    fn the_hash_changes_with_method_path_query_and_body() {
        let base = request_hash("POST", "/api/plans?x=1", b"{}");
        assert_eq!(base, request_hash("POST", "/api/plans?x=1", b"{}"));
        assert_ne!(base, request_hash("POST", "/api/plans?x=2", b"{}"));
        assert_ne!(base, request_hash("POST", "/api/plans?x=1", b"{ }"));
        assert_ne!(base, request_hash("PUT", "/api/plans?x=1", b"{}"));
        assert_ne!(base, request_hash("POST", "/api/plan?x=1", b"{}"));
    }

    #[test]
    fn only_a_post_outside_the_native_paths_can_carry_a_key() {
        assert!(applies(&Method::POST, "/api/plans"));
        assert!(!applies(&Method::GET, "/api/plans"));
        assert!(!applies(&Method::PUT, "/api/plans/x"));
        assert!(!applies(&Method::POST, "/api/plans/import"));
    }

    #[test]
    fn a_stored_row_decides_run_replay_conflict_or_wait() {
        let now = Utc::now();
        assert_eq!(decide(None, "h", now), Decision::Run);
        assert_eq!(
            decide(Some(&row("h", "done", Duration::minutes(1), now)), "h", now),
            Decision::Replay
        );
        assert_eq!(
            decide(
                Some(&row("h", "done", Duration::minutes(1), now)),
                "other",
                now
            ),
            Decision::Reused
        );
        assert_eq!(
            decide(
                Some(&row("h", "in_progress", Duration::seconds(10), now)),
                "h",
                now
            ),
            Decision::InProgress
        );
        assert_eq!(
            decide(
                Some(&row("h", "in_progress", Duration::seconds(10), now)),
                "other",
                now
            ),
            Decision::Reused
        );
    }

    #[test]
    fn an_expired_or_abandoned_row_is_taken_over() {
        let now = Utc::now();
        // Older than a day: gone, whatever it held and whatever the new request is.
        assert_eq!(
            decide(
                Some(&row("h", "done", Duration::hours(25), now)),
                "other",
                now
            ),
            Decision::TakeOver
        );
        // A run that never finished: taken over after five minutes, not before.
        assert_eq!(
            decide(
                Some(&row("h", "in_progress", Duration::minutes(6), now)),
                "h",
                now
            ),
            Decision::TakeOver
        );
        assert_eq!(
            decide(
                Some(&row("h", "in_progress", Duration::minutes(4), now)),
                "h",
                now
            ),
            Decision::InProgress
        );
    }
}
