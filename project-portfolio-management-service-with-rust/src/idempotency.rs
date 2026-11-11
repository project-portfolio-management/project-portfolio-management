//! Idempotency keys on writes (BR-1, NFR-7): `Idempotency-Key: <opaque>` on a `POST`, kept by `axa::idempotency`.
//!
//! That crate does the work: the claim, the replay with `Idempotent-Replayed: true`, the lease a running request
//! renews, and the refusals (`422` for a key reused on a different request, `409` while the first is running or when
//! its answer could not be kept, `413` for a body over 1 MiB, `503` when the store is down). This module says three
//! things it cannot know:
//!
//! * **What is covered** ([`covers`]): a `POST`, except the bulk import, which keeps a key of its own.
//! * **Whose key it is** ([`caller_mw`]): the token's subject. With enforcement off a request with no token is
//!   `anonymous`, and such requests share their keys; with it on, such a request never gets this far, and if one did
//!   the crate would refuse it (`401`) rather than run it unprotected.
//! * **Where it sits**: inside the authentication, guest and privacy guards (`app.rs`), so a replay only ever answers a
//!   caller those guards let through, and inside compression, so the answer kept is the plain body.
//!
//! Expired answers are deleted by the retention rule `idempotency_keys` (`retention.rs`), not by the crate's own sweep.

use axa::idempotency::Idempotency;
use axum::extract::Request;
use axum::http::Method;
use axum::middleware::Next;
use axum::response::Response;

/// Paths that keep their own idempotency (the bulk import returns the original job).
const NATIVE: &[&str] = &["/api/plans/import"];

/// The caller name given to a request with no token, where enforcement is off.
pub const ANONYMOUS: &str = "anonymous";

/// Who a key belongs to, left on the request by [`caller_mw`] for the crate to read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Caller(pub String);

/// Whether a request is covered: a `POST` outside the paths with their own key.
#[must_use]
pub fn covers(method: &Method, path: &str) -> bool {
    method == Method::POST && !NATIVE.contains(&path)
}

/// The key store, on the service's own pool.
#[must_use]
pub fn keys(ctx: &axa::AppContext) -> Idempotency {
    Idempotency::new(crate::db::for_axa_crate(ctx), |request| {
        request
            .extensions()
            .get::<Caller>()
            .map(|caller| caller.0.clone())
    })
    .only(covers)
}

/// The caller a request names: the token's subject, or [`ANONYMOUS`] where enforcement is off; `None` otherwise.
#[must_use]
pub fn caller_of(subject: Option<String>, enforced: bool) -> Option<String> {
    subject.or_else(|| (!enforced).then(|| ANONYMOUS.to_string()))
}

/// Name the caller of a covered request that carries a key, for the layer inside. Reads the token only then.
pub async fn caller_mw(mut request: Request, next: Next) -> Response {
    if covers(request.method(), request.uri().path())
        && request.headers().contains_key(axa::idempotency::HEADER)
    {
        let subject = crate::auth::request_claims(request.headers())
            .await
            .ok()
            .map(|claims| claims.sub);
        if let Some(caller) = caller_of(subject, crate::auth::require_auth()) {
            request.extensions_mut().insert(Caller(caller));
        }
    }
    next.run(request).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_a_post_outside_the_native_paths_is_covered() {
        assert!(covers(&Method::POST, "/api/plans"));
        assert!(!covers(&Method::GET, "/api/plans"));
        assert!(!covers(&Method::PUT, "/api/plans/x"));
        assert!(!covers(&Method::DELETE, "/api/plans/x"));
        assert!(!covers(&Method::POST, "/api/plans/import"));
    }

    #[test]
    fn a_key_belongs_to_the_subject_and_to_no_one_when_enforcement_is_on() {
        assert_eq!(caller_of(Some("ana".into()), true).as_deref(), Some("ana"));
        assert_eq!(caller_of(Some("ana".into()), false).as_deref(), Some("ana"));
        assert_eq!(caller_of(None, false).as_deref(), Some(ANONYMOUS));
        assert_eq!(caller_of(None, true), None);
    }
}
