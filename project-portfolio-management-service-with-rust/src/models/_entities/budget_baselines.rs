//! Row — `budget_baselines`. The **append-only**, versioned
//! phased budget baseline (T-28b). See
//! `migration/migrations/20260918000001_budget_baselines.up.sql` for the schema
//! rationale.

// The row's shape is documented by the migration.
#![expect(missing_docs)]

use super::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub id: i32,
    pub pid: Uuid,
    pub plan_pid: Uuid,
    pub version: i32,
    pub currency: String,
    pub approved_at: DateTimeWithTimeZone,
    /// Required for `version > 1`: a re-baseline with no reason is
    /// indistinguishable from a mistake.
    pub reason: Option<String>,
}
