//! Row — `risk_scores`. Append-only history of a risk's probability and impact, written by a trigger (OF-3).

#![expect(missing_docs)]

use super::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub id: i64,
    pub risk_pid: Uuid,
    pub probability: i32,
    pub impact: i32,
    pub score: i32,
    pub recorded_at: DateTimeWithTimeZone,
    /// An opaque reference to who set the score.
    pub actor: Option<String>,
    pub reason: Option<String>,
    /// The first score known when the history began, not the risk's past.
    pub backfilled: bool,
}
