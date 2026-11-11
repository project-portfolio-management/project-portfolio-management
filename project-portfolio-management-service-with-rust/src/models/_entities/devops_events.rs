//! Row — `devops_events`. Ingested deploy / incident /
//! recovery events: the only source the DORA-style metrics derive
//! from (nothing is inferred from data that was never ingested).

// The row's shape is documented by the migration.
#![expect(missing_docs)]

use super::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub id: i32,
    pub pid: Uuid,
    pub plan_pid: Uuid,
    pub kind: String,
    pub environment: Option<String>,
    pub version: Option<String>,
    pub reference: Option<String>,
    pub incident_pid: Option<Uuid>,
    pub caused_by_deploy_pid: Option<Uuid>,
    pub occurred_at: DateTimeWithTimeZone,
}
