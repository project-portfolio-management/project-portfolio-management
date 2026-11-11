//! Row — `watchers`. Who follows a subject (RM-3). One row per (subject, user); unwatching deletes the row. A watcher
//! joins the comment and status-change notification targets; it grants no access of its own.

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
    pub subject_kind: String,
    pub subject_pid: Uuid,
    pub sub: String,
}
