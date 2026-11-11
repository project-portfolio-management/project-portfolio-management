//! Row — `proposals`. Work-intake demand records
//! (PPM-1): the pipeline that stands between an idea and a funded
//! plan.

#![expect(missing_docs)]

use super::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub id: i64,
    pub pid: Uuid,
    pub title: String,
    pub summary: Option<String>,
    pub kind_target: String,
    pub sponsor_ref: Option<String>,
    pub strategic_rationale: Option<String>,
    pub requested_minor: Option<i64>,
    pub currency: Option<String>,
    pub status: String,
    pub promoted_plan_pid: Option<Uuid>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
    /// An ask from the sponsor (EX-13): approving it needs a named trade-off.
    pub external_ask: bool,
    /// What the ask displaces.
    pub tradeoff: Option<String>,
}
