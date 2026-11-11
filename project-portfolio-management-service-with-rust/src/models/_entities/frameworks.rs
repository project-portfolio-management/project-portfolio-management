//! Row — `frameworks`. A named, reusable description of how a team works (`ways_of_working`) or is
//! governed (`governance`), organisation-wide or owned by one plan. The family's fields are in `body`.

#![expect(missing_docs)]

use super::{DateTimeWithTimeZone, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub id: i64,
    pub pid: Uuid,
    pub family: String,
    pub name: String,
    pub description: Option<String>,
    pub body: Json,
    /// The current version; the others are in `framework_versions`.
    pub version: i32,
    pub owner_plan_pid: Option<Uuid>,
    pub created_by: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
