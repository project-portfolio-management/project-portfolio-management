//! Row — `framework_versions`. Every version a framework has had, oldest first (append-only, enforced by
//! a database trigger). `frameworks` holds the current one; a plan's link pins the version it follows (CF-4).

#![expect(missing_docs)]

use super::{DateTimeWithTimeZone, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub id: i64,
    pub framework_pid: Uuid,
    pub version: i32,
    pub name: String,
    pub description: Option<String>,
    pub body: Json,
    pub created_by: Option<String>,
}
