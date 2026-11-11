//! Row — `key_person_entries`: a single point of failure, a person or a role (EX-11).

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
    /// `person` or `role`.
    pub subject_kind: String,
    /// A person reference, or a role name.
    pub subject: String,
    pub successor_ref: Option<String>,
    /// `ready`, `partly` or `none`.
    pub readiness: String,
    pub skill_pool_pid: Option<Uuid>,
    pub note: Option<String>,
}
