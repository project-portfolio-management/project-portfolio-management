//! Row — `sprints`. Time-boxed iterations on a plan,
//! behind the honest burndown view.

// The row's shape is documented by the migration.
#![expect(missing_docs)]

use super::{Date, DateTimeWithTimeZone};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub id: i32,
    pub pid: Uuid,
    pub plan_pid: Uuid,
    pub name: String,
    pub starts_on: Date,
    pub ends_on: Date,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
