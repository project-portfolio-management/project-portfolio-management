//! Row — `issue_types`. A deployment's issue types (OC-1). A type is retired, never deleted.

#![expect(missing_docs)]

use super::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub id: i32,
    pub pid: Uuid,
    /// The stable key an issue's `kind` stores.
    pub key: String,
    pub label: String,
    /// The label by language code (OC-6).
    pub labels: serde_json::Value,
    pub position: i32,
    pub retired_at: Option<DateTimeWithTimeZone>,
}
