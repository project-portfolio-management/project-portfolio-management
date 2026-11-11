//! Row — `controlled_documents`. A controlled document; its versions carry the text.

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
    pub title: String,
    pub plan_pid: Option<Uuid>,
    pub created_by: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
