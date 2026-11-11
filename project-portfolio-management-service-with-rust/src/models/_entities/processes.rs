//! Row — `processes`. A named approval process; each edit is a new version.

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
    pub name: String,
    pub current_version: i32,
    pub created_by: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
