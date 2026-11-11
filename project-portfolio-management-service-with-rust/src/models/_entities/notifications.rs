//! Row — `notifications`. The in-app inbox written by
//! automations and by the scheduled-action sweep. In-app only: this
//! service sends no email and no push.

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
    pub recipient_ref: String,
    pub subject_kind: String,
    pub subject_pid: Uuid,
    pub kind: String,
    pub message: String,
    pub read_at: Option<DateTimeWithTimeZone>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
