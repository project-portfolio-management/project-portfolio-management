//! Notification delivery sinks (LT-6 chat and webhook, RM-11 email): the pure
//! rules and renderers. DB-free, unit-tested.
//!
//! A person registers destinations for **their own** notifications. A sink is
//! one of: a signed `webhook` (the family envelope), a chat incoming-webhook
//! (`slack`, `mattermost`, `discord`, `telegram`), or an `email` address.
//! Delivery is **never in the notification's path**: a notification enqueues a
//! delivery row and a dispatcher sends it later with bounded retries, so a dead
//! destination cannot delay or fail the write.
//!
//! The destination is a **secret** (a chat webhook URL *is* the credential): it
//! is validated on the way in and only a masked form ever comes back out.

use std::net::IpAddr;

use serde_json::{Value, json};

/// Sink kinds.
pub const KINDS: &[&str] = &[
    "webhook",
    "slack",
    "mattermost",
    "discord",
    "telegram",
    "email",
];
/// Delivery attempts before a delivery is given up on.
pub const MAX_ATTEMPTS: i32 = 5;
/// Sinks one person may hold.
pub const MAX_PER_PERSON: usize = 10;

/// Whether an address is a public, routable one. Everything a server should
/// never be talked into calling on a user's behalf is **not** public:
/// loopback, private (RFC 1918), link-local (incl. cloud metadata
/// `169.254.169.254`), carrier-grade NAT, unique-local IPv6, multicast,
/// unspecified and documentation ranges.
#[must_use]
pub fn ip_is_public(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => {
            let o = v4.octets();
            !(v4.is_loopback()
                || v4.is_private()
                || v4.is_link_local()
                || v4.is_unspecified()
                || v4.is_broadcast()
                || v4.is_multicast()
                || v4.is_documentation()
                || (o[0] == 100 && (64..=127).contains(&o[1])) // 100.64.0.0/10
                || o[0] == 0)
        }
        IpAddr::V6(v6) => {
            let seg = v6.segments();
            !(v6.is_loopback()
                || v6.is_unspecified()
                || v6.is_multicast()
                || (seg[0] & 0xfe00) == 0xfc00 // fc00::/7 unique local
                || (seg[0] & 0xffc0) == 0xfe80 // fe80::/10 link local
                || v6.to_ipv4_mapped().is_some_and(|m| !ip_is_public(IpAddr::V4(m))))
        }
    }
}

/// Whether a destination URL may be called on a user's behalf: `https` to a
/// public host; an IP literal must be public; `localhost` and `.local` /
/// `.internal` names are refused. **Loopback** (`http` or `https`) is allowed
/// only when `allow_loopback` is set — for local development and tests, never
/// the default. (A name that *resolves* to a private address is caught at send
/// time by [`resolved_ok`].)
#[must_use]
pub fn destination_url_ok(url: &str, allow_loopback: bool) -> bool {
    let Ok(u) = reqwest::Url::parse(url.trim()) else {
        return false;
    };
    let scheme = u.scheme();
    let Some(host) = u.host_str() else {
        return false;
    };
    if !u.username().is_empty() || u.password().is_some() {
        return false; // credentials in the URL
    }
    let bare = host.trim_matches(['[', ']']);
    let literal = bare.parse::<IpAddr>().ok();
    let loopback =
        literal.is_some_and(|ip| ip.is_loopback()) || bare.eq_ignore_ascii_case("localhost");
    if loopback {
        return allow_loopback && matches!(scheme, "http" | "https");
    }
    if scheme != "https" {
        return false;
    }
    if let Some(ip) = literal {
        return ip_is_public(ip);
    }
    // A name, not an address: refuse the usual internal-only suffixes.
    let lower = bare.to_ascii_lowercase();
    !lower
        .rsplit('.')
        .next()
        .is_some_and(|tld| matches!(tld, "local" | "internal" | "localhost"))
}

/// Whether **every** address a name resolved to is public (call after DNS, at
/// send time — a hostname that points inside the network is refused).
#[must_use]
pub fn resolved_ok(addrs: &[IpAddr], allow_loopback: bool) -> bool {
    !addrs.is_empty()
        && addrs
            .iter()
            .all(|ip| ip_is_public(*ip) || (allow_loopback && ip.is_loopback()))
}

/// The notification-preference channel a sink kind belongs to.
#[must_use]
pub fn channel_of(kind: &str) -> &'static str {
    match kind {
        "webhook" => "webhook",
        "email" => "email",
        _ => "chat",
    }
}

/// Whether `s` is shaped like an email address (one `@`, a dotted domain, no
/// whitespace, no header-injection characters).
#[must_use]
pub fn email_shaped(s: &str) -> bool {
    let Some((local, domain)) = s.split_once('@') else {
        return false;
    };
    !local.is_empty()
        && domain.contains('.')
        && !domain.starts_with('.')
        && !domain.ends_with('.')
        && !s
            .chars()
            .any(|c| c.is_whitespace() || c.is_control() || matches!(c, ',' | ';' | '<' | '>'))
        && s.matches('@').count() == 1
        && s.len() <= 254
}

/// Every problem with a new sink (not just the first).
#[must_use]
pub fn problems(
    kind: &str,
    destination: &str,
    settings: &Value,
    allow_loopback: bool,
) -> Vec<String> {
    let mut out = Vec::new();
    if !KINDS.contains(&kind) {
        out.push(format!("unknown kind {kind:?} (allowed: {KINDS:?})"));
        return out;
    }
    if kind == "email" {
        if !email_shaped(destination) {
            out.push("destination must be an email address".to_string());
        }
    } else if destination.len() > 2_048 || !destination_url_ok(destination, allow_loopback) {
        out.push("destination must be an https URL to a public host (no credentials, no private or internal address)".to_string());
    }
    if kind == "telegram"
        && settings["chat_id"]
            .as_str()
            .is_none_or(|c| c.trim().is_empty())
    {
        out.push("a telegram sink needs settings.chat_id".to_string());
    }
    out
}

/// A destination safe to show back: the scheme and host (or the email domain)
/// and the last four characters, never the whole secret.
#[must_use]
pub fn mask_destination(kind: &str, destination: &str) -> String {
    let tail: String = destination
        .chars()
        .rev()
        .take(4)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();
    if kind == "email" {
        let domain = destination.split_once('@').map_or("", |(_, d)| d);
        return format!("***@{domain}");
    }
    let head = reqwest::Url::parse(destination)
        .ok()
        .map(|u| format!("{}://{}", u.scheme(), u.host_str().unwrap_or("?")))
        .unwrap_or_default();
    format!("{head}/…{tail}")
}

/// One notification, as a renderer sees it.
#[derive(Debug, Clone)]
pub struct Note<'a> {
    /// Notification pid.
    pub pid: uuid::Uuid,
    /// Kind.
    pub kind: &'a str,
    /// Subject kind.
    pub subject_kind: &'a str,
    /// Subject pid.
    pub subject_pid: uuid::Uuid,
    /// The text.
    pub message: &'a str,
    /// When it was written.
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// The request body for `kind`, and its content type.
///
/// Every chat format carries **only the notification's own text** — no more
/// than the in-app inbox shows — so a message pasted into a channel cannot
/// disclose anything the recipient could not already see.
///
/// # Errors
///
/// A message when the sink's settings cannot produce a body (Telegram without
/// a chat id).
pub fn render(
    kind: &str,
    note: &Note<'_>,
    settings: &Value,
) -> Result<(String, &'static str), String> {
    let text = note.message.chars().take(1_800).collect::<String>();
    let body = match kind {
        "webhook" => json!({
            "id": note.pid, "kind": note.kind, "subject_kind": note.subject_kind,
            "subject_pid": note.subject_pid, "message": note.message,
            "created_at": note.created_at.to_rfc3339(),
        }),
        "slack" | "mattermost" => json!({ "text": text }),
        "discord" => json!({ "content": text }),
        "telegram" => {
            let chat = settings["chat_id"]
                .as_str()
                .filter(|c| !c.trim().is_empty());
            let Some(chat) = chat else {
                return Err("telegram needs settings.chat_id".to_string());
            };
            json!({ "chat_id": chat, "text": text })
        }
        other => return Err(format!("{other} has no HTTP body")),
    };
    Ok((body.to_string(), "application/json"))
}

/// The email subject and body for a notification: its text and nothing else.
#[must_use]
pub fn render_email(note: &Note<'_>) -> (String, String) {
    let subject: String = note
        .message
        .chars()
        .take(120)
        .collect::<String>()
        .replace(['\r', '\n'], " ");
    (
        subject,
        format!(
            "{}\n\n— Project Portfolio Management ({})\n",
            note.message, note.kind
        ),
    )
}

/// When to try again after `attempts` failed attempts: 1 min, 5, 25, 125 …
/// capped at six hours. `None` once [`MAX_ATTEMPTS`] is reached.
#[must_use]
pub fn next_attempt(
    now: chrono::DateTime<chrono::Utc>,
    attempts: i32,
) -> Option<chrono::DateTime<chrono::Utc>> {
    if attempts >= MAX_ATTEMPTS {
        return None;
    }
    let minutes = 5_i64
        .saturating_pow(u32::try_from(attempts.max(1) - 1).unwrap_or(0))
        .min(360);
    Some(now + chrono::Duration::minutes(minutes))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use uuid::Uuid;

    fn note(msg: &str) -> Note<'_> {
        Note {
            pid: Uuid::from_u128(1),
            kind: "task_assigned",
            subject_kind: "task",
            subject_pid: Uuid::from_u128(2),
            message: msg,
            created_at: chrono::Utc.with_ymd_and_hms(2026, 10, 3, 12, 0, 0).unwrap(),
        }
    }

    #[test]
    fn sink_kinds_map_to_preference_channels() {
        assert_eq!(channel_of("webhook"), "webhook");
        assert_eq!(channel_of("email"), "email");
        for k in ["slack", "mattermost", "discord", "telegram"] {
            assert_eq!(channel_of(k), "chat", "{k}");
        }
    }

    #[test]
    fn destinations_are_validated_per_kind() {
        let none = json!({});
        let p = |k: &str, d: &str, s: &Value| problems(k, d, s, false);
        assert!(p("slack", "https://hooks.slack.com/services/T/B/x", &none).is_empty());
        assert!(
            !p("webhook", "http://127.0.0.1:9/h", &none).is_empty(),
            "loopback is off by default"
        );
        assert!(
            problems("webhook", "http://127.0.0.1:9/h", &none, true).is_empty(),
            "…and on for dev/test"
        );
        assert!(
            !p("webhook", "http://evil.example.com/h", &none).is_empty(),
            "plain http elsewhere"
        );
        assert!(!p("slack", "javascript:1", &none).is_empty());
        let tg = "https://api.telegram.org/bot1:x/sendMessage";
        assert!(!p("telegram", tg, &none).is_empty(), "needs chat_id");
        assert!(p("telegram", tg, &json!({ "chat_id": "42" })).is_empty());
        assert!(p("email", "a@example.com", &none).is_empty());
        for bad in [
            "a@b",
            "a b@example.com",
            "a@@example.com",
            "a@example.com,b@x.com",
            "a@example.com\r\nBcc: x@y.z",
            "@example.com",
            "plain",
        ] {
            assert!(!p("email", bad, &none).is_empty(), "{bad:?}");
        }
        assert!(!p("fax", "x", &none).is_empty());
    }

    #[test]
    fn a_user_cannot_point_a_sink_at_the_inside_of_the_network() {
        let ok = |u: &str| destination_url_ok(u, false);
        assert!(ok("https://hooks.slack.com/x"));
        for bad in [
            "https://127.0.0.1/x",
            "https://localhost/x",
            "https://10.1.2.3/x",
            "https://192.168.0.5/x",
            "https://172.16.0.1/x",
            "https://169.254.169.254/latest/meta-data",
            "https://100.64.0.1/x",
            "https://[::1]/x",
            "https://[fd00::1]/x",
            "https://[fe80::1]/x",
            "https://[::ffff:10.0.0.1]/x",
            "https://db.internal/x",
            "https://printer.local/x",
            "https://0.0.0.0/x",
            "https://user:pw@hooks.slack.com/x",
            "http://hooks.slack.com/x",
            "ftp://x.com/",
        ] {
            assert!(!ok(bad), "{bad} must be refused");
        }
        assert!(ok("https://8.8.8.8/x"), "a public literal is fine");
        assert!(
            destination_url_ok("http://127.0.0.1:9/h", true),
            "loopback only when explicitly allowed"
        );
    }

    #[test]
    fn a_name_that_resolves_inside_is_refused_at_send_time() {
        let ip = |s: &str| s.parse::<IpAddr>().unwrap();
        assert!(resolved_ok(&[ip("8.8.8.8"), ip("1.1.1.1")], false));
        assert!(
            !resolved_ok(&[ip("8.8.8.8"), ip("10.0.0.1")], false),
            "any private answer refuses it"
        );
        assert!(!resolved_ok(&[], false), "no answer is no go");
        assert!(resolved_ok(&[ip("127.0.0.1")], true));
        assert!(!resolved_ok(&[ip("127.0.0.1")], false));
    }

    #[test]
    fn a_secret_destination_is_never_shown_in_full() {
        let m = mask_destination(
            "slack",
            "https://hooks.slack.com/services/T000/B000/SECRETSECRET1234",
        );
        assert_eq!(m, "https://hooks.slack.com/…1234");
        assert!(!m.contains("SECRET"));
        assert_eq!(
            mask_destination("email", "alice@example.com"),
            "***@example.com"
        );
    }

    #[test]
    fn each_chat_format_is_pinned_and_carries_only_the_text() {
        let n = note("You were assigned: Ship");
        let none = json!({});
        assert_eq!(
            render("slack", &n, &none).unwrap().0,
            r#"{"text":"You were assigned: Ship"}"#
        );
        assert_eq!(
            render("mattermost", &n, &none).unwrap().0,
            r#"{"text":"You were assigned: Ship"}"#
        );
        assert_eq!(
            render("discord", &n, &none).unwrap().0,
            r#"{"content":"You were assigned: Ship"}"#
        );
        assert_eq!(
            render("telegram", &n, &json!({ "chat_id": "42" }))
                .unwrap()
                .0,
            r#"{"chat_id":"42","text":"You were assigned: Ship"}"#
        );
        assert!(render("telegram", &n, &none).is_err());
        let hook: Value = serde_json::from_str(&render("webhook", &n, &none).unwrap().0).unwrap();
        assert_eq!(hook["kind"], "task_assigned");
        assert_eq!(hook["message"], "You were assigned: Ship");
        // No chat body carries the subject id: only the text.
        for k in ["slack", "mattermost", "discord"] {
            assert!(
                !render(k, &n, &none)
                    .unwrap()
                    .0
                    .contains(&Uuid::from_u128(2).to_string()),
                "{k}"
            );
        }
    }

    #[test]
    fn email_carries_the_text_and_cannot_be_header_injected() {
        let (subject, body) = render_email(&note("Line one\r\nBcc: x@y.z"));
        assert!(!subject.contains('\n') && !subject.contains('\r'));
        assert!(body.contains("Line one"));
    }

    #[test]
    fn retries_back_off_and_stop() {
        let t = chrono::Utc.with_ymd_and_hms(2026, 10, 3, 12, 0, 0).unwrap();
        let mins = |a: i32| next_attempt(t, a).map(|x| (x - t).num_minutes());
        assert_eq!(
            (mins(1), mins(2), mins(3), mins(4)),
            (Some(1), Some(5), Some(25), Some(125))
        );
        assert_eq!(mins(MAX_ATTEMPTS), None, "given up");
    }
}
