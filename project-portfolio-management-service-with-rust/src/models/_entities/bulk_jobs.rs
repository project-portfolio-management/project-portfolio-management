//! Row — `bulk_jobs`. One bulk import or export job (T-8, spec `bulk-import-export.md` §2). `UNIQUE (entity, kind,
//! idempotency_key)` makes a retried request return the original job. The counters always reconcile: `rows_total =
//! created + updated + unchanged + queued_for_review + failed`. A job is never deleted; only its artifacts expire.

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
    pub entity: String,
    pub kind: String,
    pub status: String,
    pub format: String,
    pub idempotency_key: Option<String>,
    pub requested_by: Option<String>,
    pub params: Json,
    pub rows_total: i64,
    pub rows_created: i64,
    pub rows_updated: i64,
    pub rows_unchanged: i64,
    pub rows_queued_for_review: i64,
    pub rows_failed: i64,
    pub rows_omitted: i64,
    pub source_artifact: Option<Uuid>,
    pub result_artifact: Option<Uuid>,
    pub error_artifact: Option<Uuid>,
    pub started_at: Option<DateTimeWithTimeZone>,
    pub finished_at: Option<DateTimeWithTimeZone>,
    pub failure_reason: Option<String>,
    pub progress: i32,
    pub cancel_requested_at: Option<DateTimeWithTimeZone>,
    pub cancelled_by: Option<String>,
}
