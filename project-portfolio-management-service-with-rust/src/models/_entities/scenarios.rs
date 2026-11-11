//! Row — `scenarios`. What-if candidate portfolios
//! (PPM-4): a membership set + constraint knobs, evaluated on read.

#![expect(missing_docs)]

use super::{DateTimeWithTimeZone, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub id: i64,
    pub pid: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub members: Json,
    pub budget_cap_minor: Option<i64>,
    pub currency: Option<String>,
    pub must_include: Json,
    pub status: String,
    pub committed_at: Option<DateTimeWithTimeZone>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
