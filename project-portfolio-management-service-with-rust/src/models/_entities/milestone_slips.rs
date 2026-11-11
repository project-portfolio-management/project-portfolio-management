//! Row — `milestone_slips`. One reschedule of a milestone: the old and new due dates, why, and who. Append-only: the
//! history is how many times and how far a milestone has moved.

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
    pub milestone_pid: Uuid,
    pub from_due: Date,
    pub to_due: Date,
    pub reason: Option<String>,
    pub actor: Option<String>,
}
