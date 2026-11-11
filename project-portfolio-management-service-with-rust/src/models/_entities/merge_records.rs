//! Row — `merge_records`. One row per record-merge: which
//! duplicate folded into which survivor, by whom, with a snapshot of the
//! transferred payload.

// The row's field-level shape is documented by the
// migration and the `merge_records` table, not by per-field rustdoc.
#![expect(missing_docs)]

use super::{DateTimeWithTimeZone, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub id: i64,
    pub main_pid: Uuid,
    pub duplicate_pid: Uuid,
    pub reason: Option<String>,
    pub actor: Option<String>,
    pub transferred: Option<Json>,
}
