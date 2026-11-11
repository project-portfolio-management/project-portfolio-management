//! Row — `custom_field_values`. The values of custom fields, one row per (subject, field key). A separate table, not
//! the plan payload: a custom value can never reach the matcher (the matchable / operational partition holds by
//! construction).

#![expect(missing_docs)]

use super::{DateTimeWithTimeZone, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub id: i64,
    pub pid: Uuid,
    pub subject_kind: String,
    pub subject_pid: Uuid,
    pub field_key: String,
    pub value: Json,
}
