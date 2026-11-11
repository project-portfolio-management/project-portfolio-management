//! Row — `objectives`. The org-level OKR registry
//! (PPM-5).

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
    pub description: Option<String>,
    pub period: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
