//! Row — `attachment_texts`. The text extracted from an attachment so it can be found by search (PL-8). Extraction is
//! best-effort and never fails an upload: a file with no extractor is recorded `unindexed` with the reason, never
//! silently skipped. The text inherits its attachment's access: a search result is shown only to someone who may read
//! the attachment's subject.

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
    pub attachment_pid: Uuid,
    pub status: String,
    pub reason: Option<String>,
    pub body: String,
}
