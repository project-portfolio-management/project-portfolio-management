//! Row — `public_roadmap_items`: one item of a public product roadmap (RE-1).

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
    pub roadmap_id: i64,
    /// Eight lower-case letters and digits, starting with a letter; never reused (RE-8).
    pub public_id: String,
    pub title: String,
    pub description: Option<String>,
    pub expected_outcome: Option<String>,
    pub phase: Option<String>,
    /// A list of strings, in order.
    pub tags: serde_json::Value,
    /// The organizations the work is for.
    pub organization_pids: Vec<Uuid>,
    /// A list of `{ text, level }`.
    pub details: serde_json::Value,
    /// One of `public_roadmap::PLACEMENTS`, or `None` (unplaced).
    pub placement: Option<String>,
    pub position: i32,
    /// Language code to field to `{ value, source }` (RE-6).
    pub translations: serde_json::Value,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
