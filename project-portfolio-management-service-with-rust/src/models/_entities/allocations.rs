//! Row — `allocations`. A person's percentage allocation
//! to a plan over a timeframe (PPM-8). The person is an
//! `EntityRef` URN — never copied demographics, never a matcher signal.

#![expect(missing_docs)]

use super::{Date, DateTimeWithTimeZone, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub id: i64,
    pub pid: Uuid,
    pub plan_pid: Uuid,
    pub person_ref: String,
    pub role: Option<String>,
    pub percent: i32,
    /// `committed` (firm, the default) or `proposed` (hoped for; PJ-4).
    pub booking: String,
    /// Asked of the resource manager (PJ-4).
    pub requested: bool,
    pub start_date: Option<Date>,
    pub end_date: Option<Date>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
    /// Short skill tags this allocation requires (T-28c). Only the
    /// requirement lands here — whether the assigned person holds a
    /// tag is resolved live against the worker service, never stored.
    pub skills_required: Json,
}
