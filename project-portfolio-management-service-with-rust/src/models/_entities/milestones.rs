//! Row — `milestones`. Dated checkpoints on a plan
//! (PPM-6).

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
    pub name: String,
    pub due: Date,
    pub done: bool,
    pub done_at: Option<DateTimeWithTimeZone>,
    pub kind: Option<String>,
    /// `manual` (a person marks it done) or `linked` (achieved when its linked tasks are all done).
    pub completion_mode: String,
    /// Why it was completed by hand while linked tasks were still open.
    pub done_reason: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
