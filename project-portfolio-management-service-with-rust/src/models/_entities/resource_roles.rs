//! Row — `resource_roles`. A person's one primary role for capacity planning (SC-7): the role whose supply they count
//! toward in the portfolio resource view. One row per person; changing it replaces the row, and the change is audited.

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
    pub person_ref: String,
    pub role: String,
}
