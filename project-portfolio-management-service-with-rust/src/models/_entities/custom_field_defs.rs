//! Row — `custom_field_defs`. Admin-declared, typed custom fields for plans and tasks (RM-1). A definition is
//! soft-deleted, never dropped: values already stored stay readable.

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
    pub key: String,
    pub label: String,
    pub field_type: String,
    pub applies_to: String,
    pub kinds: Json,
    pub required: bool,
    pub options: Json,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
