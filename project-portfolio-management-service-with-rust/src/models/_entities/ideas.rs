//! Row — `ideas`. The pre-proposal funnel (PPM-2):
//! schema-thin, voted on, convertible into a proposal.

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
    pub title: String,
    pub pitch: Option<String>,
    pub tags: Json,
    pub votes: i32,
    pub status: String,
    pub converted_proposal_pid: Option<Uuid>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
