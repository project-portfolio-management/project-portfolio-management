//! Row — `organization_roles`. The roles a deployment recognizes (OC-1), and the issue actions each
//! allows (OC-3). A role can only narrow what the caller's token already allows; see `crate::plan_roles`.

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
    /// The name, as a token's `role` attribute carries it.
    pub key: String,
    pub label: String,
    /// The label by language code (OC-6).
    pub labels: serde_json::Value,
    pub description: Option<String>,
    /// Permission names from `crate::plan_roles::PERMISSIONS`.
    pub permissions: serde_json::Value,
    pub position: i32,
    pub retired_at: Option<DateTimeWithTimeZone>,
}
