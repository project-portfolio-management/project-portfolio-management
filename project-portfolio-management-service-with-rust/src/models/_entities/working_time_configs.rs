//! Row — `working_time_configs`. The declared capacity
//! basis, so a utilization denominator is never assumed. See entity
//! spec §5.9.6 / FR-35.

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
    /// `None` is the deployment-wide default.
    pub scope_ref: Option<String>,
    pub minutes_per_day: i32,
    pub working_days_per_week: i32,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}
