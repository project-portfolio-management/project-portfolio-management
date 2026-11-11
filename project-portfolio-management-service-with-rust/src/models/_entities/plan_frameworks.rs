//! Row — `plan_frameworks`. Which framework a plan follows, per family (at most one of each). A plan
//! with no row for a family inherits its nearest ancestor's; that is resolved on read and never stored.

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
    pub family: String,
    pub framework_pid: Uuid,
    /// The version of the framework this plan follows; it stays until the plan is moved.
    pub version: i32,
    pub set_by: Option<String>,
}
