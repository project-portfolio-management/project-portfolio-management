//! The caller: the `AuthUser` and `MaybeAuthUser` extractors and the attribute helpers handlers use.

#[allow(
    clippy::wildcard_imports,
    reason = "the parent module holds the shared imports and the other parts"
)]
use super::*;

/// A request whose bearer token passed PASETO signature / issuer /
/// audience / expiry verification. The wrapped [`Claims`] identify the
/// caller (`sub` is the user `pid`). Taking this argument makes a handler
/// require authentication.
pub struct AuthUser(pub Claims);

/// Extracting an [`AuthUser`] verifies the request's bearer token against
/// the process-wide [`verifier`]; a missing/invalid token rejects with
/// `401` before the handler runs, so the type is the "require auth" gate.
impl<S: Send + Sync> FromRequestParts<S> for AuthUser {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        request_claims(&parts.headers).await.map(AuthUser)
    }
}

/// Like [`AuthUser`], but never rejects: yields `Some(claims)` when a
/// valid bearer token is present and `None` otherwise. Handlers use it to
/// stamp the caller identity (e.g. the audit `actor`) without requiring
/// authentication on the route.
pub struct MaybeAuthUser(pub Option<Claims>);

impl MaybeAuthUser {
    /// The caller's `sub` (user `pid`) if a valid token was presented.
    #[must_use]
    pub fn actor(&self) -> Option<&str> {
        self.0.as_ref().map(|c| c.sub.as_str())
    }

    /// The verified [`Claims`] if a valid token was presented, for the
    /// record-level authorization pass ([`authorize_record`]).
    #[must_use]
    pub fn claims(&self) -> Option<&Claims> {
        self.0.as_ref()
    }

    /// Whether the caller passes `test`, where a route lets **anyone** act when enforcement is off
    /// (there is no caller to refuse). With enforcement on, no claims is never a pass (CQ-2): the
    /// guard has already required a token, but one can expire, or its key be retired, between the
    /// guard and the handler, and that must refuse, not grant.
    #[must_use]
    pub fn holds_or_open(&self, test: impl FnOnce(&Claims) -> bool) -> bool {
        holds_or_open(require_auth(), self.claims(), test)
    }
}

/// [`MaybeAuthUser::holds_or_open`] with the enforcement posture passed in. Pure.
#[must_use]
pub fn holds_or_open(
    require_auth: bool,
    claims: Option<&Claims>,
    test: impl FnOnce(&Claims) -> bool,
) -> bool {
    claims.map_or(!require_auth, test)
}

/// Whether `claims` carry the attribute `key` with the value `want`.
#[must_use]
pub fn has_attr(claims: &Claims, key: &str, want: &str) -> bool {
    claims
        .attrs
        .get(key)
        .is_some_and(|v| v.iter().any(|a| a == want))
}

/// The **record-level resource attributes** of a plan for the
/// ABAC decision (`authorization-attributes.md` §9): today the
/// phase-gate `resource.stage` (PPM-3), so a deployment can write
/// e.g. "deny write past `g3_delivery` unless `access=admin`" purely
/// as policy. An item with no stage yet exposes no `stage` key.
#[must_use]
pub fn plan_resource_attrs(stage: Option<&str>) -> std::collections::BTreeMap<String, Vec<String>> {
    let mut attrs = std::collections::BTreeMap::new();
    if let Some(stage) = stage {
        attrs.insert("stage".to_string(), vec![stage.to_string()]);
    }
    attrs
}

/// **Record-level** authorization for a handler that has loaded its
/// target: evaluate the policy with the record's resource attributes.
/// Gated on the same `PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH` flag
/// as the blanket guard — a no-op returning no obligations when
/// enforcement is off.
///
/// # Errors
///
/// `401` if enforcement is on but no verified claims are present
/// (fail-safe behind the guard); `403` when the policy denies the
/// action given the record's attributes.
pub fn authorize_record(
    caller: &MaybeAuthUser,
    action: Action,
    resource: &std::collections::BTreeMap<String, Vec<String>>,
) -> Result<Vec<String>, (StatusCode, String)> {
    if !require_auth() {
        return Ok(Vec::new());
    }
    let claims = caller
        .claims()
        .ok_or((StatusCode::UNAUTHORIZED, "missing bearer token".to_string()))?;
    let decision = policy()
        .current()
        .evaluate_with_resource(claims, action, ENTITY, resource);
    if decision.allowed {
        Ok(decision.obligations)
    } else {
        Err((StatusCode::FORBIDDEN, decision.reason))
    }
}

/// Extracting a [`MaybeAuthUser`] never rejects: a valid token yields
/// `Some(claims)`, anything else `None`. Handlers use it to opportunistically
/// stamp the caller identity without requiring authentication.
impl<S: Send + Sync> FromRequestParts<S> for MaybeAuthUser {
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        Ok(Self(request_claims(&parts.headers).await.ok()))
    }
}
