//! Row — `skill_pools`. A grouping of roles and skills that a director can read.

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
    pub active: bool,
    /// The workforce service's id for the pool that means the same skills (FB-1); none is a pool that stays local.
    pub peer_pool_ref: Option<String>,
}
