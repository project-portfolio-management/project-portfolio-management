//! The announcement feed: the pure rules. A message is **scheduled** before its
//! `publish_at`, **live** from then until `expires_at` (exclusive), and
//! **expired** after. The feed shows the live ones: pinned first, then by
//! severity (urgent, important, info), then newest first, and the id breaks a
//! tie so the order is total and stable.

use chrono::{DateTime, Utc};
use serde::Serialize;

/// How loud an announcement is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    /// Routine news.
    Info,
    /// Worth reading today.
    Important,
    /// Needs attention now.
    Urgent,
}

impl Severity {
    /// Parse `info` / `important` / `urgent`.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "info" => Some(Self::Info),
            "important" => Some(Self::Important),
            "urgent" => Some(Self::Urgent),
            _ => None,
        }
    }
}

/// Where an announcement is in its window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum State {
    /// Not yet published.
    Scheduled,
    /// Showing.
    Live,
    /// Past its expiry.
    Expired,
}

/// The state of a window at `now`.
#[must_use]
pub fn state(
    publish_at: DateTime<Utc>,
    expires_at: Option<DateTime<Utc>>,
    now: DateTime<Utc>,
) -> State {
    if now < publish_at {
        State::Scheduled
    } else if expires_at.is_some_and(|e| now >= e) {
        State::Expired
    } else {
        State::Live
    }
}

/// Everything the ordering needs of one announcement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sortable {
    /// Row id, the final tie-break.
    pub id: i64,
    /// Pinned to the top.
    pub pinned: bool,
    /// Severity.
    pub severity: Severity,
    /// When it was published.
    pub publish_at: DateTime<Utc>,
}

/// Sort the feed: pinned, then severity, then newest, then the larger id.
pub fn order(items: &mut [Sortable]) {
    items.sort_by(|a, b| {
        b.pinned
            .cmp(&a.pinned)
            .then(b.severity.cmp(&a.severity))
            .then(b.publish_at.cmp(&a.publish_at))
            .then(b.id.cmp(&a.id))
    });
}

/// Problems with an announcement's fields (empty when fine).
#[must_use]
pub fn problems(
    title: &str,
    body: &str,
    severity: &str,
    publish_at: DateTime<Utc>,
    expires_at: Option<DateTime<Utc>>,
) -> Vec<String> {
    let mut out = Vec::new();
    if title.trim().is_empty() || title.chars().count() > 120 {
        out.push("title is required (up to 120 characters)".to_string());
    }
    if body.trim().is_empty() || body.chars().count() > 4000 {
        out.push("body is required (up to 4000 characters)".to_string());
    }
    if Severity::parse(severity).is_none() {
        out.push("severity must be info, important or urgent".to_string());
    }
    if expires_at.is_some_and(|e| e <= publish_at) {
        out.push("expires_at must be after publish_at".to_string());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn at(h: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 10, 5, h, 0, 0).unwrap()
    }

    #[test]
    fn a_window_is_scheduled_then_live_then_expired_with_expiry_exclusive() {
        assert_eq!(state(at(10), Some(at(12)), at(9)), State::Scheduled);
        assert_eq!(
            state(at(10), Some(at(12)), at(10)),
            State::Live,
            "live from publish_at"
        );
        assert_eq!(state(at(10), Some(at(12)), at(11)), State::Live);
        assert_eq!(
            state(at(10), Some(at(12)), at(12)),
            State::Expired,
            "expired at expires_at"
        );
        assert_eq!(
            state(at(10), None, at(23)),
            State::Live,
            "no expiry never expires"
        );
    }

    #[test]
    fn the_feed_orders_pinned_then_severity_then_newest_then_id() {
        let mk = |id, pinned, severity, h| Sortable {
            id,
            pinned,
            severity,
            publish_at: at(h),
        };
        let mut v = vec![
            mk(1, false, Severity::Urgent, 9),
            mk(2, true, Severity::Info, 8),
            mk(3, false, Severity::Info, 11),
            mk(4, false, Severity::Urgent, 10),
            mk(5, false, Severity::Urgent, 10),
            mk(6, true, Severity::Important, 7),
        ];
        order(&mut v);
        assert_eq!(
            v.iter().map(|s| s.id).collect::<Vec<_>>(),
            vec![6, 2, 5, 4, 1, 3]
        );
    }

    #[test]
    fn fields_are_checked() {
        assert!(problems("Hi", "There", "info", at(9), None).is_empty());
        assert_eq!(problems("", "x", "info", at(9), None).len(), 1);
        assert_eq!(problems("t", " ", "info", at(9), None).len(), 1);
        assert_eq!(problems("t", "x", "loud", at(9), None).len(), 1);
        assert_eq!(
            problems("t", "x", "info", at(9), Some(at(9))).len(),
            1,
            "expiry must be later"
        );
        assert_eq!(
            problems(&"a".repeat(121), "x", "info", at(9), None).len(),
            1
        );
    }
}
