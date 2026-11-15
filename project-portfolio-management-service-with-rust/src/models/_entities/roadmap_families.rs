//! Row — `roadmap_families`: a product family's colour and translated name, chosen by its editors (RE-5, RE-7).

#![expect(missing_docs)]

use super::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub id: i64,
    /// The family: a plan that holds product plans.
    pub plan_pid: Uuid,
    /// `#rrggbb`, lower case.
    pub colour: String,
    /// The family's name in other languages: an object of language code to text.
    pub translations: serde_json::Value,
}
