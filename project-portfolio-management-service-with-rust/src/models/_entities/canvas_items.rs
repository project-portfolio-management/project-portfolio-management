//! Row — `canvas_items` (LT-1 strategy canvases).

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
    pub canvas_pid: Uuid,
    pub box_key: String,
    pub body: String,
    pub owner_ref: Option<String>,
    pub position: i32,
    pub promoted_to_kind: Option<String>,
    pub promoted_to_pid: Option<Uuid>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
