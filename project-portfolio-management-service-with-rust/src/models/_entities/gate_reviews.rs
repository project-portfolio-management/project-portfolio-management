//! Row — `gate_reviews`. One phase-gate decision on a
//! plan (PPM-3); an approved review advances the item's `stage`.

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
    pub plan_pid: Uuid,
    pub gate: String,
    pub decision: String,
    pub conditions: Option<String>,
    pub approver_ref: Option<String>,
    pub decided_at: DateTimeWithTimeZone,
}
