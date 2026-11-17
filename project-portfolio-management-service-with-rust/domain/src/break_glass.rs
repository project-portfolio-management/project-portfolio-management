//! Break-glass access (RG-10): the pure rules. A named, eligible person opens a **private** plan for a
//! limited time by stating a reason; the grant ends by itself. What it grants is *visibility* (the
//! plan stops being hidden from them); what they may then do is still decided by their attributes.

use chrono::{DateTime, Duration, Utc};

/// The longest a grant may last, in minutes (four hours).
pub const MAX_MINUTES: i64 = 240;
/// How long a grant lasts when the request does not say.
pub const DEFAULT_MINUTES: i64 = 60;
/// The shortest reason, in characters: long enough that "because" is not one.
pub const MIN_REASON: usize = 20;
/// The longest reason, in characters.
pub const MAX_REASON: usize = 1_000;
/// Grants one person may open in a rolling day.
pub const MAX_PER_DAY: u64 = 5;

/// Everything wrong with a request, in plain words; empty means it can proceed.
#[must_use]
pub fn problems(
    reason: &str,
    minutes: Option<i64>,
    notify_ref: &str,
    own_sub: &str,
) -> Vec<String> {
    let mut out = Vec::new();
    let r = reason.trim().chars().count();
    if r < MIN_REASON {
        out.push(format!(
            "the reason must say why, in at least {MIN_REASON} characters"
        ));
    }
    if reason.chars().count() > MAX_REASON {
        out.push(format!("the reason is longer than {MAX_REASON} characters"));
    }
    if let Some(m) = minutes
        && !(1..=MAX_MINUTES).contains(&m)
    {
        out.push(format!("minutes must be from 1 to {MAX_MINUTES}"));
    }
    if !crate::issues::is_person_ref(notify_ref) {
        out.push("notify_ref must be a person: or worker: reference".to_string());
    } else if notify_ref
        .split_once(':')
        .is_some_and(|(_, id)| id.trim().eq_ignore_ascii_case(own_sub.trim()))
    {
        out.push("the person to notify cannot be yourself".to_string());
    }
    out
}

/// When a grant opened at `now` ends.
#[must_use]
pub fn expires_at(now: DateTime<Utc>, minutes: Option<i64>) -> DateTime<Utc> {
    now + Duration::minutes(minutes.unwrap_or(DEFAULT_MINUTES).clamp(1, MAX_MINUTES))
}

/// Whether a grant is in force at `now`: not revoked, and not yet expired.
#[must_use]
pub fn is_live(expires_at: DateTime<Utc>, revoked: bool, now: DateTime<Utc>) -> bool {
    !revoked && expires_at > now
}

/// The notice sent to the person who must be told.
#[must_use]
pub fn notice(grantee: &str, minutes: i64, reason: &str) -> String {
    let short: String = reason.trim().chars().take(200).collect();
    format!(
        "Break-glass access: {grantee} opened a private plan for {minutes} minutes. Reason given: {short}"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(h: u32, m: u32) -> DateTime<Utc> {
        DateTime::parse_from_rfc3339(&format!("2026-10-10T{h:02}:{m:02}:00Z"))
            .unwrap()
            .with_timezone(&Utc)
    }

    #[test]
    fn a_good_request_has_no_problems() {
        let reason = "Incident 4412: the plan owner is unreachable and the release is blocked";
        assert!(problems(reason, Some(90), "person:b", "a").is_empty());
        assert!(problems(reason, None, "worker:b", "a").is_empty());
    }

    #[test]
    fn a_thin_reason_a_long_window_or_a_bad_second_person_are_each_refused() {
        let good = "Incident 4412: the plan owner is unreachable and the release is blocked";
        assert_eq!(problems("because", Some(30), "person:b", "a").len(), 1);
        assert_eq!(
            problems(&" ".repeat(60), Some(30), "person:b", "a").len(),
            1,
            "spaces are not a reason"
        );
        assert_eq!(problems(good, Some(0), "person:b", "a").len(), 1);
        assert_eq!(problems(good, Some(241), "person:b", "a").len(), 1);
        assert_eq!(problems(good, Some(-5), "person:b", "a").len(), 1);
        assert_eq!(problems(good, Some(30), "b", "a").len(), 1);
        assert_eq!(problems(good, Some(30), "person:", "a").len(), 1);
        assert_eq!(
            problems(&"x".repeat(1001), Some(30), "person:b", "a").len(),
            1
        );
    }

    #[test]
    fn the_second_person_cannot_be_yourself_in_any_form() {
        let good = "Incident 4412: the plan owner is unreachable and the release is blocked";
        for me in ["person:ADA", "worker:ada", "person: ada "] {
            assert_eq!(problems(good, Some(30), me, "ada").len(), 1, "{me}");
        }
    }

    #[test]
    fn the_window_is_capped_at_four_hours_whatever_is_asked() {
        assert_eq!(expires_at(at(9, 0), None), at(10, 0));
        assert_eq!(expires_at(at(9, 0), Some(30)), at(9, 30));
        assert_eq!(expires_at(at(9, 0), Some(10_000)), at(13, 0));
        assert_eq!(expires_at(at(9, 0), Some(-3)), at(9, 1));
    }

    #[test]
    fn a_grant_is_live_until_it_expires_or_is_revoked() {
        assert!(is_live(at(10, 0), false, at(9, 59)));
        assert!(
            !is_live(at(10, 0), false, at(10, 0)),
            "the instant of expiry is over"
        );
        assert!(!is_live(at(10, 0), true, at(9, 0)));
    }
}
