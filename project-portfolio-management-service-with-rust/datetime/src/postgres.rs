//! Postgres encoding (feature `sqlx`): `timestamptz` and `date`, written and read as sqlx's `chrono` feature
//! did, so the stored values do not change.
//!
//! - `timestamptz` is microseconds since 2000-01-01T00:00:00Z; nanoseconds below a microsecond are dropped
//!   toward 2000 (toward zero), as before. Read back with offset zero. The text form (rare: simple queries) is
//!   `%Y-%m-%d %H:%M:%S%.f%#z`.
//! - `date` is days since 2000-01-01; the text form `%Y-%m-%d`.
//!
//! Unlike before, a stored value outside -9999..=9999 (`infinity`, say) is a decode error rather than a panic.

use sqlx::encode::IsNull;
use sqlx::error::BoxDynError;
use sqlx::postgres::types::Oid;
use sqlx::postgres::{
    PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueFormat, PgValueRef, Postgres,
};
use sqlx::{Decode, Encode, Type};

use crate::{Date, OffsetDateTime, Timestamp};

/// 2000-01-01T00:00:00Z in nanoseconds since 1970.
const PG_EPOCH_NANOS: i128 = 946_684_800 * 1_000_000_000;
const PG_EPOCH_DATE: jiff::civil::Date = jiff::civil::Date::constant(2000, 1, 1);

fn encode_instant(ts: jiff::Timestamp, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
    let micros = i64::try_from((ts.as_nanosecond() - PG_EPOCH_NANOS) / 1000)
        .map_err(|_| format!("instant out of range for Postgres: {ts}"))?;
    <i64 as Encode<Postgres>>::encode(micros, buf)
}

fn decode_instant(value: PgValueRef<'_>) -> Result<OffsetDateTime, BoxDynError> {
    match value.format() {
        PgValueFormat::Binary => {
            let us: i64 = <i64 as Decode<Postgres>>::decode(value)?;
            let ts = jiff::Timestamp::from_nanosecond(PG_EPOCH_NANOS + i128::from(us) * 1000)
                .map_err(|e| format!("timestamptz out of range: {e}"))?;
            Ok(Timestamp(ts).fixed_offset())
        }
        PgValueFormat::Text => {
            let s = value.as_str()?;
            let pattern = if s.contains('+') || s.contains('-') {
                "%Y-%m-%d %H:%M:%S%.f%#z"
            } else {
                "%Y-%m-%d %H:%M:%S%.f"
            };
            // As before, text without an offset is refused (not enough for an instant).
            Ok(OffsetDateTime::parse_from_str(s, pattern)?)
        }
    }
}

impl Type<Postgres> for Timestamp {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_oid(Oid(1184))
    }
}

impl Type<Postgres> for OffsetDateTime {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_oid(Oid(1184))
    }
}

impl PgHasArrayType for Timestamp {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_oid(Oid(1185))
    }
}

impl PgHasArrayType for OffsetDateTime {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_oid(Oid(1185))
    }
}

impl Encode<'_, Postgres> for Timestamp {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        encode_instant(self.0, buf)
    }

    fn size_hint(&self) -> usize {
        size_of::<i64>()
    }
}

impl Encode<'_, Postgres> for OffsetDateTime {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        encode_instant(self.to_jiff(), buf)
    }

    fn size_hint(&self) -> usize {
        size_of::<i64>()
    }
}

impl<'r> Decode<'r, Postgres> for Timestamp {
    fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        decode_instant(value).map(|t| t.to_utc())
    }
}

impl<'r> Decode<'r, Postgres> for OffsetDateTime {
    fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        decode_instant(value)
    }
}

impl Type<Postgres> for Date {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_oid(Oid(1082))
    }
}

impl PgHasArrayType for Date {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_oid(Oid(1182))
    }
}

impl Encode<'_, Postgres> for Date {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        let days =
            i32::try_from(self.0.duration_since(PG_EPOCH_DATE).as_hours() / 24).map_err(|_| {
                format!("value {self:?} would overflow binary encoding for Postgres DATE")
            })?;
        <i32 as Encode<Postgres>>::encode(days, buf)
    }

    fn size_hint(&self) -> usize {
        size_of::<i32>()
    }
}

impl<'r> Decode<'r, Postgres> for Date {
    fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        match value.format() {
            PgValueFormat::Binary => {
                let days: i32 = <i32 as Decode<Postgres>>::decode(value)?;
                Date(PG_EPOCH_DATE)
                    .checked_add_signed(crate::Duration::days(days))
                    .ok_or_else(|| format!("date out of range: {days} days from 2000-01-01").into())
            }
            PgValueFormat::Text => Ok(Date::parse_from_str(value.as_str()?, "%Y-%m-%d")?),
        }
    }
}
