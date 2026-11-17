//! The reading error: the same seven kinds, with the same messages, as `chrono::ParseError`, because the message
//! reaches clients (a request body's serde error, a query parameter's `422`).

use core::fmt;

/// Why a date or instant could not be read.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ParseErrorKind {
    /// A field is out of its range (a month of 13, the 30th of February, an offset of a day or more).
    OutOfRange,
    /// The fields contradict each other.
    Impossible,
    /// Not enough fields to name one date or instant.
    NotEnough,
    /// A character that does not belong where it is.
    Invalid,
    /// The text ends too early.
    TooShort,
    /// Text is left over after a complete value.
    TooLong,
    /// The pattern itself is not one this crate reads.
    BadFormat,
}

/// A date or instant that could not be read. Its `Display` is chrono's message for the same kind.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ParseError(pub(crate) ParseErrorKind);

impl ParseError {
    /// Which kind of failure this is.
    #[must_use]
    pub const fn kind(&self) -> ParseErrorKind {
        self.0
    }
}

pub(crate) const OUT_OF_RANGE: ParseError = ParseError(ParseErrorKind::OutOfRange);
pub(crate) const IMPOSSIBLE: ParseError = ParseError(ParseErrorKind::Impossible);
pub(crate) const NOT_ENOUGH: ParseError = ParseError(ParseErrorKind::NotEnough);
pub(crate) const INVALID: ParseError = ParseError(ParseErrorKind::Invalid);
pub(crate) const TOO_SHORT: ParseError = ParseError(ParseErrorKind::TooShort);
pub(crate) const TOO_LONG: ParseError = ParseError(ParseErrorKind::TooLong);
pub(crate) const BAD_FORMAT: ParseError = ParseError(ParseErrorKind::BadFormat);

/// The result of a read.
pub type ParseResult<T> = Result<T, ParseError>;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self.0 {
            ParseErrorKind::OutOfRange => "input is out of range",
            ParseErrorKind::Impossible => "no possible date and time matching input",
            ParseErrorKind::NotEnough => "input is not enough for unique date and time",
            ParseErrorKind::Invalid => "input contains invalid characters",
            ParseErrorKind::TooShort => "premature end of input",
            ParseErrorKind::TooLong => "trailing input",
            ParseErrorKind::BadFormat => "bad or unsupported format string",
        })
    }
}

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseError({:?})", self.0)
    }
}

impl std::error::Error for ParseError {}

/// A duration that does not fit where it was asked to go (`Duration::to_std` of a negative one, for example).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OutOfRangeError(pub(crate) ());

impl fmt::Display for OutOfRangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("out of range")
    }
}

impl std::error::Error for OutOfRangeError {}
