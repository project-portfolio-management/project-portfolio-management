//! Row — `cutover_decisions`. A cutover go/no-go decision (EX-6).

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
    pub title: String,
    pub version: i32,
    pub supersedes_pid: Option<Uuid>,
    /// `draft`, `published`, `open`, `decided` or `superseded`.
    pub status: String,
    pub published_on: Option<Date>,
    pub opened_on: Option<Date>,
    /// `go`, `no_go` or `delay_for_safety`.
    pub outcome: Option<String>,
    pub decided_on: Option<Date>,
    pub decided_by: Option<String>,
    pub decision_reason: Option<String>,
}
