//! IANA time zones, from the database bundled into the binary.
//!
//! The database is jiff's bundled copy (`jiff-tzdb`), read through `TimeZoneDatabase::bundled()` only: never
//! `/usr/share/zoneinfo`, never the process zone (`TZ`), so every deployment answers alike and the image needs
//! no zone files. A name is matched exactly (`Europe/London`, never `europe/london`), as before, although
//! jiff's own lookup ignores case. An alias (`US/Pacific`, `Asia/Calcutta`) keeps its own name.

use core::fmt;
use core::hash::{Hash, Hasher};
use core::str::FromStr;
use std::collections::{HashMap, HashSet};
use std::sync::{Mutex, OnceLock, PoisonError};

use jiff::tz::{AmbiguousOffset, TimeZone, TimeZoneDatabase};

use crate::date::NaiveDateTime;
use crate::instant::{OffsetDateTime, Timestamp};

static UTC_ZONE: TimeZone = TimeZone::UTC;

/// An IANA time zone. Cheap to copy; two zones are equal when their names are.
#[derive(Clone, Copy)]
pub struct Tz {
    name: &'static str,
    zone: &'static TimeZone,
}

/// What a civil date and time is in a zone: one instant, two (a fold, when the clocks go back), or none (a
/// gap, when they go forward).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LocalResult<T> {
    /// Exactly one instant.
    Single(T),
    /// Two instants, the earlier first.
    Ambiguous(T, T),
    /// No instant.
    None,
}

impl<T> LocalResult<T> {
    /// The instant if there is exactly one.
    pub fn single(self) -> Option<T> {
        match self {
            LocalResult::Single(t) => Some(t),
            _ => None,
        }
    }

    /// The earlier instant, if there is any.
    pub fn earliest(self) -> Option<T> {
        match self {
            LocalResult::Single(t) | LocalResult::Ambiguous(t, _) => Some(t),
            LocalResult::None => None,
        }
    }

    /// The later instant, if there is any.
    pub fn latest(self) -> Option<T> {
        match self {
            LocalResult::Single(t) | LocalResult::Ambiguous(_, t) => Some(t),
            LocalResult::None => None,
        }
    }
}

/// Why a zone name was refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnknownZone(());

impl fmt::Display for UnknownZone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("failed to parse timezone")
    }
}

impl std::error::Error for UnknownZone {}

fn database() -> &'static TimeZoneDatabase {
    static DB: OnceLock<TimeZoneDatabase> = OnceLock::new();
    DB.get_or_init(TimeZoneDatabase::bundled)
}

/// Every name in the bundled database, exactly as it is spelled there, but `Factory` (a placeholder zone with no
/// offset, which the zone list the service used before did not carry, so it stays refused).
fn names() -> &'static HashSet<String> {
    static NAMES: OnceLock<HashSet<String>> = OnceLock::new();
    NAMES.get_or_init(|| {
        database()
            .available()
            .map(|n| n.as_str().to_string())
            .filter(|n| n != "Factory")
            .collect()
    })
}

impl Tz {
    /// Coordinated Universal Time.
    pub const UTC: Tz = Tz {
        name: "UTC",
        zone: &UTC_ZONE,
    };

    /// The zone of this exact IANA name, or `None`.
    #[must_use]
    pub fn named(name: &str) -> Option<Tz> {
        static INTERNED: OnceLock<Mutex<HashMap<&'static str, Tz>>> = OnceLock::new();
        if name == "UTC" {
            return Some(Tz::UTC);
        }
        let interned = INTERNED.get_or_init(|| Mutex::new(HashMap::new()));
        let mut interned = interned.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(tz) = interned.get(name) {
            return Some(*tz);
        }
        if !names().contains(name) {
            return None;
        }
        let zone = database().get(name).ok()?;
        // At most one leak per distinct valid name, of which there are a few hundred.
        let name: &'static str = Box::leak(name.to_string().into_boxed_str());
        let zone: &'static TimeZone = Box::leak(Box::new(zone));
        let tz = Tz { name, zone };
        interned.insert(name, tz);
        Some(tz)
    }

    /// The IANA name, as given.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// The `jiff` zone underneath.
    #[must_use]
    pub const fn to_jiff(&self) -> &'static TimeZone {
        self.zone
    }

    /// The offset from UTC at an instant, in seconds east.
    #[must_use]
    pub fn offset_seconds_at(&self, t: Timestamp) -> i32 {
        self.zone.to_offset(t.to_jiff()).seconds()
    }

    /// The instant shown in this zone, with the zone's offset at that instant.
    #[must_use]
    pub fn at(&self, t: Timestamp) -> OffsetDateTime {
        OffsetDateTime::from_instant(t, self.offset_seconds_at(t))
            .unwrap_or_else(|| t.fixed_offset())
    }

    /// The instants a civil date and time names in this zone.
    #[must_use]
    pub fn from_local_datetime(&self, local: &NaiveDateTime) -> LocalResult<Timestamp> {
        let ambiguous = self.zone.to_ambiguous_timestamp(local.0);
        let at =
            |offset: jiff::tz::Offset| offset.to_timestamp(local.0).ok().map(Timestamp::from_jiff);
        match ambiguous.offset() {
            AmbiguousOffset::Unambiguous { offset } => {
                at(offset).map_or(LocalResult::None, LocalResult::Single)
            }
            AmbiguousOffset::Gap { .. } => LocalResult::None,
            AmbiguousOffset::Fold { before, after } => match (at(before), at(after)) {
                (Some(a), Some(b)) if a <= b => LocalResult::Ambiguous(a, b),
                (Some(a), Some(b)) => LocalResult::Ambiguous(b, a),
                (Some(t), None) | (None, Some(t)) => LocalResult::Single(t),
                (None, None) => LocalResult::None,
            },
        }
    }
}

impl PartialEq for Tz {
    fn eq(&self, other: &Tz) -> bool {
        self.name == other.name
    }
}

impl Eq for Tz {}

impl Hash for Tz {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

/// The IANA name.
impl fmt::Debug for Tz {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name)
    }
}

/// The IANA name.
impl fmt::Display for Tz {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name)
    }
}

impl FromStr for Tz {
    type Err = UnknownZone;

    fn from_str(s: &str) -> Result<Tz, UnknownZone> {
        Tz::named(s).ok_or(UnknownZone(()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::hash::{DefaultHasher, Hash, Hasher};

    #[test]
    fn names_match_exactly() {
        assert!(Tz::named("Europe/London").is_some());
        assert!(Tz::named("europe/london").is_none());
        assert!(Tz::named("utc").is_none());
        assert!(Tz::named("Factory").is_none());
        assert_eq!(Tz::named("Etc/UTC").unwrap().name(), "Etc/UTC");
        assert_eq!(Tz::named("UTC"), Some(Tz::UTC));
        assert_eq!(Tz::named("US/Pacific").unwrap().to_string(), "US/Pacific");
    }

    #[test]
    fn a_gap_is_none_and_a_fold_is_both() {
        let la = Tz::named("America/Los_Angeles").unwrap();
        let d = crate::Date::from_ymd_opt(2026, 3, 8).unwrap();
        assert_eq!(
            la.from_local_datetime(&d.and_hms_opt(2, 30, 0).unwrap()),
            LocalResult::None
        );
        let d = crate::Date::from_ymd_opt(2026, 11, 1).unwrap();
        let LocalResult::Ambiguous(a, b) =
            la.from_local_datetime(&d.and_hms_opt(1, 30, 0).unwrap())
        else {
            panic!("a fold");
        };
        assert_eq!(
            (format!("{a:?}"), format!("{b:?}")),
            ("2026-11-01T08:30:00Z".into(), "2026-11-01T09:30:00Z".into())
        );
    }

    #[test]
    fn local_results_and_zone_value_semantics_cover_every_shape() {
        let one = LocalResult::Single(2);
        assert_eq!(one.single(), Some(2));
        assert_eq!(one.earliest(), Some(2));
        assert_eq!(one.latest(), Some(2));
        let fold = LocalResult::Ambiguous(1, 3);
        assert_eq!(fold.single(), None);
        assert_eq!(fold.earliest(), Some(1));
        assert_eq!(fold.latest(), Some(3));
        let none = LocalResult::<i32>::None;
        assert_eq!(none.single(), None);
        assert_eq!(none.earliest(), None);
        assert_eq!(none.latest(), None);

        let london: Tz = "Europe/London".parse().unwrap();
        let cached = Tz::named("Europe/London").unwrap();
        assert_eq!(london, cached);
        assert_eq!(format!("{london:?}"), "Europe/London");
        assert_eq!(london.to_string(), "Europe/London");
        assert_eq!(london.name(), "Europe/London");
        assert_eq!(london.to_jiff().iana_name(), Some("Europe/London"));
        assert_eq!(
            "not/a-zone".parse::<Tz>().unwrap_err().to_string(),
            "failed to parse timezone"
        );

        let winter: Timestamp = "2026-01-15T12:00:00Z".parse().unwrap();
        let summer: Timestamp = "2026-07-15T12:00:00Z".parse().unwrap();
        assert_eq!(london.offset_seconds_at(winter), 0);
        assert_eq!(london.offset_seconds_at(summer), 3600);
        assert_eq!(london.at(summer).hour(), 13);

        let mut first = DefaultHasher::new();
        let mut second = DefaultHasher::new();
        london.hash(&mut first);
        cached.hash(&mut second);
        assert_eq!(first.finish(), second.finish());
    }
}
