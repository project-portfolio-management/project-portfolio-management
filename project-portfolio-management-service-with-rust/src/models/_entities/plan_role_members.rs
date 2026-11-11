//! Row — `plan_role_members`. Who holds which role on a plan (OC-3).

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
    pub plan_pid: Uuid,
    /// The person's token subject, a lowercase UUID.
    pub person_ref: String,
    /// A key in `organization_roles`.
    pub role_key: String,
    pub added_by: Option<String>,
}
