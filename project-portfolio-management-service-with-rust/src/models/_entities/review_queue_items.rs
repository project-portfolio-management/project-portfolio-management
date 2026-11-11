//! Row — `review_queue_items`. A row held for a human decision (T-8): a keyless imported plan that the matcher scores
//! as a probable duplicate of an existing one. Nothing is created or merged automatically (spec §5.1). `provenance` is
//! `import` here.

#![expect(missing_docs)]

use super::{DateTimeWithTimeZone, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub id: i64,
    pub pid: Uuid,
    pub status: String,
    pub provenance: String,
    pub candidate: Json,
    pub matches: Json,
    pub job_pid: Option<Uuid>,
    pub source_line: i32,
    pub resolved_by: Option<String>,
    pub resolved_at: Option<DateTimeWithTimeZone>,
    pub resolution_pid: Option<Uuid>,
    pub resolution_comment: Option<String>,
}
