//! Notification preferences (PL-5): the pure rules. DB-free, unit-tested.
//!
//! A user chooses, per notification **kind** and **channel**, whether it is
//! delivered and whether it arrives instantly or as a daily digest. **No
//! choice means the default** — everything on, instant, in-app — so an
//! account that never opens the settings behaves exactly as before.
//!
//! A digest does not delay or drop anything: every event is still stored.
//! It is applied **when the inbox is read**, collapsing the day's events
//! for one (kind, subject) into a single entry that lists every one of
//! them, so nothing is hidden and nothing needs a background flush.

use std::collections::{BTreeMap, BTreeSet};

use chrono::{DateTime, NaiveDate, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::tz::{self, Tz};

/// Notification kinds a user may set preferences for. (`auto_watch` is not a
/// notification: switching it off stops the service following things *for*
/// the user — RM-3.) **Add a kind here
/// when a new `notify(...)` call introduces one**, or it cannot be
/// switched off.
pub const KINDS: &[&str] = &[
    "approval_decided",
    "approval_requested",
    "auto_watch",
    "automation",
    "comment",
    "escalation",
    "exceptions_digest",
    "reschedule_proposed",
    "review_invited",
    "review_submitted",
    "scheduled",
    "task_assigned",
    "watch_update",
];

/// Delivery channels: the in-app inbox, and the three kinds of sink a person
/// may register (`webhook`, `chat`, `email` — see [`crate::sinks`]).
pub const CHANNELS: &[&str] = &["in_app", "webhook", "chat", "email"];

/// The channel every existing notification uses.
pub const DEFAULT_CHANNEL: &str = "in_app";

/// When a kind arrives.
pub const DIGESTS: &[&str] = &["instant", "daily"];

/// Every problem with a preference's shape (not just the first).
#[must_use]
pub fn problems(kind: &str, channel: &str, digest: &str) -> Vec<String> {
    let mut out = Vec::new();
    if !KINDS.contains(&kind) {
        out.push(format!("unknown kind {kind:?} (allowed: {KINDS:?})"));
    }
    if !CHANNELS.contains(&channel) {
        out.push(format!(
            "unknown channel {channel:?} (allowed: {CHANNELS:?})"
        ));
    }
    if !DIGESTS.contains(&digest) {
        out.push(format!("unknown digest {digest:?} (allowed: {DIGESTS:?})"));
    }
    out
}

/// The user a notification recipient refers to: the id of a `person:` or
/// `worker:` reference. Anything else (an `organization:` reference, a
/// malformed value) has no per-user preferences and gets the default.
#[must_use]
pub fn recipient_sub(recipient_ref: &str) -> Option<String> {
    let rest = recipient_ref
        .strip_prefix("person:")
        .or_else(|| recipient_ref.strip_prefix("worker:"))?;
    Uuid::parse_str(rest).ok().map(|u| u.to_string())
}

/// One stored notification, as the inbox sees it.
#[derive(Debug, Clone)]
pub struct Entry {
    /// Notification pid.
    pub pid: Uuid,
    /// Kind.
    pub kind: String,
    /// Subject kind.
    pub subject_kind: String,
    /// Subject pid.
    pub subject_pid: Uuid,
    /// Message text.
    pub message: String,
    /// When it was written.
    pub created_at: DateTime<Utc>,
    /// Whether it has been read.
    pub read: bool,
}

/// One inbox line: a single notification, or a digest of several.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Item {
    /// Kind.
    pub kind: String,
    /// Subject kind.
    pub subject_kind: String,
    /// Subject pid.
    pub subject_pid: Uuid,
    /// The newest message in the group.
    pub message: String,
    /// How many notifications this line stands for (≥ 1).
    pub count: usize,
    /// **Every** notification pid it stands for, oldest first.
    pub pids: Vec<Uuid>,
    /// Whether any of them is unread.
    pub unread: bool,
    /// The newest one's time.
    pub latest_at: DateTime<Utc>,
    /// Whether this line is a digest (`count` > 1 by choice, not chance).
    pub digest: bool,
}

/// Build the inbox: kinds in `daily_kinds` collapse per (kind, subject,
/// local calendar day in `zone`); every other entry stands alone. Newest
/// first. Entries are never dropped — a digest lists all of its members.
#[must_use]
pub fn collapse(entries: Vec<Entry>, daily_kinds: &BTreeSet<String>, zone: Tz) -> Vec<Item> {
    type Key = (String, String, Uuid, NaiveDate);
    let mut groups: BTreeMap<Key, Vec<Entry>> = BTreeMap::new();
    let mut singles: Vec<Entry> = Vec::new();
    for e in entries {
        if daily_kinds.contains(&e.kind) {
            let day = tz::local_today(e.created_at, zone);
            groups
                .entry((e.kind.clone(), e.subject_kind.clone(), e.subject_pid, day))
                .or_default()
                .push(e);
        } else {
            singles.push(e);
        }
    }
    let mut items: Vec<Item> = singles
        .into_iter()
        .map(|e| to_item(vec![e], false))
        .collect();
    items.extend(groups.into_values().map(|g| to_item(g, true)));
    items.sort_by(|a, b| b.latest_at.cmp(&a.latest_at).then(a.pids.cmp(&b.pids)));
    items
}

fn to_item(mut group: Vec<Entry>, digest: bool) -> Item {
    group.sort_by_key(|e| e.created_at);
    // `group` is never empty by construction.
    let newest = group.last().cloned().expect("non-empty group");
    Item {
        kind: newest.kind,
        subject_kind: newest.subject_kind,
        subject_pid: newest.subject_pid,
        message: newest.message,
        count: group.len(),
        pids: group.iter().map(|e| e.pid).collect(),
        unread: group.iter().any(|e| !e.read),
        latest_at: newest.created_at,
        digest,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    const U1: &str = "3f2c6b7e-0000-4000-8000-000000000001";

    fn at(d: u32, h: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 10, d, h, 0, 0).unwrap()
    }

    fn entry(n: u128, kind: &str, subject: u128, when: DateTime<Utc>, read: bool) -> Entry {
        Entry {
            pid: Uuid::from_u128(n),
            kind: kind.to_string(),
            subject_kind: "task".to_string(),
            subject_pid: Uuid::from_u128(subject),
            message: format!("m{n}"),
            created_at: when,
            read,
        }
    }

    fn daily(kinds: &[&str]) -> BTreeSet<String> {
        kinds.iter().map(|k| (*k).to_string()).collect()
    }

    #[test]
    fn shape_problems_are_all_reported() {
        assert!(problems("task_assigned", "in_app", "daily").is_empty());
        assert_eq!(problems("nope", "sms", "weekly").len(), 3);
    }

    #[test]
    fn a_recipient_maps_to_its_user_only_for_person_and_worker_uuids() {
        assert_eq!(recipient_sub(&format!("person:{U1}")), Some(U1.to_string()));
        assert_eq!(recipient_sub(&format!("worker:{U1}")), Some(U1.to_string()));
        assert_eq!(recipient_sub(&format!("organization:{U1}")), None);
        assert_eq!(recipient_sub("person:not-a-uuid"), None);
        assert_eq!(recipient_sub(""), None);
    }

    #[test]
    fn with_no_daily_kinds_nothing_collapses_and_nothing_is_lost() {
        let e = vec![
            entry(1, "task_assigned", 9, at(2, 8), false),
            entry(2, "task_assigned", 9, at(2, 9), false),
        ];
        let items = collapse(e, &daily(&[]), tz::parse_zone("UTC").unwrap());
        assert_eq!(items.len(), 2);
        assert!(items.iter().all(|i| i.count == 1 && !i.digest));
    }

    #[test]
    fn a_daily_kind_collapses_per_subject_and_lists_every_member() {
        let e = vec![
            entry(1, "task_assigned", 9, at(2, 8), true),
            entry(2, "task_assigned", 9, at(2, 10), false),
            entry(3, "task_assigned", 9, at(2, 12), true),
            entry(4, "task_assigned", 8, at(2, 11), true), // another subject
            entry(5, "comment", 9, at(2, 9), true),        // an instant kind
        ];
        let items = collapse(
            e,
            &daily(&["task_assigned"]),
            tz::parse_zone("UTC").unwrap(),
        );
        assert_eq!(items.len(), 3);
        let digest = items
            .iter()
            .find(|i| i.count == 3)
            .expect("one digest of 3");
        assert!(digest.digest && digest.unread, "any unread ⇒ unread");
        assert_eq!(
            digest.pids,
            [1u128, 2, 3].map(Uuid::from_u128),
            "all members, oldest first"
        );
        assert_eq!(digest.message, "m3", "newest message");
        assert_eq!(
            items.iter().map(|i| i.count).sum::<usize>(),
            5,
            "nothing dropped"
        );
    }

    #[test]
    fn the_day_is_the_callers_local_day_not_utc() {
        // 23:00Z and 02:00Z next day: one UTC-adjacent pair, but in
        // Auckland (UTC+13) both are the same local day (the 3rd/3rd),
        // while in UTC they are two days.
        let e = || {
            vec![
                entry(1, "task_assigned", 9, at(2, 23), true),
                entry(2, "task_assigned", 9, at(3, 2), true),
            ]
        };
        let k = daily(&["task_assigned"]);
        let utc = collapse(e(), &k, tz::parse_zone("UTC").unwrap());
        let nz = collapse(e(), &k, tz::parse_zone("Pacific/Auckland").unwrap());
        assert_eq!(utc.len(), 2, "two UTC days");
        assert_eq!(nz.len(), 1, "one Auckland day");
        assert_eq!(nz[0].count, 2);
    }

    #[test]
    fn items_are_newest_first() {
        let e = vec![
            entry(1, "comment", 1, at(2, 8), true),
            entry(2, "comment", 2, at(2, 12), true),
        ];
        let items = collapse(e, &daily(&[]), tz::parse_zone("UTC").unwrap());
        assert_eq!(items[0].pids, [Uuid::from_u128(2)]);
    }
}
