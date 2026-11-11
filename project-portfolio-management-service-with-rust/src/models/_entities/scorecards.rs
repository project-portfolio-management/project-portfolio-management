//! Row — `scorecards`. A named set of KPIs read by quarter (EX-10).

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
    pub name: String,
    /// The scorecard the de-escalation tracker reads (at most one).
    pub for_deescalation: bool,
    pub active: bool,
}
