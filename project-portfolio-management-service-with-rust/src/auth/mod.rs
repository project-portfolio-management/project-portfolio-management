//! Bearer-token authentication for the portfolio API.
//!
//! [`AuthUser`] is an Axum extractor that pulls `Authorization: Bearer
//! <paseto>`, verifies the PASETO `v4.public` (Ed25519) signature, issuer,
//! audience and expiry against the
//! [authentication-service](../../../authentication/authentication-service-with-loco)
//! published key set, and yields the verified [`Claims`]. Verification is
//! stateless and offline — no database hit, no introspection call — so any
//! handler can require authentication by taking an `AuthUser` argument, and
//! a handler that wants the caller identity *when present* (e.g. to stamp
//! an audit `actor`) takes [`MaybeAuthUser`] instead.
//!
//! ## Key source
//!
//! The process-wide [`verifier`] is seeded once at boot ([`init`] is
//! called from `App::after_routes`, before the app serves traffic) and
//! built from the environment:
//!
//! - `PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS_URL` — optional URL of the auth service's
//!   published key set (`/.well-known/paseto-keys`). Set (non-blank) ⇒
//!   the key set is fetched over HTTP **once at boot** via
//!   [`Verifier::from_paseto_keys_url`]; on success the fetched key set
//!   wins over `PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS` (`tracing::info!`), on failure the
//!   service logs a `tracing::warn!` and falls back to the env path
//!   below — the service always boots. There is no refresh loop; a
//!   rotation-triggered refetch is a future spec item.
//! - `PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS` — the Ed25519 key set (JSON, OKP/Ed25519
//!   JWK form) the auth service publishes at `/.well-known/paseto-keys`.
//!   Absent ⇒ an empty key set, so every token is rejected (the service
//!   still boots).
//! - `PROJECT_PORTFOLIO_MANAGEMENT_TOKEN_ISSUER` — expected `iss` (default
//!   `authentication-service`).
//! - `PROJECT_PORTFOLIO_MANAGEMENT_TOKEN_AUDIENCE` — expected `aud` (default
//!   `main-x-service`).
//!
//! ## Blanket enforcement
//!
//! Enforcement is **on by default** ([`resolve_require_auth`]): the [`enforce`]
//! decision — wired as an Axum middleware layer in `src/app.rs` — requires a
//! valid bearer token on every route except the public health/ping,
//! OpenAPI/Swagger and calendar-feed paths (see [`is_public_path`]; the
//! Prometheus endpoint needs a token like everything else). It is turned
//! off only explicitly: `PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH=false`
//! (`0`/`false`/`no`/`off`), or `settings.require_auth: false` in the active
//! `config/<env>.yaml` (the shipped `development.yaml` and `test.yaml` do
//! this; `production.yaml` never does). The environment variable wins over
//! the file. [`check_startup`] logs a loud warning when it is off, and the
//! service **refuses to start** when the loco environment is `production`
//! and it is off, or when it is on and no token verifier has keys.
//!
//! ## Authorization (ABAC)
//!
//! Inside the same guard — so it applies only when
//! `PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH` is on — a verified token is further checked
//! against an **attribute-based access control** policy per
//! `agents/share/authorization-attributes.md`: the request's action is
//! derived from the HTTP method plus this crate's destructive named
//! POSTs ([`DESTRUCTIVE_POST_SUFFIXES`]), and the shared engine in the
//! `authentication-verifier` crate evaluates the policy over the
//! token's `attrs` claim. The four collections (portfolios / projects /
//! products / programs) share one guard and one [`ENTITY`] in v1. The
//! policy is read once at first use ([`policy`], built by
//! [`policy_from_env`]) from `PROJECT_PORTFOLIO_MANAGEMENT_ABAC_POLICY` (inline JSON) or
//! `PROJECT_PORTFOLIO_MANAGEMENT_ABAC_POLICY_FILE` (path); unset or unparsable ⇒ the
//! built-in default policy (`svc=true` ⇒ everything; `access=admin` ⇒
//! destructive+write; `access=write` ⇒ write; otherwise read-only) —
//! the service always boots. **401** = missing/bad credential; **403**
//! = valid credential, policy denied (body carries the deciding rule).

use std::sync::OnceLock;

use authentication_verifier::{
    Action, ActionPattern, Claims, Effect, Policy, ReloadablePolicy, Rule,
};

#[cfg(feature = "paseto")]
use authentication_verifier::{ReloadableVerifier, Verifier};

use axum::extract::FromRequestParts;

use axum::http::header::AUTHORIZATION;

use axum::http::request::Parts;

use axum::http::{HeaderMap, Method, StatusCode};

/// The resource entity this crate guards, as seen by ABAC policies
/// (the `entity` pseudo-attribute in rule `when` clauses). The four
/// REST collections (portfolios / projects / products / programs) are
/// one entity to the guard in v1 — do not per-collection this.
pub const ENTITY: &str = "portfolio";

/// Path suffixes of this crate's **destructive named POSTs** (per
/// `authorization-attributes.md` §2), fixed family-wide: record
/// **merge** (`POST /api/{collection}/merge`, live today), batch
/// **deduplicate** and bulk **import** (`POST
/// /api/{collection}/import` per `bulk-import-export.md` §4 — both
/// deferred spec §13 items whose routes end with these suffixes when
/// they land). A POST whose path ends with one of these derives
/// [`Action::Destructive`] instead of [`Action::Write`]; the
/// import-job **status** reads (`GET …/import/{id}`) are GETs and
/// derive [`Action::Read`] — only POST consults this list.
pub const DESTRUCTIVE_POST_SUFFIXES: [&str; 3] = ["/merge", "/deduplicate", "/import"];

mod areas;
mod enforcement;
mod identity;
pub mod oidc;
mod policy;
mod verifier;

pub use areas::*;
pub use enforcement::*;
pub use identity::*;
pub use policy::*;
pub use verifier::*;

#[cfg(test)]
mod area_tests;
#[cfg(all(test, feature = "paseto"))]
mod tests;
