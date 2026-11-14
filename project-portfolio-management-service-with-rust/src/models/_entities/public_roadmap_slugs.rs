//! Row — `public_roadmap_slugs`: an earlier slug of a public product roadmap, kept as a redirect (RE-8).

#![expect(missing_docs)]

use super::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub id: i64,
    pub roadmap_id: i64,
    pub slug: String,
}
