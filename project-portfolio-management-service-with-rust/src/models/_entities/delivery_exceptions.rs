//! Row — `delivery_exceptions`. Opened when delivery confidence reaches amber-red or red.

#![expect(missing_docs)]

use super::{Date, DateTimeWithTimeZone};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub id: i64,
    pub pid: Uuid,
    pub plan_pid: Uuid,
    /// `open` or `closed`.
    pub status: String,
    pub opened_at: DateTimeWithTimeZone,
    pub opened_by: Option<String>,
    pub opened_value: String,
    pub worst_value: String,
    pub worsened_count: i32,
    pub cause: Option<String>,
    pub decision_needed: Option<String>,
    /// A reference to the named owner (used to notify; never shown in a pack).
    pub owner_ref: Option<String>,
    /// The owner's role, as shown in a pack.
    pub owner_role: Option<String>,
    pub decide_by: Option<Date>,
    pub completed_at: Option<DateTimeWithTimeZone>,
    pub breach_notified_at: Option<DateTimeWithTimeZone>,
    pub closed_at: Option<DateTimeWithTimeZone>,
    pub decision: Option<String>,
    pub decided_by: Option<String>,
    /// The threshold that opened it (EX-7); none for a confidence exception.
    pub threshold_kind: Option<String>,
}
