//! Typed links on a plan (LT-8): the pure rules. DB-free, unit-tested.
//!
//! A plan carries links to where its real documents live — a wiki page, a
//! repository, a ticket, a design file. The service stores the link and shows
//! it; it **never fetches it** and never renders it. Only `http` / `https`
//! URLs are accepted, so a link can never be `javascript:`, `data:` or `file:`.

/// The link kinds.
pub const KINDS: &[&str] = &[
    "docs", "repo", "ticket", "design", "chat", "tracker", "other",
];
/// The external work-tracking systems a link may name. `tracker` and `ticket` links are the ones that carry one.
pub const SYSTEMS: &[&str] = &[
    "jira",
    "trello",
    "asana",
    "linear",
    "monday",
    "clickup",
    "azure_devops",
    "github",
    "other",
];
/// Cap on an external identifier.
pub const MAX_EXTERNAL_ID: usize = 100;
/// Cap on a URL.
pub const MAX_URL: usize = 2_048;
/// Cap on a label.
pub const MAX_LABEL: usize = 200;
/// Cap on links per plan.
pub const MAX_PER_PLAN: usize = 100;

/// Whether `url` is an absolute `http(s)` URL with a host.
#[must_use]
pub fn safe_url(url: &str) -> bool {
    let lower = url.to_ascii_lowercase();
    let rest = lower
        .strip_prefix("https://")
        .or_else(|| lower.strip_prefix("http://"));
    let Some(rest) = rest else { return false };
    let host = rest.split(['/', '?', '#']).next().unwrap_or_default();
    // A host, no embedded credentials, no whitespace or control characters.
    !host.is_empty()
        && !host.contains('@')
        && !url.chars().any(|c| c.is_whitespace() || c.is_control())
}

/// The host of an already-checked URL, lowercased, without a port.
#[must_use]
pub fn host_of(url: &str) -> String {
    let lower = url.to_ascii_lowercase();
    let rest = lower
        .strip_prefix("https://")
        .or_else(|| lower.strip_prefix("http://"))
        .unwrap_or(&lower);
    let host = rest.split(['/', '?', '#']).next().unwrap_or_default();
    host.rsplit_once(':').map_or(host, |(h, _)| h).to_string()
}

/// The domains a hosted system lives on; `None` for a system that may be self-hosted or is unknown (Jira, GitHub
/// Enterprise, `other`), where any host is accepted.
#[must_use]
pub fn system_domains(system: &str) -> Option<&'static [&'static str]> {
    Some(match system {
        "trello" => &["trello.com"],
        "asana" => &["asana.com"],
        "linear" => &["linear.app"],
        "monday" => &["monday.com"],
        "clickup" => &["clickup.com"],
        "azure_devops" => &["dev.azure.com", "visualstudio.com"],
        _ => return None,
    })
}

/// Every problem with a link (not just the first). `system` and `external_id` describe a link into a work-tracking
/// tool: only `tracker` and `ticket` links carry them, a `tracker` link must name its system, and a hosted system's
/// link must be on that system's own domain, so a Trello board cannot be recorded as an Asana project by mistake.
#[must_use]
pub fn problems(
    kind: &str,
    url: &str,
    label: &str,
    system: Option<&str>,
    external_id: Option<&str>,
) -> Vec<String> {
    let mut out = Vec::new();
    match system {
        Some(s) if !SYSTEMS.contains(&s) => {
            out.push(format!("unknown system {s:?} (allowed: {SYSTEMS:?})"));
        }
        Some(_) if !matches!(kind, "tracker" | "ticket") => {
            out.push("system belongs on a tracker or ticket link".to_string());
        }
        None if kind == "tracker" => out.push("a tracker link must name its system".to_string()),
        _ => {}
    }
    if external_id.is_some() && system.is_none() {
        out.push("external_id needs a system".to_string());
    }
    if external_id.is_some_and(|id| {
        id.trim().is_empty()
            || id.chars().count() > MAX_EXTERNAL_ID
            || id.chars().any(|c| c.is_whitespace() || c.is_control())
    }) {
        out.push(format!(
            "external_id is 1 to {MAX_EXTERNAL_ID} characters with no spaces"
        ));
    }
    if let Some(domains) = system.and_then(system_domains)
        && safe_url(url)
    {
        let host = host_of(url);
        if !domains
            .iter()
            .any(|d| host == *d || host.ends_with(&format!(".{d}")))
        {
            out.push(format!(
                "a {} link must be on {}",
                system.unwrap_or_default(),
                domains.join(" or ")
            ));
        }
    }
    if !KINDS.contains(&kind) {
        out.push(format!("unknown kind {kind:?} (allowed: {KINDS:?})"));
    }
    if url.len() > MAX_URL {
        out.push(format!("url: exceeds {MAX_URL} characters"));
    } else if !safe_url(url) {
        out.push(
            "url must be an absolute http or https URL with a host (no credentials)".to_string(),
        );
    }
    if label.trim().is_empty() || label.chars().count() > MAX_LABEL {
        out.push(format!(
            "label is required (at most {MAX_LABEL} characters)"
        ));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_http_and_https_with_a_host_are_links() {
        for ok in [
            "https://wiki.example.com/page",
            "http://localhost:8080/x?y=1#z",
            "HTTPS://Example.com",
        ] {
            assert!(safe_url(ok), "{ok}");
        }
        for bad in [
            "javascript:alert(1)",
            "data:text/html,<b>",
            "file:///etc/passwd",
            "ftp://x.com",
            "//evil.com",
            "https://",
            "https:///path",
            "https://user:pw@evil.com/",
            "https://a b.com",
            "/relative",
            "",
            "https://exa\u{0}mple.com",
        ] {
            assert!(!safe_url(bad), "{bad:?}");
        }
    }

    #[test]
    fn problems_are_all_reported() {
        assert!(problems("docs", "https://x.com", "Spec", None, None).is_empty());
        assert_eq!(problems("nope", "javascript:1", " ", None, None).len(), 3);
        assert!(
            !problems(
                "docs",
                &format!("https://x.com/{}", "a".repeat(MAX_URL)),
                "x",
                None,
                None
            )
            .is_empty()
        );
    }

    #[test]
    fn a_link_into_a_tracker_names_its_system_and_stays_on_its_domain() {
        let ok = |kind: &str, url: &str, system: Option<&str>, id: Option<&str>| {
            problems(kind, url, "x", system, id)
        };
        assert!(
            ok(
                "tracker",
                "https://acme.atlassian.net/jira/software/projects/PPM",
                Some("jira"),
                Some("PPM")
            )
            .is_empty()
        );
        assert!(
            ok(
                "tracker",
                "https://jira.internal.example.com/projects/PPM",
                Some("jira"),
                None
            )
            .is_empty(),
            "self-hosted Jira"
        );
        assert!(
            ok(
                "tracker",
                "https://trello.com/b/abc/board",
                Some("trello"),
                None
            )
            .is_empty()
        );
        assert!(
            ok(
                "tracker",
                "https://app.asana.com/0/123/list",
                Some("asana"),
                None
            )
            .is_empty()
        );
        assert!(
            ok(
                "tracker",
                "https://linear.app/acme/project/x",
                Some("linear"),
                None
            )
            .is_empty()
        );
        assert!(
            ok(
                "tracker",
                "https://acme.monday.com/boards/1",
                Some("monday"),
                Some("1")
            )
            .is_empty()
        );
        assert!(
            ok(
                "ticket",
                "https://acme.atlassian.net/browse/PPM-7",
                Some("jira"),
                Some("PPM-7")
            )
            .is_empty()
        );
        assert!(
            ok("docs", "https://wiki.example.com", None, None).is_empty(),
            "a plain link is unchanged"
        );
    }

    #[test]
    fn a_link_into_a_tracker_is_refused_when_it_is_wrong() {
        let ok = |kind: &str, url: &str, system: Option<&str>, id: Option<&str>| {
            problems(kind, url, "x", system, id)
        };
        for (kind, url, system, id, why) in [
            (
                "tracker",
                "https://trello.com/b/abc",
                None,
                None,
                "a tracker without a system",
            ),
            (
                "tracker",
                "https://trello.com/b/abc",
                Some("trelo"),
                None,
                "an unknown system",
            ),
            (
                "docs",
                "https://trello.com/b/abc",
                Some("trello"),
                None,
                "a system on a docs link",
            ),
            (
                "tracker",
                "https://app.asana.com/0/1",
                Some("trello"),
                None,
                "a Trello board on an Asana address",
            ),
            (
                "tracker",
                "https://eviltrello.com/b/abc",
                Some("trello"),
                None,
                "a look-alike domain",
            ),
            (
                "tracker",
                "https://trello.com.evil.example/b/abc",
                Some("trello"),
                None,
                "a domain that only starts right",
            ),
            (
                "docs",
                "https://wiki.example.com",
                None,
                Some("PPM-1"),
                "an id without a system",
            ),
            (
                "tracker",
                "https://linear.app/x",
                Some("linear"),
                Some("has space"),
                "an id with a space",
            ),
            (
                "tracker",
                "https://linear.app/x",
                Some("linear"),
                Some(""),
                "an empty id",
            ),
        ] {
            assert!(!ok(kind, url, system, id).is_empty(), "{why}");
        }
    }

    #[test]
    fn the_host_is_read_without_port_path_or_case() {
        assert_eq!(
            host_of("HTTPS://Acme.Monday.com:8443/boards/1?x=1#y"),
            "acme.monday.com"
        );
        assert_eq!(host_of("http://localhost/x"), "localhost");
    }
}
