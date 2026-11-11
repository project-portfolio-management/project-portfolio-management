//! Row — `objective_links`. A weighted plan →
//! objective mapping (PPM-5); unique per pair.

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
    pub objective_pid: Uuid,
    pub plan_pid: Uuid,
    pub weight: i32,
    /// A programme's one primary strategic aim (DP-4).
    pub is_primary: bool,
}
