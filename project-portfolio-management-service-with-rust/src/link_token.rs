//! The rules every token carried in a URL follows (see `spec/09-api-surface.md`, "Link tokens").
//! DB-free, unit-tested.
//!
//! A link token is a high-entropy secret: shown once, stored only as a hash
//! ([`crate::feeds::hash_token`]), revocable and expiring. It is either **reusable** (the per-user
//! feed token: a calendar app fetches the same URL on a schedule) or **single-use** (an invite, a
//! share or export link: refused once redeemed). A single-use link is redeemed with one
//! conditional update (`UPDATE … SET redeemed_at = now WHERE hash = $1 AND redeemed_at IS NULL`,
//! succeeding only if a row changed), so two simultaneous requests cannot both win; [`check`] is
//! the read-side rule and the update is the write-side one.

use chrono::{DateTime, Duration, Utc};

/// How long a feed token lives.
pub const FEED_TOKEN_LIFETIME_DAYS: i64 = 365;

/// Why a presented link token is refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refusal {
    /// Revoked by its owner or by a newer token replacing it.
    Revoked,
    /// Past its lifetime.
    Expired,
    /// A single-use token that was already redeemed.
    Redeemed,
}

/// What is stored about one issued token.
#[derive(Debug, Clone, Copy)]
pub struct Link {
    /// When it was issued.
    pub issued_at: DateTime<Utc>,
    /// How many days it lives.
    pub lifetime_days: i64,
    /// When it was revoked, if it was.
    pub revoked_at: Option<DateTime<Utc>>,
    /// When it was redeemed, if it was.
    pub redeemed_at: Option<DateTime<Utc>>,
    /// Whether redeeming it once uses it up.
    pub single_use: bool,
}

/// Whether `link` may be used at `now`.
///
/// # Errors
/// The first of [`Refusal::Revoked`], [`Refusal::Expired`], [`Refusal::Redeemed`] that applies.
pub fn check(link: &Link, now: DateTime<Utc>) -> Result<(), Refusal> {
    if link.revoked_at.is_some() {
        return Err(Refusal::Revoked);
    }
    if now >= link.issued_at + Duration::days(link.lifetime_days) {
        return Err(Refusal::Expired);
    }
    if link.single_use && link.redeemed_at.is_some() {
        return Err(Refusal::Redeemed);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn issued() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 10, 1, 9, 0, 0).unwrap()
    }

    fn link(single_use: bool) -> Link {
        Link {
            issued_at: issued(),
            lifetime_days: 10,
            revoked_at: None,
            redeemed_at: None,
            single_use,
        }
    }

    #[test]
    fn a_fresh_token_is_usable_until_its_lifetime_ends() {
        let l = link(false);
        assert_eq!(check(&l, issued()), Ok(()));
        assert_eq!(
            check(&l, issued() + Duration::days(10) - Duration::seconds(1)),
            Ok(())
        );
        assert_eq!(
            check(&l, issued() + Duration::days(10)),
            Err(Refusal::Expired)
        );
    }

    #[test]
    fn a_revoked_token_is_refused_even_before_it_expires() {
        let l = Link {
            revoked_at: Some(issued()),
            ..link(false)
        };
        assert_eq!(check(&l, issued()), Err(Refusal::Revoked));
    }

    #[test]
    fn a_reusable_token_stays_usable_after_it_is_used() {
        let l = Link {
            redeemed_at: Some(issued()),
            ..link(false)
        };
        assert_eq!(check(&l, issued() + Duration::days(1)), Ok(()));
    }

    #[test]
    fn a_single_use_token_is_refused_once_redeemed() {
        let l = link(true);
        assert_eq!(check(&l, issued()), Ok(()));
        let used = Link {
            redeemed_at: Some(issued()),
            ..l
        };
        assert_eq!(check(&used, issued()), Err(Refusal::Redeemed));
    }

    #[test]
    fn the_feed_lifetime_is_a_year() {
        assert_eq!(FEED_TOKEN_LIFETIME_DAYS, 365);
    }
}
