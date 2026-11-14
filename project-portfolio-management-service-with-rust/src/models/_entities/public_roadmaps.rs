//! Row — `public_roadmaps`: a product plan's public product roadmap (RE-1).

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
    pub plan_pid: Uuid,
    /// The public address: lower-case letters, digits and single hyphens.
    pub slug: String,
    pub title: String,
    pub vision: Option<String>,
    pub service_description: Option<String>,
    /// A roadmap category (RE-7).
    pub category_pid: Option<Uuid>,
    pub published: bool,
    /// The date someone last said the roadmap was reviewed (RE-3); no other write changes it.
    pub reviewed_on: Option<chrono::NaiveDate>,
    /// Language code to field to `{ value, source }` (RE-6).
    pub translations: serde_json::Value,
}
