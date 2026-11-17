//! The project portfolio management service's dates, instants, durations and time zones, over `jiff` 1.0.
//!
//! The service moved from `chrono` and `chrono-tz` to `jiff` (RUSTSEC-2026-0543, RUSTSEC-2026-0555) without
//! changing anything a client, a log reader or the database can see. jiff writes and reads some forms
//! differently (it trims a fraction to its last non-zero digit, and its RFC 3339 reader is stricter), so
//! these types wrap jiff's and keep the old forms:
//!
//! | Type | Holds | Was |
//! | --- | --- | --- |
//! | [`Timestamp`] | an instant in UTC (`jiff::Timestamp`) | `DateTime<Utc>` |
//! | [`OffsetDateTime`] | an instant and a fixed offset; a `timestamptz` column | `DateTime<FixedOffset>` |
//! | [`Date`] | a calendar date (`jiff::civil::Date`) | `NaiveDate` |
//! | [`Duration`] | a signed duration (`jiff::SignedDuration`) | `TimeDelta` |
//! | [`Tz`] | an IANA zone from the bundled database (`jiff::tz::TimeZone`) | `chrono_tz::Tz` |
//!
//! Their method names are the old ones where the service called them, so call sites read as they did.
//!
//! **Written** ([`format`]): JSON `2026-10-30T09:15:00.123400Z` (a zero offset is `Z`; the fraction in groups
//! of three digits); `to_rfc3339` `…+00:00`; `Display` `2026-10-30 09:15:00.123400 UTC` (or `… +00:00` for an
//! [`OffsetDateTime`]); `Debug` `…Z` (or `…+00:00`); dates `YYYY-MM-DD`.
//!
//! **Read** ([`parse`]): a request body's instant with chrono's relaxed RFC 3339, a query parameter's with its
//! strict one, a date with its `NaiveDate` reader, each with the same error messages.
//!
//! **Arithmetic**: months clamp the day to the end of a shorter month; a duration moves a date by its whole
//! days, toward zero; differences of dates are whole days; whole-unit counts of a duration truncate toward
//! zero; unix counts round down (so before 1970 a part second is the second before).
//!
//! **Not kept** (jiff cannot represent them): a leap second (`23:59:60`, read as `:59`), years outside
//! -9999..=9999, and instants within a day of those ends.
//!
//! With the `sqlx` feature, the types bind and decode as Postgres `timestamptz` and `date` exactly as sqlx's
//! `chrono` feature did ([`postgres`]).

mod date;
mod duration;
mod error;
pub mod format;
mod instant;
pub mod parse;
#[cfg(feature = "sqlx")]
pub mod postgres;
mod schema;
mod weekday;
mod zone;

pub use date::{Date, IsoWeek, NaiveDateTime, NaiveTime};
pub use duration::{Days, Duration, Months};
pub use error::{OutOfRangeError, ParseError, ParseErrorKind, ParseResult};
pub use format::{Formatted, SecondsFormat};
pub use instant::{OffsetDateTime, Timestamp};
pub use weekday::Weekday;
pub use zone::{LocalResult, Tz, UnknownZone};
