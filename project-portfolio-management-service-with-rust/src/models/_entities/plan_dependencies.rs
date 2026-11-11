//! Row — `plan_dependencies`. Finish-start edges
//! between plans (PPM-6), with lag; cycle-free (app-enforced).

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
    pub predecessor_pid: Uuid,
    pub successor_pid: Uuid,
    pub lag_days: i32,
    /// `fs` (default), `ss`, `ff` or `sf`.
    pub kind: String,
}
