//! Row — `key_result_check_ins`. **Append-only**: correcting
//! a check-in means recording another. See entity spec §5.9.2.

// The row's shape is documented by the migration.
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
    pub key_result_pid: Uuid,
    pub observed_at: DateTimeWithTimeZone,
    pub value: i64,
    /// Recorded, and **never blended into the score**.
    pub confidence: Option<i16>,
    pub note: Option<String>,
    pub actor: Option<String>,
}
