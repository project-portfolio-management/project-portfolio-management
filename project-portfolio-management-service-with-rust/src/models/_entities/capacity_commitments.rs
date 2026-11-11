//! Row — `capacity_commitments`. Capacity promised to (supply) or work owed on (demand) a
//! programme, for a pool and a month, by a partner or by the organization itself.

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
    pub pool_pid: Uuid,
    pub plan_pid: Uuid,
    /// The partner's name as a label (until partner organizations exist); none is the organization's own.
    pub organization: Option<String>,
    /// The organization record it names (EX-3); the label above is then its name.
    pub organization_pid: Option<Uuid>,
    /// `supply` or `demand`.
    pub kind: String,
    /// The first day of the month.
    pub month: Date,
    pub committed_milli_fte: i32,
    pub delivered_milli_fte: Option<i32>,
    pub note: Option<String>,
    pub updated_by: Option<String>,
}
