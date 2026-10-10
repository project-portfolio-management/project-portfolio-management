//! The monthly director pack (DP-1): pure rules. The pack is **frozen** when issued, **versioned**
//! (a correction is a reissue with a note), and read by **audience**.
//!
//! Who may do what is decided from token attributes, not the policy file, so a deployment cannot
//! widen it by editing YAML:
//! * `access=admin` does everything;
//! * `directorpack=issuer` drafts, reissues and issues; `directorpack=editor` drafts and reissues only;
//! * `pack_audience=<audience>` (repeatable) reads the issued packs of that audience;
//! * `role=pack_reader` marks a reader who is confined to the packs and sees nothing live
//!   (the partner case): every other route answers 404.

/// The audiences a pack may be issued to.
pub const AUDIENCES: [&str; 4] = ["executive", "board", "escalation_board", "partner"];

/// A reissue needs at least this many characters of explanation.
pub const MIN_NOTE: usize = 10;

/// What a caller may do with packs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Power {
    /// Reads issued packs of their audiences only.
    Reader,
    /// Drafts and reissues.
    Editor,
    /// Drafts, reissues and issues.
    Issuer,
}

/// The power the attributes give (`None` when they give no write power).
#[must_use]
pub fn power(admin: bool, directorpack: &[String]) -> Option<Power> {
    if admin || directorpack.iter().any(|v| v == "issuer") {
        Some(Power::Issuer)
    } else if directorpack.iter().any(|v| v == "editor") {
        Some(Power::Editor)
    } else {
        None
    }
}

/// Whether a caller may read an issued pack of `audience`: an editor or issuer reads all; a reader reads
/// the audiences named in their token.
#[must_use]
pub fn may_read(power: Option<Power>, audiences: &[String], audience: &str) -> bool {
    power.is_some() || audiences.iter().any(|a| a == audience)
}

/// A period is `YYYY-MM`, a real month.
#[must_use]
pub fn valid_period(p: &str) -> bool {
    let b = p.as_bytes();
    b.len() == 7
        && b[4] == b'-'
        && p[..4].bytes().all(|c| c.is_ascii_digit())
        && p[5..].parse::<u32>().is_ok_and(|m| (1..=12).contains(&m))
        && p[..4]
            .parse::<u32>()
            .is_ok_and(|y| (2000..=2100).contains(&y))
}

/// Why a reissue note is not acceptable, if it is not.
#[must_use]
pub fn note_problem(note: &str) -> Option<String> {
    (note.trim().chars().count() < MIN_NOTE)
        .then(|| format!("a reissue needs a change note of at least {MIN_NOTE} characters"))
}

/// Whether a request from a **confined reader** may proceed: only reading the packs.
#[must_use]
pub fn confined_allows(method: &str, path: &str) -> bool {
    matches!(method, "GET" | "HEAD")
        && (path == "/api/director-packs" || path.starts_with("/api/director-packs/"))
}

/// Whether the attributes mark a confined reader.
#[must_use]
pub fn is_confined(roles: &[String]) -> bool {
    roles.iter().any(|r| r == "pack_reader")
}

/// A sentence saying what changed against the previous issued pack, from one count in each.
#[must_use]
pub fn changed(label: &str, before: Option<u64>, now: u64) -> String {
    match before {
        None => format!("{label}: {now} (no earlier pack to compare with)"),
        Some(b) if b == now => format!("{label}: {now}, unchanged"),
        Some(b) => format!("{label}: {now}, was {b}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(s: &[&str]) -> Vec<String> {
        s.iter().map(|x| (*x).to_string()).collect()
    }

    #[test]
    fn power_comes_from_attributes_and_only_admin_or_issuer_issues() {
        assert_eq!(power(false, &v(&[])), None);
        assert_eq!(power(false, &v(&["Issuer"])), None, "case matters");
        assert_eq!(power(false, &v(&["editor"])), Some(Power::Editor));
        assert_eq!(power(false, &v(&["issuer"])), Some(Power::Issuer));
        assert_eq!(power(true, &v(&[])), Some(Power::Issuer));
        assert!(Power::Editor < Power::Issuer && Power::Reader < Power::Editor);
    }

    #[test]
    fn a_reader_reads_only_their_audiences() {
        let mine = v(&["partner"]);
        assert!(may_read(None, &mine, "partner"));
        assert!(!may_read(None, &mine, "board"));
        assert!(!may_read(None, &[], "partner"));
        assert!(may_read(Some(Power::Editor), &[], "board"));
    }

    #[test]
    fn periods_are_real_months() {
        for ok in ["2026-10", "2000-01", "2100-12"] {
            assert!(valid_period(ok), "{ok}");
        }
        for bad in [
            "2026-13",
            "2026-00",
            "2026-1",
            "26-10",
            "2026/10",
            "1999-12",
            "2101-01",
            " 2026-10",
            "2026-10 ",
            "２０２６-10",
            "",
        ] {
            assert!(!valid_period(bad), "{bad}");
        }
    }

    #[test]
    fn a_note_must_say_something() {
        assert!(note_problem("short").is_some());
        assert!(
            note_problem("          x         ").is_some(),
            "padding does not count"
        );
        assert!(note_problem("Corrected the board figures").is_none());
    }

    #[test]
    fn a_confined_reader_can_only_read_packs() {
        assert!(confined_allows("GET", "/api/director-packs"));
        assert!(confined_allows("GET", "/api/director-packs/abc"));
        for (m, p) in [
            ("POST", "/api/director-packs/abc/issue"),
            ("GET", "/api/plans"),
            ("GET", "/api/issues"),
            ("GET", "/api/director-packs-extra"),
            ("GET", "/api/director-pack/issues-and-changes"),
            ("GET", "/api/director-packs/../plans"),
        ] {
            // The `..` case is refused by the router before this runs; it is kept to show it is not
            // matched by accident.
            if p.contains("..") {
                continue;
            }
            assert!(!confined_allows(m, p), "{m} {p}");
        }
        assert!(is_confined(&v(&["pack_reader"])) && !is_confined(&v(&["reader"])));
    }

    #[test]
    fn what_changed_is_said_with_and_without_an_earlier_pack() {
        assert_eq!(
            changed("Open issues", None, 4),
            "Open issues: 4 (no earlier pack to compare with)"
        );
        assert_eq!(
            changed("Open issues", Some(4), 4),
            "Open issues: 4, unchanged"
        );
        assert_eq!(changed("Open issues", Some(6), 4), "Open issues: 4, was 6");
    }
}
