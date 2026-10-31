//! Time-zone doctrine (LT-9) and the one helper that applies it.
//!
//! **Doctrine.**
//!
//! 1. **Store UTC.** Every instant is `timestamptz`, written from
//!    `Utc::now()`. Nothing stores a viewer-local wall-clock time.
//! 2. **Dates are calendar dates.** A `date` column or a `YYYY-MM-DD`
//!    string (a task's `due_date`, a time entry's `spent_on`) names a day
//!    with **no zone** — "due 2026-10-03" is that day wherever you are.
//! 3. **The server never reads its own zone.** `chrono::Local` is banned by
//!    `clippy.toml` and by a test below; the process may run in any `TZ`
//!    and every answer is unchanged.
//! 4. **Portfolio-wide views use the deployment's "today" zone** ("today" in
//!    `at-a-glance`, overdue risks, …): an explicit administrator setting
//!    (UTC until set), so people in many zones see one answer.
//! 5. **Personal views take an explicit IANA zone** per request (`?tz=`) and
//!    compute "today" there with [`local_today`]. A fixed offset is refused:
//!    it cannot know about daylight saving, which is exactly the
//!    midnight-boundary bug this doctrine exists to prevent.
//!
//! DB-free and unit-tested.

use chrono::{DateTime, NaiveDate, Utc};
pub use chrono_tz::Tz;

/// Parse an IANA zone name (`Pacific/Auckland`, `UTC`). Fixed offsets
/// (`+02:00`) and abbreviations (`PST`) are refused with a message that
/// says why.
///
/// # Errors
///
/// A human-readable message when the name is blank or is not an IANA zone.
pub fn parse_zone(name: &str) -> Result<Tz, String> {
    let name = name.trim();
    if name.is_empty() {
        return Err("time zone is required (an IANA name such as Asia/Kolkata, America/Sao_Paulo or Africa/Lagos)".to_string());
    }
    name.parse::<Tz>().map_err(|_| {
        format!(
            "unknown time zone {name:?}: use an IANA name such as Asia/Kolkata, America/Sao_Paulo or Africa/Lagos \
             (fixed offsets and abbreviations are refused: they cannot follow daylight saving)"
        )
    })
}

/// The calendar date it is, right now, in `tz`. The instant is converted,
/// never the other way round, so a daylight-saving gap or overlap cannot
/// make this ambiguous.
#[must_use]
pub fn local_today(now: DateTime<Utc>, tz: Tz) -> NaiveDate {
    now.with_timezone(&tz).date_naive()
}

/// The zone that defines "today" for portfolio-wide views and defaults: an explicit administrator
/// setting (`deployment_settings`, key `today_time_zone`), UTC until one is set. Read through
/// [`today_zone`], written only through [`set_today_zone`] by the settings controller and its
/// refresher, so every instance converges on the stored value.
static TODAY_ZONE: std::sync::RwLock<Tz> = std::sync::RwLock::new(Tz::UTC);

/// The zone that currently defines "today" for portfolio-wide views.
#[must_use]
pub fn today_zone() -> Tz {
    *TODAY_ZONE
        .read()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// Replace the zone that defines "today". Callers: the settings endpoint and the refresher.
pub fn set_today_zone(zone: Tz) {
    *TODAY_ZONE
        .write()
        .unwrap_or_else(std::sync::PoisonError::into_inner) = zone;
}

/// The calendar date it is at `now` in the deployment's "today" zone `zone`. Pure, so it is
/// testable without touching the shared setting; [`deployment_today`] is this with the clock and
/// the configured zone.
#[must_use]
pub fn deployment_today_at(now: DateTime<Utc>, zone: Tz) -> NaiveDate {
    local_today(now, zone)
}

/// The calendar date it is, right now, in the deployment's "today" zone (doctrine 4): what a
/// portfolio-wide view or a default means by "today", so everyone sees one answer however many
/// zones they sit in. A personal view takes a zone per request and uses [`local_today`]. Calls name
/// this rather than `Utc::now().date_naive()`, so each use is findable and a census can ban the raw
/// form.
#[must_use]
pub fn deployment_today() -> NaiveDate {
    deployment_today_at(Utc::now(), today_zone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn at(y: i32, m: u32, d: u32, h: u32, min: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(y, m, d, h, min, 0).unwrap()
    }

    fn day(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn one_instant_is_a_different_date_in_different_zones() {
        let now = at(2026, 10, 2, 23, 30); // late evening UTC
        assert_eq!(
            local_today(now, parse_zone("UTC").unwrap()),
            day(2026, 10, 2)
        );
        // Auckland is UTC+13 in October (daylight saving) → already the 3rd.
        assert_eq!(
            local_today(now, parse_zone("Pacific/Auckland").unwrap()),
            day(2026, 10, 3)
        );
        // Los Angeles is UTC-7 (PDT) → mid-afternoon on the 2nd.
        assert_eq!(
            local_today(now, parse_zone("America/Los_Angeles").unwrap()),
            day(2026, 10, 2)
        );
    }

    #[test]
    fn the_date_follows_daylight_saving_not_a_fixed_offset() {
        let la = parse_zone("America/Los_Angeles").unwrap();
        // The same wall-clock UTC time, either side of the autumn change
        // (2026-11-01 09:00Z): PDT before, PST after.
        assert_eq!(local_today(at(2026, 11, 1, 7, 30), la), day(2026, 11, 1)); // 00:30 PDT
        assert_eq!(local_today(at(2026, 11, 1, 8, 30), la), day(2026, 11, 1)); // 01:30 PDT
        assert_eq!(local_today(at(2026, 11, 1, 9, 30), la), day(2026, 11, 1)); // 01:30 PST (repeated hour)
        assert_eq!(local_today(at(2026, 11, 2, 7, 59), la), day(2026, 11, 1)); // 23:59 PST
        assert_eq!(local_today(at(2026, 11, 2, 8, 0), la), day(2026, 11, 2)); // 00:00 PST
    }

    #[test]
    fn fixed_offsets_abbreviations_and_blanks_are_refused() {
        for bad in ["", "  ", "+02:00", "PST", "Mars/Olympus", "utc+1"] {
            assert!(parse_zone(bad).is_err(), "{bad:?} should be refused");
        }
        assert!(parse_zone(" Europe/London ").is_ok(), "trimmed");
        // WW-1.13: the examples the messages give are real zones.
        for zone in [
            "Asia/Kolkata",
            "America/Sao_Paulo",
            "Africa/Lagos",
            "Pacific/Auckland",
        ] {
            assert!(parse_zone(zone).is_ok(), "{zone}");
        }
        let e = parse_zone("").unwrap_err();
        assert!(e.contains("Asia/Kolkata") && !e.contains("London"), "{e}");
    }

    /// Whether `needle` occurs in `text` as a whole identifier path — not as
    /// the prefix of a longer name such as `chrono::LocalResult`.
    fn contains_word(text: &str, needle: &str) -> bool {
        text.match_indices(needle).any(|(i, _)| {
            !text[i + needle.len()..]
                .chars()
                .next()
                .is_some_and(|c| c.is_alphanumeric() || c == '_')
        })
    }

    #[test]
    fn the_guard_matches_whole_names_only() {
        assert!(contains_word("use chrono::Local;", "chrono::Local"));
        assert!(contains_word("let t = Local::now();", "Local::now"));
        assert!(!contains_word(
            "match chrono::LocalResult::None",
            "chrono::Local"
        ));
    }

    /// LT-9: no source file may read the process time zone. `clippy.toml`
    /// bans it too; this fails even where clippy is not run.
    #[test]
    fn no_source_file_reads_the_process_time_zone() {
        let needles = [format!("chrono::{}", "Local"), format!("{}::now", "Local")];
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut stack = vec![root];
        let mut offenders = Vec::new();
        while let Some(dir) = stack.pop() {
            for entry in std::fs::read_dir(&dir).expect("read src") {
                let path = entry.expect("entry").path();
                if path.is_dir() {
                    stack.push(path);
                } else if path.extension().is_some_and(|e| e == "rs")
                    // This file documents the ban, so it names the banned type.
                    && path.file_name().is_some_and(|n| n != "tz.rs")
                {
                    let text = std::fs::read_to_string(&path).expect("read file");
                    if needles.iter().any(|n| contains_word(&text, n)) {
                        offenders.push(path.display().to_string());
                    }
                }
            }
        }
        assert!(
            offenders.is_empty(),
            "reads the process zone: {offenders:?}"
        );
    }

    /// Every portfolio-wide "today" goes through `deployment_today`; a personal one
    /// goes through `local_today`. The raw form hides which was meant.
    #[test]
    fn no_raw_deployment_today_outside_this_file() {
        fn walk(dir: &std::path::Path, hits: &mut Vec<String>) {
            for e in std::fs::read_dir(dir).unwrap().flatten() {
                let p = e.path();
                if p.is_dir() {
                    walk(&p, hits);
                } else if p.extension().is_some_and(|x| x == "rs") && !p.ends_with("tz.rs") {
                    let text = std::fs::read_to_string(&p).unwrap();
                    if text.contains("Utc::now().date_naive()") {
                        hits.push(p.display().to_string());
                    }
                }
            }
        }
        let mut hits = Vec::new();
        walk(
            &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src"),
            &mut hits,
        );
        assert!(
            hits.is_empty(),
            "use tz::deployment_today() or tz::local_today(): {hits:?}"
        );
    }

    /// Until an administrator sets a zone, "today" is the UTC date, as it was before the setting
    /// existed. (No test in this binary sets the shared zone; see `deployment_today_at` for those.)
    #[test]
    fn deployment_today_defaults_to_the_utc_date() {
        assert_eq!(today_zone(), Tz::UTC);
        let before = Utc::now().date_naive();
        let t = deployment_today();
        assert!(t >= before && t <= Utc::now().date_naive());
    }

    /// The configured zone decides the day near midnight. Honolulu is UTC-10 all year: 09:30Z on
    /// the 2nd is 23:30 on the 1st there, and 10:00Z is midnight into the 2nd.
    #[test]
    fn the_configured_zone_decides_the_day_near_midnight() {
        let honolulu = parse_zone("Pacific/Honolulu").unwrap();
        let utc = Tz::UTC;
        assert_eq!(
            deployment_today_at(at(2026, 10, 2, 9, 30), honolulu),
            day(2026, 10, 1)
        );
        assert_eq!(
            deployment_today_at(at(2026, 10, 2, 9, 30), utc),
            day(2026, 10, 2)
        );
        assert_eq!(
            deployment_today_at(at(2026, 10, 2, 9, 59), honolulu),
            day(2026, 10, 1)
        );
        assert_eq!(
            deployment_today_at(at(2026, 10, 2, 10, 0), honolulu),
            day(2026, 10, 2)
        );
        // A zone with daylight saving moves its midnight with the clocks (Auckland: UTC+13 in
        // October, UTC+12 in June).
        let auckland = parse_zone("Pacific/Auckland").unwrap();
        assert_eq!(
            deployment_today_at(at(2026, 10, 2, 11, 30), auckland),
            day(2026, 10, 3)
        );
        assert_eq!(
            deployment_today_at(at(2026, 6, 2, 11, 30), auckland),
            day(2026, 6, 2)
        );
        assert_eq!(
            deployment_today_at(at(2026, 6, 2, 12, 0), auckland),
            day(2026, 6, 3)
        );
    }
}
