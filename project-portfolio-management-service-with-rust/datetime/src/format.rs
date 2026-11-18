//! The writers: RFC 3339 text, `Display`, `Debug`, and the `%` patterns the service writes with.
//!
//! The forms are the ones chrono 0.4 wrote, byte for byte:
//!
//! - A fraction of a second is written in groups of three digits (none, milliseconds, microseconds or
//!   nanoseconds), never trimmed further: `.1234` is written `.123400`.
//! - An offset is written to the minute (rounded to the nearest), `+HH:MM`; JSON writes `Z` for zero.
//! - A year from 0 to 9999 is four digits; outside, it takes a sign.
//! - `%` patterns: `%Y`, `%m`, `%d`, `%H`, `%M`, `%S` and `%%`; anything else is an error (and `to_string`
//!   panics, as it did for a bad pattern before).

use core::fmt;

use jiff::civil;

/// The fields a pattern can name.
#[derive(Clone, Copy)]
pub(crate) struct Fields {
    date: civil::Date,
    time: Option<civil::Time>,
}

impl Fields {
    #[inline]
    pub(crate) fn date(date: civil::Date) -> Fields {
        Fields { date, time: None }
    }

    #[inline]
    pub(crate) fn datetime(dt: civil::DateTime) -> Fields {
        Fields {
            date: dt.date(),
            time: Some(dt.time()),
        }
    }
}

/// A value and a `%` pattern, written when displayed.
pub struct Formatted<'a> {
    fields: Fields,
    pattern: &'a str,
}

impl<'a> Formatted<'a> {
    #[inline]
    pub(crate) fn new(fields: Fields, pattern: &'a str) -> Formatted<'a> {
        Formatted { fields, pattern }
    }
}

fn write_year(f: &mut fmt::Formatter<'_>, year: i16) -> fmt::Result {
    if (0..=9999).contains(&year) {
        write!(f, "{year:04}")
    } else {
        write!(f, "{year:+05}")
    }
}

impl fmt::Display for Formatted<'_> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut rest = self.pattern;
        while let Some(i) = rest.find('%') {
            f.write_str(&rest[..i])?;
            let spec = rest[i + 1..].chars().next().ok_or(fmt::Error)?;
            let time = self.fields.time;
            match spec {
                'Y' => write_year(f, self.fields.date.year())?,
                'm' => write!(f, "{:02}", self.fields.date.month())?,
                'd' => write!(f, "{:02}", self.fields.date.day())?,
                'H' => write!(f, "{:02}", time.ok_or(fmt::Error)?.hour())?,
                'M' => write!(f, "{:02}", time.ok_or(fmt::Error)?.minute())?,
                'S' => write!(f, "{:02}", time.ok_or(fmt::Error)?.second())?,
                '%' => f.write_str("%")?,
                _ => return Err(fmt::Error),
            }
            rest = &rest[i + 1 + spec.len_utf8()..];
        }
        f.write_str(rest)
    }
}

impl fmt::Debug for Formatted<'_> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

/// How many fraction digits [`crate::Timestamp::to_rfc3339_opts`] writes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SecondsFormat {
    /// Whole seconds.
    Secs,
    /// Always three digits.
    Millis,
    /// Always six digits.
    Micros,
    /// Always nine digits.
    Nanos,
    /// As many groups of three as the value needs (none, three, six or nine).
    AutoSi,
}

/// `HH:MM:SS` and the fraction in groups of three.
pub(crate) fn write_time(w: &mut (impl fmt::Write + ?Sized), t: civil::Time) -> fmt::Result {
    let mut b = Buf::new();
    b.hms(t.hour(), t.minute(), t.second());
    b.fraction(t.subsec_nanosecond(), SecondsFormat::AutoSi);
    w.write_str(b.as_str())
}

/// An offset as `Display` and `Debug` write it: `+HH:MM`, or `+HH:MM:SS` when it has seconds.
pub(crate) fn write_offset_exact(w: &mut (impl fmt::Write + ?Sized), offset: i32) -> fmt::Result {
    let (sign, off) = if offset < 0 {
        ('-', -offset)
    } else {
        ('+', offset)
    };
    let sec = off % 60;
    let mins = off / 60;
    if sec == 0 {
        write!(w, "{sign}{:02}:{:02}", mins / 60, mins % 60)
    } else {
        write!(w, "{sign}{:02}:{:02}:{sec:02}", mins / 60, mins % 60)
    }
}

/// A small stack buffer the hot writers fill digit by digit and hand over in one `write_str` (the `{:02}`
/// machinery of `write!` is several times slower, and JSON writes many instants).
pub(crate) struct Buf {
    bytes: [u8; 48],
    len: usize,
}

impl Buf {
    #[inline]
    pub(crate) const fn new() -> Buf {
        Buf {
            bytes: [0; 48],
            len: 0,
        }
    }

    #[inline]
    fn byte(&mut self, b: u8) {
        self.bytes[self.len] = b;
        self.len += 1;
    }

    /// `n` (non-negative) as exactly `width` digits.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // a digit is 0..=9
    #[inline]
    fn digits(&mut self, n: i32, width: usize) {
        let mut v = n.unsigned_abs();
        for i in (0..width).rev() {
            self.bytes[self.len + i] = b'0' + (v % 10) as u8;
            v /= 10;
        }
        self.len += width;
    }

    /// A date: four digits from year 0 to 9999, otherwise a sign and at least four.
    #[inline]
    pub(crate) fn date(&mut self, d: civil::Date) {
        let year = i32::from(d.year());
        if (0..=9999).contains(&year) {
            self.digits(year, 4);
        } else {
            self.byte(if year < 0 { b'-' } else { b'+' });
            self.digits(year, 4);
        }
        self.byte(b'-');
        self.digits(i32::from(d.month()), 2);
        self.byte(b'-');
        self.digits(i32::from(d.day()), 2);
    }

    #[inline]
    fn hms(&mut self, h: i8, m: i8, s: i8) {
        self.digits(i32::from(h), 2);
        self.byte(b':');
        self.digits(i32::from(m), 2);
        self.byte(b':');
        self.digits(i32::from(s), 2);
    }

    #[inline]
    fn fraction(&mut self, nano: i32, secform: SecondsFormat) {
        let (v, width) = match secform {
            SecondsFormat::Secs => return,
            SecondsFormat::Millis => (nano / 1_000_000, 3),
            SecondsFormat::Micros => (nano / 1000, 6),
            SecondsFormat::AutoSi if nano == 0 => return,
            SecondsFormat::AutoSi if nano % 1_000_000 == 0 => (nano / 1_000_000, 3),
            SecondsFormat::AutoSi if nano % 1000 == 0 => (nano / 1000, 6),
            SecondsFormat::Nanos | SecondsFormat::AutoSi => (nano, 9),
        };
        self.byte(b'.');
        self.digits(v, width);
    }

    #[inline]
    fn offset_minutes(&mut self, offset: i32, use_z: bool) {
        if use_z && offset == 0 {
            self.byte(b'Z');
            return;
        }
        self.byte(if offset < 0 { b'-' } else { b'+' });
        let minutes = (offset.abs() + 30) / 60;
        self.digits(minutes / 60, 2);
        self.byte(b':');
        self.digits(minutes % 60, 2);
    }

    #[inline]
    pub(crate) fn as_str(&self) -> &str {
        // Only ASCII digits and punctuation are ever pushed.
        core::str::from_utf8(&self.bytes[..self.len]).unwrap_or_default()
    }
}

/// RFC 3339: the civil date and time at `offset`, the fraction as `secform` says, and the offset to the minute.
pub(crate) fn write_rfc3339(
    w: &mut (impl fmt::Write + ?Sized),
    dt: civil::DateTime,
    offset: i32,
    secform: SecondsFormat,
    use_z: bool,
) -> fmt::Result {
    let mut b = Buf::new();
    b.date(dt.date());
    b.byte(b'T');
    b.hms(dt.hour(), dt.minute(), dt.second());
    b.fraction(dt.subsec_nanosecond(), secform);
    b.offset_minutes(offset, use_z);
    w.write_str(b.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn minutes(offset: i32, use_z: bool) -> String {
        let mut b = Buf::new();
        b.offset_minutes(offset, use_z);
        b.as_str().to_string()
    }

    #[test]
    fn offsets_round_to_the_minute_and_show_seconds_only_where_exact() {
        let mut s = String::new();
        s.push_str(&minutes(-(43 * 60 + 8), false));
        assert_eq!(s, "-00:43");
        s.clear();
        s.push_str(&minutes(44 * 60 + 30, false));
        assert_eq!(s, "+00:45");
        s.clear();
        write_offset_exact(&mut s, -(43 * 60 + 8)).unwrap();
        assert_eq!(s, "-00:43:08");
        s.clear();
        s.push_str(&minutes(0, true));
        assert_eq!(s, "Z");
    }

    #[test]
    fn an_unknown_pattern_is_an_error() {
        let f = Formatted::new(Fields::date(civil::date(2026, 1, 2)), "%Y %q");
        assert!(fmt::write(&mut String::new(), format_args!("{f}")).is_err());
        let f = Formatted::new(Fields::date(civil::date(2026, 1, 2)), "%Y-%m-%d 100%%");
        assert_eq!(f.to_string(), "2026-01-02 100%");
    }
}
