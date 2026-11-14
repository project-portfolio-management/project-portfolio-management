//! Row — `roadmap_categories`: a category a public product roadmap is filed under, defined once (RE-7).

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
    /// The name in the deployment's own language, 1 to 100 characters, unique ignoring case.
    pub name: String,
    /// The name in other languages: an object of language code to text.
    pub translations: serde_json::Value,
    pub active: bool,
}
