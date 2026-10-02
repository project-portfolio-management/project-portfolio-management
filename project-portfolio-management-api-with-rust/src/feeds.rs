//! Feeds (LT-7 iCalendar, RM-6 Atom): the pure builders and the token rules.
//! DB-free, unit-tested.
//!
//! **Tokens.** A feed token is 244 bits from two v4 UUIDs; only its SHA-256
//! is stored ([`hash_token`]). It grants read access to **one person's own**
//! feed and nothing else.
//!
//! **iCalendar** (RFC 5545): CRLF line ends, lines folded at 75 octets,
//! text escaped, `VALUE=DATE` all-day events with the exclusive next-day
//! `DTEND`, and **stable `UID`s** so a calendar app updates an item rather
//! than duplicating it.
//!
//! **Atom** (RFC 4287): stable `urn:uuid:` entry ids, XML-escaped text.

use chrono::{DateTime, Datelike, NaiveDate, Utc};
use sha2::{Digest, Sha256};
use uuid::Uuid;

/// A fresh feed token: 64 lowercase hex characters (244 bits of entropy).
#[must_use]
pub fn new_token() -> String {
    format!("{}{}", Uuid::new_v4().simple(), Uuid::new_v4().simple())
}

/// The stored form of a token: lowercase hex SHA-256.
#[must_use]
pub fn hash_token(token: &str) -> String {
    use std::fmt::Write;
    Sha256::digest(token.as_bytes())
        .iter()
        .fold(String::new(), |mut s, b| {
            let _ = write!(s, "{b:02x}");
            s
        })
}

/// Whether a presented string is shaped like a feed token (so garbage is
/// rejected before touching the database).
#[must_use]
pub fn token_shaped(token: &str) -> bool {
    token.len() == 64
        && token
            .bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
}

/// Escape an iCalendar TEXT value (RFC 5545 §3.3.11).
#[must_use]
pub fn ical_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            ';' => out.push_str("\\;"),
            ',' => out.push_str("\\,"),
            '\n' => out.push_str("\\n"),
            '\r' => {}
            c => out.push(c),
        }
    }
    out
}

/// Fold a content line to at most 75 **octets** per physical line (RFC 5545
/// §3.1), continuing with CRLF + one space, never splitting a UTF-8 char.
#[must_use]
pub fn ical_fold(line: &str) -> String {
    let mut out = String::new();
    let mut current = 0usize;
    for c in line.chars() {
        let w = c.len_utf8();
        if current + w > 75 {
            out.push_str("\r\n ");
            current = 1;
        }
        out.push(c);
        current += w;
    }
    out
}

/// One all-day calendar event.
#[derive(Debug, Clone)]
pub struct Event {
    /// Stable identity: the same item always yields the same UID.
    pub uid: String,
    /// The day.
    pub date: NaiveDate,
    /// Title.
    pub summary: String,
    /// Optional longer text.
    pub description: Option<String>,
}

fn ymd(d: NaiveDate) -> String {
    format!("{:04}{:02}{:02}", d.year(), d.month(), d.day())
}

/// Render a VCALENDAR. `stamp` is the generation instant (DTSTAMP); events
/// are emitted in the given order, so callers pass a stable sort.
#[must_use]
pub fn ical(name: &str, stamp: DateTime<Utc>, events: &[Event]) -> String {
    let mut lines: Vec<String> = vec![
        "BEGIN:VCALENDAR".into(),
        "VERSION:2.0".into(),
        "PRODID:-//Project Portfolio Management//Feeds//EN".into(),
        "CALSCALE:GREGORIAN".into(),
        format!("X-WR-CALNAME:{}", ical_escape(name)),
    ];
    let stamp = stamp.format("%Y%m%dT%H%M%SZ").to_string();
    for e in events {
        let next = e.date.succ_opt().unwrap_or(e.date);
        lines.push("BEGIN:VEVENT".into());
        lines.push(format!("UID:{}", e.uid));
        lines.push(format!("DTSTAMP:{stamp}"));
        lines.push(format!("DTSTART;VALUE=DATE:{}", ymd(e.date)));
        lines.push(format!("DTEND;VALUE=DATE:{}", ymd(next)));
        lines.push(format!("SUMMARY:{}", ical_escape(&e.summary)));
        if let Some(d) = &e.description {
            lines.push(format!("DESCRIPTION:{}", ical_escape(d)));
        }
        lines.push("END:VEVENT".into());
    }
    lines.push("END:VCALENDAR".into());
    let mut out = String::new();
    for l in lines {
        out.push_str(&ical_fold(&l));
        out.push_str("\r\n");
    }
    out
}

/// Escape XML text / attribute content.
#[must_use]
pub fn xml_escape(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '&' => "&amp;".to_string(),
            '<' => "&lt;".to_string(),
            '>' => "&gt;".to_string(),
            '"' => "&quot;".to_string(),
            '\'' => "&apos;".to_string(),
            c => c.to_string(),
        })
        .collect()
}

/// One Atom entry.
#[derive(Debug, Clone)]
pub struct Entry {
    /// Stable id (a UUID rendered as `urn:uuid:…`).
    pub id: Uuid,
    /// Title.
    pub title: String,
    /// Last updated.
    pub updated: DateTime<Utc>,
    /// Body text.
    pub summary: String,
}

/// Render an Atom 1.0 feed. `updated` is the newest entry's time (the Unix
/// epoch when empty), so an unchanged feed is byte-identical across fetches.
#[must_use]
pub fn atom(feed_id: Uuid, title: &str, entries: &[Entry]) -> String {
    use std::fmt::Write;
    // An empty feed uses the Unix epoch, never the clock, so it too is
    // byte-identical across fetches (and `If-None-Match` can answer 304).
    let updated = entries
        .iter()
        .map(|e| e.updated)
        .max()
        .unwrap_or(DateTime::<Utc>::UNIX_EPOCH);
    let mut out = String::from("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n");
    out.push_str("<feed xmlns=\"http://www.w3.org/2005/Atom\">\n");
    let _ = writeln!(out, "  <id>urn:uuid:{feed_id}</id>");
    let _ = writeln!(out, "  <title>{}</title>", xml_escape(title));
    let _ = writeln!(out, "  <updated>{}</updated>", updated.to_rfc3339());
    for e in entries {
        out.push_str("  <entry>\n");
        let _ = writeln!(out, "    <id>urn:uuid:{}</id>", e.id);
        let _ = writeln!(out, "    <title>{}</title>", xml_escape(&e.title));
        let _ = writeln!(out, "    <updated>{}</updated>", e.updated.to_rfc3339());
        let _ = writeln!(out, "    <summary>{}</summary>", xml_escape(&e.summary));
        out.push_str("  </entry>\n");
    }
    out.push_str("</feed>\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn at() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 10, 3, 12, 0, 0).unwrap()
    }

    #[test]
    fn tokens_are_long_unique_and_only_their_hash_is_stable() {
        let (a, b) = (new_token(), new_token());
        assert_ne!(a, b);
        assert!(token_shaped(&a));
        assert_eq!(hash_token(&a), hash_token(&a));
        assert_ne!(hash_token(&a), a, "never stored in the clear");
        assert_eq!(hash_token(&a).len(), 64);
        for bad in ["", "short", &"G".repeat(64), &"A".repeat(64)] {
            assert!(!token_shaped(bad), "{bad:?}");
        }
    }

    #[test]
    fn text_is_escaped_per_rfc_5545() {
        assert_eq!(ical_escape("a,b;c\\d\ne"), "a\\,b\\;c\\\\d\\ne");
    }

    #[test]
    fn long_lines_fold_at_75_octets_without_splitting_a_character() {
        let line = format!("SUMMARY:{}", "é".repeat(60)); // 2 octets each
        let folded = ical_fold(&line);
        for physical in folded.split("\r\n") {
            assert!(
                physical.len() <= 75,
                "{} octets: {physical:?}",
                physical.len()
            );
        }
        // Unfolding restores the original exactly.
        assert_eq!(folded.replace("\r\n ", ""), line);
        assert_eq!(ical_fold("short"), "short");
    }

    #[test]
    fn a_calendar_is_well_formed_with_stable_uids_and_exclusive_dtend() {
        let events = [Event {
            uid: "task-1@ppm".into(),
            date: NaiveDate::from_ymd_opt(2026, 10, 31).unwrap(),
            summary: "Ship, then rest".into(),
            description: Some("line1\nline2".into()),
        }];
        let a = ical("My work", at(), &events);
        let b = ical("My work", at(), &events);
        assert_eq!(a, b, "deterministic for the same input");
        assert!(a.starts_with("BEGIN:VCALENDAR\r\nVERSION:2.0\r\n"));
        assert!(a.ends_with("END:VCALENDAR\r\n"));
        assert!(a.contains("UID:task-1@ppm\r\n"));
        assert!(a.contains("DTSTART;VALUE=DATE:20261031\r\n"));
        assert!(
            a.contains("DTEND;VALUE=DATE:20261101\r\n"),
            "all-day end is the next day"
        );
        assert!(a.contains("SUMMARY:Ship\\, then rest\r\n"));
        assert!(a.contains("DESCRIPTION:line1\\nline2\r\n"));
        assert!(
            !a.contains("\n\n") && a.replace("\r\n", "").find('\n').is_none(),
            "CRLF only"
        );
    }

    #[test]
    fn atom_is_escaped_and_unchanged_feeds_are_byte_identical() {
        let id = Uuid::from_u128(7);
        let e = Entry {
            id: Uuid::from_u128(1),
            title: "A <b> & \"c\"".into(),
            updated: at(),
            summary: "x".into(),
        };
        let a = atom(id, "Inbox", std::slice::from_ref(&e));
        assert!(a.contains("A &lt;b&gt; &amp; &quot;c&quot;"));
        assert!(a.contains(&format!("<id>urn:uuid:{}</id>", e.id)));
        assert_eq!(
            a,
            atom(id, "Inbox", &[e]),
            "updated follows the newest entry, not the clock"
        );
        let empty = atom(id, "Inbox", &[]);
        assert!(
            empty.contains("<updated>1970-01-01"),
            "an empty feed is epoch-stamped"
        );
        assert_eq!(empty, atom(id, "Inbox", &[]), "and byte-identical");
    }
}
