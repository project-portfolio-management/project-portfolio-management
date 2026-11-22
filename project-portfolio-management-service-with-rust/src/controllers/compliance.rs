//! Integrity-verification endpoints.
//!
//! | Endpoint | What it checks |
//! |---|---|
//! | `GET /api/compliance/records/verify` | Recomputes each plan's three digests and names any row that differs. |
//! | `GET /api/compliance/audit/verify` | Recomputes each audit row's MAC and names any row whose content was altered. |
//!
//! Both sit under `/api/*`, so they are behind the blanket auth + ABAC
//! guard when `PORTFOLIO_REQUIRE_AUTH` is on. They are reads, so the
//! default policy admits any authenticated caller — appropriate, since
//! the point is that an auditor can check the posture, and neither
//! response discloses plan data beyond ids and names.
//!
//! ## Read the caveat in the response
//!
//! A `verified: true` from the audit endpoint attests that no examined
//! row's *content* was altered without the key. It does **not** attest
//! that no row was deleted — nothing in a row can prove its own continued
//! existence. Detecting deletion needs a hash chain plus external-witness
//! checkpoints, which this service does not yet have. The response
//! carries that caveat inline rather than leaving it to documentation,
//! because a bare `verified: true` reads as more than it means.

use crate::prelude::*;
use axa::extract::Query;
use serde::Deserialize;

use crate::compliance::{audit_integrity, record_integrity};
use crate::models::_entities::{audit_logs, plans};

/// Default rows examined in one call.
pub const VERIFY_DEFAULT_LIMIT: u64 = 1000;

/// Hard cap on rows examined in one call.
///
/// Verification is O(rows) with several digests per row, so an unbounded
/// `limit` is a CPU denial-of-service on a large table (the SEC-M1
/// bound-every-input invariant).
pub const VERIFY_MAX_LIMIT: u64 = 10_000;

/// Query string for both verification endpoints.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct VerifyParams {
    /// How many rows to verify: 1 to [`VERIFY_MAX_LIMIT`], [`VERIFY_DEFAULT_LIMIT`] when absent; outside that, `422`.
    limit: Option<u64>,
}

impl VerifyParams {
    /// The limit: the default when absent, `422` when supplied outside `1..=VERIFY_MAX_LIMIT`.
    fn limit(&self) -> Result<u64> {
        super::within(
            self.limit,
            "limit",
            (1, VERIFY_MAX_LIMIT),
            VERIFY_DEFAULT_LIMIT,
        )
    }
}

/// A validated limit as the `bigint` a `LIMIT` binds (it is at most [`VERIFY_MAX_LIMIT`], so it always fits).
fn limit_param(limit: u64) -> i64 {
    i64::try_from(limit).unwrap_or(i64::MAX)
}

/// The verifiers read every plan and every audit row, private ones included, and report counts and the rows that
/// differ, so they are for administrators (and, with enforcement off, for anyone, as for every write).
fn require_admin(caller: &crate::auth::MaybeAuthUser) -> Result<()> {
    if caller.holds_or_open(|c| super::plan_access::is_admin(Some(c))) {
        return Ok(());
    }
    Err(Error::CustomError(
        axum::http::StatusCode::FORBIDDEN,
        axa::error::ErrorDetail::new(
            "forbidden",
            "the integrity verifiers cover every record, private ones included: they need access=admin",
        ),
    ))
}

/// Verify row-level record integrity.
///
/// `GET /api/compliance/records/verify?limit=1000`
///
/// # Errors
///
/// When the query fails.
#[debug_handler]
async fn verify_records(
    State(ctx): State<AppContext>,
    caller: crate::auth::MaybeAuthUser,
    Query(params): Query<VerifyParams>,
) -> Result<Response> {
    require_admin(&caller)?;
    // Every plan, deleted ones included, newest change first, as before.
    let rows: Vec<plans::Model> =
        sqlx::query_as("SELECT * FROM plans ORDER BY updated_at DESC LIMIT $1")
            .bind(limit_param(params.limit()?))
            .fetch_all(crate::db::pool(&ctx))
            .await?;
    format::json(record_integrity::verify(&rows))
}

/// Verify audit-row integrity.
///
/// `GET /api/compliance/audit/verify?limit=1000`
///
/// # Errors
///
/// When the query fails.
#[debug_handler]
async fn verify_audit(
    State(ctx): State<AppContext>,
    caller: crate::auth::MaybeAuthUser,
    Query(params): Query<VerifyParams>,
) -> Result<Response> {
    require_admin(&caller)?;
    let rows: Vec<audit_logs::Model> =
        sqlx::query_as("SELECT * FROM audit_logs ORDER BY id DESC LIMIT $1")
            .bind(limit_param(params.limit()?))
            .fetch_all(crate::db::pool(&ctx))
            .await?;
    format::json(audit_integrity::verify(&rows))
}

/// Route registration.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/compliance")
        .add("/records/verify", get(verify_records))
        .add("/audit/verify", get(verify_audit))
}

#[cfg(test)]
mod tests {
    use super::{VERIFY_DEFAULT_LIMIT, VERIFY_MAX_LIMIT, VerifyParams};

    /// An absent limit is the default; a limit outside the range is refused, not moved to a bound (RCA-IN-2). An
    /// unbounded limit is a CPU denial-of-service: verification recomputes several digests per row.
    #[test]
    fn the_limit_is_refused_outside_its_range() {
        assert_eq!(
            VerifyParams { limit: None }.limit().ok(),
            Some(VERIFY_DEFAULT_LIMIT),
            "an absent limit uses the default"
        );
        for bad in [0, VERIFY_MAX_LIMIT + 1, u64::MAX] {
            assert!(
                VerifyParams { limit: Some(bad) }.limit().is_err(),
                "{bad} is refused"
            );
        }
        assert_eq!(VerifyParams { limit: Some(42) }.limit().ok(), Some(42));
        assert_eq!(
            VerifyParams {
                limit: Some(VERIFY_MAX_LIMIT)
            }
            .limit()
            .ok(),
            Some(VERIFY_MAX_LIMIT)
        );
    }
}
