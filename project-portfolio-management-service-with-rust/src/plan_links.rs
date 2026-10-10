//! Typed links on a plan (LT-8): the pure rules. DB-free, unit-tested.
//!
//! A plan carries links to where its real documents live — a wiki page, a
//! repository, a ticket, a design file. The service stores the link and shows
//! it; it **never fetches it** and never renders it. Only `http` / `https`
//! URLs are accepted, so a link can never be `javascript:`, `data:` or `file:`.

/// The link kinds.
pub const KINDS: &[&str] = &["docs", "repo", "ticket", "design", "chat", "other"];
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

/// Every problem with a link (not just the first).
#[must_use]
pub fn problems(kind: &str, url: &str, label: &str) -> Vec<String> {
    let mut out = Vec::new();
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
        assert!(problems("docs", "https://x.com", "Spec").is_empty());
        assert_eq!(problems("nope", "javascript:1", " ").len(), 3);
        assert!(
            !problems(
                "docs",
                &format!("https://x.com/{}", "a".repeat(MAX_URL)),
                "x"
            )
            .is_empty()
        );
    }
}
